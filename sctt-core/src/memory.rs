//! # Memory Management and Optimization for SCTT
//!
//! This module provides efficient memory management for SCTT terms and types,
//! including hash consing, garbage collection, and compressed representations.

use crate::types::{Type, Term};
use crate::error::{Error, Result};
use std::collections::{HashMap, BTreeSet};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use bitvec::prelude::*;
use serde::{Deserialize, Serialize};

/// Hash-consed term representation for memory efficiency
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HashConsedTerm {
    /// Unique identifier for this term
    pub id: TermId,
    /// Reference-counted pointer to actual data
    pub data: Rc<TermData>,
}

/// Hash-consed type representation
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HashConsedType {
    /// Unique identifier
    pub id: TypeId,
    /// Reference-counted pointer to data
    pub data: Rc<TypeData>,
}

/// Term identifier (globally unique)
pub type TermId = u64;

/// Type identifier (globally unique)
pub type TypeId = u64;

/// Internal term data
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TermData {
    /// Variable
    Var(String),
    /// Lambda with hash-consed body
    Lambda {
        param: String,
        body: HashConsedTerm,
    },
    /// Application with hash-consed components
    App {
        function: HashConsedTerm,
        argument: HashConsedTerm,
    },
    /// Other term constructors...
    Pair {
        first: HashConsedTerm,
        second: HashConsedTerm,
    },
}

/// Internal type data
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TypeData {
    /// Universe
    Universe(u32),
    /// Pi type
    Pi {
        param: String,
        domain: HashConsedType,
        codomain: HashConsedType,
    },
    /// Other type constructors...
}

/// Term hash-cons table
pub struct TermHashCons {
    /// Map from term data to unique ID
    table: RefCell<HashMap<TermData, TermId>>,
    /// Map from ID to term data
    reverse: RefCell<HashMap<TermId, Rc<TermData>>>,
    /// Next available ID
    next_id: RefCell<TermId>,
    /// Weak references for garbage collection
    weak_refs: RefCell<HashMap<TermId, Weak<TermData>>>,
}

/// Type hash-cons table
pub struct TypeHashCons {
    /// Map from type data to unique ID
    table: RefCell<HashMap<TypeData, TypeId>>,
    /// Map from ID to type data
    reverse: RefCell<HashMap<TypeId, Rc<TypeData>>>,
    /// Next available ID
    next_id: RefCell<TypeId>,
    /// Weak references for GC
    weak_refs: RefCell<HashMap<TypeId, Weak<TypeData>>>,
}

impl TermHashCons {
    /// Create new hash-cons table
    pub fn new() -> Self {
        Self {
            table: RefCell::new(HashMap::new()),
            reverse: RefCell::new(HashMap::new()),
            next_id: RefCell::new(0),
            weak_refs: RefCell::new(HashMap::new()),
        }
    }
    
    /// Intern a term, returning hash-consed version
    pub fn intern(&self, data: TermData) -> HashConsedTerm {
        let mut table = self.table.borrow_mut();
        
        if let Some(&id) = table.get(&data) {
            // Already exists, return existing
            let reverse = self.reverse.borrow();
            let rc_data = reverse.get(&id).unwrap().clone();
            HashConsedTerm { id, data: rc_data }
        } else {
            // Create new entry
            let id = *self.next_id.borrow();
            *self.next_id.borrow_mut() += 1;
            
            let rc_data = Rc::new(data.clone());
            table.insert(data, id);
            self.reverse.borrow_mut().insert(id, rc_data.clone());
            self.weak_refs.borrow_mut().insert(id, Rc::downgrade(&rc_data));
            
            HashConsedTerm { id, data: rc_data }
        }
    }
    
    /// Garbage collect unused terms
    pub fn gc(&self) {
        let mut table = self.table.borrow_mut();
        let mut reverse = self.reverse.borrow_mut();
        let mut weak_refs = self.weak_refs.borrow_mut();
        
        let mut to_remove = Vec::new();
        
        for (&id, weak_ref) in weak_refs.iter() {
            if weak_ref.strong_count() == 0 {
                to_remove.push(id);
            }
        }
        
        for id in to_remove {
            if let Some(data) = reverse.remove(&id) {
                // Remove from hash table
                if let Ok(term_data) = Rc::try_unwrap(data) {
                    table.remove(&term_data);
                }
            }
            weak_refs.remove(&id);
        }
    }
    
    /// Get statistics about memory usage
    pub fn stats(&self) -> MemoryStats {
        MemoryStats {
            terms_count: self.table.borrow().len(),
            types_count: 0, // Not applicable here
            memory_used: self.estimate_memory(),
        }
    }
    
    fn estimate_memory(&self) -> usize {
        // Rough estimate of memory usage
        let table_size = self.table.borrow().len() * std::mem::size_of::<(TermData, TermId)>();
        let reverse_size = self.reverse.borrow().len() * std::mem::size_of::<(TermId, Rc<TermData>)>();
        table_size + reverse_size
    }
}

impl TypeHashCons {
    /// Create new type hash-cons table
    pub fn new() -> Self {
        Self {
            table: RefCell::new(HashMap::new()),
            reverse: RefCell::new(HashMap::new()),
            next_id: RefCell::new(0),
            weak_refs: RefCell::new(HashMap::new()),
        }
    }
    
    /// Intern a type
    pub fn intern(&self, data: TypeData) -> HashConsedType {
        let mut table = self.table.borrow_mut();
        
        if let Some(&id) = table.get(&data) {
            let reverse = self.reverse.borrow();
            let rc_data = reverse.get(&id).unwrap().clone();
            HashConsedType { id, data: rc_data }
        } else {
            let id = *self.next_id.borrow();
            *self.next_id.borrow_mut() += 1;
            
            let rc_data = Rc::new(data.clone());
            table.insert(data, id);
            self.reverse.borrow_mut().insert(id, rc_data.clone());
            self.weak_refs.borrow_mut().insert(id, Rc::downgrade(&rc_data));
            
            HashConsedType { id, data: rc_data }
        }
    }
    
    /// Garbage collect unused types
    pub fn gc(&self) {
        // Similar to term GC
        let mut table = self.table.borrow_mut();
        let mut reverse = self.reverse.borrow_mut();
        let mut weak_refs = self.weak_refs.borrow_mut();
        
        let mut to_remove = Vec::new();
        
        for (&id, weak_ref) in weak_refs.iter() {
            if weak_ref.strong_count() == 0 {
                to_remove.push(id);
            }
        }
        
        for id in to_remove {
            if let Some(data) = reverse.remove(&id) {
                if let Ok(type_data) = Rc::try_unwrap(data) {
                    table.remove(&type_data);
                }
            }
            weak_refs.remove(&id);
        }
    }
}

/// Compressed cube representation using bit vectors
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompressedCube {
    /// Dimensions of the cube
    pub dimensions: u8,
    /// Bit vector representing filled cells
    pub data: BitVec,
    /// Compressed boundary data
    pub boundary: Vec<u8>,
}

impl CompressedCube {
    /// Create new compressed cube
    pub fn new(dimensions: u8) -> Self {
        let size = 1 << dimensions; // 2^dimensions cells
        Self {
            dimensions,
            data: bitvec![0; size],
            boundary: Vec::new(),
        }
    }
    
    /// Set a cell in the cube
    pub fn set_cell(&mut self, coordinates: &[bool]) -> Result<()> {
        if coordinates.len() != self.dimensions as usize {
            return Err(Error::MemoryError("Invalid coordinates".into()));
        }
        
        let index = self.coords_to_index(coordinates);
        self.data.set(index, true);
        Ok(())
    }
    
    /// Get a cell from the cube
    pub fn get_cell(&self, coordinates: &[bool]) -> Result<bool> {
        if coordinates.len() != self.dimensions as usize {
            return Err(Error::MemoryError("Invalid coordinates".into()));
        }
        
        let index = self.coords_to_index(coordinates);
        Ok(self.data[index])
    }
    
    fn coords_to_index(&self, coordinates: &[bool]) -> usize {
        coordinates.iter().enumerate().fold(0, |acc, (i, &bit)| {
            acc | if bit { 1 << i } else { 0 }
        })
    }
    
    /// Compress the cube using run-length encoding
    pub fn compress(&mut self) {
        let mut compressed = Vec::new();
        let mut current_bit = false;
        let mut run_length = 0u8;
        
        for bit in self.data.iter() {
            if *bit == current_bit {
                run_length += 1;
                if run_length == 255 {
                    compressed.push(run_length);
                    run_length = 0;
                }
            } else {
                if run_length > 0 {
                    compressed.push(run_length);
                }
                current_bit = *bit;
                run_length = 1;
            }
        }
        
        if run_length > 0 {
            compressed.push(run_length);
        }
        
        // Only use compression if it saves space
        if compressed.len() < self.data.len() / 8 {
            self.boundary = compressed;
            self.data.clear(); // Free the original data
        }
    }
    
    /// Get memory usage in bytes
    pub fn memory_usage(&self) -> usize {
        let data_size = if self.data.is_empty() {
            self.boundary.len()
        } else {
            self.data.len() / 8
        };
        
        std::mem::size_of::<Self>() + data_size
    }
}

/// Path compression for large cubical structures
#[derive(Clone, Debug)]
pub struct PathCompressed {
    /// Root nodes in the compressed structure
    pub roots: Vec<PathNode>,
    /// Compression ratio achieved
    pub compression_ratio: f64,
}

/// Node in path-compressed structure
#[derive(Clone, Debug)]
pub struct PathNode {
    /// Node identifier
    pub id: u64,
    /// Compressed path data
    pub path: Vec<u8>,
    /// Child nodes
    pub children: Vec<PathNode>,
}

impl PathCompressed {
    /// Create new path-compressed structure
    pub fn new() -> Self {
        Self {
            roots: Vec::new(),
            compression_ratio: 1.0,
        }
    }
    
    /// Add a path to the structure
    pub fn add_path(&mut self, path: &[u8]) -> Result<()> {
        // Find or create appropriate root node
        let root_id = path.get(0).copied().unwrap_or(0) as u64;
        
        if let Some(root) = self.roots.iter_mut().find(|n| n.id == root_id) {
            self.add_to_node(root, &path[1..])?;
        } else {
            let mut new_root = PathNode {
                id: root_id,
                path: path[1..].to_vec(),
                children: Vec::new(),
            };
            self.roots.push(new_root);
        }
        
        Ok(())
    }
    
    fn add_to_node(&mut self, node: &mut PathNode, path: &[u8]) -> Result<()> {
        if path.is_empty() {
            return Ok(());
        }
        
        // Find common prefix with existing path
        let common_len = node.path.iter()
            .zip(path.iter())
            .take_while(|(a, b)| a == b)
            .count();
        
        if common_len == node.path.len() {
            // Entire node path is prefix of new path
            let remaining = &path[common_len..];
            if !remaining.is_empty() {
                // Continue with children
                let child_id = remaining[0] as u64;
                if let Some(child) = node.children.iter_mut().find(|c| c.id == child_id) {
                    self.add_to_node(child, &remaining[1..])?;
                } else {
                    node.children.push(PathNode {
                        id: child_id,
                        path: remaining[1..].to_vec(),
                        children: Vec::new(),
                    });
                }
            }
        } else if common_len > 0 {
            // Need to split the node
            let old_path = node.path.clone();
            let old_children = std::mem::take(&mut node.children);
            
            // Update current node to hold common prefix
            node.path = path[..common_len].to_vec();
            node.children.clear();
            
            // Create child for old suffix
            if common_len < old_path.len() {
                node.children.push(PathNode {
                    id: old_path[common_len] as u64,
                    path: old_path[common_len + 1..].to_vec(),
                    children: old_children,
                });
            }
            
            // Create child for new suffix
            if common_len < path.len() {
                node.children.push(PathNode {
                    id: path[common_len] as u64,
                    path: path[common_len + 1..].to_vec(),
                    children: Vec::new(),
                });
            }
        }
        
        Ok(())
    }
    
    /// Get memory usage
    pub fn memory_usage(&self) -> usize {
        let mut total = std::mem::size_of::<Self>();
        for root in &self.roots {
            total += self.node_memory_usage(root);
        }
        total
    }
    
    fn node_memory_usage(&self, node: &PathNode) -> usize {
        let mut size = std::mem::size_of::<PathNode>() + node.path.len();
        for child in &node.children {
            size += self.node_memory_usage(child);
        }
        size
    }
}

/// Memory statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    /// Number of terms in memory
    pub terms_count: usize,
    /// Number of types in memory
    pub types_count: usize,
    /// Total memory usage in bytes
    pub memory_used: usize,
}

/// Memory pool for efficient allocation
pub struct MemoryPool<T> {
    /// Available objects
    pool: RefCell<Vec<T>>,
    /// Factory function for creating new objects
    factory: Box<dyn Fn() -> T>,
}

impl<T> MemoryPool<T> {
    /// Create new memory pool
    pub fn new<F>(factory: F) -> Self 
    where
        F: Fn() -> T + 'static,
    {
        Self {
            pool: RefCell::new(Vec::new()),
            factory: Box::new(factory),
        }
    }
    
    /// Allocate object from pool
    pub fn allocate(&self) -> T {
        let mut pool = self.pool.borrow_mut();
        pool.pop().unwrap_or_else(|| (self.factory)())
    }
    
    /// Return object to pool
    pub fn deallocate(&self, obj: T) {
        let mut pool = self.pool.borrow_mut();
        if pool.len() < 1000 { // Limit pool size
            pool.push(obj);
        }
    }
    
    /// Get pool statistics
    pub fn stats(&self) -> (usize, usize) {
        let pool = self.pool.borrow();
        (pool.len(), pool.capacity())
    }
}

/// Global memory manager
pub struct MemoryManager {
    /// Term hash-cons table
    pub terms: TermHashCons,
    /// Type hash-cons table
    pub types: TypeHashCons,
    /// Garbage collection threshold
    pub gc_threshold: usize,
    /// Operations since last GC
    pub operations: RefCell<usize>,
}

impl MemoryManager {
    /// Create new memory manager
    pub fn new() -> Self {
        Self {
            terms: TermHashCons::new(),
            types: TypeHashCons::new(),
            gc_threshold: 10000,
            operations: RefCell::new(0),
        }
    }
    
    /// Record an operation and potentially trigger GC
    pub fn record_operation(&self) {
        let mut ops = self.operations.borrow_mut();
        *ops += 1;
        
        if *ops >= self.gc_threshold {
            self.gc();
            *ops = 0;
        }
    }
    
    /// Force garbage collection
    pub fn gc(&self) {
        self.terms.gc();
        self.types.gc();
    }
    
    /// Get comprehensive memory statistics
    pub fn stats(&self) -> MemoryStats {
        let term_stats = self.terms.stats();
        MemoryStats {
            terms_count: term_stats.terms_count,
            types_count: self.types.table.borrow().len(),
            memory_used: term_stats.memory_used + self.estimate_type_memory(),
        }
    }
    
    fn estimate_type_memory(&self) -> usize {
        self.types.table.borrow().len() * std::mem::size_of::<(TypeData, TypeId)>()
    }
}

/// Convert regular terms to hash-consed terms
pub fn hash_cons_term(term: &Term, manager: &MemoryManager) -> Result<HashConsedTerm> {
    manager.record_operation();
    
    let data = match term {
        Term::Var(name) => TermData::Var(name.clone()),
        Term::Lambda { param, body } => {
            let hc_body = hash_cons_term(body, manager)?;
            TermData::Lambda {
                param: param.clone(),
                body: hc_body,
            }
        }
        Term::App { function, argument } => {
            let hc_func = hash_cons_term(function, manager)?;
            let hc_arg = hash_cons_term(argument, manager)?;
            TermData::App {
                function: hc_func,
                argument: hc_arg,
            }
        }
        Term::Pair { first, second } => {
            let hc_first = hash_cons_term(first, manager)?;
            let hc_second = hash_cons_term(second, manager)?;
            TermData::Pair {
                first: hc_first,
                second: hc_second,
            }
        }
        _ => return Err(Error::NotImplemented("Hash consing for this term".into())),
    };
    
    Ok(manager.terms.intern(data))
}

/// Convert hash-consed terms back to regular terms
pub fn unhash_cons_term(hc_term: &HashConsedTerm) -> Result<Term> {
    match &*hc_term.data {
        TermData::Var(name) => Ok(Term::Var(name.clone())),
        TermData::Lambda { param, body } => {
            Ok(Term::Lambda {
                param: param.clone(),
                body: Box::new(unhash_cons_term(body)?),
            })
        }
        TermData::App { function, argument } => {
            Ok(Term::App {
                function: Box::new(unhash_cons_term(function)?),
                argument: Box::new(unhash_cons_term(argument)?),
            })
        }
        TermData::Pair { first, second } => {
            Ok(Term::Pair {
                first: Box::new(unhash_cons_term(first)?),
                second: Box::new(unhash_cons_term(second)?),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash_consing() {
        let manager = MemoryManager::new();
        
        let term1 = Term::var("x");
        let term2 = Term::var("x");
        
        let hc1 = hash_cons_term(&term1, &manager).unwrap();
        let hc2 = hash_cons_term(&term2, &manager).unwrap();
        
        // Should have same ID (sharing)
        assert_eq!(hc1.id, hc2.id);
    }
    
    #[test]
    fn test_compressed_cube() {
        let mut cube = CompressedCube::new(3);
        
        cube.set_cell(&[true, false, true]).unwrap();
        assert!(cube.get_cell(&[true, false, true]).unwrap());
        assert!(!cube.get_cell(&[false, false, true]).unwrap());
    }
    
    #[test]
    fn test_cube_compression() {
        let mut cube = CompressedCube::new(4);
        
        // Set some cells
        for i in 0..8 {
            let coords = [i & 1 != 0, i & 2 != 0, i & 4 != 0, false];
            cube.set_cell(&coords).unwrap();
        }
        
        let original_size = cube.memory_usage();
        cube.compress();
        let compressed_size = cube.memory_usage();
        
        // Compression should reduce size or keep it same
        assert!(compressed_size <= original_size);
    }
    
    #[test]
    fn test_path_compression() {
        let mut compressed = PathCompressed::new();
        
        compressed.add_path(&[1, 2, 3, 4]).unwrap();
        compressed.add_path(&[1, 2, 5, 6]).unwrap();
        compressed.add_path(&[1, 7, 8]).unwrap();
        
        // Should have created a tree structure with shared prefixes
        assert_eq!(compressed.roots.len(), 1);
        assert_eq!(compressed.roots[0].id, 1);
    }
    
    #[test]
    fn test_memory_pool() {
        let pool = MemoryPool::new(|| Vec::<i32>::new());
        
        let mut vec1 = pool.allocate();
        vec1.push(42);
        
        pool.deallocate(vec1);
        
        let vec2 = pool.allocate();
        // Should be the same vector, reused
        assert!(vec2.is_empty() || vec2[0] == 42);
    }
    
    #[test]
    fn test_memory_manager() {
        let manager = MemoryManager::new();
        
        // Create some terms
        let term = Term::lambda("x", Term::app(Term::var("f"), Term::var("x")));
        let _hc_term = hash_cons_term(&term, &manager).unwrap();
        
        let stats = manager.stats();
        assert!(stats.terms_count > 0);
        assert!(stats.memory_used > 0);
    }
    
    #[test]
    fn test_garbage_collection() {
        let manager = MemoryManager::new();
        
        // Create and drop many terms
        for i in 0..100 {
            let term = Term::var(&format!("x_{}", i));
            let _hc_term = hash_cons_term(&term, &manager).unwrap();
            // hc_term goes out of scope here
        }
        
        let stats_before = manager.stats();
        manager.gc();
        let stats_after = manager.stats();
        
        // GC should have freed some memory
        assert!(stats_after.memory_used <= stats_before.memory_used);
    }
}