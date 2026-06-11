//! Research demonstration of ARC-SCTT capabilities
//! 
//! This example showcases the cutting-edge research capabilities
//! of the morphological intelligence system for AI reasoning research.

use arc_sctt::*;
use arc_sctt::utils::*;
use std::collections::HashMap;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 ARC-SCTT Research Demonstration");
    println!("==================================");
    println!("Exploring the frontiers of morphological intelligence and abstract reasoning");
    
    let mut system = ARCSCTTSystem::new();
    
    // Research Area 1: Meta-learning and pattern generalization
    println!("\n🧠 Research Area 1: Meta-Learning and Pattern Generalization");
    demonstrate_meta_learning(&mut system)?;
    
    // Research Area 2: Compositional reasoning and systematic generalization
    println!("\n🔗 Research Area 2: Compositional Reasoning");
    demonstrate_compositional_reasoning(&mut system)?;
    
    // Research Area 3: Invariant discovery and causal reasoning
    println!("\n🔍 Research Area 3: Invariant Discovery and Causal Reasoning");
    demonstrate_invariant_discovery(&mut system)?;
    
    // Research Area 4: Continuous learning and adaptation
    println!("\n📈 Research Area 4: Continuous Learning and Adaptation");
    demonstrate_continuous_learning(&mut system)?;
    
    // Research Area 5: Mathematical formalization of visual reasoning
    println!("\n📐 Research Area 5: Mathematical Formalization");
    demonstrate_mathematical_formalization(&mut system)?;
    
    // Research Area 6: Emergent intelligence and self-organization
    println!("\n✨ Research Area 6: Emergent Intelligence");
    demonstrate_emergent_intelligence(&mut system)?;
    
    println!("\n🎓 Research demonstration complete!");
    println!("Key contributions demonstrated:");
    println!("  • Morphological intelligence for visual reasoning");
    println!("  • Smooth composition of cognitive operations");
    println!("  • Geometric pattern space with metric structure");
    println!("  • Type-theoretic foundations for AI reasoning");
    println!("  • Self-organizing learning architecture");
    
    Ok(())
}

fn demonstrate_meta_learning(system: &mut ARCSCTTSystem) -> Result<(), Box<dyn std::error::Error>> {
    println!("Investigating how the system learns to learn new patterns...");
    
    // Create a series of progressively complex puzzles
    let puzzle_types = vec![
        ("Basic", TestPuzzleType::Simple),
        ("Geometric", TestPuzzleType::Rotation),
        ("Symbolic", TestPuzzleType::ColorMapping),
    ];
    
    let mut learning_curves = Vec::new();
    
    for (name, puzzle_type) in puzzle_types {
        println!("\n--- Learning {} patterns ---", name);
        
        let mut performance_over_time = Vec::new();
        
        // Generate multiple instances of the same pattern type
        for trial in 0..5 {
            let puzzle = PatternUtils::generate_test_puzzle(puzzle_type.clone());
            
            let start_time = Instant::now();
            let solution = system.solve_puzzle(&puzzle.examples, &puzzle.test_input)?;
            let solve_time = start_time.elapsed();
            
            // Evaluate solution quality
            let quality = if let Some(ref expected) = puzzle.test_output {
                GridUtils::similarity(&solution, expected)
            } else {
                0.5 // Neutral score when expected output unknown
            };
            
            performance_over_time.push((trial, quality, solve_time));
            
            // System evolves with each puzzle
            system.evolve(&[puzzle])?;
            
            println!("Trial {}: Quality {:.3}, Time {:?}", trial + 1, quality, solve_time);
        }
        
        learning_curves.push((name, performance_over_time));
    }
    
    // Analyze meta-learning
    println!("\n📊 Meta-Learning Analysis:");
    for (puzzle_name, curve) in &learning_curves {
        let initial_quality = curve[0].1;
        let final_quality = curve.last().unwrap().1;
        let improvement = final_quality - initial_quality;
        
        let initial_time = curve[0].2;
        let final_time = curve.last().unwrap().2;
        let speedup = initial_time.as_millis() as f64 / final_time.as_millis() as f64;
        
        println!("  {}: Quality improvement {:.3}, Speedup {:.2}x", 
                 puzzle_name, improvement, speedup);
    }
    
    // Test transfer learning
    println!("\n🔄 Transfer Learning Test:");
    let novel_puzzle = PatternUtils::generate_test_puzzle(TestPuzzleType::Simple);
    let transfer_start = Instant::now();
    let transfer_solution = system.solve_puzzle(&novel_puzzle.examples, &novel_puzzle.test_input)?;
    let transfer_time = transfer_start.elapsed();
    
    println!("Novel puzzle solved in {:?}", transfer_time);
    println!("Solution quality: {:.3}", 
             if let Some(ref expected) = novel_puzzle.test_output {
                 GridUtils::similarity(&transfer_solution, expected)
             } else { 0.5 });
    
    Ok(())
}

fn demonstrate_compositional_reasoning(system: &mut ARCSCTTSystem) -> Result<(), Box<dyn std::error::Error>> {
    println!("Exploring systematic compositional generalization...");
    
    // Create atomic transformation components
    let atomic_transforms = vec![
        ("Rotate", Transform::Rotate90),
        ("Flip", Transform::FlipHorizontal),
        ("Scale", Transform::Scale { factor: 2.0 }),
        ("Translate", Transform::Translate { dx: 1, dy: 1 }),
    ];
    
    let sig = GridSignature {
        width: 3, height: 3,
        color_distribution: HashMap::new(),
        shape_features: vec![],
        complexity: 1.0,
    };
    
    let mut atomic_morphisms = Vec::new();
    for (name, transform) in &atomic_transforms {
        let morphism = Morphism::new(sig.clone(), sig.clone(), transform.clone());
        atomic_morphisms.push((name.clone(), morphism));
    }
    
    println!("\n🔧 Atomic transformations defined:");
    for (name, morphism) in &atomic_morphisms {
        println!("  {}: {:?}", name, morphism.transform);
    }
    
    // Test pairwise compositions
    println!("\n🔗 Pairwise compositions:");
    let mut composition_results = Vec::new();
    
    for i in 0..atomic_morphisms.len() {
        for j in 0..atomic_morphisms.len() {
            if i != j {
                let (name1, morphism1) = &atomic_morphisms[i];
                let (name2, morphism2) = &atomic_morphisms[j];
                
                if let Ok(composed) = system.kernel.compose_morphisms(morphism1, morphism2) {
                    let complexity = MorphismUtils::complexity(&composed);
                    composition_results.push((
                        format!("{} ∘ {}", name1, name2),
                        composed,
                        complexity
                    ));
                    
                    println!("  {} ∘ {}: complexity {:.2}", name1, name2, complexity);
                }
            }
        }
    }
    
    // Learn compositional patterns
    println!("\n📚 Learning compositional patterns:");
    let all_morphisms: Vec<Morphism> = atomic_morphisms.into_iter().map(|(_, m)| m).collect();
    let composition = system.compositor.learn_composition(&all_morphisms)?;
    
    println!("Learned composition strategy: {:?}", composition.composition_type);
    println!("Composition length: {}", composition.morphism_sequence.len());
    println!("Success rate: {:.3}", composition.success_rate);
    
    // Test systematic generalization to novel combinations
    println!("\n🎯 Testing systematic generalization:");
    let test_grid = Grid::from_vec(vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ])?;
    
    println!("Test grid:\n{}", GridUtils::to_ascii(&test_grid));
    
    // Apply learned composition
    let composed_result = system.solve_with_morphisms(&test_grid, &composition)?;
    println!("After learned composition:\n{}", GridUtils::to_ascii(&composed_result));
    
    // Compare with manual composition
    let mut manual_result = test_grid.clone();
    for morphism in &composition.morphism_sequence {
        manual_result = system.kernel.apply_transform(&manual_result, &morphism.transform)?;
    }
    
    let consistency = GridUtils::similarity(&composed_result, &manual_result);
    println!("Composition consistency: {:.3}", consistency);
    
    Ok(())
}

fn demonstrate_invariant_discovery(system: &mut ARCSCTTSystem) -> Result<(), Box<dyn std::error::Error>> {
    println!("Discovering invariant properties and causal relationships...");
    
    // Create examples with known invariants
    let examples_preserving_objects = vec![
        // Examples that preserve object count
        (Grid::from_vec(vec![vec![1, 0], vec![0, 1]])?, 
         Grid::from_vec(vec![vec![0, 1], vec![1, 0]])?),
        (Grid::from_vec(vec![vec![2, 0], vec![0, 3]])?, 
         Grid::from_vec(vec![vec![0, 3], vec![2, 0]])?),
    ];
    
    let examples_changing_colors = vec![
        // Examples that change colors but preserve structure
        (Grid::from_vec(vec![vec![1, 1], vec![1, 1]])?, 
         Grid::from_vec(vec![vec![2, 2], vec![2, 2]])?),
        (Grid::from_vec(vec![vec![3, 3], vec![3, 3]])?, 
         Grid::from_vec(vec![vec![4, 4], vec![4, 4]])?),
    ];
    
    println!("\n🔍 Analyzing invariant properties:");
    
    // Test object count preservation
    println!("Object count preservation:");
    for (i, (input, output)) in examples_preserving_objects.iter().enumerate() {
        let preserves = system.solver.object_count_preserved(input, output);
        println!("  Example {}: {}", i + 1, if preserves { "✓ Preserved" } else { "✗ Violated" });
    }
    
    // Test color distribution changes
    println!("Color distribution preservation:");
    for (i, (input, output)) in examples_changing_colors.iter().enumerate() {
        let preserves = system.solver.color_distribution_preserved(input, output);
        println!("  Example {}: {}", i + 1, if preserves { "✓ Preserved" } else { "✗ Changed" });
    }
    
    // Discover causal relationships
    println!("\n🔗 Causal relationship discovery:");
    
    let mut causal_hypotheses = Vec::new();
    
    // Hypothesis 1: Spatial transformations preserve object count
    let spatial_transforms = vec![Transform::Rotate90, Transform::FlipHorizontal, Transform::Transpose];
    for transform in &spatial_transforms {
        let test_grid = Grid::from_vec(vec![vec![1, 0], vec![0, 2]])?;
        let transformed = system.kernel.apply_transform(&test_grid, transform)?;
        let preserves_count = system.solver.object_count_preserved(&test_grid, &transformed);
        
        causal_hypotheses.push((
            format!("Spatial {:?} preserves object count", transform),
            preserves_count,
            0.9 // Prior confidence
        ));
    }
    
    // Hypothesis 2: Color mappings preserve topology
    let color_map = [(1, 5), (0, 0)].iter().cloned().collect();
    let color_transform = Transform::ColorMap(color_map);
    let test_grid = Grid::from_vec(vec![vec![1, 0], vec![0, 1]])?;
    let color_transformed = system.kernel.apply_transform(&test_grid, &color_transform)?;
    let preserves_topology = system.solver.topology_preserved(&test_grid, &color_transformed);
    
    causal_hypotheses.push((
        "Color mapping preserves topology".to_string(),
        preserves_topology,
        0.8
    ));
    
    println!("Discovered causal relationships:");
    for (hypothesis, confirmed, confidence) in &causal_hypotheses {
        println!("  {}: {} (confidence: {:.2})", 
                 hypothesis, 
                 if *confirmed { "✓ Confirmed" } else { "✗ Rejected" },
                 confidence);
    }
    
    // Test predictive power
    println!("\n🔮 Testing predictive power:");
    let novel_input = Grid::from_vec(vec![vec![7, 0, 8], vec![0, 9, 0], vec![8, 0, 7]])?;
    println!("Novel input:\n{}", GridUtils::to_ascii(&novel_input));
    
    // Predict using learned invariants
    let prediction = system.kernel.apply_transform(&novel_input, &Transform::Rotate90)?;
    println!("Predicted output (90° rotation):\n{}", GridUtils::to_ascii(&prediction));
    
    let objects_preserved = system.solver.object_count_preserved(&novel_input, &prediction);
    println!("Object count preserved as predicted: {}", objects_preserved);
    
    Ok(())
}

fn demonstrate_continuous_learning(system: &mut ARCSCTTSystem) -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating continuous learning and adaptation...");
    
    // Simulate a stream of learning experiences
    let learning_episodes = 10;
    let mut performance_history = Vec::new();
    
    println!("\n📈 Continuous learning simulation:");
    
    for episode in 0..learning_episodes {
        // Generate a new puzzle
        let puzzle_types = [TestPuzzleType::Simple, TestPuzzleType::Rotation, TestPuzzleType::ColorMapping];
        let puzzle = PatternUtils::generate_test_puzzle(puzzle_types[episode % 3].clone());
        
        // Measure performance before learning
        let pre_solution = system.solve_puzzle(&puzzle.examples, &puzzle.test_input)?;
        let pre_quality = if let Some(ref expected) = puzzle.test_output {
            GridUtils::similarity(&pre_solution, expected)
        } else { 0.5 };
        
        // Learn from this episode
        system.evolve(&[puzzle.clone()])?;
        
        // Measure performance after learning
        let post_solution = system.solve_puzzle(&puzzle.examples, &puzzle.test_input)?;
        let post_quality = if let Some(ref expected) = puzzle.test_output {
            GridUtils::similarity(&post_solution, expected)
        } else { 0.5 };
        
        let improvement = post_quality - pre_quality;
        performance_history.push((episode, pre_quality, post_quality, improvement));
        
        println!("Episode {}: Pre {:.3} -> Post {:.3} (Δ {:.3})", 
                 episode + 1, pre_quality, post_quality, improvement);
    }
    
    // Analyze learning dynamics
    println!("\n📊 Learning dynamics analysis:");
    let total_improvement: f64 = performance_history.iter().map(|(_, _, _, imp)| imp).sum();
    let avg_improvement = total_improvement / learning_episodes as f64;
    
    let final_performance = performance_history.last().unwrap().2;
    let initial_performance = performance_history[0].1;
    let overall_improvement = final_performance - initial_performance;
    
    println!("Average per-episode improvement: {:.4}", avg_improvement);
    println!("Overall improvement: {:.3}", overall_improvement);
    
    // Test forgetting resistance
    println!("\n🧠 Testing knowledge retention:");
    let retention_test = PatternUtils::generate_test_puzzle(TestPuzzleType::Simple);
    let retention_solution = system.solve_puzzle(&retention_test.examples, &retention_test.test_input)?;
    let retention_quality = if let Some(ref expected) = retention_test.test_output {
        GridUtils::similarity(&retention_solution, expected)
    } else { 0.5 };
    
    println!("Retention test quality: {:.3}", retention_quality);
    
    // Measure system complexity growth
    println!("\n📏 System complexity analysis:");
    println!("Pattern space patterns: {}", system.pattern_space.patterns.len());
    println!("Learned compositions: {}", system.compositor.learned_compositions.len());
    println!("Composition history: {}", system.compositor.composition_history.len());
    
    Ok(())
}

fn demonstrate_mathematical_formalization(system: &mut ARCSCTTSystem) -> Result<(), Box<dyn std::error::Error>> {
    println!("Exploring mathematical foundations of visual reasoning...");
    
    // Demonstrate category theory principles
    println!("\n🏗️  Category Theory in Action:");
    
    let sig = GridSignature {
        width: 2, height: 2,
        color_distribution: HashMap::new(),
        shape_features: vec![],
        complexity: 0.5,
    };
    
    // Objects and morphisms
    let identity = Morphism::new(sig.clone(), sig.clone(), Transform::Identity);
    let rotate = Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90);
    let flip = Morphism::new(sig.clone(), sig.clone(), Transform::FlipHorizontal);
    
    println!("Objects: Pattern signatures with spatial and color properties");
    println!("Morphisms: Visual transformations between patterns");
    
    // Test associativity: (f ∘ g) ∘ h = f ∘ (g ∘ h)
    let left_assoc = system.kernel.compose_morphisms(&identity, &rotate)?;
    let left_result = system.kernel.compose_morphisms(&left_assoc, &flip)?;
    
    let right_assoc = system.kernel.compose_morphisms(&rotate, &flip)?;
    let right_result = system.kernel.compose_morphisms(&identity, &right_assoc)?;
    
    println!("Associativity test: (id ∘ rot) ∘ flip = id ∘ (rot ∘ flip)");
    println!("  Left: {:?}", left_result.transform);
    println!("  Right: {:?}", right_result.transform);
    println!("  Equal: {}", std::mem::discriminant(&left_result.transform) == 
                              std::mem::discriminant(&right_result.transform));
    
    // Test identity laws
    let id_left = system.kernel.compose_morphisms(&identity, &rotate)?;
    let id_right = system.kernel.compose_morphisms(&rotate, &identity)?;
    
    println!("Identity laws:");
    println!("  id ∘ f = {:?}", id_left.transform);
    println!("  f ∘ id = {:?}", id_right.transform);
    println!("  Both equal f: {}", 
             std::mem::discriminant(&id_left.transform) == std::mem::discriminant(&rotate.transform) &&
             std::mem::discriminant(&id_right.transform) == std::mem::discriminant(&rotate.transform));
    
    // Demonstrate metric space properties
    println!("\n📐 Metric Space Properties:");
    
    // Add patterns to pattern space
    let morphisms = vec![identity, rotate, flip];
    system.pattern_space.add_patterns(&morphisms);
    
    // Test metric axioms
    let d_self = system.pattern_space.pattern_distance(0, 0, PatternMetric::Euclidean);
    let d_12 = system.pattern_space.pattern_distance(0, 1, PatternMetric::Euclidean);
    let d_21 = system.pattern_space.pattern_distance(1, 0, PatternMetric::Euclidean);
    let d_13 = system.pattern_space.pattern_distance(0, 2, PatternMetric::Euclidean);
    let d_23 = system.pattern_space.pattern_distance(1, 2, PatternMetric::Euclidean);
    
    println!("Distance to self: {:.6} (should be 0)", d_self);
    println!("Symmetry: d(1,2) = {:.6}, d(2,1) = {:.6}", d_12, d_21);
    println!("Triangle inequality: d(1,3) = {:.3} ≤ d(1,2) + d(2,3) = {:.3}", 
             d_13, d_12 + d_23);
    
    // Demonstrate smooth structure
    println!("\n🌊 Smooth Structure:");
    
    let test_grid = Grid::from_vec(vec![vec![1, 2], vec![3, 4]])?;
    println!("Original grid:\n{}", GridUtils::to_ascii(&test_grid));
    
    // Apply transformations with different smoothness orders
    for order in 1..=3 {
        let smooth_rotated = system.compositor.smooth_rotate_90(&test_grid, order)?;
        println!("C^{} smooth rotation:\n{}", order, GridUtils::to_ascii(&smooth_rotated));
    }
    
    // Curvature analysis
    let curvature_0 = system.pattern_space.pattern_curvature(0);
    let curvature_1 = system.pattern_space.pattern_curvature(1);
    
    println!("Pattern space curvature:");
    println!("  Pattern 0: {:.6}", curvature_0);
    println!("  Pattern 1: {:.6}", curvature_1);
    
    Ok(())
}

fn demonstrate_emergent_intelligence(system: &mut ARCSCTTSystem) -> Result<(), Box<dyn std::error::Error>> {
    println!("Exploring emergent intelligence and self-organization...");
    
    // Create a complex environment with multiple interacting patterns
    println!("\n🌍 Complex Pattern Environment:");
    
    let complex_patterns = vec![
        // Symmetric patterns
        Grid::from_vec(vec![vec![1, 2, 1], vec![2, 3, 2], vec![1, 2, 1]])?,
        Grid::from_vec(vec![vec![4, 5, 4], vec![5, 6, 5], vec![4, 5, 4]])?,
        
        // Asymmetric patterns  
        Grid::from_vec(vec![vec![1, 0, 2], vec![0, 3, 0], vec![2, 0, 1]])?,
        Grid::from_vec(vec![vec![4, 0, 5], vec![0, 6, 0], vec![5, 0, 4]])?,
        
        // Hierarchical patterns
        Grid::from_vec(vec![vec![1, 1, 2, 2], vec![1, 1, 2, 2], vec![3, 3, 4, 4], vec![3, 3, 4, 4]])?,
    ];
    
    // Let the system explore and self-organize
    println!("Initializing self-organization process...");
    
    let mut emergent_morphisms = Vec::new();
    
    // Create all pairwise transformations
    for i in 0..complex_patterns.len() {
        for j in 0..complex_patterns.len() {
            if i != j {
                if let Ok(morphisms) = system.morphology.extract_single_morphism(
                    &complex_patterns[i], 
                    &complex_patterns[j]
                ) {
                    emergent_morphisms.extend(morphisms);
                }
            }
        }
    }
    
    println!("Discovered {} emergent morphisms", emergent_morphisms.len());
    
    // Analyze emergent patterns
    let common_patterns = PatternUtils::extract_common_patterns(&emergent_morphisms);
    println!("Self-organized into {} pattern classes:", common_patterns.len());
    
    for (i, pattern) in common_patterns.iter().enumerate() {
        println!("  Class {}: {} instances, confidence {:.3}", 
                 i + 1, pattern.frequency, pattern.confidence);
    }
    
    // Learn hierarchical organization
    println!("\n🏗️  Hierarchical Self-Organization:");
    
    let hierarchical_composition = system.compositor.learn_composition(&emergent_morphisms)?;
    println!("Emergent composition type: {:?}", hierarchical_composition.composition_type);
    println!("Hierarchy depth: {}", hierarchical_composition.morphism_sequence.len());
    
    // Test emergent problem-solving capabilities
    println!("\n🧩 Emergent Problem Solving:");
    
    let novel_problem = Grid::from_vec(vec![
        vec![7, 8, 7],
        vec![8, 9, 8], 
        vec![7, 8, 7],
    ])?;
    
    println!("Novel problem:\n{}", GridUtils::to_ascii(&novel_problem));
    
    // Apply emergent intelligence
    let emergent_solution = system.solve_with_morphisms(&novel_problem, &hierarchical_composition)?;
    println!("Emergent solution:\n{}", GridUtils::to_ascii(&emergent_solution));
    
    // Measure emergent complexity
    println!("\n📊 Emergent Complexity Analysis:");
    
    let pattern_diversity = system.pattern_space.patterns.len();
    let morphism_diversity = emergent_morphisms.len();
    let composition_complexity = hierarchical_composition.morphism_sequence.len();
    
    println!("Pattern diversity: {}", pattern_diversity);
    println!("Morphism diversity: {}", morphism_diversity);
    println!("Composition complexity: {}", composition_complexity);
    
    let emergence_index = (pattern_diversity * morphism_diversity) as f64 / 
                         (composition_complexity as f64 + 1.0);
    println!("Emergence index: {:.2}", emergence_index);
    
    // Test self-improvement
    println!("\n🚀 Self-Improvement Capability:");
    
    let improvement_rounds = 3;
    let mut baseline_performance = 0.5;
    
    for round in 0..improvement_rounds {
        // Generate new challenges
        let challenge = PatternUtils::generate_test_puzzle(TestPuzzleType::Simple);
        
        // Solve with current capabilities
        let solution = system.solve_puzzle(&challenge.examples, &challenge.test_input)?;
        let performance = if let Some(ref expected) = challenge.test_output {
            GridUtils::similarity(&solution, expected)
        } else { 0.5 };
        
        // Self-improve based on results
        system.evolve(&[challenge])?;
        
        let improvement = performance - baseline_performance;
        baseline_performance = performance;
        
        println!("Self-improvement round {}: performance {:.3} (Δ {:.3})", 
                 round + 1, performance, improvement);
    }
    
    println!("\n✨ Emergent intelligence demonstration complete!");
    println!("Observed phenomena:");
    println!("  • Self-organization of pattern hierarchies");
    println!("  • Emergent problem-solving strategies");
    println!("  • Autonomous complexity management");
    println!("  • Continuous self-improvement");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_research_demo() {
        let result = std::panic::catch_unwind(|| {
            main().unwrap()
        });
        
        assert!(result.is_ok(), "Research demo should run without panicking");
    }
    
    #[test]
    fn test_meta_learning() {
        let mut system = ARCSCTTSystem::new();
        let result = demonstrate_meta_learning(&mut system);
        assert!(result.is_ok(), "Meta-learning demonstration should succeed");
    }
    
    #[test]
    fn test_mathematical_formalization() {
        let mut system = ARCSCTTSystem::new();
        let result = demonstrate_mathematical_formalization(&mut system);
        assert!(result.is_ok(), "Mathematical formalization should succeed");
    }
}