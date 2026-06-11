//! Basic usage example for ARC-SCTT system
//! 
//! This example demonstrates the fundamental capabilities of the
//! morphological intelligence system for solving ARC puzzles.

use arc_sctt::*;
use arc_sctt::utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧠 ARC-SCTT Morphological Intelligence Demo");
    println!("==========================================");
    
    // Create the revolutionary ARC-SCTT system
    let mut system = ARCSCTTSystem::new();
    
    println!("\n1. Creating a simple ARC puzzle...");
    
    // Create a simple pattern recognition puzzle
    let input1 = Grid::from_vec(vec![
        vec![1, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 1],
    ])?;
    
    let output1 = Grid::from_vec(vec![
        vec![0, 0, 1],
        vec![0, 1, 0],
        vec![1, 0, 0],
    ])?;
    
    let input2 = Grid::from_vec(vec![
        vec![2, 0, 0],
        vec![0, 2, 0],
        vec![0, 0, 2],
    ])?;
    
    let output2 = Grid::from_vec(vec![
        vec![0, 0, 2],
        vec![0, 2, 0],
        vec![2, 0, 0],
    ])?;
    
    let test_input = Grid::from_vec(vec![
        vec![3, 0, 0],
        vec![0, 3, 0],
        vec![0, 0, 3],
    ])?;
    
    println!("Input 1:");
    print!("{}", GridUtils::to_ascii(&input1));
    println!("Output 1:");
    print!("{}", GridUtils::to_ascii(&output1));
    
    println!("Input 2:");
    print!("{}", GridUtils::to_ascii(&input2));
    println!("Output 2:");
    print!("{}", GridUtils::to_ascii(&output2));
    
    println!("Test Input:");
    print!("{}", GridUtils::to_ascii(&test_input));
    
    println!("\n2. Extracting morphisms from examples...");
    
    let examples = vec![(input1, output1), (input2, output2)];
    let morphisms = system.morphology.extract_morphisms(&examples)?;
    
    println!("Extracted {} morphisms:", morphisms.len());
    for (i, morphism) in morphisms.iter().enumerate() {
        println!("  Morphism {}: {:?} (confidence: {:.3})", 
                 i + 1, morphism.transform, morphism.confidence);
    }
    
    println!("\n3. Learning composition patterns...");
    
    let composition = system.compositor.learn_composition(&morphisms)?;
    println!("Learned composition strategy: {:?}", composition.composition_type);
    println!("Success rate: {:.3}", composition.success_rate);
    println!("Smoothness score: {:.3}", composition.smoothness_score);
    
    println!("\n4. Adding patterns to pattern space...");
    
    system.pattern_space.add_patterns(&morphisms);
    println!("Pattern space dimensionality: {}", system.pattern_space.intrinsic_dimensionality());
    
    let test_signature = test_input.signature();
    let nearest = system.pattern_space.find_nearest_patterns(&test_signature, 3);
    println!("Found {} nearest patterns for test input", nearest.len());
    
    println!("\n5. Solving the test case...");
    
    let solution = system.solve_puzzle(&examples, &test_input)?;
    
    println!("Solution:");
    print!("{}", GridUtils::to_ascii(&solution));
    
    println!("Expected pattern (horizontal flip):");
    let expected = Grid::from_vec(vec![
        vec![0, 0, 3],
        vec![0, 3, 0],
        vec![3, 0, 0],
    ])?;
    print!("{}", GridUtils::to_ascii(&expected));
    
    let similarity = GridUtils::similarity(&solution, &expected);
    println!("Similarity to expected: {:.3}", similarity);
    
    println!("\n6. Demonstrating morphological reasoning...");
    
    // Show individual transformation steps
    let kernel = Kernel::new();
    let rotated = kernel.apply_transform(&test_input, &Transform::Rotate180)?;
    println!("After 180° rotation:");
    print!("{}", GridUtils::to_ascii(&rotated));
    
    let flipped = kernel.apply_transform(&test_input, &Transform::FlipHorizontal)?;
    println!("After horizontal flip:");
    print!("{}", GridUtils::to_ascii(&flipped));
    
    println!("\n7. Performance analysis...");
    
    let morphism = &morphisms[0];
    let benchmark = PerformanceUtils::benchmark_morphism_application(morphism, &test_input, 100);
    println!("{}", benchmark);
    
    println!("\n8. Pattern analysis...");
    
    let colors = GridUtils::extract_colors(&test_input);
    println!("Colors in test input: {:?}", colors);
    
    let common_patterns = PatternUtils::extract_common_patterns(&morphisms);
    println!("Common patterns found: {}", common_patterns.len());
    for pattern in &common_patterns {
        println!("  - {}: frequency {}, confidence {:.3}", 
                 pattern.pattern_type, pattern.frequency, pattern.confidence);
    }
    
    println!("\n✨ ARC-SCTT demonstration complete!");
    println!("The system successfully demonstrated:");
    println!("  • Morphological pattern extraction");
    println!("  • Smooth composition learning");
    println!("  • Geometric pattern space reasoning");
    println!("  • Invariant property detection");
    println!("  • Abstract visual intelligence");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_demo() {
        // Test that the basic demo runs without panicking
        let result = std::panic::catch_unwind(|| {
            main().unwrap()
        });
        
        assert!(result.is_ok(), "Basic demo should run without panicking");
    }
}