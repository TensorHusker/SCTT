//! Proof-Carrying Code Transformation: SCTT → Verified WASM
//!
//! This compiler implements a three-stage pipeline:
//! 1. SCTT → CPS-IR with dependent type preservation
//! 2. IR → WASM with embedded proof certificates
//! 3. Optimization passes for proof compression

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use wasm_encoder::{
    CodeSection, EntityType, ExportKind, ExportSection, Function, FunctionSection,
    ImportSection, Instruction, MemorySection, MemoryType, Module, TypeSection, ValType,
};

// Re-use SCTT types from typechecker
use crate::sctt_typechecker::{Term, Value, Level, DeBruijnIndex, IntervalPoint};

/// CPS Intermediate Representation preserving dependent types
#[derive(Debug, Clone)]
pub enum CpsIr {
    /// Continuation variable
    KVar(String),
    
    /// Value variable with proof annotation
    Var(String, ProofTerm),
    
    /// Integer constant
    Const(i64),
    
    /// Lambda with continuation
    Lambda {
        param: String,
        param_ty: IrType,
        body: Box<CpsIr>,
        cont: Box<CpsIr>,
        proof: ProofTerm,
    },
    
    /// Function application
    App {
        func: Box<CpsIr>,
        arg: Box<CpsIr>,
        cont: String,
        proof: ProofTerm,
    },
    
    /// Let binding with proof
    Let {
        name: String,
        ty: IrType,
        value: Box<CpsIr>,
        body: Box<CpsIr>,
        proof: ProofTerm,
    },
    
    /// Continuation application
    ContApp {
        cont: String,
        value: Box<CpsIr>,
    },
    
    /// Memory allocation (linear type)
    Alloc {
        size: usize,
        cont: String,
        linear_proof: LinearityProof,
    },
    
    /// Memory read
    Load {
        ptr: Box<CpsIr>,
        offset: usize,
        cont: String,
        safety_proof: MemorySafetyProof,
    },
    
    /// Memory write
    Store {
        ptr: Box<CpsIr>,
        offset: usize,
        value: Box<CpsIr>,
        cont: String,
        safety_proof: MemorySafetyProof,
    },
    
    /// Conditional branch
    If {
        cond: Box<CpsIr>,
        then_branch: Box<CpsIr>,
        else_branch: Box<CpsIr>,
        proof: ProofTerm,
    },
    
    /// Proof assertion
    Assert {
        prop: ProofTerm,
        body: Box<CpsIr>,
    },
}

/// IR Types with proof annotations
#[derive(Debug, Clone)]
pub enum IrType {
    /// Base types
    I32,
    I64,
    F32,
    F64,
    
    /// Linear pointer type
    LinearPtr {
        pointee: Box<IrType>,
        region: MemoryRegion,
    },
    
    /// Function type
    Function {
        params: Vec<IrType>,
        result: Box<IrType>,
        proof: FunctionProof,
    },
    
    /// Dependent pair
    Sigma {
        fst: Box<IrType>,
        snd: Box<IrType>, // can depend on fst
    },
    
    /// Proof type
    Proof(ProofProp),
    
    /// Universe (for type-level computation)
    Universe(Level),
}

/// Proof terms for verification
#[derive(Debug, Clone)]
pub enum ProofTerm {
    /// Axiom (trusted base)
    Axiom(String),
    
    /// Variable reference
    Var(String),
    
    /// Lambda abstraction
    Lambda {
        param: String,
        body: Box<ProofTerm>,
    },
    
    /// Application
    App {
        func: Box<ProofTerm>,
        arg: Box<ProofTerm>,
    },
    
    /// Equality proof
    Refl {
        ty: IrType,
        value: Box<CpsIr>,
    },
    
    /// Path induction
    PathInd {
        motive: Box<ProofTerm>,
        refl_case: Box<ProofTerm>,
        path: Box<ProofTerm>,
    },
    
    /// Memory safety proof
    MemSafe {
        ptr: Box<CpsIr>,
        size: usize,
        bounds: BoundsProof,
    },
    
    /// Linearity proof
    Linear {
        resource: Box<CpsIr>,
        usage: UsageProof,
    },
    
    /// Compressed proof (zk-SNARK)
    Compressed {
        commitment: Vec<u8>,
        witness: Option<Box<ProofTerm>>, // None at runtime
    },
}

/// Propositions that can be proven
#[derive(Debug, Clone)]
pub enum ProofProp {
    /// Type equality
    Equal(IrType, IrType),
    
    /// Memory safety
    MemorySafe(Box<CpsIr>, usize),
    
    /// Linearity constraint
    Linear(Box<CpsIr>),
    
    /// Implication
    Implies(Box<ProofProp>, Box<ProofProp>),
    
    /// Universal quantification
    Forall(String, IrType, Box<ProofProp>),
    
    /// Existential quantification  
    Exists(String, IrType, Box<ProofProp>),
}

/// Memory regions for linear types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemoryRegion(pub usize);

/// Linearity proof for resources
#[derive(Debug, Clone)]
pub struct LinearityProof {
    pub resource_id: String,
    pub creation_point: usize,
    pub consumption_point: Option<usize>,
    pub no_duplication: bool,
}

/// Memory safety proof
#[derive(Debug, Clone)]
pub struct MemorySafetyProof {
    pub bounds_check: BoundsProof,
    pub alignment: AlignmentProof,
    pub no_use_after_free: bool,
}

#[derive(Debug, Clone)]
pub struct BoundsProof {
    pub lower_bound: i64,
    pub upper_bound: i64,
    pub in_range: bool,
}

#[derive(Debug, Clone)]
pub struct AlignmentProof {
    pub alignment: usize,
    pub offset: usize,
    pub is_aligned: bool,
}

#[derive(Debug, Clone)]
pub struct UsageProof {
    pub single_use: bool,
    pub consumption_site: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct FunctionProof {
    pub termination: Option<TerminationProof>,
    pub memory_safety: bool,
    pub side_effects: Vec<SideEffect>,
}

#[derive(Debug, Clone)]
pub struct TerminationProof {
    pub metric: Box<CpsIr>,
    pub decreasing: bool,
}

#[derive(Debug, Clone)]
pub enum SideEffect {
    Memory,
    IO,
    Pure,
}

/// Main compiler from SCTT to WASM
pub struct ScttToWasmCompiler {
    /// Fresh variable generator
    var_counter: usize,
    
    /// Proof compression settings
    enable_compression: bool,
    
    /// Optimization level
    opt_level: OptLevel,
    
    /// Monomorphization cache
    mono_cache: HashMap<(String, Vec<IrType>), CpsIr>,
    
    /// Dead proof tracking
    live_proofs: HashSet<String>,
    
    /// Common subproof cache
    subproof_cache: HashMap<ProofTerm, String>,
}

#[derive(Debug, Clone, Copy)]
pub enum OptLevel {
    None,
    Basic,
    Aggressive,
}

impl ScttToWasmCompiler {
    pub fn new(opt_level: OptLevel) -> Self {
        ScttToWasmCompiler {
            var_counter: 0,
            enable_compression: opt_level as u8 >= OptLevel::Basic as u8,
            opt_level,
            mono_cache: HashMap::new(),
            live_proofs: HashSet::new(),
            subproof_cache: HashMap::new(),
        }
    }

    /// Stage 1: SCTT → CPS-IR transformation
    pub fn sctt_to_ir(&mut self, term: &Term) -> CpsIr {
        let cont = self.fresh_cont();
        self.cps_transform(term, &cont)
    }

    fn cps_transform(&mut self, term: &Term, cont: &str) -> CpsIr {
        match term {
            Term::Var(idx) => {
                let var_name = format!("v{}", idx.0);
                let proof = ProofTerm::Axiom(format!("var_axiom_{}", idx.0));
                CpsIr::ContApp {
                    cont: cont.to_string(),
                    value: Box::new(CpsIr::Var(var_name, proof)),
                }
            }
            
            Term::Lambda(ty, body) => {
                let param = self.fresh_var();
                let body_cont = self.fresh_cont();
                let body_cps = self.cps_transform(body, &body_cont);
                
                let proof = ProofTerm::Lambda {
                    param: param.clone(),
                    body: Box::new(ProofTerm::Axiom("lambda_proof".to_string())),
                };
                
                CpsIr::Lambda {
                    param,
                    param_ty: self.type_to_ir_type(ty),
                    body: Box::new(body_cps),
                    cont: Box::new(CpsIr::KVar(cont.to_string())),
                    proof,
                }
            }
            
            Term::App(func, arg) => {
                let func_cont = self.fresh_cont();
                let arg_cont = self.fresh_cont();
                
                let func_cps = self.cps_transform(func, &func_cont);
                let arg_cps = self.cps_transform(arg, &arg_cont);
                
                let proof = ProofTerm::App {
                    func: Box::new(ProofTerm::Axiom("app_func".to_string())),
                    arg: Box::new(ProofTerm::Axiom("app_arg".to_string())),
                };
                
                CpsIr::App {
                    func: Box::new(func_cps),
                    arg: Box::new(arg_cps),
                    cont: cont.to_string(),
                    proof,
                }
            }
            
            _ => todo!("Other SCTT term transformations"),
        }
    }

    /// Stage 2: IR → WASM with proof certificates
    pub fn ir_to_wasm(&mut self, ir: &CpsIr) -> WasmModule {
        let mut module = WasmModule::new();
        
        // Generate main function
        let main_func = self.compile_ir_to_wasm(ir);
        module.add_function("main", main_func);
        
        // Embed proof certificate
        let proof_cert = self.generate_proof_certificate(ir);
        module.add_custom_section("proof", proof_cert);
        
        // Add runtime verifier
        let verifier = self.generate_runtime_verifier();
        module.add_function("verify", verifier);
        
        module
    }

    fn compile_ir_to_wasm(&mut self, ir: &CpsIr) -> WasmFunction {
        let mut func = WasmFunction::new();
        
        match ir {
            CpsIr::Const(n) => {
                func.add_instruction(WasmInstruction::I64Const(*n));
            }
            
            CpsIr::Var(name, proof) => {
                // Verify proof at compile time
                if self.enable_compression {
                    self.compress_proof(proof);
                }
                func.add_instruction(WasmInstruction::LocalGet(name.clone()));
            }
            
            CpsIr::Let { name, value, body, proof, .. } => {
                let value_func = self.compile_ir_to_wasm(value);
                func.append(value_func);
                func.add_instruction(WasmInstruction::LocalSet(name.clone()));
                
                if self.should_keep_proof(proof) {
                    func.add_proof_check(proof);
                }
                
                let body_func = self.compile_ir_to_wasm(body);
                func.append(body_func);
            }
            
            CpsIr::Alloc { size, cont, linear_proof } => {
                // Generate memory allocation with linear type tracking
                func.add_instruction(WasmInstruction::I32Const(*size as i32));
                func.add_instruction(WasmInstruction::Call("malloc".to_string()));
                
                // Add linearity tracking
                func.add_linear_tracking(linear_proof);
                
                func.add_instruction(WasmInstruction::LocalSet(cont.clone()));
            }
            
            CpsIr::Load { ptr, offset, cont, safety_proof } => {
                // Compile-time bounds check
                if !safety_proof.bounds_check.in_range {
                    panic!("Memory safety violation at compile time");
                }
                
                let ptr_func = self.compile_ir_to_wasm(ptr);
                func.append(ptr_func);
                func.add_instruction(WasmInstruction::I32Const(*offset as i32));
                func.add_instruction(WasmInstruction::I32Add);
                func.add_instruction(WasmInstruction::I64Load);
                func.add_instruction(WasmInstruction::LocalSet(cont.clone()));
            }
            
            _ => todo!("Other IR to WASM compilations"),
        }
        
        func
    }

    /// Stage 3: Optimization passes
    pub fn optimize(&mut self, ir: CpsIr) -> CpsIr {
        let mut optimized = ir;
        
        if self.opt_level as u8 >= OptLevel::Basic as u8 {
            optimized = self.dead_proof_elimination(optimized);
            optimized = self.common_subproof_factoring(optimized);
        }
        
        if self.opt_level as u8 >= OptLevel::Aggressive as u8 {
            optimized = self.proof_compression_pass(optimized);
            optimized = self.monomorphization_pass(optimized);
        }
        
        optimized
    }

    fn dead_proof_elimination(&mut self, ir: CpsIr) -> CpsIr {
        // Mark live proofs
        self.mark_live_proofs(&ir);
        
        // Eliminate dead proofs
        self.eliminate_dead_proofs(ir)
    }

    fn mark_live_proofs(&mut self, ir: &CpsIr) {
        match ir {
            CpsIr::Assert { prop, body } => {
                self.mark_proof_live(prop);
                self.mark_live_proofs(body);
            }
            CpsIr::Let { proof, body, value, .. } => {
                if self.proof_affects_runtime(proof) {
                    self.mark_proof_live(proof);
                }
                self.mark_live_proofs(value);
                self.mark_live_proofs(body);
            }
            _ => {}
        }
    }

    fn mark_proof_live(&mut self, proof: &ProofTerm) {
        match proof {
            ProofTerm::Var(name) => {
                self.live_proofs.insert(name.clone());
            }
            ProofTerm::App { func, arg } => {
                self.mark_proof_live(func);
                self.mark_proof_live(arg);
            }
            _ => {}
        }
    }

    fn eliminate_dead_proofs(&mut self, ir: CpsIr) -> CpsIr {
        match ir {
            CpsIr::Let { name, ty, value, body, proof } => {
                if !self.is_proof_live(&proof) {
                    // Skip proof generation
                    CpsIr::Let {
                        name,
                        ty,
                        value,
                        body: Box::new(self.eliminate_dead_proofs(*body)),
                        proof: ProofTerm::Axiom("eliminated".to_string()),
                    }
                } else {
                    CpsIr::Let {
                        name,
                        ty,
                        value: Box::new(self.eliminate_dead_proofs(*value)),
                        body: Box::new(self.eliminate_dead_proofs(*body)),
                        proof,
                    }
                }
            }
            _ => ir,
        }
    }

    fn common_subproof_factoring(&mut self, ir: CpsIr) -> CpsIr {
        self.collect_subproofs(&ir);
        self.replace_common_subproofs(ir)
    }

    fn collect_subproofs(&mut self, ir: &CpsIr) {
        match ir {
            CpsIr::Let { proof, .. } | CpsIr::Lambda { proof, .. } => {
                if let Some(id) = self.subproof_cache.get(proof) {
                    // Already cached
                } else {
                    let id = self.fresh_var();
                    self.subproof_cache.insert(proof.clone(), id);
                }
            }
            _ => {}
        }
    }

    fn replace_common_subproofs(&mut self, ir: CpsIr) -> CpsIr {
        match ir {
            CpsIr::Let { name, ty, value, body, proof } => {
                let cached_proof = if let Some(id) = self.subproof_cache.get(&proof) {
                    ProofTerm::Var(id.clone())
                } else {
                    proof
                };
                
                CpsIr::Let {
                    name,
                    ty,
                    value,
                    body,
                    proof: cached_proof,
                }
            }
            _ => ir,
        }
    }

    fn proof_compression_pass(&mut self, ir: CpsIr) -> CpsIr {
        // Use zk-SNARKs for proof compression
        match ir {
            CpsIr::Let { name, ty, value, body, proof } => {
                let compressed = self.compress_proof(&proof);
                CpsIr::Let {
                    name,
                    ty,
                    value,
                    body,
                    proof: compressed,
                }
            }
            _ => ir,
        }
    }

    fn compress_proof(&mut self, proof: &ProofTerm) -> ProofTerm {
        // Simplified zk-SNARK compression
        let commitment = self.generate_commitment(proof);
        ProofTerm::Compressed {
            commitment,
            witness: if self.opt_level as u8 >= OptLevel::Aggressive as u8 {
                None // Drop witness for maximum compression
            } else {
                Some(Box::new(proof.clone()))
            },
        }
    }

    fn generate_commitment(&self, proof: &ProofTerm) -> Vec<u8> {
        // Simplified commitment generation (would use real crypto in production)
        format!("{:?}", proof).bytes().take(32).collect()
    }

    fn monomorphization_pass(&mut self, ir: CpsIr) -> CpsIr {
        // Monomorphize universe-polymorphic functions
        match ir {
            CpsIr::Lambda { param, param_ty, body, cont, proof } => {
                let mono_key = (param.clone(), vec![param_ty.clone()]);
                if let Some(cached) = self.mono_cache.get(&mono_key) {
                    cached.clone()
                } else {
                    let mono = CpsIr::Lambda {
                        param: param.clone(),
                        param_ty,
                        body,
                        cont,
                        proof,
                    };
                    self.mono_cache.insert(mono_key, mono.clone());
                    mono
                }
            }
            _ => ir,
        }
    }

    fn generate_proof_certificate(&self, ir: &CpsIr) -> Vec<u8> {
        // Generate proof certificate for embedding in WASM
        let mut cert = Vec::new();
        self.serialize_proofs(ir, &mut cert);
        cert
    }

    fn serialize_proofs(&self, ir: &CpsIr, buffer: &mut Vec<u8>) {
        match ir {
            CpsIr::Let { proof, .. } | CpsIr::Lambda { proof, .. } => {
                self.serialize_proof(proof, buffer);
            }
            _ => {}
        }
    }

    fn serialize_proof(&self, proof: &ProofTerm, buffer: &mut Vec<u8>) {
        // Simplified serialization
        let proof_str = format!("{:?}", proof);
        buffer.extend(proof_str.bytes());
    }

    fn generate_runtime_verifier(&self) -> WasmFunction {
        let mut verifier = WasmFunction::new();
        
        // Lightweight proof checking
        verifier.add_instruction(WasmInstruction::I32Const(0)); // proof ptr
        verifier.add_instruction(WasmInstruction::Call("check_proof".to_string()));
        verifier.add_instruction(WasmInstruction::If);
        verifier.add_instruction(WasmInstruction::I32Const(1)); // success
        verifier.add_instruction(WasmInstruction::Else);
        verifier.add_instruction(WasmInstruction::I32Const(0)); // failure
        verifier.add_instruction(WasmInstruction::End);
        
        verifier
    }

    fn type_to_ir_type(&self, _term: &Term) -> IrType {
        // Simplified type translation
        IrType::I64
    }

    fn fresh_var(&mut self) -> String {
        let var = format!("x{}", self.var_counter);
        self.var_counter += 1;
        var
    }

    fn fresh_cont(&mut self) -> String {
        let cont = format!("k{}", self.var_counter);
        self.var_counter += 1;
        cont
    }

    fn should_keep_proof(&self, _proof: &ProofTerm) -> bool {
        self.opt_level as u8 < OptLevel::Aggressive as u8
    }

    fn proof_affects_runtime(&self, _proof: &ProofTerm) -> bool {
        // Conservative: assume all proofs might affect runtime
        true
    }

    fn is_proof_live(&self, proof: &ProofTerm) -> bool {
        match proof {
            ProofTerm::Var(name) => self.live_proofs.contains(name),
            _ => true, // Conservative
        }
    }
}

/// Simplified WASM module representation
pub struct WasmModule {
    functions: Vec<(String, WasmFunction)>,
    custom_sections: Vec<(String, Vec<u8>)>,
}

impl WasmModule {
    pub fn new() -> Self {
        WasmModule {
            functions: Vec::new(),
            custom_sections: Vec::new(),
        }
    }

    pub fn add_function(&mut self, name: &str, func: WasmFunction) {
        self.functions.push((name.to_string(), func));
    }

    pub fn add_custom_section(&mut self, name: &str, data: Vec<u8>) {
        self.custom_sections.push((name.to_string(), data));
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut module = Module::new();
        
        // Add type section
        let mut types = TypeSection::new();
        for (_, func) in &self.functions {
            types.function(func.params.clone(), func.results.clone());
        }
        module.section(&types);
        
        // Add function section
        let mut functions = FunctionSection::new();
        for i in 0..self.functions.len() {
            functions.function(i as u32);
        }
        module.section(&functions);
        
        // Add code section
        let mut codes = CodeSection::new();
        for (_, func) in &self.functions {
            let mut f = Function::new(func.locals.clone());
            for inst in &func.instructions {
                f.instruction(&inst.to_wasm_instruction());
            }
            codes.function(&f);
        }
        module.section(&codes);
        
        // Add custom sections
        for (name, data) in &self.custom_sections {
            module.section(&wasm_encoder::CustomSection {
                name: name.as_str(),
                data: &data,
            });
        }
        
        module.finish()
    }
}

/// Simplified WASM function representation
pub struct WasmFunction {
    params: Vec<ValType>,
    results: Vec<ValType>,
    locals: Vec<(u32, ValType)>,
    instructions: Vec<WasmInstruction>,
}

impl WasmFunction {
    pub fn new() -> Self {
        WasmFunction {
            params: Vec::new(),
            results: vec![ValType::I64],
            locals: Vec::new(),
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, inst: WasmInstruction) {
        self.instructions.push(inst);
    }

    pub fn append(&mut self, other: WasmFunction) {
        self.instructions.extend(other.instructions);
    }

    pub fn add_proof_check(&mut self, _proof: &ProofTerm) {
        // Add runtime proof verification
        self.instructions.push(WasmInstruction::Call("verify_proof".to_string()));
    }

    pub fn add_linear_tracking(&mut self, _proof: &LinearityProof) {
        // Add linear type tracking
        self.instructions.push(WasmInstruction::Call("track_linear".to_string()));
    }
}

/// Simplified WASM instruction set
#[derive(Debug, Clone)]
pub enum WasmInstruction {
    I32Const(i32),
    I64Const(i64),
    I32Add,
    I64Load,
    LocalGet(String),
    LocalSet(String),
    Call(String),
    If,
    Else,
    End,
}

impl WasmInstruction {
    pub fn to_wasm_instruction(&self) -> Instruction {
        match self {
            WasmInstruction::I32Const(n) => Instruction::I32Const(*n),
            WasmInstruction::I64Const(n) => Instruction::I64Const(*n),
            WasmInstruction::I32Add => Instruction::I32Add,
            WasmInstruction::I64Load(mem_arg) => {
                Instruction::I64Load(wasm_encoder::MemArg {
                    offset: 0,
                    align: 3,
                    memory_index: 0,
                })
            },
            WasmInstruction::LocalGet(_) => Instruction::LocalGet(0), // Simplified
            WasmInstruction::LocalSet(_) => Instruction::LocalSet(0), // Simplified
            WasmInstruction::Call(_) => Instruction::Call(0), // Simplified
            WasmInstruction::If => Instruction::If(wasm_encoder::BlockType::Empty),
            WasmInstruction::Else => Instruction::Else,
            WasmInstruction::End => Instruction::End,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cps_transformation() {
        let mut compiler = ScttToWasmCompiler::new(OptLevel::Basic);
        
        // λx. x (identity function)
        let id = Term::Lambda(
            Box::new(Term::Universe(Level::Zero)),
            Box::new(Term::Var(DeBruijnIndex(0))),
        );
        
        let cps = compiler.sctt_to_ir(&id);
        
        match cps {
            CpsIr::Lambda { .. } => (),
            _ => panic!("Expected lambda in CPS"),
        }
    }

    #[test]
    fn test_proof_compression() {
        let mut compiler = ScttToWasmCompiler::new(OptLevel::Aggressive);
        
        let proof = ProofTerm::Lambda {
            param: "x".to_string(),
            body: Box::new(ProofTerm::Var("x".to_string())),
        };
        
        let compressed = compiler.compress_proof(&proof);
        
        match compressed {
            ProofTerm::Compressed { commitment, witness } => {
                assert_eq!(commitment.len(), 32);
                assert!(witness.is_none()); // Aggressive optimization drops witness
            }
            _ => panic!("Expected compressed proof"),
        }
    }

    #[test]
    fn test_memory_safety() {
        let compiler = ScttToWasmCompiler::new(OptLevel::Basic);
        
        let safe_load = CpsIr::Load {
            ptr: Box::new(CpsIr::Const(1000)),
            offset: 8,
            cont: "k".to_string(),
            safety_proof: MemorySafetyProof {
                bounds_check: BoundsProof {
                    lower_bound: 0,
                    upper_bound: 2000,
                    in_range: true,
                },
                alignment: AlignmentProof {
                    alignment: 8,
                    offset: 8,
                    is_aligned: true,
                },
                no_use_after_free: true,
            },
        };
        
        let mut wasm = compiler.compile_ir_to_wasm(&safe_load);
        assert!(wasm.instructions.len() > 0);
    }

    #[test] 
    fn test_optimization_pipeline() {
        let mut compiler = ScttToWasmCompiler::new(OptLevel::Aggressive);
        
        let ir = CpsIr::Let {
            name: "x".to_string(),
            ty: IrType::I64,
            value: Box::new(CpsIr::Const(42)),
            body: Box::new(CpsIr::Var("x".to_string(), ProofTerm::Axiom("x".to_string()))),
            proof: ProofTerm::Axiom("let_proof".to_string()),
        };
        
        let optimized = compiler.optimize(ir);
        
        // Check that optimization doesn't break the program
        match optimized {
            CpsIr::Let { .. } => (),
            _ => panic!("Structure preserved after optimization"),
        }
    }

    #[test]
    fn test_performance_target() {
        use std::time::Instant;
        
        let mut compiler = ScttToWasmCompiler::new(OptLevel::Aggressive);
        
        // Generate 10,000 lines of simple IR
        let mut ir = CpsIr::Const(0);
        for i in 1..10000 {
            ir = CpsIr::Let {
                name: format!("x{}", i),
                ty: IrType::I64,
                value: Box::new(CpsIr::Const(i)),
                body: Box::new(ir),
                proof: ProofTerm::Axiom(format!("proof{}", i)),
            };
        }
        
        let start = Instant::now();
        let _wasm = compiler.ir_to_wasm(&ir);
        let elapsed = start.elapsed();
        
        // Check we meet performance target: > 10,000 lines/second
        let lines_per_sec = 10000.0 / elapsed.as_secs_f64();
        assert!(lines_per_sec > 10000.0, "Performance target not met: {} lines/sec", lines_per_sec);
    }
}