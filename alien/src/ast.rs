/// Abstract Syntax Tree for Alien language
/// Represents parsed program structure with SCTT features

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub modules: Vec<Module>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub name: String,
    pub params: Vec<Parameter>,
    pub imports: Vec<Import>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub module_path: Vec<String>,
    pub alias: Option<String>,
    pub exposing: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Function(FunctionDecl),
    Data(DataDecl),
    Type(TypeDecl),
    Value(ValueDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub visibility: Visibility,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub body: Expression,
    pub is_smooth: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataDecl {
    pub name: String,
    pub params: Vec<String>,
    pub constructors: Vec<Constructor>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Constructor {
    pub name: String,
    pub fields: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDecl {
    pub name: String,
    pub params: Vec<String>,
    pub definition: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValueDecl {
    pub name: String,
    pub ty: Type,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    // Base types
    Universe,                           // Type
    Smooth,                            // Smooth
    Real,                              // ℝ
    Natural,                           // ℕ
    Boolean,                           // 𝔹
    Complex,                           // ℂ
    String,
    Unit,
    
    // Type variables
    Variable(String),
    
    // Function types
    Function(Box<Type>, Box<Type>),                    // A → B
    SmoothFunction(Box<Type>, Box<Type>),              // A ⇒ B
    DependentFunction(String, Box<Type>, Box<Type>),   // (x : A) → B(x)
    
    // Product types
    Product(Box<Type>, Box<Type>),                     // A × B
    DependentProduct(String, Box<Type>, Box<Type>),    // (x : A) × B(x)
    
    // Path types
    Path(Box<Type>, Box<Expression>, Box<Expression>), // Path A x y
    
    // Dimension type
    Dimension,
    
    // Application
    Application(Box<Type>, Box<Type>),
    
    // Higher inductive types
    Circle,                            // S¹
    Torus,
    Suspension(Box<Type>),
    
    // Manifold types
    Manifold(Box<Type>),
    TangentBundle(Box<Type>),
    DifferentialForm(usize, Box<Type>),
    
    // Quantum types
    Qubit,
    QuantumState(usize),               // n-qubit state
    
    // List and vector types
    List(Box<Type>),
    Vector(Box<Type>, Box<Expression>), // Vec A n
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    // Variables and literals
    Variable(String),
    Number(f64),
    StringLit(String),
    BoolLit(bool),
    Unit,
    
    // Function expressions
    Lambda(Vec<Parameter>, Box<Expression>),
    Application(Box<Expression>, Box<Expression>),
    
    // Let binding
    Let(String, Option<Type>, Box<Expression>, Box<Expression>),
    
    // Conditionals
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    
    // Pattern matching
    Match(Box<Expression>, Vec<(Pattern, Expression)>),
    
    // Pairs
    Pair(Box<Expression>, Box<Expression>),
    First(Box<Expression>),
    Second(Box<Expression>),
    
    // Paths
    PathLambda(String, Box<Expression>),              // <i> expr
    PathApplication(Box<Expression>, DimExpr),        // path @ r
    PathComposition(Box<Expression>, Box<Expression>), // p ∙ q
    Refl(Box<Expression>),                           // reflexivity
    
    // Cubical operations
    Comp {
        ty: Box<Type>,
        faces: Vec<(Face, Expression)>,
        base: Box<Expression>,
    },
    Coe {
        ty: Box<Type>,
        from: DimExpr,
        to: DimExpr,
        expr: Box<Expression>,
    },
    HCom {
        ty: Box<Type>,
        faces: Vec<(Face, Expression)>,
        base: Box<Expression>,
    },
    
    // Smooth operations
    Derivative(Box<Expression>),                      // ∂ f
    NthDerivative(Box<Expression>, usize),            // ∂ⁿ f
    Gradient(Box<Expression>),                        // ∇ f
    Integral {
        integrand: Box<Expression>,
        variable: String,
        from: Box<Expression>,
        to: Box<Expression>,
    },
    PathIntegral(Box<Expression>, Box<Expression>),   // ∮ f along path
    Taylor {
        function: Box<Expression>,
        center: Box<Expression>,
        order: usize,
    },
    SmoothPath {
        from: Box<Expression>,
        to: Box<Expression>,
        smoothness: f64,
    },
    
    // Data constructors
    Constructor(String, Vec<Expression>),
    
    // List operations
    ListLit(Vec<Expression>),
    ListCons(Box<Expression>, Box<Expression>),
    
    // Type annotations
    Annotated(Box<Expression>, Box<Type>),
    
    // Proof tactics
    Auto,
    Hole(Option<String>),
    
    // Monadic operations
    Do(Vec<DoStatement>),
    
    // Binary operations
    BinOp(BinaryOp, Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Variable(String),
    Constructor(String, Vec<Pattern>),
    Literal(Literal),
    Pair(Box<Pattern>, Box<Pattern>),
    PathPattern(String, Box<Pattern>),  // <i> pattern
    Wildcard,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DimExpr {
    Zero,                              // 0
    One,                               // 1
    Variable(String),                  // i, j, k
    Meet(Box<DimExpr>, Box<DimExpr>), // i ∧ j
    Join(Box<DimExpr>, Box<DimExpr>), // i ∨ j
    Neg(Box<DimExpr>),                // ~i
}

#[derive(Debug, Clone, PartialEq)]
pub enum Face {
    Eq(DimExpr, DimExpr),              // i = j
    And(Box<Face>, Box<Face>),         // φ ∧ ψ
    Or(Box<Face>, Box<Face>),          // φ ∨ ψ
    True,
    False,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DoStatement {
    Bind(String, Expression),          // x <- expr
    Let(String, Expression),           // let x = expr
    Expression(Expression),            // expr
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    
    // Comparison
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    
    // Logical
    And,
    Or,
    
    // Path operations
    PathConcat,                        // ∙
    
    // Type operations
    TypeProduct,                       // ×
    TypeSum,                          // +
}

// Helper functions for AST construction
impl Expression {
    pub fn lambda(params: Vec<Parameter>, body: Expression) -> Self {
        Expression::Lambda(params, Box::new(body))
    }
    
    pub fn app(fun: Expression, arg: Expression) -> Self {
        Expression::Application(Box::new(fun), Box::new(arg))
    }
    
    pub fn path_lambda(var: String, body: Expression) -> Self {
        Expression::PathLambda(var, Box::new(body))
    }
    
    pub fn let_expr(name: String, ty: Option<Type>, value: Expression, body: Expression) -> Self {
        Expression::Let(name, ty, Box::new(value), Box::new(body))
    }
}

impl Type {
    pub fn function(from: Type, to: Type) -> Self {
        Type::Function(Box::new(from), Box::new(to))
    }
    
    pub fn smooth_function(from: Type, to: Type) -> Self {
        Type::SmoothFunction(Box::new(from), Box::new(to))
    }
    
    pub fn product(left: Type, right: Type) -> Self {
        Type::Product(Box::new(left), Box::new(right))
    }
    
    pub fn path(ty: Type, from: Expression, to: Expression) -> Self {
        Type::Path(Box::new(ty), Box::new(from), Box::new(to))
    }
}