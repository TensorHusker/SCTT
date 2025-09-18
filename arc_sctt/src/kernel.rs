//! The SCTT Kernel: Core morphological intelligence engine
//! 
//! This module implements the fundamental operations for treating
//! visual patterns as mathematical objects in a smooth category.

use crate::types::*;
use ndarray::Array2;
use std::collections::HashMap;
// use cached::proc_macro::cached;

/// The core kernel for morphological pattern operations
#[derive(Debug, Clone)]
pub struct Kernel {
    dimension: usize,
    smoothness_threshold: f64,
    composition_cache: HashMap<(Transform, Transform), Transform>,
}

impl Kernel {
    /// Create a new kernel with default parameters
    pub fn new() -> Self {
        Kernel {
            dimension: 2,
            smoothness_threshold: 0.01,
            composition_cache: HashMap::new(),
        }
    }
    
    /// Get the working dimension of this kernel
    pub fn dimension(&self) -> usize {
        self.dimension
    }
    
    /// Apply a transformation to a grid
    pub fn apply_transform(&self, grid: &Grid, transform: &Transform) -> Result<Grid, SCTTError> {
        match transform {
            Transform::Identity => Ok(grid.clone()),
            
            Transform::Rotate90 => self.rotate_90(grid),
            Transform::Rotate180 => self.rotate_180(grid),
            Transform::Rotate270 => self.rotate_270(grid),
            
            Transform::FlipHorizontal => self.flip_horizontal(grid),
            Transform::FlipVertical => self.flip_vertical(grid),
            Transform::Transpose => self.transpose(grid),
            
            Transform::ColorMap(map) => self.apply_color_map(grid, map),
            Transform::InvertColors => self.invert_colors(grid),
            
            Transform::Translate { dx, dy } => self.translate(grid, *dx, *dy),
            Transform::ScaleInt { numerator, denominator } => {
                let factor = *numerator as f64 / *denominator as f64;
                self.scale(grid, factor)
            },
            Transform::Crop { x, y, width, height } => self.crop(grid, *x, *y, *width, *height),
            
            Transform::FillShape { color, shape_filter } => self.fill_shape(grid, *color, shape_filter),
            Transform::ConnectComponents { color } => self.connect_components(grid, *color),
            Transform::ExtractSubpattern { pattern } => self.extract_subpattern(grid, pattern),
            
            Transform::Sequence(transforms) => self.apply_sequence(grid, transforms),
            Transform::Parallel(transforms) => self.apply_parallel(grid, transforms),
        }
    }
    
    /// Extract the morphism between two grids
    pub fn extract_morphism(&self, source: &Grid, target: &Grid) -> Result<Morphism, SCTTError> {
        let source_sig = source.signature();
        let target_sig = target.signature();
        
        // Try different transformations to find the best match
        let candidates = vec![
            Transform::Identity,
            Transform::Rotate90,
            Transform::Rotate180,
            Transform::Rotate270,
            Transform::FlipHorizontal,
            Transform::FlipVertical,
            Transform::Transpose,
        ];
        
        let mut best_morphism = None;
        let mut best_score = 0.0;
        
        for transform in candidates {
            if let Ok(result) = self.apply_transform(source, &transform) {
                let score = self.calculate_similarity(&result, target);
                if score > best_score {
                    best_score = score;
                    best_morphism = Some(Morphism::new(
                        source_sig.clone(),
                        target_sig.clone(),
                        transform,
                    ));
                }
            }
        }
        
        // If no simple transformation works, try to find a color mapping
        if best_score < 0.8 {
            if let Some(color_map) = self.infer_color_mapping(source, target) {
                let transform = Transform::ColorMap(color_map);
                if let Ok(result) = self.apply_transform(source, &transform) {
                    let score = self.calculate_similarity(&result, target);
                    if score > best_score {
                        best_morphism = Some(Morphism::new(
                            source_sig.clone(),
                            target_sig.clone(),
                            transform,
                        ));
                    }
                }
            }
        }
        
        best_morphism.ok_or_else(|| SCTTError::PatternExtractionFailed {
            reason: "No suitable transformation found".to_string(),
        })
    }
    
    /// Compose two morphisms if possible
    pub fn compose_morphisms(&mut self, m1: &Morphism, m2: &Morphism) -> Result<Morphism, SCTTError> {
        if !m1.composable_with(m2) {
            return Err(SCTTError::CompositionFailed {
                source: format!("{:?}", m1.transform),
                target: format!("{:?}", m2.transform),
            });
        }
        
        // Check cache first
        let cache_key = (m1.transform.clone(), m2.transform.clone());
        if let Some(cached_transform) = self.composition_cache.get(&cache_key) {
            return Ok(Morphism::new(
                m2.source.clone(),
                m1.target.clone(),
                cached_transform.clone(),
            ));
        }
        
        // Compute composition
        let composed_transform = m1.transform.compose(&m2.transform)?;
        
        // Cache result
        self.composition_cache.insert(cache_key, composed_transform.clone());
        
        // Verify composition maintains smoothness
        let composed_smoothness = composed_transform.smoothness_order();
        if composed_smoothness < m1.smoothness_order.min(m2.smoothness_order) {
            return Err(SCTTError::SmoothnessViolation { 
                order: composed_smoothness 
            });
        }
        
        Ok(Morphism::new(
            m2.source.clone(),
            m1.target.clone(),
            composed_transform,
        ))
    }
    
    /// Check if a transformation preserves essential structure
    pub fn preserves_structure(&self, transform: &Transform, grid: &Grid) -> bool {
        match transform {
            Transform::Identity => true,
            Transform::Rotate90 | Transform::Rotate180 | Transform::Rotate270 => {
                // Rotation preserves connectivity and shape
                true
            },
            Transform::FlipHorizontal | Transform::FlipVertical => {
                // Reflection preserves most structure
                true
            },
            Transform::ColorMap(_) => {
                // Color mapping preserves spatial structure
                true
            },
            Transform::ScaleInt { numerator, denominator } if *numerator > 0 && *denominator > 0 => {
                // Positive scaling preserves topological structure
                true
            },
            _ => false, // Conservative default
        }
    }
    
    // Private implementation methods
    
    fn rotate_90(&self, grid: &Grid) -> Result<Grid, SCTTError> {
        let new_data = Array2::from_shape_fn((grid.width, grid.height), |(x, y)| {
            grid.data[[grid.height - 1 - y, x]]
        });
        
        Ok(Grid {
            data: new_data,
            width: grid.height,
            height: grid.width,
        })
    }
    
    fn rotate_180(&self, grid: &Grid) -> Result<Grid, SCTTError> {
        let new_data = Array2::from_shape_fn((grid.height, grid.width), |(y, x)| {
            grid.data[[grid.height - 1 - y, grid.width - 1 - x]]
        });
        
        Ok(Grid {
            data: new_data,
            width: grid.width,
            height: grid.height,
        })
    }
    
    fn rotate_270(&self, grid: &Grid) -> Result<Grid, SCTTError> {
        let new_data = Array2::from_shape_fn((grid.width, grid.height), |(x, y)| {
            grid.data[[y, grid.width - 1 - x]]
        });
        
        Ok(Grid {
            data: new_data,
            width: grid.height,
            height: grid.width,
        })
    }
    
    fn flip_horizontal(&self, grid: &Grid) -> Result<Grid, SCTTError> {
        let new_data = Array2::from_shape_fn((grid.height, grid.width), |(y, x)| {
            grid.data[[y, grid.width - 1 - x]]
        });
        
        Ok(Grid {
            data: new_data,
            width: grid.width,
            height: grid.height,
        })
    }
    
    fn flip_vertical(&self, grid: &Grid) -> Result<Grid, SCTTError> {
        let new_data = Array2::from_shape_fn((grid.height, grid.width), |(y, x)| {
            grid.data[[grid.height - 1 - y, x]]
        });
        
        Ok(Grid {
            data: new_data,
            width: grid.width,
            height: grid.height,
        })
    }
    
    fn transpose(&self, grid: &Grid) -> Result<Grid, SCTTError> {
        let new_data = Array2::from_shape_fn((grid.width, grid.height), |(x, y)| {
            grid.data[[x, y]]
        });
        
        Ok(Grid {
            data: new_data,
            width: grid.height,
            height: grid.width,
        })
    }
    
    fn apply_color_map(&self, grid: &Grid, map: &HashMap<u8, u8>) -> Result<Grid, SCTTError> {
        let new_data = grid.data.mapv(|color| {
            *map.get(&color).unwrap_or(&color)
        });
        
        Ok(Grid {
            data: new_data,
            width: grid.width,
            height: grid.height,
        })
    }
    
    fn invert_colors(&self, grid: &Grid) -> Result<Grid, SCTTError> {
        let max_color = grid.data.iter().max().copied().unwrap_or(0);
        let new_data = grid.data.mapv(|color| max_color - color);
        
        Ok(Grid {
            data: new_data,
            width: grid.width,
            height: grid.height,
        })
    }
    
    fn translate(&self, grid: &Grid, dx: i32, dy: i32) -> Result<Grid, SCTTError> {
        let mut new_data = Array2::zeros((grid.height, grid.width));
        
        for y in 0..grid.height {
            for x in 0..grid.width {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;
                
                if new_x >= 0 && new_x < grid.width as i32 && 
                   new_y >= 0 && new_y < grid.height as i32 {
                    new_data[[new_y as usize, new_x as usize]] = grid.data[[y, x]];
                }
            }
        }
        
        Ok(Grid {
            data: new_data,
            width: grid.width,
            height: grid.height,
        })
    }
    
    fn scale(&self, grid: &Grid, factor: f64) -> Result<Grid, SCTTError> {
        if factor <= 0.0 {
            return Err(SCTTError::InvalidTransformation {
                reason: "Scale factor must be positive".to_string(),
            });
        }
        
        let new_width = (grid.width as f64 * factor).round() as usize;
        let new_height = (grid.height as f64 * factor).round() as usize;
        
        let new_data = Array2::from_shape_fn((new_height, new_width), |(y, x)| {
            let orig_x = (x as f64 / factor).round() as usize;
            let orig_y = (y as f64 / factor).round() as usize;
            
            if orig_x < grid.width && orig_y < grid.height {
                grid.data[[orig_y, orig_x]]
            } else {
                0
            }
        });
        
        Ok(Grid {
            data: new_data,
            width: new_width,
            height: new_height,
        })
    }
    
    fn crop(&self, grid: &Grid, x: usize, y: usize, width: usize, height: usize) -> Result<Grid, SCTTError> {
        if x + width > grid.width || y + height > grid.height {
            return Err(SCTTError::InvalidTransformation {
                reason: "Crop region exceeds grid bounds".to_string(),
            });
        }
        
        let new_data = grid.data.slice(s![y..y+height, x..x+width]).to_owned();
        
        Ok(Grid {
            data: new_data,
            width,
            height,
        })
    }
    
    fn fill_shape(&self, grid: &Grid, color: u8, _shape_filter: &ShapeFilter) -> Result<Grid, SCTTError> {
        // Simplified implementation - fill all non-zero pixels with the given color
        let new_data = grid.data.mapv(|pixel| {
            if pixel != 0 { color } else { pixel }
        });
        
        Ok(Grid {
            data: new_data,
            width: grid.width,
            height: grid.height,
        })
    }
    
    fn connect_components(&self, grid: &Grid, color: u8) -> Result<Grid, SCTTError> {
        // Simplified implementation - find and connect nearby components of the same color
        let mut new_data = grid.data.clone();
        
        // This would implement proper connected component analysis and connection
        // For now, return the original grid
        Ok(Grid {
            data: new_data,
            width: grid.width,
            height: grid.height,
        })
    }
    
    fn extract_subpattern(&self, grid: &Grid, _pattern: &Grid) -> Result<Grid, SCTTError> {
        // Simplified implementation - return original grid
        Ok(grid.clone())
    }
    
    fn smooth_morph(&self, grid: &Grid, _control_points: &[(f64, f64)]) -> Result<Grid, SCTTError> {
        // Simplified implementation - apply smooth interpolation between control points
        // This would implement proper smooth morphing using splines or other techniques
        Ok(grid.clone())
    }
    
    fn apply_vector_field(&self, grid: &Grid, _vector_field: &VectorField) -> Result<Grid, SCTTError> {
        // Simplified implementation - apply differential transformation
        // This would implement proper vector field application
        Ok(grid.clone())
    }
    
    fn apply_sequence(&self, grid: &Grid, transforms: &[Transform]) -> Result<Grid, SCTTError> {
        let mut result = grid.clone();
        for transform in transforms {
            result = self.apply_transform(&result, transform)?;
        }
        Ok(result)
    }
    
    fn apply_parallel(&self, grid: &Grid, transforms: &[Transform]) -> Result<Grid, SCTTError> {
        // Apply transforms in parallel and combine results
        // For now, just apply the first transform
        if let Some(first) = transforms.first() {
            self.apply_transform(grid, first)
        } else {
            Ok(grid.clone())
        }
    }
    
    fn apply_conditional(&self, grid: &Grid, predicate: &GridPredicate, 
                        then_transform: &Transform, else_transform: &Transform) -> Result<Grid, SCTTError> {
        let condition_met = self.evaluate_predicate(grid, predicate);
        let transform = if condition_met { then_transform } else { else_transform };
        self.apply_transform(grid, transform)
    }
    
    fn evaluate_predicate(&self, grid: &Grid, predicate: &GridPredicate) -> bool {
        match predicate {
            GridPredicate::HasColor(color) => {
                grid.data.iter().any(|&pixel| pixel == *color)
            },
            GridPredicate::SizeEquals { width, height } => {
                grid.width == *width && grid.height == *height
            },
            GridPredicate::ComplexityThreshold(threshold) => {
                grid.signature().complexity > *threshold
            },
            GridPredicate::SymmetryDetected(_symmetry_type) => {
                // Simplified - always return false for now
                false
            },
        }
    }
    
    fn calculate_similarity(&self, grid1: &Grid, grid2: &Grid) -> f64 {
        if grid1.width != grid2.width || grid1.height != grid2.height {
            return 0.0;
        }
        
        let total_pixels = (grid1.width * grid1.height) as f64;
        let matching_pixels = grid1.data.iter()
            .zip(grid2.data.iter())
            .filter(|(&a, &b)| a == b)
            .count() as f64;
        
        matching_pixels / total_pixels
    }
    
    fn infer_color_mapping(&self, source: &Grid, target: &Grid) -> Option<HashMap<u8, u8>> {
        if source.width != target.width || source.height != target.height {
            return None;
        }
        
        let mut mapping = HashMap::new();
        
        for y in 0..source.height {
            for x in 0..source.width {
                let src_color = source.data[[y, x]];
                let tgt_color = target.data[[y, x]];
                
                if let Some(&existing_mapping) = mapping.get(&src_color) {
                    if existing_mapping != tgt_color {
                        return None; // Inconsistent mapping
                    }
                } else {
                    mapping.insert(src_color, tgt_color);
                }
            }
        }
        
        Some(mapping)
    }
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}

// Import ndarray's s! macro
use ndarray::s;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_kernel_creation() {
        let kernel = Kernel::new();
        assert_eq!(kernel.dimension(), 2);
    }
    
    #[test]
    fn test_identity_transform() {
        let kernel = Kernel::new();
        let grid = Grid::from_vec(vec![vec![1, 2], vec![3, 4]]).unwrap();
        let result = kernel.apply_transform(&grid, &Transform::Identity).unwrap();
        assert_eq!(result, grid);
    }
    
    #[test]
    fn test_rotation_90() {
        let kernel = Kernel::new();
        let grid = Grid::from_vec(vec![vec![1, 2], vec![3, 4]]).unwrap();
        let result = kernel.apply_transform(&grid, &Transform::Rotate90).unwrap();
        let expected = Grid::from_vec(vec![vec![3, 1], vec![4, 2]]).unwrap();
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_morphism_extraction() {
        let kernel = Kernel::new();
        let source = Grid::from_vec(vec![vec![1, 2], vec![3, 4]]).unwrap();
        let target = Grid::from_vec(vec![vec![3, 1], vec![4, 2]]).unwrap();
        
        let morphism = kernel.extract_morphism(&source, &target).unwrap();
        assert_eq!(morphism.transform, Transform::Rotate90);
    }
    
    #[test]
    fn test_morphism_composition() {
        let mut kernel = Kernel::new();
        let sig = GridSignature {
            width: 2,
            height: 2,
            color_distribution: HashMap::new(),
            shape_features: vec![],
            complexity: 0.0,
        };
        
        let m1 = Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90);
        let m2 = Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90);
        
        let composed = kernel.compose_morphisms(&m1, &m2).unwrap();
        assert_eq!(composed.transform, Transform::Rotate180);
    }
}