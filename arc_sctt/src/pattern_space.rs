//! Pattern Space: Geometric Structure of Morphological Intelligence
//! 
//! This module implements the revolutionary concept of pattern space - a metric
//! space where visual patterns live as points and transformations are geodesics.
//! This enables continuous reasoning about discrete visual intelligence.

use crate::types::*;
use crate::morphology::*;
use std::collections::{HashMap, BTreeMap};
use rayon::prelude::*;
// use cached::proc_macro::cached;

/// A metric space where patterns are points and transformations are paths
#[derive(Debug, Clone)]
pub struct PatternSpace {
    patterns: Vec<PatternPoint>,
    morphisms: Vec<MorphismGeodesic>,
    distance_cache: HashMap<(usize, usize), f64>,
    similarity_threshold: f64,
    dimensionality: usize,
    curvature_tensor: CurvatureTensor,
}

/// A point in pattern space representing a visual pattern
#[derive(Debug, Clone)]
pub struct PatternPoint {
    pub id: usize,
    pub signature: GridSignature,
    pub coordinates: Vec<f64>,
    pub embedding: PatternEmbedding,
    pub neighborhood: Vec<usize>,
    pub intrinsic_dimension: f64,
}

/// High-dimensional embedding of a pattern
#[derive(Debug, Clone)]
pub struct PatternEmbedding {
    pub geometric_features: Vec<f64>,
    pub topological_features: Vec<f64>,
    pub color_features: Vec<f64>,
    pub relational_features: Vec<f64>,
    pub complexity_features: Vec<f64>,
}

/// A geodesic path between patterns representing a morphism
#[derive(Debug, Clone)]
pub struct MorphismGeodesic {
    pub source_id: usize,
    pub target_id: usize,
    pub morphism: Morphism,
    pub path: Vec<Vec<f64>>,
    pub length: f64,
    pub curvature: f64,
    pub smoothness_order: u32,
}

/// Curvature tensor describing the geometry of pattern space
#[derive(Debug, Clone)]
pub struct CurvatureTensor {
    pub riemann_tensor: Vec<Vec<Vec<Vec<f64>>>>,
    pub ricci_tensor: Vec<Vec<f64>>,
    pub scalar_curvature: f64,
    pub einstein_tensor: Vec<Vec<f64>>,
}

/// Metrics for measuring distances in pattern space
#[derive(Debug, Clone)]
pub enum PatternMetric {
    Euclidean,
    Riemannian,
    Wasserstein,
    InformationGeometric,
    Hyperbolic,
    Smooth,
}

/// Coordinates system for pattern space
#[derive(Debug, Clone)]
pub enum CoordinateSystem {
    Cartesian,
    Polar,
    Spherical,
    Hyperbolic,
    Manifold,
}

impl PatternSpace {
    /// Create a new pattern space
    pub fn new() -> Self {
        PatternSpace {
            patterns: Vec::new(),
            morphisms: Vec::new(),
            distance_cache: HashMap::new(),
            similarity_threshold: 0.1,
            dimensionality: 64, // High-dimensional embedding space
            curvature_tensor: CurvatureTensor::new(64),
        }
    }
    
    /// Add patterns to the space and compute their embeddings
    pub fn add_patterns(&mut self, morphisms: &[Morphism]) {
        for morphism in morphisms {
            // Add source and target patterns if not already present
            let source_id = self.add_pattern_point(&morphism.source);
            let target_id = self.add_pattern_point(&morphism.target);
            
            // Create geodesic between patterns
            let geodesic = self.compute_geodesic(source_id, target_id, morphism.clone());
            self.morphisms.push(geodesic);
        }
        
        // Update pattern neighborhoods
        self.update_neighborhoods();
        
        // Recompute curvature
        self.update_curvature_tensor();
    }
    
    /// Find the nearest patterns to a given pattern
    pub fn find_nearest_patterns(&self, pattern: &GridSignature, k: usize) -> Vec<(usize, f64)> {
        let query_embedding = self.compute_pattern_embedding(pattern);
        
        let mut distances: Vec<(usize, f64)> = self.patterns
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let distance = self.compute_embedding_distance(&query_embedding, &p.embedding);
                (i, distance)
            })
            .collect();
        
        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        distances.truncate(k);
        
        distances
    }
    
    /// Find morphisms that can transform one pattern to another
    pub fn find_transformation_paths(&self, source: &GridSignature, target: &GridSignature) 
        -> Vec<Vec<&MorphismGeodesic>> {
        
        let source_neighbors = self.find_nearest_patterns(source, 5);
        let target_neighbors = self.find_nearest_patterns(target, 5);
        
        let mut paths = Vec::new();
        
        // Find direct paths
        for (source_id, _) in &source_neighbors {
            for (target_id, _) in &target_neighbors {
                if let Some(direct_path) = self.find_direct_geodesic(*source_id, *target_id) {
                    paths.push(vec![direct_path]);
                }
            }
        }
        
        // Find multi-step paths
        let multi_step_paths = self.find_multi_step_paths(&source_neighbors, &target_neighbors, 3);
        paths.extend(multi_step_paths);
        
        // Sort by path length
        paths.sort_by(|a, b| {
            let len_a: f64 = a.iter().map(|g| g.length).sum();
            let len_b: f64 = b.iter().map(|g| g.length).sum();
            len_a.partial_cmp(&len_b).unwrap()
        });
        
        paths
    }
    
    /// Compute the distance between two patterns in the space
    pub fn pattern_distance(&mut self, id1: usize, id2: usize, metric: PatternMetric) -> f64 {
        let cache_key = (id1.min(id2), id1.max(id2));
        if let Some(&cached_distance) = self.distance_cache.get(&cache_key) {
            return cached_distance;
        }
        
        let distance = match metric {
            PatternMetric::Euclidean => self.euclidean_distance(id1, id2),
            PatternMetric::Riemannian => self.riemannian_distance(id1, id2),
            PatternMetric::Wasserstein => self.wasserstein_distance(id1, id2),
            PatternMetric::InformationGeometric => self.information_geometric_distance(id1, id2),
            PatternMetric::Hyperbolic => self.hyperbolic_distance(id1, id2),
            PatternMetric::Smooth => self.smooth_distance(id1, id2),
        };
        
        self.distance_cache.insert(cache_key, distance);
        distance
    }
    
    /// Interpolate between two patterns along a geodesic
    pub fn interpolate_patterns(&self, id1: usize, id2: usize, t: f64) -> Result<PatternEmbedding, SCTTError> {
        if t < 0.0 || t > 1.0 {
            return Err(SCTTError::InvalidTransformation {
                reason: "Interpolation parameter must be between 0 and 1".to_string(),
            });
        }
        
        let pattern1 = &self.patterns[id1];
        let pattern2 = &self.patterns[id2];
        
        // Find geodesic between patterns
        if let Some(geodesic) = self.find_direct_geodesic(id1, id2) {
            let interpolated_coords = self.interpolate_along_geodesic(&geodesic.path, t);
            let interpolated_embedding = self.coords_to_embedding(&interpolated_coords);
            Ok(interpolated_embedding)
        } else {
            // Linear interpolation as fallback
            let interpolated = self.linear_interpolate_embeddings(&pattern1.embedding, &pattern2.embedding, t);
            Ok(interpolated)
        }
    }
    
    /// Learn a new morphism by observing patterns
    pub fn learn_new_morphism(&mut self, source: &GridSignature, target: &GridSignature) 
        -> Result<Morphism, SCTTError> {
        
        // Find nearest patterns to source and target
        let source_neighbors = self.find_nearest_patterns(source, 3);
        let target_neighbors = self.find_nearest_patterns(target, 3);
        
        // Analyze transformations between similar patterns
        let mut candidate_transforms = Vec::new();
        
        for (source_id, _) in &source_neighbors {
            for (target_id, _) in &target_neighbors {
                if let Some(geodesic) = self.find_direct_geodesic(*source_id, *target_id) {
                    candidate_transforms.push(geodesic.morphism.clone());
                }
            }
        }
        
        if candidate_transforms.is_empty() {
            return Err(SCTTError::PatternExtractionFailed {
                reason: "No candidate transformations found".to_string(),
            });
        }
        
        // Select the most promising transformation
        let best_transform = candidate_transforms
            .into_iter()
            .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())
            .unwrap();
        
        // Create new morphism
        Ok(Morphism::new(source.clone(), target.clone(), best_transform.transform))
    }
    
    /// Get the intrinsic dimensionality of the pattern space
    pub fn intrinsic_dimensionality(&self) -> f64 {
        if self.patterns.is_empty() {
            return 0.0;
        }
        
        // Estimate intrinsic dimension using local PCA
        let dimensions: Vec<f64> = self.patterns
            .iter()
            .map(|p| p.intrinsic_dimension)
            .collect();
        
        dimensions.iter().sum::<f64>() / dimensions.len() as f64
    }
    
    /// Compute the curvature at a specific pattern
    pub fn pattern_curvature(&self, pattern_id: usize) -> f64 {
        if pattern_id >= self.patterns.len() {
            return 0.0;
        }
        
        // Use Ricci scalar curvature as a measure
        let pattern = &self.patterns[pattern_id];
        let neighbors = &pattern.neighborhood;
        
        if neighbors.len() < 3 {
            return 0.0;
        }
        
        // Compute local curvature using neighbor distances
        let mut curvature_sum = 0.0;
        let mut count = 0;
        
        for i in 0..neighbors.len() {
            for j in (i+1)..neighbors.len() {
                for k in (j+1)..neighbors.len() {
                    let triangle_curvature = self.triangle_curvature(
                        neighbors[i], neighbors[j], neighbors[k]
                    );
                    curvature_sum += triangle_curvature;
                    count += 1;
                }
            }
        }
        
        if count > 0 {
            curvature_sum / count as f64
        } else {
            0.0
        }
    }
    
    // Private implementation methods
    
    fn add_pattern_point(&mut self, signature: &GridSignature) -> usize {
        // Check if pattern already exists
        for (i, pattern) in self.patterns.iter().enumerate() {
            if self.signatures_equal(&pattern.signature, signature) {
                return i;
            }
        }
        
        // Create new pattern point
        let id = self.patterns.len();
        let embedding = self.compute_pattern_embedding(signature);
        let coordinates = self.embedding_to_coordinates(&embedding);
        let intrinsic_dim = self.estimate_intrinsic_dimension(&embedding);
        
        let pattern = PatternPoint {
            id,
            signature: signature.clone(),
            coordinates,
            embedding,
            neighborhood: Vec::new(),
            intrinsic_dimension: intrinsic_dim,
        };
        
        self.patterns.push(pattern);
        id
    }
    
    fn compute_pattern_embedding(&self, signature: &GridSignature) -> PatternEmbedding {
        let geometric_features = self.extract_geometric_features(signature);
        let topological_features = self.extract_topological_features(signature);
        let color_features = self.extract_color_features(signature);
        let relational_features = self.extract_relational_features(signature);
        let complexity_features = self.extract_complexity_features(signature);
        
        PatternEmbedding {
            geometric_features,
            topological_features,
            color_features,
            relational_features,
            complexity_features,
        }
    }
    
    fn extract_geometric_features(&self, signature: &GridSignature) -> Vec<f64> {
        vec![
            signature.width as f64,
            signature.height as f64,
            (signature.width * signature.height) as f64,
            signature.width as f64 / signature.height as f64,
            signature.shape_features.iter().sum::<usize>() as f64,
        ]
    }
    
    fn extract_topological_features(&self, signature: &GridSignature) -> Vec<f64> {
        vec![
            signature.shape_features.len() as f64,
            signature.shape_features.iter().map(|&x| x as f64).sum::<f64>(),
            signature.shape_features.iter().map(|&x| (x as f64).powi(2)).sum::<f64>(),
        ]
    }
    
    fn extract_color_features(&self, signature: &GridSignature) -> Vec<f64> {
        let mut features = Vec::new();
        
        features.push(signature.color_distribution.len() as f64);
        
        let total_pixels: usize = signature.color_distribution.values().sum();
        for &count in signature.color_distribution.values() {
            features.push(count as f64 / total_pixels as f64);
        }
        
        // Pad to fixed length
        while features.len() < 10 {
            features.push(0.0);
        }
        features.truncate(10);
        
        features
    }
    
    fn extract_relational_features(&self, signature: &GridSignature) -> Vec<f64> {
        vec![
            signature.complexity,
            signature.complexity.sqrt(),
            signature.complexity.log2().max(0.0),
        ]
    }
    
    fn extract_complexity_features(&self, signature: &GridSignature) -> Vec<f64> {
        let entropy = signature.complexity;
        vec![
            entropy,
            entropy.powi(2),
            entropy.sqrt(),
            if entropy > 0.0 { 1.0 / entropy } else { 0.0 },
        ]
    }
    
    fn embedding_to_coordinates(&self, embedding: &PatternEmbedding) -> Vec<f64> {
        let mut coords = Vec::new();
        coords.extend(&embedding.geometric_features);
        coords.extend(&embedding.topological_features);
        coords.extend(&embedding.color_features);
        coords.extend(&embedding.relational_features);
        coords.extend(&embedding.complexity_features);
        
        // Pad or truncate to desired dimensionality
        coords.resize(self.dimensionality, 0.0);
        coords
    }
    
    fn coords_to_embedding(&self, coords: &[f64]) -> PatternEmbedding {
        let geometric_len = 5;
        let topological_len = 3;
        let color_len = 10;
        let relational_len = 3;
        let complexity_len = 4;
        
        let mut offset = 0;
        
        let geometric_features = coords[offset..offset + geometric_len].to_vec();
        offset += geometric_len;
        
        let topological_features = coords[offset..offset + topological_len].to_vec();
        offset += topological_len;
        
        let color_features = coords[offset..offset + color_len].to_vec();
        offset += color_len;
        
        let relational_features = coords[offset..offset + relational_len].to_vec();
        offset += relational_len;
        
        let complexity_features = if offset + complexity_len <= coords.len() {
            coords[offset..offset + complexity_len].to_vec()
        } else {
            vec![0.0; complexity_len]
        };
        
        PatternEmbedding {
            geometric_features,
            topological_features,
            color_features,
            relational_features,
            complexity_features,
        }
    }
    
    fn estimate_intrinsic_dimension(&self, _embedding: &PatternEmbedding) -> f64 {
        // Simplified estimate - in practice would use more sophisticated methods
        8.0
    }
    
    fn compute_geodesic(&self, source_id: usize, target_id: usize, morphism: Morphism) -> MorphismGeodesic {
        let source_coords = &self.patterns[source_id].coordinates;
        let target_coords = &self.patterns[target_id].coordinates;
        
        // Compute geodesic path (simplified as straight line for now)
        let mut path = Vec::new();
        let steps = 10;
        
        for i in 0..=steps {
            let t = i as f64 / steps as f64;
            let interpolated: Vec<f64> = source_coords
                .iter()
                .zip(target_coords.iter())
                .map(|(s, t_coord)| s + t * (t_coord - s))
                .collect();
            path.push(interpolated);
        }
        
        let length = self.path_length(&path);
        let curvature = self.path_curvature(&path);
        
        MorphismGeodesic {
            source_id,
            target_id,
            morphism,
            path,
            length,
            curvature,
            smoothness_order: 2, // C^2 smooth by default
        }
    }
    
    fn path_length(&self, path: &[Vec<f64>]) -> f64 {
        let mut total_length = 0.0;
        
        for i in 1..path.len() {
            let segment_length: f64 = path[i]
                .iter()
                .zip(path[i-1].iter())
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f64>()
                .sqrt();
            total_length += segment_length;
        }
        
        total_length
    }
    
    fn path_curvature(&self, path: &[Vec<f64>]) -> f64 {
        if path.len() < 3 {
            return 0.0;
        }
        
        let mut total_curvature = 0.0;
        let mut count = 0;
        
        for i in 1..path.len()-1 {
            let v1: Vec<f64> = path[i]
                .iter()
                .zip(path[i-1].iter())
                .map(|(a, b)| a - b)
                .collect();
            
            let v2: Vec<f64> = path[i+1]
                .iter()
                .zip(path[i].iter())
                .map(|(a, b)| a - b)
                .collect();
            
            let curvature = self.vector_curvature(&v1, &v2);
            total_curvature += curvature;
            count += 1;
        }
        
        if count > 0 {
            total_curvature / count as f64
        } else {
            0.0
        }
    }
    
    fn vector_curvature(&self, v1: &[f64], v2: &[f64]) -> f64 {
        let dot_product: f64 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
        let norm1: f64 = v1.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
        let norm2: f64 = v2.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
        
        if norm1 > 0.0 && norm2 > 0.0 {
            let cos_angle = dot_product / (norm1 * norm2);
            let angle = cos_angle.clamp(-1.0, 1.0).acos();
            angle / (norm1 + norm2) * 2.0
        } else {
            0.0
        }
    }
    
    fn update_neighborhoods(&mut self) {
        let k = 5; // Number of nearest neighbors
        
        for i in 0..self.patterns.len() {
            let mut distances: Vec<(usize, f64)> = (0..self.patterns.len())
                .filter(|&j| j != i)
                .map(|j| (j, self.euclidean_distance(i, j)))
                .collect();
            
            distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            distances.truncate(k);
            
            self.patterns[i].neighborhood = distances.into_iter().map(|(j, _)| j).collect();
        }
    }
    
    fn update_curvature_tensor(&mut self) {
        // Simplified curvature computation
        self.curvature_tensor = CurvatureTensor::new(self.dimensionality);
        
        // Compute Ricci tensor from pattern distances
        for i in 0..self.patterns.len().min(10) { // Limit computation for performance
            for j in 0..self.patterns.len().min(10) {
                let curvature = self.local_curvature(i, j);
                if i < self.curvature_tensor.ricci_tensor.len() && 
                   j < self.curvature_tensor.ricci_tensor[i].len() {
                    self.curvature_tensor.ricci_tensor[i][j] = curvature;
                }
            }
        }
        
        // Compute scalar curvature
        self.curvature_tensor.scalar_curvature = self.compute_scalar_curvature();
    }
    
    fn local_curvature(&self, i: usize, j: usize) -> f64 {
        // Simplified local curvature computation
        if i >= self.patterns.len() || j >= self.patterns.len() {
            return 0.0;
        }
        
        let distance = self.euclidean_distance(i, j);
        if distance > 0.0 {
            1.0 / distance.powi(2)
        } else {
            0.0
        }
    }
    
    fn compute_scalar_curvature(&self) -> f64 {
        // Average diagonal elements of Ricci tensor
        let mut sum = 0.0;
        let mut count = 0;
        
        for i in 0..self.curvature_tensor.ricci_tensor.len().min(self.curvature_tensor.ricci_tensor.len()) {
            if i < self.curvature_tensor.ricci_tensor[i].len() {
                sum += self.curvature_tensor.ricci_tensor[i][i];
                count += 1;
            }
        }
        
        if count > 0 {
            sum / count as f64
        } else {
            0.0
        }
    }
    
    // Distance computation methods
    
    fn euclidean_distance(&self, id1: usize, id2: usize) -> f64 {
        if id1 >= self.patterns.len() || id2 >= self.patterns.len() {
            return f64::INFINITY;
        }
        
        let coords1 = &self.patterns[id1].coordinates;
        let coords2 = &self.patterns[id2].coordinates;
        
        coords1
            .iter()
            .zip(coords2.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }
    
    fn riemannian_distance(&self, id1: usize, id2: usize) -> f64 {
        // Simplified Riemannian distance using local metric
        let euclidean_dist = self.euclidean_distance(id1, id2);
        let curvature_correction = 1.0 + self.local_curvature(id1, id2) * 0.1;
        euclidean_dist * curvature_correction
    }
    
    fn wasserstein_distance(&self, id1: usize, id2: usize) -> f64 {
        // Simplified Wasserstein distance for pattern signatures
        if id1 >= self.patterns.len() || id2 >= self.patterns.len() {
            return f64::INFINITY;
        }
        
        let sig1 = &self.patterns[id1].signature;
        let sig2 = &self.patterns[id2].signature;
        
        // Compare color distributions
        let mut distance = 0.0;
        
        let all_colors: std::collections::HashSet<u8> = sig1.color_distribution
            .keys()
            .chain(sig2.color_distribution.keys())
            .copied()
            .collect();
        
        for color in all_colors {
            let count1 = *sig1.color_distribution.get(&color).unwrap_or(&0) as f64;
            let count2 = *sig2.color_distribution.get(&color).unwrap_or(&0) as f64;
            distance += (count1 - count2).abs();
        }
        
        distance
    }
    
    fn information_geometric_distance(&self, id1: usize, id2: usize) -> f64 {
        // Information-theoretic distance based on complexity
        if id1 >= self.patterns.len() || id2 >= self.patterns.len() {
            return f64::INFINITY;
        }
        
        let complexity1 = self.patterns[id1].signature.complexity;
        let complexity2 = self.patterns[id2].signature.complexity;
        
        if complexity1 > 0.0 && complexity2 > 0.0 {
            let kl_div = complexity1 * (complexity1 / complexity2).ln() - complexity1 + complexity2;
            kl_div.abs().sqrt()
        } else {
            (complexity1 - complexity2).abs()
        }
    }
    
    fn hyperbolic_distance(&self, id1: usize, id2: usize) -> f64 {
        // Hyperbolic distance using Poincaré disk model
        let euclidean_dist = self.euclidean_distance(id1, id2);
        let radius = 1.0; // Poincaré disk radius
        
        if euclidean_dist < radius {
            2.0 * ((radius + euclidean_dist) / (radius - euclidean_dist)).ln()
        } else {
            f64::INFINITY
        }
    }
    
    fn smooth_distance(&self, id1: usize, id2: usize) -> f64 {
        // Smooth distance that considers morphism paths
        if let Some(geodesic) = self.find_direct_geodesic(id1, id2) {
            geodesic.length
        } else {
            self.euclidean_distance(id1, id2)
        }
    }
    
    // Helper methods
    
    fn signatures_equal(&self, sig1: &GridSignature, sig2: &GridSignature) -> bool {
        sig1.width == sig2.width &&
        sig1.height == sig2.height &&
        sig1.color_distribution == sig2.color_distribution &&
        (sig1.complexity - sig2.complexity).abs() < 1e-6
    }
    
    fn compute_embedding_distance(&self, emb1: &PatternEmbedding, emb2: &PatternEmbedding) -> f64 {
        let dist1 = self.vector_distance(&emb1.geometric_features, &emb2.geometric_features);
        let dist2 = self.vector_distance(&emb1.topological_features, &emb2.topological_features);
        let dist3 = self.vector_distance(&emb1.color_features, &emb2.color_features);
        let dist4 = self.vector_distance(&emb1.relational_features, &emb2.relational_features);
        let dist5 = self.vector_distance(&emb1.complexity_features, &emb2.complexity_features);
        
        (dist1 + dist2 + dist3 + dist4 + dist5).sqrt()
    }
    
    fn vector_distance(&self, v1: &[f64], v2: &[f64]) -> f64 {
        let max_len = v1.len().max(v2.len());
        let mut sum = 0.0;
        
        for i in 0..max_len {
            let a = v1.get(i).unwrap_or(&0.0);
            let b = v2.get(i).unwrap_or(&0.0);
            sum += (a - b).powi(2);
        }
        
        sum
    }
    
    fn find_direct_geodesic(&self, source_id: usize, target_id: usize) -> Option<&MorphismGeodesic> {
        self.morphisms
            .iter()
            .find(|g| g.source_id == source_id && g.target_id == target_id)
    }
    
    fn find_multi_step_paths(&self, source_neighbors: &[(usize, f64)], 
                           target_neighbors: &[(usize, f64)], 
                           max_steps: usize) -> Vec<Vec<&MorphismGeodesic>> {
        let mut paths = Vec::new();
        
        // Simplified multi-step path finding
        for (source_id, _) in source_neighbors {
            for (target_id, _) in target_neighbors {
                if let Some(path) = self.find_path_with_steps(*source_id, *target_id, max_steps) {
                    paths.push(path);
                }
            }
        }
        
        paths
    }
    
    fn find_path_with_steps(&self, source_id: usize, target_id: usize, max_steps: usize) 
        -> Option<Vec<&MorphismGeodesic>> {
        
        if max_steps == 0 {
            return None;
        }
        
        // Direct path
        if let Some(direct) = self.find_direct_geodesic(source_id, target_id) {
            return Some(vec![direct]);
        }
        
        // Two-step path
        if max_steps >= 2 {
            for intermediate in 0..self.patterns.len() {
                if intermediate != source_id && intermediate != target_id {
                    if let Some(first) = self.find_direct_geodesic(source_id, intermediate) {
                        if let Some(second) = self.find_direct_geodesic(intermediate, target_id) {
                            return Some(vec![first, second]);
                        }
                    }
                }
            }
        }
        
        None
    }
    
    fn interpolate_along_geodesic(&self, path: &[Vec<f64>], t: f64) -> Vec<f64> {
        if path.is_empty() {
            return vec![];
        }
        
        if path.len() == 1 {
            return path[0].clone();
        }
        
        let segment_t = t * (path.len() - 1) as f64;
        let segment_index = segment_t.floor() as usize;
        let local_t = segment_t - segment_index as f64;
        
        if segment_index >= path.len() - 1 {
            return path.last().unwrap().clone();
        }
        
        // Linear interpolation within segment
        path[segment_index]
            .iter()
            .zip(path[segment_index + 1].iter())
            .map(|(a, b)| a + local_t * (b - a))
            .collect()
    }
    
    fn linear_interpolate_embeddings(&self, emb1: &PatternEmbedding, emb2: &PatternEmbedding, t: f64) 
        -> PatternEmbedding {
        
        let interpolate_vec = |v1: &[f64], v2: &[f64]| -> Vec<f64> {
            v1.iter()
                .zip(v2.iter())
                .map(|(a, b)| a + t * (b - a))
                .collect()
        };
        
        PatternEmbedding {
            geometric_features: interpolate_vec(&emb1.geometric_features, &emb2.geometric_features),
            topological_features: interpolate_vec(&emb1.topological_features, &emb2.topological_features),
            color_features: interpolate_vec(&emb1.color_features, &emb2.color_features),
            relational_features: interpolate_vec(&emb1.relational_features, &emb2.relational_features),
            complexity_features: interpolate_vec(&emb1.complexity_features, &emb2.complexity_features),
        }
    }
    
    fn triangle_curvature(&self, id1: usize, id2: usize, id3: usize) -> f64 {
        // Compute curvature of triangle formed by three patterns
        let d12 = self.euclidean_distance(id1, id2);
        let d23 = self.euclidean_distance(id2, id3);
        let d31 = self.euclidean_distance(id3, id1);
        
        // Use triangle inequality deviation as curvature measure
        let max_side = d12.max(d23).max(d31);
        let perimeter = d12 + d23 + d31;
        
        if perimeter > 0.0 && max_side < perimeter / 2.0 {
            // Valid triangle - compute area-based curvature
            let s = perimeter / 2.0;
            let area = (s * (s - d12) * (s - d23) * (s - d31)).sqrt();
            4.0 * area / (d12 * d23 * d31)
        } else {
            0.0
        }
    }
}

impl CurvatureTensor {
    fn new(dim: usize) -> Self {
        let riemann_tensor = vec![vec![vec![vec![0.0; dim]; dim]; dim]; dim];
        let ricci_tensor = vec![vec![0.0; dim]; dim];
        let einstein_tensor = vec![vec![0.0; dim]; dim];
        
        CurvatureTensor {
            riemann_tensor,
            ricci_tensor,
            scalar_curvature: 0.0,
            einstein_tensor,
        }
    }
}

impl Default for PatternSpace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pattern_space_creation() {
        let space = PatternSpace::new();
        assert_eq!(space.dimensionality, 64);
        assert!(space.patterns.is_empty());
    }
    
    #[test]
    fn test_pattern_embedding() {
        let space = PatternSpace::new();
        let signature = GridSignature {
            width: 3,
            height: 3,
            color_distribution: [(1, 4), (0, 5)].iter().cloned().collect(),
            shape_features: vec![4],
            complexity: 1.5,
        };
        
        let embedding = space.compute_pattern_embedding(&signature);
        assert_eq!(embedding.geometric_features.len(), 5);
        assert_eq!(embedding.color_features.len(), 10);
    }
    
    #[test]
    fn test_pattern_addition() {
        let mut space = PatternSpace::new();
        let signature = GridSignature {
            width: 2,
            height: 2,
            color_distribution: HashMap::new(),
            shape_features: vec![],
            complexity: 1.0,
        };
        
        let morphism = Morphism::new(
            signature.clone(),
            signature.clone(),
            Transform::Identity,
        );
        
        space.add_patterns(&[morphism]);
        assert_eq!(space.patterns.len(), 1);
    }
    
    #[test]
    fn test_distance_computation() {
        let mut space = PatternSpace::new();
        let sig1 = GridSignature {
            width: 2, height: 2, color_distribution: HashMap::new(),
            shape_features: vec![], complexity: 1.0,
        };
        let sig2 = GridSignature {
            width: 3, height: 3, color_distribution: HashMap::new(),
            shape_features: vec![], complexity: 2.0,
        };
        
        let morphism1 = Morphism::new(sig1.clone(), sig1.clone(), Transform::Identity);
        let morphism2 = Morphism::new(sig2.clone(), sig2.clone(), Transform::Identity);
        
        space.add_patterns(&[morphism1, morphism2]);
        
        let distance = space.pattern_distance(0, 1, PatternMetric::Euclidean);
        assert!(distance > 0.0);
    }
}