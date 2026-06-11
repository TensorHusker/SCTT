//! Advanced pattern recognition example
//! 
//! This example showcases the sophisticated pattern recognition capabilities
//! of the ARC-SCTT system with complex transformations and compositions.

use arc_sctt::*;
use arc_sctt::utils::*;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌟 Advanced Pattern Recognition with ARC-SCTT");
    println!("============================================");
    
    let mut system = ARCSCTTSystem::new();
    
    // Example 1: Complex color mapping with spatial transformation
    println!("\n🎨 Example 1: Complex Color Mapping");
    demonstrate_color_mapping(&mut system)?;
    
    // Example 2: Multi-step geometric transformations
    println!("\n🔄 Example 2: Multi-Step Geometric Transformations");
    demonstrate_geometric_sequence(&mut system)?;
    
    // Example 3: Conditional transformations
    println!("\n🔀 Example 3: Conditional Pattern Recognition");
    demonstrate_conditional_patterns(&mut system)?;
    
    // Example 4: Smooth morphing between patterns
    println!("\n🌊 Example 4: Smooth Pattern Morphing");
    demonstrate_smooth_morphing(&mut system)?;
    
    // Example 5: Pattern space exploration
    println!("\n🗺️  Example 5: Pattern Space Exploration");
    demonstrate_pattern_space(&mut system)?;
    
    // Example 6: Hierarchical pattern composition
    println!("\n🌳 Example 6: Hierarchical Pattern Composition");
    demonstrate_hierarchical_composition(&mut system)?;
    
    println!("\n✨ Advanced pattern demonstration complete!");
    
    Ok(())
}

fn demonstrate_color_mapping(system: &mut ARCSCTTSystem) -> Result<(), Box<dyn std::error::Error>> {
    // Create patterns with complex color mappings
    let input1 = Grid::from_vec(vec![
        vec![1, 2, 1],
        vec![2, 3, 2],
        vec![1, 2, 1],
    ])?;
    
    // Color mapping: 1->5, 2->6, 3->7
    let output1 = Grid::from_vec(vec![
        vec![5, 6, 5],
        vec![6, 7, 6],
        vec![5, 6, 5],
    ])?;
    
    let input2 = Grid::from_vec(vec![
        vec![2, 1, 3],
        vec![1, 2, 1],
        vec![3, 1, 2],
    ])?;
    
    let output2 = Grid::from_vec(vec![
        vec![6, 5, 7],
        vec![5, 6, 5],
        vec![7, 5, 6],
    ])?;
    
    println!("Input patterns with color mapping rule:");
    println!("Input 1:\n{}", GridUtils::to_ascii(&input1));
    println!("Output 1:\n{}", GridUtils::to_ascii(&output1));
    
    let examples = vec![(input1, output1), (input2, output2)];
    let morphisms = system.morphology.extract_morphisms(&examples)?;
    
    println!("Extracted {} morphisms", morphisms.len());
    for morphism in &morphisms {
        if let Transform::ColorMap(map) = &morphism.transform {
            println!("Color mapping detected: {:?}", map);
        }
    }
    
    // Test on new input
    let test_input = Grid::from_vec(vec![
        vec![3, 2, 3],
        vec![2, 1, 2],
        vec![3, 2, 3],
    ])?;
    
    println!("Test input:\n{}", GridUtils::to_ascii(&test_input));
    
    let solution = system.solve_puzzle(&examples, &test_input)?;
    println!("Predicted output:\n{}", GridUtils::to_ascii(&solution));
    
    Ok(())
}

fn demonstrate_geometric_sequence(system: &mut ARCSCTTSystem) -> Result<(), Box<dyn std::error::Error>> {
    // Multi-step transformation: Rotate 90° then flip horizontally
    let input = Grid::from_vec(vec![
        vec![1, 0, 0, 0],
        vec![0, 2, 0, 0],
        vec![0, 0, 3, 0],
        vec![0, 0, 0, 4],
    ])?;
    
    // First rotate 90°
    let rotated = system.kernel.apply_transform(&input, &Transform::Rotate90)?;
    
    // Then flip horizontally  
    let final_output = system.kernel.apply_transform(&rotated, &Transform::FlipHorizontal)?;
    
    println!("Original pattern:\n{}", GridUtils::to_ascii(&input));
    println!("After 90° rotation:\n{}", GridUtils::to_ascii(&rotated));
    println!("After horizontal flip:\n{}", GridUtils::to_ascii(&final_output));
    
    // Extract morphisms for the complete transformation
    let morphisms = system.morphology.extract_single_morphism(&input, &final_output)?;
    
    println!("Complex transformation extracted as {} morphisms", morphisms.len());
    for (i, morphism) in morphisms.iter().enumerate() {
        println!("  Morphism {}: {:?} (confidence: {:.3})", 
                 i + 1, morphism.transform, morphism.confidence);
    }
    
    // Learn the composition
    let composition = system.compositor.learn_composition(&morphisms)?;
    println!("Learned composition type: {:?}", composition.composition_type);
    
    Ok(())
}

fn demonstrate_conditional_patterns(system: &mut ARCSCTTSystem) -> Result<(), Box<dyn std::error::Error>> {
    // Pattern that depends on input properties
    let input1 = Grid::from_vec(vec![
        vec![1, 1],  // Symmetric pattern
        vec![1, 1],
    ])?;
    
    let output1 = Grid::from_vec(vec![
        vec![2, 2],  // Color change for symmetric patterns
        vec![2, 2],
    ])?;
    
    let input2 = Grid::from_vec(vec![
        vec![1, 0],  // Asymmetric pattern
        vec![0, 1],
    ])?;
    
    let output2 = Grid::from_vec(vec![
        vec![0, 1],  // Flip for asymmetric patterns
        vec![1, 0],
    ])?;
    
    println!("Conditional pattern examples:");
    println!("Symmetric input:\n{}", GridUtils::to_ascii(&input1));
    println!("-> Color change:\n{}", GridUtils::to_ascii(&output1));
    
    println!("Asymmetric input:\n{}", GridUtils::to_ascii(&input2));
    println!("-> Flip transformation:\n{}", GridUtils::to_ascii(&output2));
    
    let examples = vec![(input1, output1), (input2, output2)];
    let morphisms = system.morphology.extract_morphisms(&examples)?;
    
    println!("Detected {} different transformation patterns", morphisms.len());
    
    // Test conditional reasoning
    let test_symmetric = Grid::from_vec(vec![
        vec![3, 3],
        vec![3, 3],
    ])?;
    
    let test_asymmetric = Grid::from_vec(vec![
        vec![3, 0],
        vec![0, 3],
    ])?;
    
    println!("Testing symmetric pattern:\n{}", GridUtils::to_ascii(&test_symmetric));
    let solution1 = system.solve_puzzle(&examples, &test_symmetric)?;
    println!("Result:\n{}", GridUtils::to_ascii(&solution1));
    
    println!("Testing asymmetric pattern:\n{}", GridUtils::to_ascii(&test_asymmetric));
    let solution2 = system.solve_puzzle(&examples, &test_asymmetric)?;
    println!("Result:\n{}", GridUtils::to_ascii(&solution2));
    
    Ok(())
}

fn demonstrate_smooth_morphing(system: &mut ARCSCTTSystem) -> Result<(), Box<dyn std::error::Error>> {
    let sig1 = GridSignature {
        width: 3, height: 3,
        color_distribution: [(1, 4), (0, 5)].iter().cloned().collect(),
        shape_features: vec![2],
        complexity: 1.0,
    };
    
    let sig2 = GridSignature {
        width: 3, height: 3,
        color_distribution: [(2, 4), (0, 5)].iter().cloned().collect(),
        shape_features: vec![2],
        complexity: 1.2,
    };
    
    let morphism = Morphism::new(sig1.clone(), sig2.clone(), Transform::Rotate90);
    system.pattern_space.add_patterns(&[morphism]);
    
    println!("Smooth interpolation between patterns:");
    
    for i in 0..=5 {
        let t = i as f64 / 5.0;
        if let Ok(interpolated) = system.pattern_space.interpolate_patterns(0, 1, t) {
            println!("t = {:.1}: geometric_features = {:?}", 
                     t, &interpolated.geometric_features[..3]);
        }
    }
    
    // Demonstrate smooth transformations
    let grid = Grid::from_vec(vec![
        vec![1, 0, 1],
        vec![0, 2, 0],
        vec![1, 0, 1],
    ])?;
    
    println!("Original grid:\n{}", GridUtils::to_ascii(&grid));
    
    let smooth_rotated = system.compositor.smooth_rotate_90(&grid, 2)?;
    println!("Smooth 90° rotation:\n{}", GridUtils::to_ascii(&smooth_rotated));
    
    let smooth_scaled = system.compositor.smooth_scale(&grid, 1.5, 1)?;
    println!("Smooth 1.5x scaling:\n{}", GridUtils::to_ascii(&smooth_scaled));
    
    Ok(())
}

fn demonstrate_pattern_space(system: &mut ARCSCTTSystem) -> Result<(), Box<dyn std::error::Error>> {
    // Create diverse patterns for pattern space analysis
    let patterns = vec![
        (GridPattern::Checkerboard, "Checkerboard"),
        (GridPattern::Diagonal, "Diagonal"),
        (GridPattern::Border, "Border"),
        (GridPattern::Center, "Center"),
        (GridPattern::Random(42), "Random"),
    ];
    
    let mut signatures = Vec::new();
    let mut morphisms = Vec::new();
    
    for (pattern, name) in &patterns {
        let grid = GridUtils::create_pattern(4, 4, pattern.clone())?;
        let signature = grid.signature();
        signatures.push((signature.clone(), name));
        
        // Create identity morphism for each pattern
        let morphism = Morphism::new(signature.clone(), signature, Transform::Identity);
        morphisms.push(morphism);
    }
    
    system.pattern_space.add_patterns(&morphisms);
    
    println!("Pattern space analysis:");
    println!("Intrinsic dimensionality: {:.2}", system.pattern_space.intrinsic_dimensionality());
    
    // Compute distances between patterns
    println!("\nPattern distance matrix:");
    for i in 0..patterns.len() {
        for j in 0..patterns.len() {
            let distance = system.pattern_space.pattern_distance(i, j, PatternMetric::Euclidean);
            print!("{:6.2} ", distance);
        }
        println!("  ({})", patterns[i].1);
    }
    
    // Find transformation paths
    let checkerboard_sig = &signatures[0].0;
    let diagonal_sig = &signatures[1].0;
    
    let paths = system.pattern_space.find_transformation_paths(checkerboard_sig, diagonal_sig);
    println!("\nTransformation paths from Checkerboard to Diagonal: {}", paths.len());
    
    for (i, path) in paths.iter().take(3).enumerate() {
        println!("Path {}: {} steps", i + 1, path.len());
        for step in path {
            println!("  {:?} (length: {:.3})", step.morphism.transform, step.length);
        }
    }
    
    Ok(())
}

fn demonstrate_hierarchical_composition(system: &mut ARCSCTTSystem) -> Result<(), Box<dyn std::error::Error>> {
    // Create a hierarchical pattern: (Rotate -> Scale) ∘ (Flip -> Transpose)
    let sig = GridSignature {
        width: 3, height: 3,
        color_distribution: HashMap::new(),
        shape_features: vec![],
        complexity: 1.0,
    };
    
    // Level 1: Basic transformations
    let rotate_morphism = Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90);
    let flip_morphism = Morphism::new(sig.clone(), sig.clone(), Transform::FlipHorizontal);
    let transpose_morphism = Morphism::new(sig.clone(), sig.clone(), Transform::Transpose);
    let scale_morphism = Morphism::new(sig.clone(), sig.clone(), Transform::Scale { factor: 1.2 });
    
    // Level 2: Compose pairs
    let composed1 = system.kernel.compose_morphisms(&rotate_morphism, &scale_morphism)?;
    let composed2 = system.kernel.compose_morphisms(&flip_morphism, &transpose_morphism)?;
    
    println!("Hierarchical composition demonstration:");
    println!("Level 1 morphisms:");
    println!("  Rotate90 + Scale1.2 -> {:?}", composed1.transform);
    println!("  FlipH + Transpose -> {:?}", composed2.transform);
    
    // Level 3: Compose the composed morphisms
    let final_composition = system.kernel.compose_morphisms(&composed1, &composed2)?;
    println!("Final composition: {:?}", final_composition.transform);
    
    // Learn hierarchical composition patterns
    let all_morphisms = vec![rotate_morphism, flip_morphism, transpose_morphism, scale_morphism];
    let learned_composition = system.compositor.learn_composition(&all_morphisms)?;
    
    println!("Learned hierarchical pattern:");
    println!("  Type: {:?}", learned_composition.composition_type);
    println!("  Success rate: {:.3}", learned_composition.success_rate);
    println!("  Smoothness: {:.3}", learned_composition.smoothness_score);
    println!("  Sequence length: {}", learned_composition.morphism_sequence.len());
    
    // Apply to test grid
    let test_grid = Grid::from_vec(vec![
        vec![1, 0, 2],
        vec![0, 3, 0],
        vec![2, 0, 1],
    ])?;
    
    println!("\nApplying hierarchical composition:");
    println!("Original:\n{}", GridUtils::to_ascii(&test_grid));
    
    let result = system.solve_with_morphisms(&test_grid, &learned_composition)?;
    println!("After hierarchical transformation:\n{}", GridUtils::to_ascii(&result));
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_advanced_patterns() {
        let result = std::panic::catch_unwind(|| {
            main().unwrap()
        });
        
        assert!(result.is_ok(), "Advanced patterns demo should run without panicking");
    }
    
    #[test]
    fn test_color_mapping_extraction() {
        let mut system = ARCSCTTSystem::new();
        let result = demonstrate_color_mapping(&mut system);
        assert!(result.is_ok(), "Color mapping demonstration should succeed");
    }
    
    #[test]
    fn test_geometric_sequence() {
        let mut system = ARCSCTTSystem::new();
        let result = demonstrate_geometric_sequence(&mut system);
        assert!(result.is_ok(), "Geometric sequence demonstration should succeed");
    }
}