//! ARC Solver: Morphological Reasoning for Abstract Intelligence
//! 
//! This module implements the complete ARC solving pipeline using morphological
//! intelligence, treating each puzzle as a problem in categorical composition
//! and smooth transformation learning.

use crate::types::*;
use crate::kernel::Kernel;
use crate::morphology::PatternExtractor;
use crate::composition::{Compositor, LearnedComposition};
use crate::pattern_space::PatternSpace;
use std::collections::{HashMap, VecDeque};
use rayon::prelude::*;

/// The main ARC solver using morphological reasoning
#[derive(Debug, Clone)]
pub struct ARCSolver {
    kernel: Kernel,
    pattern_extractor: PatternExtractor,
    compositor: Compositor,
    pattern_space: PatternSpace,
    solution_cache: HashMap<String, SolutionCandidate>,
    reasoning_depth: usize,
    confidence_threshold: f64,
    exploration_budget: usize,
}

/// A candidate solution with its reasoning trace
#[derive(Debug, Clone)]
pub struct SolutionCandidate {
    pub output_grid: Grid,
    pub morphism_sequence: Vec<Morphism>,
    pub confidence: f64,
    pub reasoning_trace: ReasoningTrace,
    pub generalization_score: f64,
}

/// Trace of the reasoning process
#[derive(Debug, Clone)]
pub struct ReasoningTrace {
    pub steps: Vec<ReasoningStep>,
    pub pattern_discoveries: Vec<PatternDiscovery>,
    pub composition_insights: Vec<CompositionInsight>,
    pub invariant_hypotheses: Vec<InvariantHypothesis>,
}

/// A single step in the reasoning process
#[derive(Debug, Clone)]
pub struct ReasoningStep {
    pub step_type: ReasoningStepType,
    pub input_state: GridSignature,
    pub output_state: GridSignature,
    pub applied_morphism: Morphism,
    pub confidence: f64,
    pub explanation: String,
}

/// Types of reasoning steps
#[derive(Debug, Clone)]
pub enum ReasoningStepType {
    PatternRecognition,
    MorphismApplication,
    CompositionInference,
    InvariantDetection,
    Generalization,
    Verification,
}

/// A discovered pattern with its properties
#[derive(Debug, Clone)]
pub struct PatternDiscovery {
    pub pattern_type: PatternType,
    pub morphism: Morphism,
    pub occurrence_count: usize,
    pub consistency_score: f64,
    pub description: String,
}

/// Types of patterns that can be discovered
#[derive(Debug, Clone)]
pub enum PatternType {
    Geometric,
    ColorTransformation,
    SpatialRelation,
    SequentialRule,
    ConditionalRule,
    Emergent,
}

/// Insights about morphism composition
#[derive(Debug, Clone)]
pub struct CompositionInsight {
    pub composition_rule: CompositionRule,
    pub supporting_evidence: Vec<Morphism>,
    pub predictive_power: f64,
    pub description: String,
}

/// Rules for composing morphisms
#[derive(Debug, Clone)]
pub enum CompositionRule {
    Sequential,
    Parallel,
    Conditional,
    Hierarchical,
    Emergent,
}

/// Hypotheses about invariant properties
#[derive(Debug, Clone)]
pub struct InvariantHypothesis {
    pub property: InvariantProperty,
    pub examples_supporting: usize,
    pub examples_violating: usize,
    pub confidence: f64,
    pub description: String,
}

/// Properties that might be invariant across transformations
#[derive(Debug, Clone)]
pub enum InvariantProperty {
    ObjectCount,
    ColorDistribution,
    TopologicalStructure,
    Symmetry,
    Connectivity,
    Ratio,
    Pattern,
}

impl ARCSolver {
    /// Create a new ARC solver
    pub fn new() -> Self {
        ARCSolver {
            kernel: Kernel::new(),
            pattern_extractor: PatternExtractor::new(),
            compositor: Compositor::new(),
            pattern_space: PatternSpace::new(),
            solution_cache: HashMap::new(),
            reasoning_depth: 5,
            confidence_threshold: 0.8,
            exploration_budget: 1000,
        }
    }
    
    /// Solve an ARC puzzle using morphological reasoning
    pub fn solve(&mut self, puzzle: &ARCPuzzle) -> Result<SolutionCandidate, SCTTError> {
        // Check cache first
        let cache_key = self.generate_puzzle_key(puzzle);
        if let Some(cached) = self.solution_cache.get(&cache_key) {
            return Ok(cached.clone());
        }
        
        // Initialize reasoning trace
        let mut reasoning_trace = ReasoningTrace {
            steps: Vec::new(),
            pattern_discoveries: Vec::new(),
            composition_insights: Vec::new(),
            invariant_hypotheses: Vec::new(),
        };
        
        // Phase 1: Pattern Discovery
        let patterns = self.discover_patterns(&puzzle.examples, &mut reasoning_trace)?;
        
        // Phase 2: Invariant Detection
        let invariants = self.detect_invariants(&puzzle.examples, &mut reasoning_trace)?;
        
        // Phase 3: Composition Learning
        let composition = self.learn_composition(&patterns, &mut reasoning_trace)?;
        
        // Phase 4: Solution Generation
        let candidates = self.generate_solutions(&puzzle.test_input, &composition, &invariants, &mut reasoning_trace)?;
        
        // Phase 5: Solution Selection
        let best_solution = self.select_best_solution(candidates, &reasoning_trace)?;
        
        // Cache the solution
        self.solution_cache.insert(cache_key, best_solution.clone());
        
        Ok(best_solution)
    }
    
    /// Solve using learned morphisms directly
    pub fn solve_with_morphisms(&self, input: &Grid, composition: &LearnedComposition) -> Result<Grid, SCTTError> {
        let mut current_grid = input.clone();
        
        for morphism in &composition.morphism_sequence {
            current_grid = self.kernel.apply_transform(&current_grid, &morphism.transform)?;
        }
        
        Ok(current_grid)
    }
    
    /// Discover patterns from training examples
    fn discover_patterns(&mut self, examples: &[(Grid, Grid)], reasoning_trace: &mut ReasoningTrace) 
        -> Result<Vec<Morphism>, SCTTError> {
        
        let mut all_patterns = Vec::new();
        
        // Extract morphisms from each example
        for (i, (input, output)) in examples.iter().enumerate() {
            let morphisms = self.pattern_extractor.extract_single_morphism(input, output)?;
            
            for morphism in morphisms {
                // Record pattern discovery
                let discovery = PatternDiscovery {
                    pattern_type: self.classify_pattern_type(&morphism),
                    morphism: morphism.clone(),
                    occurrence_count: 1,
                    consistency_score: morphism.confidence,
                    description: format!("Pattern discovered in example {}: {:?}", i, morphism.transform),
                };
                reasoning_trace.pattern_discoveries.push(discovery);
                
                // Add reasoning step
                let step = ReasoningStep {
                    step_type: ReasoningStepType::PatternRecognition,
                    input_state: input.signature(),
                    output_state: output.signature(),
                    applied_morphism: morphism.clone(),
                    confidence: morphism.confidence,
                    explanation: format!("Recognized pattern: {:?}", morphism.transform),
                };
                reasoning_trace.steps.push(step);
                
                all_patterns.push(morphism);
            }
        }
        
        // Find consistent patterns across examples
        let consistent_patterns = self.find_consistent_patterns(&all_patterns, reasoning_trace)?;
        
        Ok(consistent_patterns)
    }
    
    /// Detect invariant properties across transformations
    fn detect_invariants(&self, examples: &[(Grid, Grid)], reasoning_trace: &mut ReasoningTrace) 
        -> Result<Vec<InvariantHypothesis>, SCTTError> {
        
        let mut invariant_hypotheses = Vec::new();
        
        // Check different invariant properties
        let properties = vec![
            InvariantProperty::ObjectCount,
            InvariantProperty::ColorDistribution,
            InvariantProperty::TopologicalStructure,
            InvariantProperty::Symmetry,
            InvariantProperty::Connectivity,
        ];
        
        for property in properties {
            let hypothesis = self.test_invariant_property(&property, examples)?;
            
            if hypothesis.confidence > 0.5 {
                // Add reasoning step
                let step = ReasoningStep {
                    step_type: ReasoningStepType::InvariantDetection,
                    input_state: examples[0].0.signature(),
                    output_state: examples[0].1.signature(),
                    applied_morphism: Morphism::new(
                        examples[0].0.signature(),
                        examples[0].1.signature(),
                        Transform::Identity,
                    ),
                    confidence: hypothesis.confidence,
                    explanation: hypothesis.description.clone(),
                };
                reasoning_trace.steps.push(step);
                
                invariant_hypotheses.push(hypothesis);
            }
        }
        
        reasoning_trace.invariant_hypotheses.extend(invariant_hypotheses.clone());
        
        Ok(invariant_hypotheses)
    }
    
    /// Learn composition rules from patterns
    fn learn_composition(&mut self, patterns: &[Morphism], reasoning_trace: &mut ReasoningTrace) 
        -> Result<LearnedComposition, SCTTError> {
        
        let composition = self.compositor.learn_composition(patterns)?;
        
        // Record composition insight
        let insight = CompositionInsight {
            composition_rule: self.classify_composition_rule(&composition),
            supporting_evidence: patterns.to_vec(),
            predictive_power: composition.success_rate,
            description: format!("Learned {:?} composition with {:.2} success rate", 
                               composition.composition_type, composition.success_rate),
        };
        reasoning_trace.composition_insights.push(insight);
        
        // Add reasoning step
        let step = ReasoningStep {
            step_type: ReasoningStepType::CompositionInference,
            input_state: GridSignature {
                width: 0, height: 0, color_distribution: HashMap::new(),
                shape_features: vec![], complexity: 0.0,
            },
            output_state: GridSignature {
                width: 0, height: 0, color_distribution: HashMap::new(),
                shape_features: vec![], complexity: 0.0,
            },
            applied_morphism: Morphism::new(
                GridSignature {
                    width: 0, height: 0, color_distribution: HashMap::new(),
                    shape_features: vec![], complexity: 0.0,
                },
                GridSignature {
                    width: 0, height: 0, color_distribution: HashMap::new(),
                    shape_features: vec![], complexity: 0.0,
                },
                Transform::Identity,
            ),
            confidence: composition.success_rate,
            explanation: format!("Learned composition strategy: {:?}", composition.composition_type),
        };
        reasoning_trace.steps.push(step);
        
        Ok(composition)
    }
    
    /// Generate solution candidates
    fn generate_solutions(&self, test_input: &Grid, composition: &LearnedComposition, 
                         invariants: &[InvariantHypothesis], reasoning_trace: &mut ReasoningTrace) 
        -> Result<Vec<SolutionCandidate>, SCTTError> {
        
        let mut candidates = Vec::new();
        
        // Apply learned composition directly
        if let Ok(solution_grid) = self.solve_with_morphisms(test_input, composition) {
            let candidate = SolutionCandidate {
                output_grid: solution_grid,
                morphism_sequence: composition.morphism_sequence.clone(),
                confidence: composition.success_rate,
                reasoning_trace: reasoning_trace.clone(),
                generalization_score: composition.generalization_score,
            };
            candidates.push(candidate);
        }
        
        // Try variations based on invariants
        for invariant in invariants {
            if let Ok(variant_candidates) = self.generate_invariant_preserving_solutions(
                test_input, composition, invariant, reasoning_trace
            ) {
                candidates.extend(variant_candidates);
            }
        }
        
        // Try exploratory solutions
        let exploratory = self.generate_exploratory_solutions(test_input, reasoning_trace)?;
        candidates.extend(exploratory);
        
        Ok(candidates)
    }
    
    /// Select the best solution from candidates
    fn select_best_solution(&self, candidates: Vec<SolutionCandidate>, reasoning_trace: &ReasoningTrace) 
        -> Result<SolutionCandidate, SCTTError> {
        
        if candidates.is_empty() {
            return Err(SCTTError::PatternExtractionFailed {
                reason: "No solution candidates generated".to_string(),
            });
        }
        
        // Score candidates based on multiple criteria
        let mut scored_candidates: Vec<(SolutionCandidate, f64)> = candidates
            .into_iter()
            .map(|candidate| {
                let score = self.calculate_solution_score(&candidate, reasoning_trace);
                (candidate, score)
            })
            .collect();
        
        // Sort by score (highest first)
        scored_candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        Ok(scored_candidates.into_iter().next().unwrap().0)
    }
    
    // Helper methods
    
    fn generate_puzzle_key(&self, puzzle: &ARCPuzzle) -> String {
        // Generate a unique key for caching
        format!("{}_{}", puzzle.id, puzzle.examples.len())
    }
    
    fn classify_pattern_type(&self, morphism: &Morphism) -> PatternType {
        match &morphism.transform {
            Transform::Rotate90 | Transform::Rotate180 | Transform::Rotate270 |
            Transform::FlipHorizontal | Transform::FlipVertical | Transform::Transpose => PatternType::Geometric,
            
            Transform::ColorMap(_) | Transform::InvertColors => PatternType::ColorTransformation,
            
            Transform::Translate { .. } | Transform::Scale { .. } | Transform::Crop { .. } => PatternType::SpatialRelation,
            
            Transform::Sequence(_) => PatternType::SequentialRule,
            
            Transform::Conditional { .. } => PatternType::ConditionalRule,
            
            _ => PatternType::Emergent,
        }
    }
    
    fn find_consistent_patterns(&self, patterns: &[Morphism], reasoning_trace: &mut ReasoningTrace) 
        -> Result<Vec<Morphism>, SCTTError> {
        
        let mut consistent_patterns = Vec::new();
        let mut pattern_counts: HashMap<String, (Morphism, usize)> = HashMap::new();
        
        // Count pattern occurrences
        for pattern in patterns {
            let key = format!("{:?}", pattern.transform);
            pattern_counts.entry(key)
                .and_modify(|(_, count)| *count += 1)
                .or_insert((pattern.clone(), 1));
        }
        
        // Keep patterns that appear multiple times
        for (_, (pattern, count)) in pattern_counts {
            if count >= 2 {
                let pattern_clone = pattern.clone();
                consistent_patterns.push(pattern_clone.clone());
                
                // Update reasoning trace
                let step = ReasoningStep {
                    step_type: ReasoningStepType::Verification,
                    input_state: pattern_clone.source.clone(),
                    output_state: pattern_clone.target.clone(),
                    applied_morphism: pattern_clone.clone(),
                    confidence: pattern_clone.confidence * (count as f64 / patterns.len() as f64),
                    explanation: format!("Pattern confirmed across {} examples", count),
                };
                reasoning_trace.steps.push(step);
            }
        }
        
        Ok(consistent_patterns)
    }
    
    fn test_invariant_property(&self, property: &InvariantProperty, examples: &[(Grid, Grid)]) 
        -> Result<InvariantHypothesis, SCTTError> {
        
        let mut supporting = 0;
        let mut violating = 0;
        
        for (input, output) in examples {
            let preserved = match property {
                InvariantProperty::ObjectCount => self.object_count_preserved(input, output),
                InvariantProperty::ColorDistribution => self.color_distribution_preserved(input, output),
                InvariantProperty::TopologicalStructure => self.topology_preserved(input, output),
                InvariantProperty::Symmetry => self.symmetry_preserved(input, output),
                InvariantProperty::Connectivity => self.connectivity_preserved(input, output),
                _ => false,
            };
            
            if preserved {
                supporting += 1;
            } else {
                violating += 1;
            }
        }
        
        let confidence = supporting as f64 / (supporting + violating) as f64;
        
        Ok(InvariantHypothesis {
            property: property.clone(),
            examples_supporting: supporting,
            examples_violating: violating,
            confidence,
            description: format!("{:?} preserved in {}/{} examples", 
                               property, supporting, supporting + violating),
        })
    }
    
    fn object_count_preserved(&self, input: &Grid, output: &Grid) -> bool {
        let input_objects = self.count_objects(input);
        let output_objects = self.count_objects(output);
        input_objects == output_objects
    }
    
    fn color_distribution_preserved(&self, input: &Grid, output: &Grid) -> bool {
        let input_sig = input.signature();
        let output_sig = output.signature();
        input_sig.color_distribution.len() == output_sig.color_distribution.len()
    }
    
    fn topology_preserved(&self, input: &Grid, output: &Grid) -> bool {
        // Simplified topology check
        let input_components = self.count_connected_components(input);
        let output_components = self.count_connected_components(output);
        input_components == output_components
    }
    
    fn symmetry_preserved(&self, input: &Grid, output: &Grid) -> bool {
        // Check if symmetry properties are preserved
        let input_symmetric = self.has_symmetry(input);
        let output_symmetric = self.has_symmetry(output);
        input_symmetric == output_symmetric
    }
    
    fn connectivity_preserved(&self, input: &Grid, output: &Grid) -> bool {
        // Check if connectivity structure is preserved
        self.topology_preserved(input, output)
    }
    
    fn count_objects(&self, grid: &Grid) -> usize {
        let mut visited = vec![vec![false; grid.width]; grid.height];
        let mut object_count = 0;
        
        for y in 0..grid.height {
            for x in 0..grid.width {
                if !visited[y][x] && grid.get(x, y).unwrap() != 0 {
                    self.flood_fill(grid, x, y, &mut visited);
                    object_count += 1;
                }
            }
        }
        
        object_count
    }
    
    fn count_connected_components(&self, grid: &Grid) -> usize {
        self.count_objects(grid)
    }
    
    fn has_symmetry(&self, grid: &Grid) -> bool {
        // Check for horizontal symmetry
        for y in 0..grid.height {
            for x in 0..grid.width / 2 {
                if grid.get(x, y) != grid.get(grid.width - 1 - x, y) {
                    return false;
                }
            }
        }
        true
    }
    
    fn flood_fill(&self, grid: &Grid, start_x: usize, start_y: usize, visited: &mut [Vec<bool>]) {
        let mut queue = VecDeque::new();
        let target_color = grid.get(start_x, start_y).unwrap();
        
        queue.push_back((start_x, start_y));
        visited[start_y][start_x] = true;
        
        while let Some((x, y)) = queue.pop_front() {
            // Check 4-connected neighbors
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && ny >= 0 && 
                   (nx as usize) < grid.width && (ny as usize) < grid.height {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    
                    if !visited[ny][nx] && grid.get(nx, ny).unwrap() == target_color {
                        visited[ny][nx] = true;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }
    
    fn classify_composition_rule(&self, composition: &LearnedComposition) -> CompositionRule {
        match composition.composition_type {
            crate::composition::CompositionType::Sequential => CompositionRule::Sequential,
            crate::composition::CompositionType::Parallel => CompositionRule::Parallel,
            crate::composition::CompositionType::Conditional => CompositionRule::Conditional,
            crate::composition::CompositionType::Hierarchical => CompositionRule::Hierarchical,
            _ => CompositionRule::Emergent,
        }
    }
    
    fn generate_invariant_preserving_solutions(&self, test_input: &Grid, 
                                             composition: &LearnedComposition,
                                             invariant: &InvariantHypothesis,
                                             reasoning_trace: &mut ReasoningTrace) 
        -> Result<Vec<SolutionCandidate>, SCTTError> {
        
        let mut candidates = Vec::new();
        
        // Generate solutions that preserve the invariant
        if let Ok(solution_grid) = self.solve_with_morphisms(test_input, composition) {
            // Verify invariant preservation
            let preserves_invariant = match &invariant.property {
                InvariantProperty::ObjectCount => self.object_count_preserved(test_input, &solution_grid),
                InvariantProperty::ColorDistribution => self.color_distribution_preserved(test_input, &solution_grid),
                _ => true, // Simplified for other properties
            };
            
            if preserves_invariant {
                let candidate = SolutionCandidate {
                    output_grid: solution_grid,
                    morphism_sequence: composition.morphism_sequence.clone(),
                    confidence: composition.success_rate * invariant.confidence,
                    reasoning_trace: reasoning_trace.clone(),
                    generalization_score: composition.generalization_score,
                };
                candidates.push(candidate);
            }
        }
        
        Ok(candidates)
    }
    
    fn generate_exploratory_solutions(&self, test_input: &Grid, reasoning_trace: &mut ReasoningTrace) 
        -> Result<Vec<SolutionCandidate>, SCTTError> {
        
        let mut candidates = Vec::new();
        
        // Try basic transformations as fallback
        let basic_transforms = vec![
            Transform::Identity,
            Transform::Rotate90,
            Transform::Rotate180,
            Transform::FlipHorizontal,
            Transform::FlipVertical,
        ];
        
        for transform in basic_transforms {
            if let Ok(result_grid) = self.kernel.apply_transform(test_input, &transform) {
                let morphism = Morphism::new(
                    test_input.signature(),
                    result_grid.signature(),
                    transform,
                );
                
                let candidate = SolutionCandidate {
                    output_grid: result_grid,
                    morphism_sequence: vec![morphism],
                    confidence: 0.3, // Low confidence for exploratory solutions
                    reasoning_trace: reasoning_trace.clone(),
                    generalization_score: 0.2,
                };
                candidates.push(candidate);
            }
        }
        
        Ok(candidates)
    }
    
    fn calculate_solution_score(&self, candidate: &SolutionCandidate, _reasoning_trace: &ReasoningTrace) -> f64 {
        // Combine multiple factors for solution scoring
        let confidence_weight = 0.4;
        let generalization_weight = 0.3;
        let simplicity_weight = 0.2;
        let consistency_weight = 0.1;
        
        let simplicity_score = 1.0 / (candidate.morphism_sequence.len() as f64 + 1.0);
        let consistency_score = 1.0; // Simplified for now
        
        candidate.confidence * confidence_weight +
        candidate.generalization_score * generalization_weight +
        simplicity_score * simplicity_weight +
        consistency_score * consistency_weight
    }
}

impl Default for ARCSolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_solver_creation() {
        let solver = ARCSolver::new();
        assert_eq!(solver.reasoning_depth, 5);
        assert_eq!(solver.confidence_threshold, 0.8);
    }
    
    #[test]
    fn test_pattern_discovery() {
        let mut solver = ARCSolver::new();
        let mut reasoning_trace = ReasoningTrace {
            steps: Vec::new(),
            pattern_discoveries: Vec::new(),
            composition_insights: Vec::new(),
            invariant_hypotheses: Vec::new(),
        };
        
        let input = Grid::from_vec(vec![vec![1, 0], vec![0, 1]]).unwrap();
        let output = Grid::from_vec(vec![vec![0, 1], vec![1, 0]]).unwrap();
        let examples = vec![(input, output)];
        
        let patterns = solver.discover_patterns(&examples, &mut reasoning_trace).unwrap();
        assert!(!patterns.is_empty());
        assert!(!reasoning_trace.pattern_discoveries.is_empty());
    }
    
    #[test]
    fn test_invariant_detection() {
        let solver = ARCSolver::new();
        let input = Grid::from_vec(vec![vec![1, 0], vec![0, 1]]).unwrap();
        let output = Grid::from_vec(vec![vec![0, 1], vec![1, 0]]).unwrap();
        
        // Test object count preservation
        let preserved = solver.object_count_preserved(&input, &output);
        assert!(preserved); // Both grids have 2 objects
    }
    
    #[test]
    fn test_solution_scoring() {
        let solver = ARCSolver::new();
        let grid = Grid::from_vec(vec![vec![1, 0], vec![0, 1]]).unwrap();
        let morphism = Morphism::new(
            grid.signature(),
            grid.signature(),
            Transform::Identity,
        );
        
        let candidate = SolutionCandidate {
            output_grid: grid,
            morphism_sequence: vec![morphism],
            confidence: 0.9,
            reasoning_trace: ReasoningTrace {
                steps: Vec::new(),
                pattern_discoveries: Vec::new(),
                composition_insights: Vec::new(),
                invariant_hypotheses: Vec::new(),
            },
            generalization_score: 0.8,
        };
        
        let reasoning_trace = ReasoningTrace {
            steps: Vec::new(),
            pattern_discoveries: Vec::new(),
            composition_insights: Vec::new(),
            invariant_hypotheses: Vec::new(),
        };
        
        let score = solver.calculate_solution_score(&candidate, &reasoning_trace);
        assert!(score > 0.0);
        assert!(score <= 1.0);
    }
}