//! Quick demo to test basic ARC-SCTT functionality
//! 
//! This simplified demo shows the core morphological intelligence
//! without the more complex features that are still being debugged.

use arc_sctt::*;
use arc_sctt::utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧠 ARC-SCTT Quick Demo");
    println!("======================");
    
    // Create basic grids
    let input = Grid::from_vec(vec![
        vec![1, 0],
        vec![0, 1],
    ])?;
    
    let output = Grid::from_vec(vec![
        vec![0, 1],
        vec![1, 0],
    ])?;
    
    println!("Input grid:");
    print!("{}", GridUtils::to_ascii(&input));
    
    println!("Output grid:");
    print!("{}", GridUtils::to_ascii(&output));
    
    // Test basic kernel operations
    let kernel = Kernel::new();
    
    // Try rotation
    let rotated = kernel.apply_transform(&input, &Transform::Rotate180)?;
    println!("After 180° rotation:");
    print!("{}", GridUtils::to_ascii(&rotated));
    
    // Try flip
    let flipped = kernel.apply_transform(&input, &Transform::FlipHorizontal)?;
    println!("After horizontal flip:");
    print!("{}", GridUtils::to_ascii(&flipped));
    
    // Test similarity
    let similarity = GridUtils::similarity(&output, &flipped);
    println!("Similarity between expected and flipped: {:.2}", similarity);
    
    // Extract morphism
    let morphism = kernel.extract_morphism(&input, &output)?;
    println!("Extracted morphism: {:?}", morphism.transform);
    println!("Confidence: {:.3}", morphism.confidence);
    
    println!("\n✨ Quick demo complete!");
    
    Ok(())
}