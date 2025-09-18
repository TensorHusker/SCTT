//! Utility functions for ARC-SCTT system
//! 
//! This module provides common utilities and helper functions used
//! throughout the morphological intelligence system.

use crate::types::*;
use std::collections::HashMap;
use serde_json;

/// Utilities for working with grids
pub struct GridUtils;

impl GridUtils {
    /// Create a grid from a JSON representation
    pub fn from_json(json_str: &str) -> Result<Grid, SCTTError> {
        let data: Vec<Vec<u8>> = serde_json::from_str(json_str)
            .map_err(|e| SCTTError::InvalidTransformation {
                reason: format!("JSON parsing error: {}", e),
            })?;
        
        Grid::from_vec(data)
    }
    
    /// Convert a grid to JSON representation
    pub fn to_json(grid: &Grid) -> Result<String, SCTTError> {
        let data: Vec<Vec<u8>> = (0..grid.height)
            .map(|y| {
                (0..grid.width)
                    .map(|x| grid.get(x, y).unwrap_or(0))
                    .collect()
            })
            .collect();
        
        serde_json::to_string(&data)
            .map_err(|e| SCTTError::InvalidTransformation {
                reason: format!("JSON serialization error: {}", e),
            })
    }
    
    /// Visualize a grid as ASCII art
    pub fn to_ascii(grid: &Grid) -> String {
        let mut result = String::new();
        
        for y in 0..grid.height {
            for x in 0..grid.width {
                let value = grid.get(x, y).unwrap_or(0);
                let char = match value {
                    0 => '.',
                    1 => '#',
                    2 => '@',
                    3 => '*',
                    4 => '+',
                    5 => 'o',
                    6 => '=',
                    7 => '%',
                    8 => '&',
                    9 => '$',
                    _ => '?',
                };
                result.push(char);
            }
            result.push('\n');
        }
        
        result
    }
    
    /// Create a grid with a specific pattern
    pub fn create_pattern(width: usize, height: usize, pattern: GridPattern) -> Result<Grid, SCTTError> {
        let mut grid = Grid::zeros(width, height);
        
        match pattern {
            GridPattern::Checkerboard => {
                for y in 0..height {
                    for x in 0..width {
                        let value = if (x + y) % 2 == 0 { 1 } else { 0 };
                        grid.set(x, y, value)?;
                    }
                }
            },
            GridPattern::Diagonal => {
                for i in 0..width.min(height) {
                    grid.set(i, i, 1)?;
                }
            },
            GridPattern::Border => {
                for x in 0..width {
                    grid.set(x, 0, 1)?;
                    if height > 1 {
                        grid.set(x, height - 1, 1)?;
                    }
                }
                for y in 0..height {
                    grid.set(0, y, 1)?;
                    if width > 1 {
                        grid.set(width - 1, y, 1)?;
                    }
                }
            },
            GridPattern::Center => {
                let center_x = width / 2;
                let center_y = height / 2;
                grid.set(center_x, center_y, 1)?;
            },
            GridPattern::Random(seed) => {
                let mut rng = SimpleRng::new(seed);
                for y in 0..height {
                    for x in 0..width {
                        let value = if rng.next() % 2 == 0 { 1 } else { 0 };
                        grid.set(x, y, value)?;
                    }
                }
            },
        }
        
        Ok(grid)
    }
    
    /// Compare two grids and return similarity score
    pub fn similarity(grid1: &Grid, grid2: &Grid) -> f64 {
        if grid1.width != grid2.width || grid1.height != grid2.height {
            return 0.0;
        }
        
        let total_pixels = (grid1.width * grid1.height) as f64;
        let matching_pixels = (0..grid1.height)
            .flat_map(|y| (0..grid1.width).map(move |x| (x, y)))
            .filter(|&(x, y)| grid1.get(x, y) == grid2.get(x, y))
            .count() as f64;
        
        matching_pixels / total_pixels
    }
    
    /// Extract all unique colors from a grid
    pub fn extract_colors(grid: &Grid) -> Vec<u8> {
        let mut colors = std::collections::HashSet::new();
        
        for y in 0..grid.height {
            for x in 0..grid.width {
                if let Some(color) = grid.get(x, y) {
                    colors.insert(color);
                }
            }
        }
        
        let mut color_vec: Vec<u8> = colors.into_iter().collect();
        color_vec.sort();
        color_vec
    }
}

/// Common grid patterns
#[derive(Debug, Clone)]
pub enum GridPattern {
    Checkerboard,
    Diagonal,
    Border,
    Center,
    Random(u64),
}

/// Simple random number generator for reproducible patterns
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        SimpleRng { state: seed }
    }
    
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        self.state
    }
}

/// Utilities for working with morphisms
pub struct MorphismUtils;

impl MorphismUtils {
    /// Create a morphism from a transformation name
    pub fn from_name(name: &str, source: GridSignature, target: GridSignature) -> Result<Morphism, SCTTError> {
        let transform = match name.to_lowercase().as_str() {
            "identity" => Transform::Identity,
            "rotate90" | "rotate_90" => Transform::Rotate90,
            "rotate180" | "rotate_180" => Transform::Rotate180,
            "rotate270" | "rotate_270" => Transform::Rotate270,
            "flip_horizontal" | "fliph" => Transform::FlipHorizontal,
            "flip_vertical" | "flipv" => Transform::FlipVertical,
            "transpose" => Transform::Transpose,
            "invert" | "invert_colors" => Transform::InvertColors,
            _ => return Err(SCTTError::InvalidTransformation {
                reason: format!("Unknown transformation: {}", name),
            }),
        };
        
        Ok(Morphism::new(source, target, transform))
    }
    
    /// Get the inverse of a morphism (if it exists)
    pub fn inverse(morphism: &Morphism) -> Option<Morphism> {
        let inverse_transform = match &morphism.transform {
            Transform::Identity => Transform::Identity,
            Transform::Rotate90 => Transform::Rotate270,
            Transform::Rotate180 => Transform::Rotate180,
            Transform::Rotate270 => Transform::Rotate90,
            Transform::FlipHorizontal => Transform::FlipHorizontal,
            Transform::FlipVertical => Transform::FlipVertical,
            Transform::Transpose => Transform::Transpose,
            Transform::InvertColors => Transform::InvertColors,
            Transform::ColorMap(map) => {
                // Create inverse color mapping
                let mut inverse_map = HashMap::new();
                for (&k, &v) in map {
                    inverse_map.insert(v, k);
                }
                Transform::ColorMap(inverse_map)
            },
            Transform::Translate { dx, dy } => Transform::Translate { dx: -dx, dy: -dy },
            Transform::Scale { factor } => {
                if *factor != 0.0 {
                    Transform::Scale { factor: 1.0 / factor }
                } else {
                    return None;
                }
            },
            _ => return None, // No inverse for complex transforms
        };
        
        Some(Morphism::new(
            morphism.target.clone(),
            morphism.source.clone(),
            inverse_transform,
        ))
    }
    
    /// Check if a morphism is invertible
    pub fn is_invertible(morphism: &Morphism) -> bool {
        Self::inverse(morphism).is_some()
    }
    
    /// Compute the complexity of a morphism
    pub fn complexity(morphism: &Morphism) -> f64 {
        match &morphism.transform {
            Transform::Identity => 0.0,
            Transform::Rotate90 | Transform::Rotate180 | Transform::Rotate270 => 1.0,
            Transform::FlipHorizontal | Transform::FlipVertical => 1.0,
            Transform::Transpose => 1.0,
            Transform::ColorMap(map) => map.len() as f64,
            Transform::InvertColors => 1.0,
            Transform::Translate { .. } => 2.0,
            Transform::Scale { .. } => 2.0,
            Transform::Crop { .. } => 3.0,
            Transform::Sequence(transforms) => transforms.iter().map(|t| {
                let dummy_morphism = Morphism::new(
                    GridSignature { width: 0, height: 0, color_distribution: HashMap::new(), shape_features: vec![], complexity: 0.0 },
                    GridSignature { width: 0, height: 0, color_distribution: HashMap::new(), shape_features: vec![], complexity: 0.0 },
                    t.clone(),
                );
                Self::complexity(&dummy_morphism)
            }).sum(),
            _ => 5.0, // High complexity for unknown transforms
        }
    }
}

/// Utilities for working with patterns
pub struct PatternUtils;

impl PatternUtils {
    /// Generate example ARC puzzles for testing
    pub fn generate_test_puzzle(puzzle_type: TestPuzzleType) -> ARCPuzzle {
        match puzzle_type {
            TestPuzzleType::Simple => {
                let input1 = Grid::from_vec(vec![vec![1, 0], vec![0, 1]]).unwrap();
                let output1 = Grid::from_vec(vec![vec![0, 1], vec![1, 0]]).unwrap();
                
                let input2 = Grid::from_vec(vec![vec![2, 0], vec![0, 2]]).unwrap();
                let output2 = Grid::from_vec(vec![vec![0, 2], vec![2, 0]]).unwrap();
                
                let test_input = Grid::from_vec(vec![vec![3, 0], vec![0, 3]]).unwrap();
                let test_output = Grid::from_vec(vec![vec![0, 3], vec![3, 0]]).unwrap();
                
                ARCPuzzle {
                    id: "test_simple".to_string(),
                    examples: vec![(input1, output1), (input2, output2)],
                    test_input,
                    test_output: Some(test_output),
                }
            },
            TestPuzzleType::Rotation => {
                let input1 = Grid::from_vec(vec![
                    vec![1, 0, 0],
                    vec![0, 0, 0],
                    vec![0, 0, 0],
                ]).unwrap();
                let output1 = Grid::from_vec(vec![
                    vec![0, 0, 0],
                    vec![0, 0, 0],
                    vec![1, 0, 0],
                ]).unwrap();
                
                let test_input = Grid::from_vec(vec![
                    vec![2, 0, 0],
                    vec![0, 0, 0],
                    vec![0, 0, 0],
                ]).unwrap();
                
                ARCPuzzle {
                    id: "test_rotation".to_string(),
                    examples: vec![(input1, output1)],
                    test_input,
                    test_output: None,
                }
            },
            TestPuzzleType::ColorMapping => {
                let input1 = Grid::from_vec(vec![vec![1, 2], vec![3, 4]]).unwrap();
                let output1 = Grid::from_vec(vec![vec![5, 6], vec![7, 8]]).unwrap();
                
                let test_input = Grid::from_vec(vec![vec![1, 3], vec![2, 4]]).unwrap();
                
                ARCPuzzle {
                    id: "test_color_mapping".to_string(),
                    examples: vec![(input1, output1)],
                    test_input,
                    test_output: None,
                }
            },
        }
    }
    
    /// Validate that a puzzle solution is correct
    pub fn validate_solution(puzzle: &ARCPuzzle, solution: &Grid) -> bool {
        if let Some(ref expected) = puzzle.test_output {
            GridUtils::similarity(solution, expected) > 0.99
        } else {
            true // Can't validate without expected output
        }
    }
    
    /// Extract common patterns from a set of morphisms
    pub fn extract_common_patterns(morphisms: &[Morphism]) -> Vec<CommonPattern> {
        let mut patterns = Vec::new();
        
        // Group by transformation type
        let mut transform_groups: HashMap<String, Vec<&Morphism>> = HashMap::new();
        
        for morphism in morphisms {
            let key = format!("{:?}", std::mem::discriminant(&morphism.transform));
            transform_groups.entry(key).or_default().push(morphism);
        }
        
        // Extract patterns from groups with multiple instances
        for (transform_type, group) in transform_groups {
            if group.len() >= 2 {
                let avg_confidence = group.iter().map(|m| m.confidence).sum::<f64>() / group.len() as f64;
                
                patterns.push(CommonPattern {
                    pattern_type: transform_type,
                    frequency: group.len(),
                    confidence: avg_confidence,
                    examples: group.into_iter().cloned().collect(),
                });
            }
        }
        
        patterns.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        patterns
    }
}

/// Types of test puzzles
#[derive(Debug, Clone)]
pub enum TestPuzzleType {
    Simple,
    Rotation,
    ColorMapping,
}

/// A common pattern extracted from multiple morphisms
#[derive(Debug, Clone)]
pub struct CommonPattern {
    pub pattern_type: String,
    pub frequency: usize,
    pub confidence: f64,
    pub examples: Vec<Morphism>,
}

/// Performance utilities
pub struct PerformanceUtils;

impl PerformanceUtils {
    /// Measure execution time of a function
    pub fn time_execution<F, R>(f: F) -> (R, std::time::Duration)
    where
        F: FnOnce() -> R,
    {
        let start = std::time::Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }
    
    /// Create a simple benchmark report
    pub fn benchmark_morphism_application(morphism: &Morphism, grid: &Grid, iterations: usize) -> BenchmarkReport {
        use crate::kernel::Kernel;
        
        let kernel = Kernel::new();
        let mut durations = Vec::new();
        let mut success_count = 0;
        
        for _ in 0..iterations {
            let (result, duration) = Self::time_execution(|| {
                kernel.apply_transform(grid, &morphism.transform)
            });
            
            durations.push(duration);
            if result.is_ok() {
                success_count += 1;
            }
        }
        
        let total_time: std::time::Duration = durations.iter().sum();
        let avg_time = total_time / iterations as u32;
        let min_time = durations.iter().min().unwrap().clone();
        let max_time = durations.iter().max().unwrap().clone();
        
        BenchmarkReport {
            operation: format!("Morphism application: {:?}", morphism.transform),
            iterations,
            success_rate: success_count as f64 / iterations as f64,
            avg_time,
            min_time,
            max_time,
            total_time,
        }
    }
}

/// Benchmark report
#[derive(Debug, Clone)]
pub struct BenchmarkReport {
    pub operation: String,
    pub iterations: usize,
    pub success_rate: f64,
    pub avg_time: std::time::Duration,
    pub min_time: std::time::Duration,
    pub max_time: std::time::Duration,
    pub total_time: std::time::Duration,
}

impl std::fmt::Display for BenchmarkReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Benchmark Report: {}\n\
             Iterations: {}\n\
             Success Rate: {:.2}%\n\
             Average Time: {:?}\n\
             Min Time: {:?}\n\
             Max Time: {:?}\n\
             Total Time: {:?}",
            self.operation,
            self.iterations,
            self.success_rate * 100.0,
            self.avg_time,
            self.min_time,
            self.max_time,
            self.total_time
        )
    }
}

/// Debugging utilities
pub struct DebugUtils;

impl DebugUtils {
    /// Print detailed information about a morphism
    pub fn print_morphism_info(morphism: &Morphism) {
        println!("=== Morphism Information ===");
        println!("Transform: {:?}", morphism.transform);
        println!("Confidence: {:.4}", morphism.confidence);
        println!("Smoothness Order: {}", morphism.smoothness_order);
        println!("Source Signature:");
        Self::print_signature(&morphism.source, 2);
        println!("Target Signature:");
        Self::print_signature(&morphism.target, 2);
        println!("Complexity: {:.4}", MorphismUtils::complexity(morphism));
        println!("Invertible: {}", MorphismUtils::is_invertible(morphism));
        println!("=============================");
    }
    
    /// Print grid signature information
    pub fn print_signature(signature: &GridSignature, indent: usize) {
        let indent_str = " ".repeat(indent);
        println!("{}Dimensions: {}x{}", indent_str, signature.width, signature.height);
        println!("{}Colors: {:?}", indent_str, signature.color_distribution);
        println!("{}Shape Features: {:?}", indent_str, signature.shape_features);
        println!("{}Complexity: {:.4}", indent_str, signature.complexity);
    }
    
    /// Visualize the transformation process
    pub fn visualize_transformation(input: &Grid, output: &Grid, morphism: &Morphism) {
        println!("=== Transformation Visualization ===");
        println!("Transform: {:?}", morphism.transform);
        println!("Confidence: {:.4}", morphism.confidence);
        println!();
        
        println!("INPUT:");
        print!("{}", GridUtils::to_ascii(input));
        println!();
        
        println!("OUTPUT:");
        print!("{}", GridUtils::to_ascii(output));
        println!();
        
        let similarity = GridUtils::similarity(input, output);
        println!("Similarity: {:.4}", similarity);
        println!("=====================================");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_grid_utils_ascii() {
        let grid = Grid::from_vec(vec![vec![0, 1], vec![1, 0]]).unwrap();
        let ascii = GridUtils::to_ascii(&grid);
        assert_eq!(ascii, ".#\n#.\n");
    }
    
    #[test]
    fn test_grid_utils_similarity() {
        let grid1 = Grid::from_vec(vec![vec![1, 0], vec![0, 1]]).unwrap();
        let grid2 = Grid::from_vec(vec![vec![1, 0], vec![0, 1]]).unwrap();
        let grid3 = Grid::from_vec(vec![vec![0, 1], vec![1, 0]]).unwrap();
        
        assert_eq!(GridUtils::similarity(&grid1, &grid2), 1.0);
        assert_eq!(GridUtils::similarity(&grid1, &grid3), 0.0);
    }
    
    #[test]
    fn test_morphism_utils_inverse() {
        let sig = GridSignature {
            width: 2, height: 2, color_distribution: HashMap::new(),
            shape_features: vec![], complexity: 0.0,
        };
        
        let morphism = Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90);
        let inverse = MorphismUtils::inverse(&morphism).unwrap();
        
        assert_eq!(inverse.transform, Transform::Rotate270);
    }
    
    #[test]
    fn test_pattern_utils_test_puzzle() {
        let puzzle = PatternUtils::generate_test_puzzle(TestPuzzleType::Simple);
        assert_eq!(puzzle.examples.len(), 2);
        assert_eq!(puzzle.id, "test_simple");
    }
    
    #[test]
    fn test_performance_utils_timing() {
        let (result, duration) = PerformanceUtils::time_execution(|| {
            std::thread::sleep(std::time::Duration::from_millis(10));
            42
        });
        
        assert_eq!(result, 42);
        assert!(duration >= std::time::Duration::from_millis(10));
    }
}