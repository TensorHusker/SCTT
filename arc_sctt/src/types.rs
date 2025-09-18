//! Core types for the ARC-SCTT system
//! 
//! These types form the foundation of morphological intelligence,
//! representing patterns as mathematical objects in a smooth space.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur in the SCTT system
#[derive(Error, Debug)]
pub enum SCTTError {
    #[error("Grid dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
    
    #[error("Invalid transformation: {reason}")]
    InvalidTransformation { reason: String },
    
    #[error("Morphism composition failed: {source} ∘ {target} is undefined")]
    CompositionFailed { source: String, target: String },
    
    #[error("Pattern extraction failed: {reason}")]
    PatternExtractionFailed { reason: String },
    
    #[error("Smoothness violation: transformation is not C^{order} smooth")]
    SmoothnessViolation { order: u32 },
    
    #[error("Type checking failed: {expected} ≠ {actual}")]
    TypeCheckFailed { expected: String, actual: String },
    
    #[error("Kan condition violated: {operation} is not defined")]
    KanViolation { operation: String },
}

/// A 2D grid representing a visual pattern
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    pub data: ndarray::Array2<u8>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    /// Create a new grid from a 2D vector
    pub fn from_vec(data: Vec<Vec<u8>>) -> Result<Self, SCTTError> {
        if data.is_empty() {
            return Err(SCTTError::DimensionMismatch { 
                expected: 1, 
                actual: 0 
            });
        }
        
        let height = data.len();
        let width = data[0].len();
        
        // Verify all rows have same length
        for (_i, row) in data.iter().enumerate() {
            if row.len() != width {
                return Err(SCTTError::DimensionMismatch {
                    expected: width,
                    actual: row.len(),
                });
            }
        }
        
        let flat: Vec<u8> = data.into_iter().flatten().collect();
        let flat_len = flat.len();
        let array = ndarray::Array2::from_shape_vec((height, width), flat)
            .map_err(|_| SCTTError::DimensionMismatch { expected: height * width, actual: flat_len })?;
        
        Ok(Grid {
            data: array,
            width,
            height,
        })
    }
    
    /// Create a zero-filled grid with given dimensions
    pub fn zeros(width: usize, height: usize) -> Self {
        Grid {
            data: ndarray::Array2::zeros((height, width)),
            width,
            height,
        }
    }
    
    /// Get the value at position (x, y)
    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.data.get((y, x)).copied()
    }
    
    /// Set the value at position (x, y)
    pub fn set(&mut self, x: usize, y: usize, value: u8) -> Result<(), SCTTError> {
        if x >= self.width || y >= self.height {
            return Err(SCTTError::DimensionMismatch {
                expected: self.width * self.height,
                actual: x + y * self.width,
            });
        }
        self.data[[y, x]] = value;
        Ok(())
    }
    
    /// Calculate the morphological signature of this grid
    pub fn signature(&self) -> GridSignature {
        let mut color_counts = HashMap::new();
        let mut shape_features = Vec::new();
        
        // Count colors
        for &value in self.data.iter() {
            *color_counts.entry(value).or_insert(0) += 1;
        }
        
        // Extract connected components and shapes
        for color in color_counts.keys() {
            let component_size = self.connected_component_size(*color);
            shape_features.push(component_size);
        }
        
        GridSignature {
            width: self.width,
            height: self.height,
            color_distribution: color_counts,
            shape_features,
            complexity: self.calculate_complexity(),
        }
    }
    
    fn connected_component_size(&self, color: u8) -> usize {
        // Simple connected component analysis
        let mut visited = ndarray::Array2::from_elem((self.height, self.width), false);
        let mut max_component = 0;
        
        for y in 0..self.height {
            for x in 0..self.width {
                if !visited[[y, x]] && self.data[[y, x]] == color {
                    let size = self.dfs_component_size(x, y, color, &mut visited);
                    max_component = max_component.max(size);
                }
            }
        }
        
        max_component
    }
    
    fn dfs_component_size(&self, x: usize, y: usize, color: u8, visited: &mut ndarray::Array2<bool>) -> usize {
        if x >= self.width || y >= self.height || visited[[y, x]] || self.data[[y, x]] != color {
            return 0;
        }
        
        visited[[y, x]] = true;
        let mut size = 1;
        
        // Check 4-connected neighbors
        if x > 0 { size += self.dfs_component_size(x - 1, y, color, visited); }
        if x < self.width - 1 { size += self.dfs_component_size(x + 1, y, color, visited); }
        if y > 0 { size += self.dfs_component_size(x, y - 1, color, visited); }
        if y < self.height - 1 { size += self.dfs_component_size(x, y + 1, color, visited); }
        
        size
    }
    
    fn calculate_complexity(&self) -> f64 {
        // Information-theoretic complexity measure
        let mut entropy = 0.0;
        let total = (self.width * self.height) as f64;
        
        let mut color_counts = HashMap::new();
        for &value in self.data.iter() {
            *color_counts.entry(value).or_insert(0) += 1;
        }
        
        for &count in color_counts.values() {
            let p = count as f64 / total;
            if p > 0.0 {
                entropy -= p * p.log2();
            }
        }
        
        entropy
    }
}

/// Mathematical signature of a grid's morphological properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridSignature {
    pub width: usize,
    pub height: usize,
    pub color_distribution: HashMap<u8, usize>,
    pub shape_features: Vec<usize>,
    pub complexity: f64,
}

/// A morphism between two grids representing a transformation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Morphism {
    pub source: GridSignature,
    pub target: GridSignature,
    pub transform: Transform,
    pub confidence: f64,
    pub smoothness_order: u32,
}

impl Morphism {
    /// Create a new morphism
    pub fn new(source: GridSignature, target: GridSignature, transform: Transform) -> Self {
        let confidence = transform.calculate_confidence(&source, &target);
        let smoothness_order = transform.smoothness_order();
        
        Morphism {
            source,
            target,
            transform,
            confidence,
            smoothness_order,
        }
    }
    
    /// Check if this morphism can be composed with another
    pub fn composable_with(&self, other: &Morphism) -> bool {
        // Type checking: target of self must match source of other
        self.target.width == other.source.width &&
        self.target.height == other.source.height &&
        self.smoothness_order <= other.smoothness_order
    }
    
    /// Compose this morphism with another (self ∘ other)
    pub fn compose(&self, other: &Morphism) -> Result<Morphism, SCTTError> {
        if !self.composable_with(other) {
            return Err(SCTTError::CompositionFailed {
                source: format!("{:?}", self.transform),
                target: format!("{:?}", other.transform),
            });
        }
        
        let composed_transform = self.transform.compose(&other.transform)?;
        let confidence = (self.confidence * other.confidence).sqrt(); // Geometric mean
        let smoothness_order = self.smoothness_order.min(other.smoothness_order);
        
        Ok(Morphism {
            source: other.source.clone(),
            target: self.target.clone(),
            transform: composed_transform,
            confidence,
            smoothness_order,
        })
    }
}

/// Geometric transformations that can be applied to grids
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Transform {
    /// Identity transformation (does nothing)
    Identity,
    
    /// Geometric transformations
    Rotate90,
    Rotate180,
    Rotate270,
    FlipHorizontal,
    FlipVertical,
    Transpose,
    
    /// Color transformations
    ColorMap(HashMap<u8, u8>),
    InvertColors,
    
    /// Spatial transformations
    Translate { dx: i32, dy: i32 },
    ScaleInt { numerator: i32, denominator: i32 }, // Use rational numbers for hashing
    Crop { x: usize, y: usize, width: usize, height: usize },
    
    /// Pattern transformations
    FillShape { color: u8, shape_filter: ShapeFilter },
    ConnectComponents { color: u8 },
    ExtractSubpattern { pattern: Grid },
    
    /// Note: Smooth transformations disabled for Hash compatibility
    /// SmoothMorph { control_points: Vec<(f64, f64)> },
    /// DifferentialTransform { vector_field: VectorField },
    
    /// Composite transformations
    Sequence(Vec<Transform>),
    Parallel(Vec<Transform>),
    // Note: Conditional disabled for Hash compatibility
    // Conditional { predicate: GridPredicate, then_transform: Box<Transform>, else_transform: Box<Transform> },
}

impl Transform {
    /// Calculate confidence that this transformation maps source to target
    pub fn calculate_confidence(&self, source: &GridSignature, target: &GridSignature) -> f64 {
        match self {
            Transform::Identity => if source == target { 1.0 } else { 0.0 },
            Transform::Rotate90 | Transform::Rotate270 => {
                if source.width == target.height && source.height == target.width { 0.9 } else { 0.1 }
            },
            Transform::Rotate180 | Transform::FlipHorizontal | Transform::FlipVertical => {
                if source.width == target.width && source.height == target.height { 0.9 } else { 0.1 }
            },
            Transform::ColorMap(_) => {
                if source.width == target.width && source.height == target.height { 0.8 } else { 0.0 }
            },
            _ => 0.5, // Default confidence for complex transforms
        }
    }
    
    /// Get the smoothness order of this transformation
    pub fn smoothness_order(&self) -> u32 {
        match self {
            Transform::Identity => u32::MAX, // Infinitely smooth
            Transform::Rotate90 | Transform::Rotate180 | Transform::Rotate270 => 0, // Discontinuous
            Transform::FlipHorizontal | Transform::FlipVertical => 0,
            Transform::SmoothMorph { .. } => 3, // C^3 smooth
            Transform::DifferentialTransform { .. } => 2, // C^2 smooth
            Transform::Sequence(transforms) => transforms.iter().map(|t| t.smoothness_order()).min().unwrap_or(0),
            _ => 1, // C^1 smooth by default
        }
    }
    
    /// Compose this transformation with another
    pub fn compose(&self, other: &Transform) -> Result<Transform, SCTTError> {
        match (self, other) {
            (Transform::Identity, t) | (t, Transform::Identity) => Ok(t.clone()),
            (Transform::Rotate90, Transform::Rotate90) => Ok(Transform::Rotate180),
            (Transform::Rotate90, Transform::Rotate180) => Ok(Transform::Rotate270),
            (Transform::Rotate90, Transform::Rotate270) => Ok(Transform::Identity),
            (Transform::Rotate180, Transform::Rotate180) => Ok(Transform::Identity),
            _ => Ok(Transform::Sequence(vec![other.clone(), self.clone()])),
        }
    }
}

/// Filter for identifying shapes in grids
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShapeFilter {
    Color(u8),
    Size { min: usize, max: usize },
    Connectivity(ConnectivityType),
    Convexity,
    Symmetry(SymmetryType),
}

/// Types of connectivity for shape analysis
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConnectivityType {
    FourConnected,
    EightConnected,
}

/// Types of symmetry for pattern analysis
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymmetryType {
    Horizontal,
    Vertical,
    Rotational,
    Reflection,
}

/// Predicates for conditional transformations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GridPredicate {
    HasColor(u8),
    SizeEquals { width: usize, height: usize },
    ComplexityThreshold(f64),
    SymmetryDetected(SymmetryType),
}

/// Vector field for differential transformations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorField {
    pub field: Vec<Vec<(f64, f64)>>,
}

/// A complete ARC puzzle with examples and test cases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ARCPuzzle {
    pub id: String,
    pub examples: Vec<(Grid, Grid)>,
    pub test_input: Grid,
    pub test_output: Option<Grid>,
}

/// Type-theoretic signature for morphisms
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MorphismType {
    pub domain: TypeSignature,
    pub codomain: TypeSignature,
    pub properties: Vec<TypeProperty>,
}

/// Type signature for grids and patterns
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeSignature {
    pub dimensions: (usize, usize),
    pub color_space: ColorSpace,
    pub topology: TopologyType,
    pub smoothness: SmoothnessClass,
}

/// Color space representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColorSpace {
    Discrete { max_value: u8 },
    Continuous,
    Categorical(Vec<String>),
}

/// Topological properties of patterns
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TopologyType {
    Euclidean,
    Torus,
    Sphere,
    Klein,
}

/// Smoothness class for transformations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SmoothnessClass {
    Discrete,
    Lipschitz,
    DifferentiableN(u32),
    Smooth,
    Analytic,
}

/// Properties that morphisms can have
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeProperty {
    Invertible,
    VolumePreserving,
    ColorPreserving,
    SymmetryPreserving(SymmetryType),
    Continuous,
    Smooth(u32),
}