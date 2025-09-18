//! Morphological Intelligence: Pattern Extraction and Decomposition
//! 
//! This module implements the revolutionary concept of treating visual patterns
//! as morphisms in a mathematical category, enabling compositional reasoning
//! about transformations and their algebraic structure.

use crate::types::*;
use crate::kernel::Kernel;
use std::collections::{HashMap, HashSet, VecDeque};
use rayon::prelude::*;
// use cached::proc_macro::cached;

/// Extracts morphisms (patterns) from visual transformations
#[derive(Debug, Clone)]
pub struct PatternExtractor {
    kernel: Kernel,
    atomic_decomposition_depth: usize,
    similarity_threshold: f64,
    pattern_cache: HashMap<(Grid, Grid), Vec<Morphism>>,
}

impl PatternExtractor {
    /// Create a new pattern extractor
    pub fn new() -> Self {
        PatternExtractor {
            kernel: Kernel::new(),
            atomic_decomposition_depth: 3,
            similarity_threshold: 0.95,
            pattern_cache: HashMap::new(),
        }
    }
    
    /// Extract morphisms from a set of input-output examples
    pub fn extract_morphisms(&mut self, examples: &[(Grid, Grid)]) -> Result<Vec<Morphism>, SCTTError> {
        let mut all_morphisms = Vec::new();
        
        // Extract morphisms from each example
        for (input, output) in examples {
            let morphisms = self.extract_single_morphism(input, output)?;
            all_morphisms.extend(morphisms);
        }
        
        // Find common patterns across examples
        let common_patterns = self.find_common_patterns(&all_morphisms)?;
        
        // Decompose complex morphisms into atomic operations
        let atomic_morphisms = self.decompose_to_atomic(&common_patterns)?;
        
        // Discover generating morphisms (minimal basis)
        let generators = self.find_generators(&atomic_morphisms)?;
        
        Ok(generators)
    }
    
    /// Extract morphism(s) between a single input-output pair
    pub fn extract_single_morphism(&mut self, input: &Grid, output: &Grid) -> Result<Vec<Morphism>, SCTTError> {
        // Check cache first
        let cache_key = (input.clone(), output.clone());
        if let Some(cached) = self.pattern_cache.get(&cache_key) {
            return Ok(cached.clone());
        }
        
        let mut morphisms = Vec::new();
        
        // Try direct transformation
        if let Ok(direct_morphism) = self.kernel.extract_morphism(input, output) {
            if direct_morphism.confidence > self.similarity_threshold {
                morphisms.push(direct_morphism);
            }
        }
        
        // Try decomposed transformations
        let decomposed = self.extract_decomposed_morphisms(input, output)?;
        morphisms.extend(decomposed);
        
        // Try shape-based morphisms
        let shape_morphisms = self.extract_shape_morphisms(input, output)?;
        morphisms.extend(shape_morphisms);
        
        // Try color-based morphisms
        let color_morphisms = self.extract_color_morphisms(input, output)?;
        morphisms.extend(color_morphisms);
        
        // Try spatial morphisms
        let spatial_morphisms = self.extract_spatial_morphisms(input, output)?;
        morphisms.extend(spatial_morphisms);
        
        // Sort by confidence and keep the best ones
        morphisms.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        morphisms.truncate(5); // Keep top 5 candidates
        
        // Cache result
        self.pattern_cache.insert(cache_key, morphisms.clone());
        
        Ok(morphisms)
    }
    
    /// Find common patterns across multiple morphisms
    fn find_common_patterns(&self, morphisms: &[Morphism]) -> Result<Vec<Morphism>, SCTTError> {
        let mut pattern_groups: HashMap<String, Vec<&Morphism>> = HashMap::new();
        
        // Group morphisms by their transformation type
        for morphism in morphisms {
            let pattern_key = self.get_pattern_key(&morphism.transform);
            pattern_groups.entry(pattern_key).or_default().push(morphism);
        }
        
        let mut common_patterns = Vec::new();
        
        // Extract patterns that appear in multiple examples
        for (pattern_type, group) in pattern_groups {
            if group.len() >= 2 { // Appears in at least 2 examples
                // Create a canonical morphism for this pattern
                let canonical = self.create_canonical_morphism(&group)?;
                common_patterns.push(canonical);
            }
        }
        
        Ok(common_patterns)
    }
    
    /// Decompose complex morphisms into atomic operations
    fn decompose_to_atomic(&self, morphisms: &[Morphism]) -> Result<Vec<Morphism>, SCTTError> {
        let mut atomic_morphisms = Vec::new();
        
        for morphism in morphisms {
            let decomposed = self.decompose_morphism(morphism, self.atomic_decomposition_depth)?;
            atomic_morphisms.extend(decomposed);
        }
        
        // Remove duplicates and low-confidence morphisms
        atomic_morphisms.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        atomic_morphisms.dedup_by(|a, b| self.morphisms_equivalent(a, b));
        
        Ok(atomic_morphisms)
    }
    
    /// Find generating morphisms (minimal basis for the pattern space)
    fn find_generators(&self, morphisms: &[Morphism]) -> Result<Vec<Morphism>, SCTTError> {
        let mut generators = Vec::new();
        let mut covered_patterns = HashSet::new();
        
        // Use a greedy algorithm to find minimal generating set
        for morphism in morphisms {
            let pattern_signature = self.get_morphism_signature(morphism);
            
            if !covered_patterns.contains(&pattern_signature) {
                generators.push(morphism.clone());
                
                // Mark patterns covered by this generator
                let covered = self.get_patterns_covered_by(morphism);
                covered_patterns.extend(covered);
            }
        }
        
        Ok(generators)
    }
    
    /// Extract decomposed morphisms by trying intermediate transformations
    fn extract_decomposed_morphisms(&self, input: &Grid, output: &Grid) -> Result<Vec<Morphism>, SCTTError> {
        let mut morphisms = Vec::new();
        
        // Try two-step transformations
        let intermediate_transforms = vec![
            Transform::Rotate90,
            Transform::Rotate180,
            Transform::FlipHorizontal,
            Transform::FlipVertical,
        ];
        
        for first_transform in &intermediate_transforms {
            if let Ok(intermediate) = self.kernel.apply_transform(input, first_transform) {
                if let Ok(second_morphism) = self.kernel.extract_morphism(&intermediate, output) {
                    // Create composite morphism
                    let first_sig = input.signature();
                    let intermediate_sig = intermediate.signature();
                    let first_morphism = Morphism::new(first_sig, intermediate_sig, first_transform.clone());
                    
                    if let Ok(composed) = first_morphism.compose(&second_morphism) {
                        morphisms.push(composed);
                    }
                }
            }
        }
        
        Ok(morphisms)
    }
    
    /// Extract shape-based morphisms
    fn extract_shape_morphisms(&self, input: &Grid, output: &Grid) -> Result<Vec<Morphism>, SCTTError> {
        let mut morphisms = Vec::new();
        
        // Analyze shape changes
        let input_shapes = self.extract_shapes(input);
        let output_shapes = self.extract_shapes(output);
        
        // Look for shape transformations
        if input_shapes.len() == output_shapes.len() {
            // Same number of shapes - likely shape modification
            for (i, (input_shape, output_shape)) in input_shapes.iter().zip(output_shapes.iter()).enumerate() {
                if let Some(shape_transform) = self.infer_shape_transformation(input_shape, output_shape) {
                    let morphism = Morphism::new(
                        input.signature(),
                        output.signature(),
                        shape_transform,
                    );
                    morphisms.push(morphism);
                }
            }
        }
        
        Ok(morphisms)
    }
    
    /// Extract color-based morphisms
    fn extract_color_morphisms(&self, input: &Grid, output: &Grid) -> Result<Vec<Morphism>, SCTTError> {
        let mut morphisms = Vec::new();
        
        // Check if it's just a color transformation
        if input.width == output.width && input.height == output.height {
            let mut color_mapping = HashMap::new();
            let mut consistent = true;
            
            for y in 0..input.height {
                for x in 0..input.width {
                    let input_color = input.get(x, y).unwrap();
                    let output_color = output.get(x, y).unwrap();
                    
                    if let Some(&existing) = color_mapping.get(&input_color) {
                        if existing != output_color {
                            consistent = false;
                            break;
                        }
                    } else {
                        color_mapping.insert(input_color, output_color);
                    }
                }
                if !consistent { break; }
            }
            
            if consistent && !color_mapping.is_empty() {
                let transform = Transform::ColorMap(color_mapping);
                let morphism = Morphism::new(
                    input.signature(),
                    output.signature(),
                    transform,
                );
                morphisms.push(morphism);
            }
        }
        
        Ok(morphisms)
    }
    
    /// Extract spatial morphisms (translation, scaling, etc.)
    fn extract_spatial_morphisms(&self, input: &Grid, output: &Grid) -> Result<Vec<Morphism>, SCTTError> {
        let mut morphisms = Vec::new();
        
        // Check for translation
        if let Some((dx, dy)) = self.detect_translation(input, output) {
            let transform = Transform::Translate { dx, dy };
            let morphism = Morphism::new(
                input.signature(),
                output.signature(),
                transform,
            );
            morphisms.push(morphism);
        }
        
        // Check for scaling
        if let Some(scale_factor) = self.detect_scaling(input, output) {
            let transform = Transform::Scale { factor: scale_factor };
            let morphism = Morphism::new(
                input.signature(),
                output.signature(),
                transform,
            );
            morphisms.push(morphism);
        }
        
        // Check for cropping
        if let Some((x, y, width, height)) = self.detect_cropping(input, output) {
            let transform = Transform::Crop { x, y, width, height };
            let morphism = Morphism::new(
                input.signature(),
                output.signature(),
                transform,
            );
            morphisms.push(morphism);
        }
        
        Ok(morphisms)
    }
    
    /// Extract connected shapes from a grid
    fn extract_shapes(&self, grid: &Grid) -> Vec<Shape> {
        let mut shapes = Vec::new();
        let mut visited = vec![vec![false; grid.width]; grid.height];
        
        for y in 0..grid.height {
            for x in 0..grid.width {
                if !visited[y][x] && grid.get(x, y).unwrap() != 0 {
                    let shape = self.extract_connected_shape(grid, x, y, &mut visited);
                    shapes.push(shape);
                }
            }
        }
        
        shapes
    }
    
    /// Extract a single connected shape using flood fill
    fn extract_connected_shape(&self, grid: &Grid, start_x: usize, start_y: usize, 
                              visited: &mut [Vec<bool>]) -> Shape {
        let mut pixels = Vec::new();
        let mut queue = VecDeque::new();
        let target_color = grid.get(start_x, start_y).unwrap();
        
        queue.push_back((start_x, start_y));
        visited[start_y][start_x] = true;
        
        while let Some((x, y)) = queue.pop_front() {
            pixels.push((x, y));
            
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
        
        Shape {
            pixels: pixels.clone(),
            color: target_color,
            bounding_box: self.calculate_bounding_box(&pixels),
        }
    }
    
    /// Calculate bounding box of a set of pixels
    fn calculate_bounding_box(&self, pixels: &[(usize, usize)]) -> (usize, usize, usize, usize) {
        if pixels.is_empty() {
            return (0, 0, 0, 0);
        }
        
        let min_x = pixels.iter().map(|(x, _)| *x).min().unwrap();
        let max_x = pixels.iter().map(|(x, _)| *x).max().unwrap();
        let min_y = pixels.iter().map(|(_, y)| *y).min().unwrap();
        let max_y = pixels.iter().map(|(_, y)| *y).max().unwrap();
        
        (min_x, min_y, max_x - min_x + 1, max_y - min_y + 1)
    }
    
    /// Infer transformation between two shapes
    fn infer_shape_transformation(&self, input_shape: &Shape, output_shape: &Shape) -> Option<Transform> {
        // Check if it's just a color change
        if input_shape.pixels.len() == output_shape.pixels.len() && 
           input_shape.bounding_box == output_shape.bounding_box {
            if input_shape.color != output_shape.color {
                let mut color_map = HashMap::new();
                color_map.insert(input_shape.color, output_shape.color);
                return Some(Transform::ColorMap(color_map));
            }
        }
        
        // Check for translation
        if input_shape.pixels.len() == output_shape.pixels.len() {
            let input_center = self.calculate_center(&input_shape.pixels);
            let output_center = self.calculate_center(&output_shape.pixels);
            
            let dx = output_center.0 as i32 - input_center.0 as i32;
            let dy = output_center.1 as i32 - input_center.1 as i32;
            
            if dx != 0 || dy != 0 {
                return Some(Transform::Translate { dx, dy });
            }
        }
        
        None
    }
    
    /// Calculate center of a set of pixels
    fn calculate_center(&self, pixels: &[(usize, usize)]) -> (f64, f64) {
        if pixels.is_empty() {
            return (0.0, 0.0);
        }
        
        let sum_x: usize = pixels.iter().map(|(x, _)| *x).sum();
        let sum_y: usize = pixels.iter().map(|(_, y)| *y).sum();
        
        (sum_x as f64 / pixels.len() as f64, sum_y as f64 / pixels.len() as f64)
    }
    
    /// Detect translation between two grids
    fn detect_translation(&self, input: &Grid, output: &Grid) -> Option<(i32, i32)> {
        if input.width != output.width || input.height != output.height {
            return None;
        }
        
        // Find first non-zero pixel in input
        let mut input_pixel = None;
        for y in 0..input.height {
            for x in 0..input.width {
                if input.get(x, y).unwrap() != 0 {
                    input_pixel = Some((x, y));
                    break;
                }
            }
            if input_pixel.is_some() { break; }
        }
        
        // Find corresponding pixel in output
        if let Some((ix, iy)) = input_pixel {
            let input_color = input.get(ix, iy).unwrap();
            
            for y in 0..output.height {
                for x in 0..output.width {
                    if output.get(x, y).unwrap() == input_color {
                        let dx = x as i32 - ix as i32;
                        let dy = y as i32 - iy as i32;
                        
                        // Verify this translation works for the whole pattern
                        if self.verify_translation(input, output, dx, dy) {
                            return Some((dx, dy));
                        }
                    }
                }
            }
        }
        
        None
    }
    
    /// Verify that a translation works for the entire pattern
    fn verify_translation(&self, input: &Grid, output: &Grid, dx: i32, dy: i32) -> bool {
        for y in 0..input.height {
            for x in 0..input.width {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                let input_color = input.get(x, y).unwrap();
                
                if nx >= 0 && ny >= 0 && 
                   (nx as usize) < output.width && (ny as usize) < output.height {
                    let output_color = output.get(nx as usize, ny as usize).unwrap();
                    if input_color != 0 && input_color != output_color {
                        return false;
                    }
                } else if input_color != 0 {
                    return false; // Non-zero pixel would be out of bounds
                }
            }
        }
        true
    }
    
    /// Detect scaling between two grids
    fn detect_scaling(&self, input: &Grid, output: &Grid) -> Option<f64> {
        let width_ratio = output.width as f64 / input.width as f64;
        let height_ratio = output.height as f64 / input.height as f64;
        
        // Only consider uniform scaling
        if (width_ratio - height_ratio).abs() < 0.1 {
            Some(width_ratio)
        } else {
            None
        }
    }
    
    /// Detect cropping between two grids
    fn detect_cropping(&self, input: &Grid, output: &Grid) -> Option<(usize, usize, usize, usize)> {
        if output.width >= input.width || output.height >= input.height {
            return None;
        }
        
        // Try to find where the output matches a region in the input
        for y in 0..=(input.height - output.height) {
            for x in 0..=(input.width - output.width) {
                if self.verify_crop(input, output, x, y) {
                    return Some((x, y, output.width, output.height));
                }
            }
        }
        
        None
    }
    
    /// Verify that a crop operation produces the expected output
    fn verify_crop(&self, input: &Grid, output: &Grid, start_x: usize, start_y: usize) -> bool {
        for y in 0..output.height {
            for x in 0..output.width {
                let input_color = input.get(start_x + x, start_y + y).unwrap();
                let output_color = output.get(x, y).unwrap();
                if input_color != output_color {
                    return false;
                }
            }
        }
        true
    }
    
    // Helper methods for pattern analysis
    
    fn get_pattern_key(&self, transform: &Transform) -> String {
        match transform {
            Transform::Identity => "identity".to_string(),
            Transform::Rotate90 => "rotate_90".to_string(),
            Transform::Rotate180 => "rotate_180".to_string(),
            Transform::Rotate270 => "rotate_270".to_string(),
            Transform::FlipHorizontal => "flip_h".to_string(),
            Transform::FlipVertical => "flip_v".to_string(),
            Transform::Transpose => "transpose".to_string(),
            Transform::ColorMap(_) => "color_map".to_string(),
            Transform::InvertColors => "invert".to_string(),
            Transform::Translate { .. } => "translate".to_string(),
            Transform::Scale { .. } => "scale".to_string(),
            Transform::Crop { .. } => "crop".to_string(),
            _ => "complex".to_string(),
        }
    }
    
    fn create_canonical_morphism(&self, group: &[&Morphism]) -> Result<Morphism, SCTTError> {
        // Create a canonical morphism by averaging properties
        if group.is_empty() {
            return Err(SCTTError::PatternExtractionFailed {
                reason: "Empty group for canonical morphism".to_string(),
            });
        }
        
        let first = group[0];
        let avg_confidence = group.iter().map(|m| m.confidence).sum::<f64>() / group.len() as f64;
        
        Ok(Morphism {
            source: first.source.clone(),
            target: first.target.clone(),
            transform: first.transform.clone(),
            confidence: avg_confidence,
            smoothness_order: first.smoothness_order,
        })
    }
    
    fn decompose_morphism(&self, morphism: &Morphism, max_depth: usize) -> Result<Vec<Morphism>, SCTTError> {
        if max_depth == 0 {
            return Ok(vec![morphism.clone()]);
        }
        
        match &morphism.transform {
            Transform::Sequence(transforms) => {
                // Already decomposed
                let mut atomic = Vec::new();
                for transform in transforms {
                    let sub_morphism = Morphism::new(
                        morphism.source.clone(),
                        morphism.target.clone(),
                        transform.clone(),
                    );
                    let sub_decomposed = self.decompose_morphism(&sub_morphism, max_depth - 1)?;
                    atomic.extend(sub_decomposed);
                }
                Ok(atomic)
            },
            _ => {
                // Try to decompose into simpler operations
                Ok(vec![morphism.clone()])
            }
        }
    }
    
    fn morphisms_equivalent(&self, m1: &Morphism, m2: &Morphism) -> bool {
        m1.transform == m2.transform && 
        (m1.confidence - m2.confidence).abs() < 0.01
    }
    
    fn get_morphism_signature(&self, morphism: &Morphism) -> String {
        format!("{:?}_{:.2}", morphism.transform, morphism.confidence)
    }
    
    fn get_patterns_covered_by(&self, morphism: &Morphism) -> Vec<String> {
        // Return patterns that this morphism can generate
        vec![self.get_morphism_signature(morphism)]
    }
}

/// Represents a connected shape in a grid
#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    pub pixels: Vec<(usize, usize)>,
    pub color: u8,
    pub bounding_box: (usize, usize, usize, usize), // (x, y, width, height)
}

impl Default for PatternExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pattern_extractor_creation() {
        let extractor = PatternExtractor::new();
        assert_eq!(extractor.atomic_decomposition_depth, 3);
    }
    
    #[test]
    fn test_shape_extraction() {
        let extractor = PatternExtractor::new();
        let grid = Grid::from_vec(vec![
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
            vec![2, 2, 0, 0],
        ]).unwrap();
        
        let shapes = extractor.extract_shapes(&grid);
        assert_eq!(shapes.len(), 2); // Two separate shapes
    }
    
    #[test]
    fn test_translation_detection() {
        let extractor = PatternExtractor::new();
        let input = Grid::from_vec(vec![
            vec![1, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ]).unwrap();
        let output = Grid::from_vec(vec![
            vec![0, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 0],
        ]).unwrap();
        
        let translation = extractor.detect_translation(&input, &output);
        assert_eq!(translation, Some((1, 1)));
    }
    
    #[test]
    fn test_color_morphism_extraction() {
        let mut extractor = PatternExtractor::new();
        let input = Grid::from_vec(vec![vec![1, 2], vec![3, 4]]).unwrap();
        let output = Grid::from_vec(vec![vec![5, 6], vec![7, 8]]).unwrap();
        
        let morphisms = extractor.extract_color_morphisms(&input, &output).unwrap();
        assert!(!morphisms.is_empty());
        
        if let Transform::ColorMap(map) = &morphisms[0].transform {
            assert_eq!(map.get(&1), Some(&5));
            assert_eq!(map.get(&2), Some(&6));
        }
    }
}