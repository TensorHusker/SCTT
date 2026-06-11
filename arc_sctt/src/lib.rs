//! # ARC-SCTT: The Revolution in Abstract Reasoning
//! 
//! This crate implements the world's first morphological intelligence system
//! for solving ARC puzzles using Smooth Cubical Type Theory principles.
//! 
//! ## Core Philosophy
//! 
//! Visual patterns are morphisms in a category where:
//! - Objects are grids (spatial configurations)
//! - Morphisms are transformations (rules/patterns)
//! - Composition is pattern combination
//! - Identity is the trivial transformation
//! 
//! ## Revolutionary Features
//! 
//! 1. **Morphological Intelligence**: Sees transformations as first-class mathematical objects
//! 2. **Smooth Composition**: Learns by smoothly composing pattern morphisms
//! 3. **Type-Theoretic Reasoning**: Every pattern has a precise type signature
//! 4. **Geometric Pattern Space**: Patterns live in a metric space of transformations
//! 5. **Self-Improving Architecture**: The system evolves its own pattern recognition

pub mod kernel;
pub mod morphology;
pub mod composition;
pub mod solver;
pub mod pattern_space;
pub mod types;
pub mod utils;

#[cfg(test)]
pub mod tests;

pub use kernel::*;
pub use morphology::*;
pub use composition::*;
pub use solver::*;
pub use pattern_space::*;
pub use types::*;

/// The main ARC-SCTT system that combines all components
pub struct ARCSCTTSystem {
    pub kernel: Kernel,
    pub morphology: PatternExtractor,
    pub compositor: Compositor,
    pub solver: ARCSolver,
    pub pattern_space: PatternSpace,
}

impl ARCSCTTSystem {
    /// Create a new ARC-SCTT system with default configuration
    pub fn new() -> Self {
        let kernel = Kernel::new();
        let morphology = PatternExtractor::new();
        let compositor = Compositor::new();
        let solver = ARCSolver::new();
        let pattern_space = PatternSpace::new();
        
        Self {
            kernel,
            morphology,
            compositor,
            solver,
            pattern_space,
        }
    }
    
    /// Solve an ARC puzzle using morphological intelligence
    pub fn solve_puzzle(&mut self, examples: &[(Grid, Grid)], test_input: &Grid) -> Result<Grid, SCTTError> {
        // Extract morphisms from training examples
        let morphisms = self.morphology.extract_morphisms(examples)?;
        
        // Learn to compose morphisms smoothly
        let learned_compositor = self.compositor.learn_composition(&morphisms)?;
        
        // Apply learned composition to test input
        self.solver.solve_with_morphisms(test_input, &learned_compositor)
    }
    
    /// Evolve the system's pattern recognition capabilities
    pub fn evolve(&mut self, training_data: &[ARCPuzzle]) -> Result<(), SCTTError> {
        for puzzle in training_data {
            let morphisms = self.morphology.extract_morphisms(&puzzle.examples)?;
            self.pattern_space.add_patterns(&morphisms);
            self.compositor.learn_from_patterns(&morphisms)?;
        }
        Ok(())
    }
}

impl Default for ARCSCTTSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_system_initialization() {
        let system = ARCSCTTSystem::new();
        assert_eq!(system.kernel.dimension(), 2);
    }
    
    #[test]
    fn test_basic_morphism_extraction() {
        let mut system = ARCSCTTSystem::new();
        
        // Simple identity transformation
        let grid1 = Grid::from_vec(vec![vec![1, 0], vec![0, 1]]).unwrap();
        let grid2 = Grid::from_vec(vec![vec![1, 0], vec![0, 1]]).unwrap();
        
        let examples = vec![(grid1, grid2)];
        let morphisms = system.morphology.extract_morphisms(&examples).unwrap();
        
        assert!(!morphisms.is_empty());
    }
}