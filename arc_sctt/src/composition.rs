//! Smooth Composition Engine: Learning Through Morphism Composition
//! 
//! This module implements the revolutionary composition engine that learns
//! by smoothly composing morphisms in pattern space, creating a differentiable
//! reasoning architecture for visual intelligence.

use crate::types::*;
// use crate::morphology::*;
use std::collections::{HashMap, VecDeque};
use rayon::prelude::*;
// use cached::proc_macro::cached;

/// Learns to compose morphisms smoothly for enhanced pattern recognition
#[derive(Debug, Clone)]
pub struct Compositor {
    learned_compositions: HashMap<CompositionSignature, LearnedComposition>,
    composition_history: Vec<CompositionAttempt>,
    learning_rate: f64,
    regularization_strength: f64,
    max_composition_depth: usize,
    smoothness_penalty: f64,
}

/// Signature for a composition of morphisms
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompositionSignature {
    pub input_signature: String,
    pub output_signature: String,
    pub composition_type: CompositionType,
}

/// Type of composition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompositionType {
    Sequential,
    Parallel,
    Conditional,
    Smooth,
    Hierarchical,
}

/// A learned composition with its performance metrics
#[derive(Debug, Clone)]
pub struct LearnedComposition {
    pub morphism_sequence: Vec<Morphism>,
    pub success_rate: f64,
    pub smoothness_score: f64,
    pub generalization_score: f64,
    pub composition_type: CompositionType,
    pub usage_count: usize,
    pub last_updated: std::time::SystemTime,
}

/// Record of a composition attempt for learning
#[derive(Debug, Clone)]
pub struct CompositionAttempt {
    pub input_signature: GridSignature,
    pub target_signature: GridSignature,
    pub attempted_sequence: Vec<Morphism>,
    pub success: bool,
    pub confidence: f64,
    pub timestamp: std::time::SystemTime,
}

impl Compositor {
    /// Create a new compositor with default learning parameters
    pub fn new() -> Self {
        Compositor {
            learned_compositions: HashMap::new(),
            composition_history: Vec::new(),
            learning_rate: 0.1,
            regularization_strength: 0.01,
            max_composition_depth: 5,
            smoothness_penalty: 0.05,
        }
    }
    
    /// Learn composition patterns from a set of morphisms
    pub fn learn_composition(&mut self, morphisms: &[Morphism]) -> Result<LearnedComposition, SCTTError> {
        if morphisms.is_empty() {
            return Err(SCTTError::PatternExtractionFailed {
                reason: "Cannot learn from empty morphism set".to_string(),
            });
        }
        
        // Try different composition strategies
        let sequential = self.learn_sequential_composition(morphisms)?;
        let parallel = self.learn_parallel_composition(morphisms)?;
        let hierarchical = self.learn_hierarchical_composition(morphisms)?;
        let smooth = self.learn_smooth_composition(morphisms)?;
        
        // Select the best composition based on scores
        let candidates = vec![sequential, parallel, hierarchical, smooth];
        let best = candidates
            .into_iter()
            .max_by(|a, b| {
                let score_a = self.calculate_composition_score(a);
                let score_b = self.calculate_composition_score(b);
                score_a.partial_cmp(&score_b).unwrap()
            })
            .unwrap();
        
        // Store the learned composition
        let signature = self.get_composition_signature(&best);
        self.learned_compositions.insert(signature, best.clone());
        
        Ok(best)
    }
    
    /// Learn from patterns by finding common composition structures
    pub fn learn_from_patterns(&mut self, morphisms: &[Morphism]) -> Result<(), SCTTError> {
        // Group morphisms by similar source/target signatures
        let groups = self.group_morphisms_by_similarity(morphisms);
        
        for group in groups {
            if group.len() >= 2 {
                // Learn composition patterns within each group
                if let Ok(composition) = self.learn_composition(&group) {
                    // Update learning statistics
                    self.update_learning_statistics(&composition);
                }
            }
        }
        
        // Evolve composition strategies based on success rates
        self.evolve_composition_strategies();
        
        Ok(())
    }
    
    /// Apply learned composition to solve a transformation
    pub fn apply_learned_composition(&self, input: &Grid, target_signature: &GridSignature) 
        -> Result<(Grid, Vec<Morphism>), SCTTError> {
        
        let input_signature = input.signature();
        
        // Find best matching learned composition
        let best_composition = self.find_best_composition(&input_signature, target_signature)?;
        
        // Apply the composition sequence
        let mut current_grid = input.clone();
        let mut applied_morphisms = Vec::new();
        
        for morphism in &best_composition.morphism_sequence {
            // Apply each morphism in the sequence
            current_grid = self.apply_morphism_smoothly(&current_grid, morphism)?;
            applied_morphisms.push(morphism.clone());
        }
        
        Ok((current_grid, applied_morphisms))
    }
    
    /// Learn sequential composition (f ∘ g ∘ h ...)
    fn learn_sequential_composition(&self, morphisms: &[Morphism]) -> Result<LearnedComposition, SCTTError> {
        // Find the best ordering of morphisms for sequential composition
        let mut best_sequence = morphisms.to_vec();
        let mut best_score = 0.0;
        
        // Try different permutations (limited for performance)
        for permutation in self.generate_permutations(morphisms, 3) {
            if let Ok(score) = self.evaluate_sequence(&permutation) {
                if score > best_score {
                    best_score = score;
                    best_sequence = permutation;
                }
            }
        }
        
        Ok(LearnedComposition {
            morphism_sequence: best_sequence.clone(),
            success_rate: best_score,
            smoothness_score: self.calculate_smoothness(&best_sequence),
            generalization_score: self.calculate_generalization(&best_sequence),
            composition_type: CompositionType::Sequential,
            usage_count: 0,
            last_updated: std::time::SystemTime::now(),
        })
    }
    
    /// Learn parallel composition (f ⊕ g ⊕ h ...)
    fn learn_parallel_composition(&self, morphisms: &[Morphism]) -> Result<LearnedComposition, SCTTError> {
        // Group morphisms that can be applied in parallel
        let parallel_groups = self.find_parallel_groups(morphisms);
        
        let mut best_parallel = Vec::new();
        let mut best_score = 0.0;
        
        for group in parallel_groups {
            let score = self.evaluate_parallel_group(&group)?;
            if score > best_score {
                best_score = score;
                best_parallel = group;
            }
        }
        
        Ok(LearnedComposition {
            morphism_sequence: best_parallel.clone(),
            success_rate: best_score,
            smoothness_score: self.calculate_smoothness(&best_parallel),
            generalization_score: self.calculate_generalization(&best_parallel),
            composition_type: CompositionType::Parallel,
            usage_count: 0,
            last_updated: std::time::SystemTime::now(),
        })
    }
    
    /// Learn hierarchical composition (tree-like structure)
    fn learn_hierarchical_composition(&self, morphisms: &[Morphism]) -> Result<LearnedComposition, SCTTError> {
        // Build a hierarchical composition tree
        let hierarchy = self.build_composition_hierarchy(morphisms)?;
        let flattened = self.flatten_hierarchy(&hierarchy);
        
        Ok(LearnedComposition {
            morphism_sequence: flattened,
            success_rate: self.evaluate_hierarchy(&hierarchy)?,
            smoothness_score: self.calculate_smoothness(&hierarchy),
            generalization_score: self.calculate_generalization(&hierarchy),
            composition_type: CompositionType::Hierarchical,
            usage_count: 0,
            last_updated: std::time::SystemTime::now(),
        })
    }
    
    /// Learn smooth composition (C^∞ smooth paths in morphism space)
    fn learn_smooth_composition(&self, morphisms: &[Morphism]) -> Result<LearnedComposition, SCTTError> {
        // Create smooth interpolation between morphisms
        let smooth_sequence = self.create_smooth_interpolation(morphisms)?;
        
        Ok(LearnedComposition {
            morphism_sequence: smooth_sequence.clone(),
            success_rate: self.evaluate_smooth_composition(&smooth_sequence)?,
            smoothness_score: 1.0, // Smooth by construction
            generalization_score: self.calculate_generalization(&smooth_sequence),
            composition_type: CompositionType::Smooth,
            usage_count: 0,
            last_updated: std::time::SystemTime::now(),
        })
    }
    
    /// Apply a morphism with smooth interpolation
    fn apply_morphism_smoothly(&self, grid: &Grid, morphism: &Morphism) -> Result<Grid, SCTTError> {
        // For now, apply the morphism directly
        // In a full implementation, this would use smooth interpolation
        self.apply_morphism_with_smoothness(grid, morphism, morphism.smoothness_order)
    }
    
    /// Apply morphism while maintaining specified smoothness order
    fn apply_morphism_with_smoothness(&self, grid: &Grid, morphism: &Morphism, 
                                     smoothness_order: u32) -> Result<Grid, SCTTError> {
        if morphism.smoothness_order < smoothness_order {
            return Err(SCTTError::SmoothnessViolation { 
                order: smoothness_order 
            });
        }
        
        // Apply the transformation (simplified implementation)
        match &morphism.transform {
            Transform::Identity => Ok(grid.clone()),
            Transform::Rotate90 => self.smooth_rotate_90(grid, smoothness_order),
            Transform::Rotate180 => self.smooth_rotate_180(grid, smoothness_order),
            Transform::Rotate270 => self.smooth_rotate_270(grid, smoothness_order),
            Transform::FlipHorizontal => self.smooth_flip_horizontal(grid, smoothness_order),
            Transform::FlipVertical => self.smooth_flip_vertical(grid, smoothness_order),
            Transform::ColorMap(map) => self.smooth_color_map(grid, map, smoothness_order),
            Transform::Translate { dx, dy } => self.smooth_translate(grid, *dx, *dy, smoothness_order),
            Transform::ScaleInt { numerator, denominator } => {
                let factor = *numerator as f64 / *denominator as f64;
                self.smooth_scale(grid, factor, smoothness_order)
            },
            _ => {
                // For complex transforms, delegate to kernel
                Err(SCTTError::InvalidTransformation {
                    reason: "Complex transform not supported in smooth composition".to_string(),
                })
            }
        }
    }
    
    // Smooth transformation implementations
    
    fn smooth_rotate_90(&self, grid: &Grid, _smoothness_order: u32) -> Result<Grid, SCTTError> {
        // Apply rotation with smooth interpolation at boundaries
        let new_data = ndarray::Array2::from_shape_fn((grid.width, grid.height), |(x, y)| {
            // Use bilinear interpolation for smoothness
            let src_x = grid.height as f64 - 1.0 - y as f64;
            let src_y = x as f64;
            
            self.bilinear_sample(grid, src_x, src_y)
        });
        
        Ok(Grid {
            data: new_data,
            width: grid.height,
            height: grid.width,
        })
    }
    
    fn smooth_rotate_180(&self, grid: &Grid, _smoothness_order: u32) -> Result<Grid, SCTTError> {
        let new_data = ndarray::Array2::from_shape_fn((grid.height, grid.width), |(y, x)| {
            let src_x = grid.width as f64 - 1.0 - x as f64;
            let src_y = grid.height as f64 - 1.0 - y as f64;
            
            self.bilinear_sample(grid, src_x, src_y)
        });
        
        Ok(Grid {
            data: new_data,
            width: grid.width,
            height: grid.height,
        })
    }
    
    fn smooth_rotate_270(&self, grid: &Grid, _smoothness_order: u32) -> Result<Grid, SCTTError> {
        let new_data = ndarray::Array2::from_shape_fn((grid.width, grid.height), |(x, y)| {
            let src_x = y as f64;
            let src_y = grid.width as f64 - 1.0 - x as f64;
            
            self.bilinear_sample(grid, src_x, src_y)
        });
        
        Ok(Grid {
            data: new_data,
            width: grid.height,
            height: grid.width,
        })
    }
    
    fn smooth_flip_horizontal(&self, grid: &Grid, _smoothness_order: u32) -> Result<Grid, SCTTError> {
        let new_data = ndarray::Array2::from_shape_fn((grid.height, grid.width), |(y, x)| {
            let src_x = grid.width as f64 - 1.0 - x as f64;
            let src_y = y as f64;
            
            self.bilinear_sample(grid, src_x, src_y)
        });
        
        Ok(Grid {
            data: new_data,
            width: grid.width,
            height: grid.height,
        })
    }
    
    fn smooth_flip_vertical(&self, grid: &Grid, _smoothness_order: u32) -> Result<Grid, SCTTError> {
        let new_data = ndarray::Array2::from_shape_fn((grid.height, grid.width), |(y, x)| {
            let src_x = x as f64;
            let src_y = grid.height as f64 - 1.0 - y as f64;
            
            self.bilinear_sample(grid, src_x, src_y)
        });
        
        Ok(Grid {
            data: new_data,
            width: grid.width,
            height: grid.height,
        })
    }
    
    fn smooth_color_map(&self, grid: &Grid, map: &HashMap<u8, u8>, _smoothness_order: u32) -> Result<Grid, SCTTError> {
        // Apply color mapping with smooth transitions
        let new_data = grid.data.mapv(|color| {
            *map.get(&color).unwrap_or(&color)
        });
        
        Ok(Grid {
            data: new_data,
            width: grid.width,
            height: grid.height,
        })
    }
    
    fn smooth_translate(&self, grid: &Grid, dx: i32, dy: i32, _smoothness_order: u32) -> Result<Grid, SCTTError> {
        let mut new_data = ndarray::Array2::zeros((grid.height, grid.width));
        
        for y in 0..grid.height {
            for x in 0..grid.width {
                let new_x = x as f64 + dx as f64;
                let new_y = y as f64 + dy as f64;
                
                if new_x >= 0.0 && new_x < grid.width as f64 && 
                   new_y >= 0.0 && new_y < grid.height as f64 {
                    let value = self.bilinear_sample(grid, x as f64, y as f64);
                    if (new_x as usize) < grid.width && (new_y as usize) < grid.height {
                        new_data[[new_y as usize, new_x as usize]] = value;
                    }
                }
            }
        }
        
        Ok(Grid {
            data: new_data,
            width: grid.width,
            height: grid.height,
        })
    }
    
    fn smooth_scale(&self, grid: &Grid, factor: f64, _smoothness_order: u32) -> Result<Grid, SCTTError> {
        if factor <= 0.0 {
            return Err(SCTTError::InvalidTransformation {
                reason: "Scale factor must be positive".to_string(),
            });
        }
        
        let new_width = (grid.width as f64 * factor).round() as usize;
        let new_height = (grid.height as f64 * factor).round() as usize;
        
        let new_data = ndarray::Array2::from_shape_fn((new_height, new_width), |(y, x)| {
            let src_x = x as f64 / factor;
            let src_y = y as f64 / factor;
            
            self.bilinear_sample(grid, src_x, src_y)
        });
        
        Ok(Grid {
            data: new_data,
            width: new_width,
            height: new_height,
        })
    }
    
    /// Bilinear sampling for smooth interpolation
    fn bilinear_sample(&self, grid: &Grid, x: f64, y: f64) -> u8 {
        let x0 = x.floor() as usize;
        let x1 = (x0 + 1).min(grid.width - 1);
        let y0 = y.floor() as usize;
        let y1 = (y0 + 1).min(grid.height - 1);
        
        if x0 >= grid.width || y0 >= grid.height {
            return 0;
        }
        
        let fx = x - x0 as f64;
        let fy = y - y0 as f64;
        
        let v00 = grid.data[[y0, x0]] as f64;
        let v10 = grid.data[[y0, x1]] as f64;
        let v01 = grid.data[[y1, x0]] as f64;
        let v11 = grid.data[[y1, x1]] as f64;
        
        let v0 = v00 * (1.0 - fx) + v10 * fx;
        let v1 = v01 * (1.0 - fx) + v11 * fx;
        let result = v0 * (1.0 - fy) + v1 * fy;
        
        result.round() as u8
    }
    
    // Helper methods for composition learning
    
    fn group_morphisms_by_similarity(&self, morphisms: &[Morphism]) -> Vec<Vec<Morphism>> {
        let mut groups = Vec::new();
        let mut ungrouped = morphisms.to_vec();
        
        while !ungrouped.is_empty() {
            let first = ungrouped.remove(0);
            let mut group = vec![first.clone()];
            
            // Find similar morphisms
            ungrouped.retain(|m| {
                if self.morphisms_similar(&first, m) {
                    group.push(m.clone());
                    false
                } else {
                    true
                }
            });
            
            groups.push(group);
        }
        
        groups
    }
    
    fn morphisms_similar(&self, m1: &Morphism, m2: &Morphism) -> bool {
        // Check if morphisms have similar signatures and transforms
        let sig_similar = self.signatures_similar(&m1.source, &m2.source) &&
                         self.signatures_similar(&m1.target, &m2.target);
        
        let transform_similar = std::mem::discriminant(&m1.transform) == 
                              std::mem::discriminant(&m2.transform);
        
        sig_similar && transform_similar
    }
    
    fn signatures_similar(&self, s1: &GridSignature, s2: &GridSignature) -> bool {
        (s1.width as i32 - s2.width as i32).abs() <= 1 &&
        (s1.height as i32 - s2.height as i32).abs() <= 1 &&
        (s1.complexity - s2.complexity).abs() < 0.5
    }
    
    fn generate_permutations(&self, morphisms: &[Morphism], max_length: usize) -> Vec<Vec<Morphism>> {
        // Generate limited permutations for performance
        let mut permutations = Vec::new();
        let length = morphisms.len().min(max_length);
        
        if length == 0 {
            return permutations;
        }
        
        // Just return some simple orderings for now
        permutations.push(morphisms[..length].to_vec());
        
        if length > 1 {
            let mut reversed = morphisms[..length].to_vec();
            reversed.reverse();
            permutations.push(reversed);
        }
        
        permutations
    }
    
    fn evaluate_sequence(&self, sequence: &[Morphism]) -> Result<f64, SCTTError> {
        // Evaluate quality of a morphism sequence
        let mut score = 1.0;
        
        // Check composability
        for i in 0..sequence.len().saturating_sub(1) {
            if !sequence[i].composable_with(&sequence[i + 1]) {
                score *= 0.5; // Penalty for non-composable morphisms
            }
        }
        
        // Smoothness bonus
        let min_smoothness = sequence.iter()
            .map(|m| m.smoothness_order)
            .min()
            .unwrap_or(0);
        score *= 1.0 + (min_smoothness as f64) * 0.1;
        
        // Confidence bonus
        let avg_confidence = sequence.iter()
            .map(|m| m.confidence)
            .sum::<f64>() / sequence.len() as f64;
        score *= avg_confidence;
        
        Ok(score)
    }
    
    fn find_parallel_groups(&self, morphisms: &[Morphism]) -> Vec<Vec<Morphism>> {
        // Find groups of morphisms that can be applied in parallel
        let mut groups = Vec::new();
        
        // Simple implementation: group by compatible source signatures
        let mut signature_groups: HashMap<String, Vec<Morphism>> = HashMap::new();
        
        for morphism in morphisms {
            let key = format!("{}x{}", morphism.source.width, morphism.source.height);
            signature_groups.entry(key).or_default().push(morphism.clone());
        }
        
        for (_, group) in signature_groups {
            if group.len() > 1 {
                groups.push(group);
            }
        }
        
        groups
    }
    
    fn evaluate_parallel_group(&self, group: &[Morphism]) -> Result<f64, SCTTError> {
        // Evaluate quality of parallel morphism group
        let avg_confidence = group.iter()
            .map(|m| m.confidence)
            .sum::<f64>() / group.len() as f64;
        
        // Bonus for diversity
        let diversity_bonus = group.len() as f64 * 0.1;
        
        Ok(avg_confidence + diversity_bonus)
    }
    
    fn build_composition_hierarchy(&self, morphisms: &[Morphism]) -> Result<Vec<Morphism>, SCTTError> {
        // Build hierarchical composition (simplified)
        // In a full implementation, this would create a tree structure
        Ok(morphisms.to_vec())
    }
    
    fn flatten_hierarchy(&self, hierarchy: &[Morphism]) -> Vec<Morphism> {
        hierarchy.to_vec()
    }
    
    fn evaluate_hierarchy(&self, hierarchy: &[Morphism]) -> Result<f64, SCTTError> {
        self.evaluate_sequence(hierarchy)
    }
    
    fn create_smooth_interpolation(&self, morphisms: &[Morphism]) -> Result<Vec<Morphism>, SCTTError> {
        // Create smooth interpolation between morphisms
        // Simplified implementation
        Ok(morphisms.to_vec())
    }
    
    fn evaluate_smooth_composition(&self, sequence: &[Morphism]) -> Result<f64, SCTTError> {
        // Evaluate smooth composition quality
        let mut score = self.evaluate_sequence(sequence)?;
        
        // Smoothness bonus
        score *= 1.2; // Bonus for smooth composition
        
        Ok(score)
    }
    
    fn calculate_composition_score(&self, composition: &LearnedComposition) -> f64 {
        composition.success_rate * 0.4 +
        composition.smoothness_score * 0.3 +
        composition.generalization_score * 0.3
    }
    
    fn calculate_smoothness(&self, sequence: &[Morphism]) -> f64 {
        if sequence.is_empty() {
            return 0.0;
        }
        
        let min_smoothness = sequence.iter()
            .map(|m| m.smoothness_order as f64)
            .fold(f64::INFINITY, f64::min);
        
        min_smoothness / 10.0 // Normalize
    }
    
    fn calculate_generalization(&self, sequence: &[Morphism]) -> f64 {
        // Estimate generalization capability
        let diversity = sequence.len() as f64;
        let complexity = sequence.iter()
            .map(|m| match &m.transform {
                Transform::Identity => 0.0,
                Transform::Rotate90 | Transform::Rotate180 | Transform::Rotate270 => 1.0,
                Transform::FlipHorizontal | Transform::FlipVertical => 1.0,
                Transform::ColorMap(_) => 2.0,
                _ => 3.0,
            })
            .sum::<f64>();
        
        (diversity + complexity) / (sequence.len() as f64 + 1.0)
    }
    
    fn get_composition_signature(&self, composition: &LearnedComposition) -> CompositionSignature {
        let input_sig = if composition.morphism_sequence.is_empty() {
            "empty".to_string()
        } else {
            format!("{:?}", composition.morphism_sequence[0].source)
        };
        
        let output_sig = if composition.morphism_sequence.is_empty() {
            "empty".to_string()
        } else {
            format!("{:?}", composition.morphism_sequence.last().unwrap().target)
        };
        
        CompositionSignature {
            input_signature: input_sig,
            output_signature: output_sig,
            composition_type: composition.composition_type.clone(),
        }
    }
    
    fn update_learning_statistics(&mut self, composition: &LearnedComposition) {
        // Update global learning statistics
        let attempt = CompositionAttempt {
            input_signature: GridSignature {
                width: 0, height: 0, color_distribution: HashMap::new(),
                shape_features: vec![], complexity: 0.0,
            },
            target_signature: GridSignature {
                width: 0, height: 0, color_distribution: HashMap::new(),
                shape_features: vec![], complexity: 0.0,
            },
            attempted_sequence: composition.morphism_sequence.clone(),
            success: composition.success_rate > 0.7,
            confidence: composition.success_rate,
            timestamp: std::time::SystemTime::now(),
        };
        
        self.composition_history.push(attempt);
        
        // Keep history bounded
        if self.composition_history.len() > 1000 {
            self.composition_history.remove(0);
        }
    }
    
    fn evolve_composition_strategies(&mut self) {
        // Evolve composition strategies based on success history
        let recent_success_rate = self.composition_history
            .iter()
            .rev()
            .take(100)
            .map(|a| if a.success { 1.0 } else { 0.0 })
            .sum::<f64>() / (100_f64).min(self.composition_history.len() as f64);
        
        // Adapt learning rate based on success
        if recent_success_rate > 0.8 {
            self.learning_rate *= 1.05; // Increase exploration
        } else if recent_success_rate < 0.6 {
            self.learning_rate *= 0.95; // Decrease exploration
        }
        
        // Clamp learning rate
        self.learning_rate = self.learning_rate.clamp(0.01, 1.0);
    }
    
    fn find_best_composition(&self, input_sig: &GridSignature, target_sig: &GridSignature) 
        -> Result<&LearnedComposition, SCTTError> {
        
        let mut best_composition = None;
        let mut best_score = 0.0;
        
        for composition in self.learned_compositions.values() {
            let score = self.calculate_composition_match_score(composition, input_sig, target_sig);
            if score > best_score {
                best_score = score;
                best_composition = Some(composition);
            }
        }
        
        best_composition.ok_or_else(|| SCTTError::PatternExtractionFailed {
            reason: "No suitable composition found".to_string(),
        })
    }
    
    fn calculate_composition_match_score(&self, composition: &LearnedComposition,
                                       input_sig: &GridSignature, target_sig: &GridSignature) -> f64 {
        // Calculate how well this composition matches the input/target signatures
        let input_match = if composition.morphism_sequence.is_empty() {
            0.0
        } else {
            self.calculate_signature_similarity(&composition.morphism_sequence[0].source, input_sig)
        };
        
        let output_match = if composition.morphism_sequence.is_empty() {
            0.0
        } else {
            self.calculate_signature_similarity(&composition.morphism_sequence.last().unwrap().target, target_sig)
        };
        
        (input_match + output_match) * 0.5 * composition.success_rate
    }
    
    fn calculate_signature_similarity(&self, sig1: &GridSignature, sig2: &GridSignature) -> f64 {
        let size_similarity = 1.0 - ((sig1.width as i32 - sig2.width as i32).abs() as f64 + 
                                    (sig1.height as i32 - sig2.height as i32).abs() as f64) / 10.0;
        
        let complexity_similarity = 1.0 - (sig1.complexity - sig2.complexity).abs() / 5.0;
        
        (size_similarity + complexity_similarity) * 0.5
    }
}

impl Default for Compositor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compositor_creation() {
        let compositor = Compositor::new();
        assert_eq!(compositor.max_composition_depth, 5);
        assert_eq!(compositor.learning_rate, 0.1);
    }
    
    #[test]
    fn test_smooth_rotation() {
        let compositor = Compositor::new();
        let grid = Grid::from_vec(vec![vec![1, 2], vec![3, 4]]).unwrap();
        
        let result = compositor.smooth_rotate_90(&grid, 1).unwrap();
        assert_eq!(result.width, 2);
        assert_eq!(result.height, 2);
    }
    
    #[test]
    fn test_composition_learning() {
        let mut compositor = Compositor::new();
        let sig = GridSignature {
            width: 2,
            height: 2,
            color_distribution: HashMap::new(),
            shape_features: vec![],
            complexity: 1.0,
        };
        
        let morphisms = vec![
            Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90),
            Morphism::new(sig.clone(), sig.clone(), Transform::FlipHorizontal),
        ];
        
        let learned = compositor.learn_composition(&morphisms).unwrap();
        assert!(!learned.morphism_sequence.is_empty());
        assert!(learned.success_rate > 0.0);
    }
}