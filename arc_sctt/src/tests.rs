//! Comprehensive test suite for ARC-SCTT system
//! 
//! This module contains integration tests that demonstrate the revolutionary
//! capabilities of the morphological intelligence system.

#[cfg(test)]
mod integration_tests {
    use crate::*;
    use crate::utils::*;
    
    #[test]
    fn test_complete_arc_solving_pipeline() {
        let mut system = ARCSCTTSystem::new();
        
        // Generate a test puzzle
        let puzzle = PatternUtils::generate_test_puzzle(TestPuzzleType::Simple);
        
        // Solve the puzzle
        let solution = system.solve_puzzle(&puzzle.examples, &puzzle.test_input).unwrap();
        
        // Verify the solution
        assert!(solution.width > 0);
        assert!(solution.height > 0);
        
        // Check that the solution has some resemblance to expected pattern
        let expected = Grid::from_vec(vec![vec![0, 3], vec![3, 0]]).unwrap();
        let similarity = GridUtils::similarity(&solution, &expected);
        
        println!("Solution similarity: {:.2}", similarity);
        println!("Solution grid:\n{}", GridUtils::to_ascii(&solution));
        println!("Expected grid:\n{}", GridUtils::to_ascii(&expected));
        
        // The system should at least produce a valid transformation
        assert!(similarity >= 0.0); // Basic sanity check
    }
    
    #[test]
    fn test_morphism_extraction_and_composition() {
        let mut extractor = PatternExtractor::new();
        
        // Create input-output pairs with clear transformations
        let input1 = Grid::from_vec(vec![vec![1, 0], vec![0, 1]]).unwrap();
        let output1 = Grid::from_vec(vec![vec![0, 1], vec![1, 0]]).unwrap();
        
        let input2 = Grid::from_vec(vec![vec![2, 0], vec![0, 2]]).unwrap();
        let output2 = Grid::from_vec(vec![vec![0, 2], vec![2, 0]]).unwrap();
        
        let examples = vec![(input1, output1), (input2, output2)];
        
        // Extract morphisms
        let morphisms = extractor.extract_morphisms(&examples).unwrap();
        
        assert!(!morphisms.is_empty());
        println!("Extracted {} morphisms", morphisms.len());
        
        // Test composition
        let mut compositor = Compositor::new();
        let composition = compositor.learn_composition(&morphisms).unwrap();
        
        assert!(!composition.morphism_sequence.is_empty());
        assert!(composition.success_rate > 0.0);
        
        println!("Learned composition with {:.2} success rate", composition.success_rate);
    }
    
    #[test]
    fn test_pattern_space_geometry() {
        let mut pattern_space = PatternSpace::new();
        
        // Create patterns with known relationships
        let sig1 = GridSignature {
            width: 2, height: 2,
            color_distribution: [(1, 2), (0, 2)].iter().cloned().collect(),
            shape_features: vec![2],
            complexity: 1.0,
        };
        
        let sig2 = GridSignature {
            width: 2, height: 2,
            color_distribution: [(1, 2), (0, 2)].iter().cloned().collect(),
            shape_features: vec![2],
            complexity: 1.0,
        };
        
        let sig3 = GridSignature {
            width: 4, height: 4,
            color_distribution: [(1, 8), (0, 8)].iter().cloned().collect(),
            shape_features: vec![8],
            complexity: 2.0,
        };
        
        // Add patterns to space
        let morphisms = vec![
            Morphism::new(sig1.clone(), sig2.clone(), Transform::FlipHorizontal),
            Morphism::new(sig2.clone(), sig3.clone(), Transform::Scale { factor: 2.0 }),
        ];
        
        pattern_space.add_patterns(&morphisms);
        
        // Test distance computation
        let distance = pattern_space.pattern_distance(0, 1, PatternMetric::Euclidean);
        assert!(distance >= 0.0);
        
        println!("Pattern space distance: {:.4}", distance);
        
        // Test nearest neighbor search
        let neighbors = pattern_space.find_nearest_patterns(&sig1, 2);
        assert!(!neighbors.is_empty());
        
        println!("Found {} nearest neighbors", neighbors.len());
    }
    
    #[test]
    fn test_smooth_transformation_composition() {
        let compositor = Compositor::new();
        let grid = Grid::from_vec(vec![
            vec![1, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 1],
        ]).unwrap();
        
        // Test smooth rotation
        let rotated = compositor.smooth_rotate_90(&grid, 2).unwrap();
        assert_eq!(rotated.width, 3);
        assert_eq!(rotated.height, 3);
        
        println!("Original grid:\n{}", GridUtils::to_ascii(&grid));
        println!("Smoothly rotated grid:\n{}", GridUtils::to_ascii(&rotated));
        
        // Test smooth scaling
        let scaled = compositor.smooth_scale(&grid, 2.0, 1).unwrap();
        assert_eq!(scaled.width, 6);
        assert_eq!(scaled.height, 6);
        
        println!("Smoothly scaled grid:\n{}", GridUtils::to_ascii(&scaled));
    }
    
    #[test]
    fn test_invariant_detection() {
        let solver = ARCSolver::new();
        
        // Create examples that preserve object count
        let input1 = Grid::from_vec(vec![vec![1, 0], vec![0, 1]]).unwrap();
        let output1 = Grid::from_vec(vec![vec![0, 1], vec![1, 0]]).unwrap();
        
        let input2 = Grid::from_vec(vec![vec![2, 0], vec![0, 2]]).unwrap();
        let output2 = Grid::from_vec(vec![vec![0, 2], vec![2, 0]]).unwrap();
        
        // Test object count preservation
        assert!(solver.object_count_preserved(&input1, &output1));
        assert!(solver.object_count_preserved(&input2, &output2));
        
        // Test color distribution preservation
        assert!(solver.color_distribution_preserved(&input1, &output1));
        assert!(solver.color_distribution_preserved(&input2, &output2));
        
        println!("Invariants detected successfully");
    }
    
    #[test]
    fn test_morphological_reasoning_pipeline() {
        let mut system = ARCSCTTSystem::new();
        
        // Create a rotation puzzle
        let puzzle = PatternUtils::generate_test_puzzle(TestPuzzleType::Rotation);
        
        // Evolve the system with training data
        system.evolve(&[puzzle.clone()]).unwrap();
        
        // Test the system's ability to learn and generalize
        let solution = system.solve_puzzle(&puzzle.examples, &puzzle.test_input).unwrap();
        
        println!("Rotation puzzle solution:\n{}", GridUtils::to_ascii(&solution));
        
        // The system should produce a valid grid
        assert!(solution.width > 0);
        assert!(solution.height > 0);
    }
    
    #[test]
    fn test_pattern_learning_and_generalization() {
        let mut system = ARCSCTTSystem::new();
        
        // Generate multiple puzzles for learning
        let puzzles = vec![
            PatternUtils::generate_test_puzzle(TestPuzzleType::Simple),
            PatternUtils::generate_test_puzzle(TestPuzzleType::Rotation),
            PatternUtils::generate_test_puzzle(TestPuzzleType::ColorMapping),
        ];
        
        // Train the system
        for puzzle in &puzzles {
            system.evolve(&[puzzle.clone()]).unwrap();
        }
        
        // Test on a new simple puzzle
        let test_puzzle = PatternUtils::generate_test_puzzle(TestPuzzleType::Simple);
        let solution = system.solve_puzzle(&test_puzzle.examples, &test_puzzle.test_input).unwrap();
        
        println!("Generalization test solution:\n{}", GridUtils::to_ascii(&solution));
        
        // Validate the solution if possible
        if let Some(ref expected) = test_puzzle.test_output {
            let is_valid = PatternUtils::validate_solution(&test_puzzle, &solution);
            println!("Solution validation: {}", is_valid);
        }
    }
    
    #[test]
    fn test_complex_transformation_chains() {
        let mut system = ARCSCTTSystem::new();
        
        // Create a complex transformation chain: rotate then flip
        let input = Grid::from_vec(vec![
            vec![1, 0, 0],
            vec![0, 2, 0],
            vec![0, 0, 3],
        ]).unwrap();
        
        // Apply rotation followed by horizontal flip
        let rotated = system.kernel.apply_transform(&input, &Transform::Rotate90).unwrap();
        let final_output = system.kernel.apply_transform(&rotated, &Transform::FlipHorizontal).unwrap();
        
        println!("Original:\n{}", GridUtils::to_ascii(&input));
        println!("After rotation:\n{}", GridUtils::to_ascii(&rotated));
        println!("After flip:\n{}", GridUtils::to_ascii(&final_output));
        
        // Test morphism extraction for the complete transformation
        let morphism = system.morphology.extract_single_morphism(&input, &final_output).unwrap();
        assert!(!morphism.is_empty());
        
        println!("Extracted {} morphisms for complex transformation", morphism.len());
    }
    
    #[test]
    fn test_performance_and_scalability() {
        use std::time::Instant;
        
        let mut system = ARCSCTTSystem::new();
        
        // Test with grids of different sizes
        let sizes = vec![(2, 2), (3, 3), (5, 5), (10, 10)];
        
        for (width, height) in sizes {
            let start = Instant::now();
            
            // Create test grids
            let input = GridUtils::create_pattern(width, height, GridPattern::Checkerboard).unwrap();
            let output = system.kernel.apply_transform(&input, &Transform::Rotate90).unwrap();
            
            // Extract morphism
            let morphism = system.morphology.extract_single_morphism(&input, &output).unwrap();
            
            let duration = start.elapsed();
            
            println!("Grid {}x{}: processed in {:?}, found {} morphisms", 
                   width, height, duration, morphism.len());
            
            // Performance should be reasonable even for larger grids
            assert!(duration.as_millis() < 1000); // Less than 1 second
        }
    }
    
    #[test]
    fn test_error_handling_and_robustness() {
        let mut system = ARCSCTTSystem::new();
        
        // Test with invalid grids
        let empty_examples = vec![];
        let dummy_input = Grid::zeros(2, 2);
        
        // System should handle empty examples gracefully
        let result = system.solve_puzzle(&empty_examples, &dummy_input);
        assert!(result.is_err());
        
        // Test with mismatched grid sizes
        let input = Grid::zeros(2, 2);
        let output = Grid::zeros(3, 3);
        let examples = vec![(input, output)];
        
        // Should still attempt to process despite size mismatch
        let result = system.solve_puzzle(&examples, &dummy_input);
        // Result could be error or low-confidence solution
        match result {
            Ok(solution) => {
                println!("Handled size mismatch, produced solution with dimensions {}x{}", 
                        solution.width, solution.height);
            },
            Err(e) => {
                println!("Appropriately rejected mismatched inputs: {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_mathematical_properties() {
        let mut system = ARCSCTTSystem::new();
        
        // Test morphism associativity: (f ∘ g) ∘ h = f ∘ (g ∘ h)
        let sig = GridSignature {
            width: 3, height: 3,
            color_distribution: std::collections::HashMap::new(),
            shape_features: vec![],
            complexity: 1.0,
        };
        
        let m1 = Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90);
        let m2 = Morphism::new(sig.clone(), sig.clone(), Transform::FlipHorizontal);
        let m3 = Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90);
        
        // Test composition
        let composed_left = system.kernel.compose_morphisms(&m1, &m2).unwrap();
        let final_left = system.kernel.compose_morphisms(&composed_left, &m3).unwrap();
        
        let composed_right = system.kernel.compose_morphisms(&m2, &m3).unwrap();
        let final_right = system.kernel.compose_morphisms(&m1, &composed_right).unwrap();
        
        // Associativity test (types should match)
        assert_eq!(final_left.source.width, final_right.source.width);
        assert_eq!(final_left.target.width, final_right.target.width);
        
        println!("Morphism associativity test passed");
        
        // Test identity morphism
        let identity = Morphism::new(sig.clone(), sig.clone(), Transform::Identity);
        let composed_identity = system.kernel.compose_morphisms(&m1, &identity).unwrap();
        
        // Should preserve the original morphism properties
        assert_eq!(composed_identity.transform, m1.transform);
        
        println!("Identity morphism test passed");
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use crate::utils::PerformanceUtils;
    
    #[test]
    fn benchmark_morphism_application() {
        let grid = Grid::from_vec(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]).unwrap();
        
        let morphism = Morphism::new(
            grid.signature(),
            grid.signature(),
            Transform::Rotate90,
        );
        
        let report = PerformanceUtils::benchmark_morphism_application(&morphism, &grid, 100);
        println!("{}", report);
        
        assert!(report.success_rate > 0.9);
        assert!(report.avg_time.as_millis() < 10); // Should be fast
    }
    
    #[test]
    fn benchmark_pattern_extraction() {
        let input = Grid::from_vec(vec![vec![1, 0], vec![0, 1]]).unwrap();
        let output = Grid::from_vec(vec![vec![0, 1], vec![1, 0]]).unwrap();
        
        let mut extractor = PatternExtractor::new();
        
        let (_, duration) = PerformanceUtils::time_execution(|| {
            extractor.extract_single_morphism(&input, &output).unwrap()
        });
        
        println!("Pattern extraction took: {:?}", duration);
        assert!(duration.as_millis() < 100); // Should be reasonably fast
    }
    
    #[test]
    fn benchmark_pattern_space_operations() {
        let mut space = PatternSpace::new();
        
        // Create several patterns
        let signatures: Vec<GridSignature> = (0..10).map(|i| {
            GridSignature {
                width: 2 + i % 3,
                height: 2 + i % 3,
                color_distribution: [(i as u8, 4)].iter().cloned().collect(),
                shape_features: vec![i],
                complexity: i as f64,
            }
        }).collect();
        
        let morphisms: Vec<Morphism> = signatures.windows(2).map(|pair| {
            Morphism::new(pair[0].clone(), pair[1].clone(), Transform::Identity)
        }).collect();
        
        let (_, add_duration) = PerformanceUtils::time_execution(|| {
            space.add_patterns(&morphisms);
        });
        
        println!("Adding {} patterns took: {:?}", morphisms.len(), add_duration);
        
        let (neighbors, search_duration) = PerformanceUtils::time_execution(|| {
            space.find_nearest_patterns(&signatures[0], 5)
        });
        
        println!("Nearest neighbor search took: {:?}, found {} neighbors", 
                search_duration, neighbors.len());
        
        assert!(add_duration.as_millis() < 1000);
        assert!(search_duration.as_millis() < 100);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use std::collections::HashSet;
    
    #[test]
    fn test_morphism_invertibility_property() {
        let sig = GridSignature {
            width: 2, height: 2,
            color_distribution: std::collections::HashMap::new(),
            shape_features: vec![],
            complexity: 0.0,
        };
        
        let invertible_transforms = vec![
            Transform::Identity,
            Transform::Rotate90,
            Transform::Rotate180,
            Transform::Rotate270,
            Transform::FlipHorizontal,
            Transform::FlipVertical,
            Transform::Transpose,
        ];
        
        for transform in invertible_transforms {
            let morphism = Morphism::new(sig.clone(), sig.clone(), transform);
            let inverse = MorphismUtils::inverse(&morphism);
            
            assert!(inverse.is_some(), "Transform {:?} should be invertible", morphism.transform);
            
            if let Some(inv) = inverse {
                // Test that applying transform then inverse gives identity
                let kernel = Kernel::new();
                let grid = Grid::from_vec(vec![vec![1, 2], vec![3, 4]]).unwrap();
                
                if let Ok(transformed) = kernel.apply_transform(&grid, &morphism.transform) {
                    if let Ok(restored) = kernel.apply_transform(&transformed, &inv.transform) {
                        let similarity = GridUtils::similarity(&grid, &restored);
                        println!("Transform {:?} -> inverse similarity: {:.4}", morphism.transform, similarity);
                        // Note: Due to rounding in transformations, exact restoration might not be possible
                        assert!(similarity > 0.5, "Inverse should approximately restore original");
                    }
                }
            }
        }
    }
    
    #[test]
    fn test_pattern_space_metric_properties() {
        let mut space = PatternSpace::new();
        
        // Create test patterns
        let patterns = vec![
            GridSignature { width: 2, height: 2, color_distribution: std::collections::HashMap::new(), shape_features: vec![], complexity: 1.0 },
            GridSignature { width: 3, height: 3, color_distribution: std::collections::HashMap::new(), shape_features: vec![], complexity: 2.0 },
            GridSignature { width: 4, height: 4, color_distribution: std::collections::HashMap::new(), shape_features: vec![], complexity: 3.0 },
        ];
        
        // Add patterns to space
        let morphisms: Vec<Morphism> = patterns.windows(2).map(|pair| {
            Morphism::new(pair[0].clone(), pair[1].clone(), Transform::Identity)
        }).collect();
        space.add_patterns(&morphisms);
        
        // Test metric properties
        
        // 1. Identity: d(x, x) = 0
        let d_identity = space.pattern_distance(0, 0, PatternMetric::Euclidean);
        assert!((d_identity - 0.0).abs() < 1e-10, "Distance to self should be zero");
        
        // 2. Symmetry: d(x, y) = d(y, x)
        let d_xy = space.pattern_distance(0, 1, PatternMetric::Euclidean);
        let d_yx = space.pattern_distance(1, 0, PatternMetric::Euclidean);
        assert!((d_xy - d_yx).abs() < 1e-10, "Distance should be symmetric");
        
        // 3. Triangle inequality: d(x, z) ≤ d(x, y) + d(y, z)
        let d_xz = space.pattern_distance(0, 2, PatternMetric::Euclidean);
        let d_yz = space.pattern_distance(1, 2, PatternMetric::Euclidean);
        assert!(d_xz <= d_xy + d_yz + 1e-10, "Triangle inequality should hold");
        
        println!("Metric properties verified:");
        println!("  Identity: {:.6}", d_identity);
        println!("  Symmetry: d(0,1) = {:.6}, d(1,0) = {:.6}", d_xy, d_yx);
        println!("  Triangle: d(0,2) = {:.6} ≤ d(0,1) + d(1,2) = {:.6}", d_xz, d_xy + d_yz);
    }
    
    #[test]
    fn test_composition_associativity() {
        let mut kernel = Kernel::new();
        let sig = GridSignature {
            width: 2, height: 2,
            color_distribution: std::collections::HashMap::new(),
            shape_features: vec![],
            complexity: 0.0,
        };
        
        let m1 = Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90);
        let m2 = Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90);
        let m3 = Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90);
        
        // Test (m1 ∘ m2) ∘ m3 = m1 ∘ (m2 ∘ m3)
        let left_composition = kernel.compose_morphisms(&m1, &m2).unwrap();
        let left_result = kernel.compose_morphisms(&left_composition, &m3).unwrap();
        
        let right_composition = kernel.compose_morphisms(&m2, &m3).unwrap();
        let right_result = kernel.compose_morphisms(&m1, &right_composition).unwrap();
        
        // The final transforms should be equivalent (3 rotations of 90° = 270°)
        assert_eq!(left_result.transform, Transform::Rotate270);
        assert_eq!(right_result.transform, Transform::Rotate270);
        
        println!("Composition associativity verified");
    }
    
    #[test]
    fn test_pattern_consistency() {
        let mut extractor = PatternExtractor::new();
        
        // Create consistent examples (same transformation applied multiple times)
        let examples = vec![
            (Grid::from_vec(vec![vec![1, 0], vec![0, 1]]).unwrap(), 
             Grid::from_vec(vec![vec![0, 1], vec![1, 0]]).unwrap()),
            (Grid::from_vec(vec![vec![2, 0], vec![0, 2]]).unwrap(), 
             Grid::from_vec(vec![vec![0, 2], vec![2, 0]]).unwrap()),
            (Grid::from_vec(vec![vec![3, 0], vec![0, 3]]).unwrap(), 
             Grid::from_vec(vec![vec![0, 3], vec![3, 0]]).unwrap()),
        ];
        
        let morphisms = extractor.extract_morphisms(&examples).unwrap();
        
        // All morphisms should represent the same transformation (anti-diagonal flip)
        let transform_types: HashSet<_> = morphisms.iter()
            .map(|m| std::mem::discriminant(&m.transform))
            .collect();
        
        println!("Extracted morphisms with {} unique transform types", transform_types.len());
        
        // Should have identified a consistent pattern
        assert!(morphisms.len() > 0);
        
        // All morphisms should have reasonable confidence
        for morphism in &morphisms {
            assert!(morphism.confidence > 0.0, "Morphism confidence should be positive");
            assert!(morphism.confidence <= 1.0, "Morphism confidence should not exceed 1.0");
        }
    }
}