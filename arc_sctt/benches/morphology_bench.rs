//! Benchmarks for morphological intelligence performance
//! 
//! These benchmarks measure the performance of the ARC-SCTT system
//! across different scales and complexity levels.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use arc_sctt::*;
use arc_sctt::utils::*;

fn benchmark_morphism_extraction(c: &mut Criterion) {
    let mut group = c.benchmark_group("morphism_extraction");
    
    let sizes = vec![2, 3, 5, 8, 10];
    
    for size in sizes {
        group.throughput(Throughput::Elements(size as u64 * size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("extract_single_morphism", size),
            &size,
            |b, &size| {
                let mut extractor = PatternExtractor::new();
                let input = GridUtils::create_pattern(size, size, GridPattern::Checkerboard).unwrap();
                let output = Kernel::new().apply_transform(&input, &Transform::Rotate90).unwrap();
                
                b.iter(|| {
                    black_box(extractor.extract_single_morphism(
                        black_box(&input),
                        black_box(&output)
                    ).unwrap())
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_pattern_space_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("pattern_space");
    
    let pattern_counts = vec![10, 50, 100, 200];
    
    for count in pattern_counts {
        group.throughput(Throughput::Elements(count as u64));
        
        // Benchmark pattern addition
        group.bench_with_input(
            BenchmarkId::new("add_patterns", count),
            &count,
            |b, &count| {
                b.iter_with_setup(
                    || {
                        let signatures: Vec<GridSignature> = (0..count).map(|i| {
                            GridSignature {
                                width: 2 + i % 5,
                                height: 2 + i % 5,
                                color_distribution: [(i as u8 % 10, 4)].iter().cloned().collect(),
                                shape_features: vec![i % 10],
                                complexity: (i as f64 + 1.0).ln(),
                            }
                        }).collect();
                        
                        let morphisms: Vec<Morphism> = signatures.windows(2).map(|pair| {
                            Morphism::new(pair[0].clone(), pair[1].clone(), Transform::Rotate90)
                        }).collect();
                        
                        (PatternSpace::new(), morphisms)
                    },
                    |(mut space, morphisms)| {
                        black_box(space.add_patterns(black_box(&morphisms)));
                    }
                );
            },
        );
        
        // Benchmark nearest neighbor search
        group.bench_with_input(
            BenchmarkId::new("nearest_neighbors", count),
            &count,
            |b, &count| {
                b.iter_with_setup(
                    || {
                        let mut space = PatternSpace::new();
                        let signatures: Vec<GridSignature> = (0..count).map(|i| {
                            GridSignature {
                                width: 2 + i % 5,
                                height: 2 + i % 5,
                                color_distribution: [(i as u8 % 10, 4)].iter().cloned().collect(),
                                shape_features: vec![i % 10],
                                complexity: (i as f64 + 1.0).ln(),
                            }
                        }).collect();
                        
                        let morphisms: Vec<Morphism> = signatures.windows(2).map(|pair| {
                            Morphism::new(pair[0].clone(), pair[1].clone(), Transform::Identity)
                        }).collect();
                        
                        space.add_patterns(&morphisms);
                        (space, signatures[0].clone())
                    },
                    |(space, query_signature)| {
                        black_box(space.find_nearest_patterns(black_box(&query_signature), 5));
                    }
                );
            },
        );
    }
    
    group.finish();
}

fn benchmark_composition_learning(c: &mut Criterion) {
    let mut group = c.benchmark_group("composition_learning");
    
    let morphism_counts = vec![2, 5, 10, 20];
    
    for count in morphism_counts {
        group.throughput(Throughput::Elements(count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("learn_composition", count),
            &count,
            |b, &count| {
                b.iter_with_setup(
                    || {
                        let sig = GridSignature {
                            width: 3, height: 3,
                            color_distribution: std::collections::HashMap::new(),
                            shape_features: vec![],
                            complexity: 1.0,
                        };
                        
                        let transforms = vec![
                            Transform::Rotate90,
                            Transform::Rotate180,
                            Transform::FlipHorizontal,
                            Transform::FlipVertical,
                            Transform::Transpose,
                        ];
                        
                        let morphisms: Vec<Morphism> = (0..count).map(|i| {
                            let transform = transforms[i % transforms.len()].clone();
                            Morphism::new(sig.clone(), sig.clone(), transform)
                        }).collect();
                        
                        (Compositor::new(), morphisms)
                    },
                    |(mut compositor, morphisms)| {
                        black_box(compositor.learn_composition(black_box(&morphisms)).unwrap());
                    }
                );
            },
        );
    }
    
    group.finish();
}

fn benchmark_arc_solving(c: &mut Criterion) {
    let mut group = c.benchmark_group("arc_solving");
    
    let complexity_levels = vec!["simple", "medium", "complex"];
    
    for complexity in complexity_levels {
        group.bench_with_input(
            BenchmarkId::new("solve_puzzle", complexity),
            &complexity,
            |b, &complexity| {
                b.iter_with_setup(
                    || {
                        let puzzle = match complexity {
                            "simple" => PatternUtils::generate_test_puzzle(TestPuzzleType::Simple),
                            "medium" => PatternUtils::generate_test_puzzle(TestPuzzleType::Rotation),
                            "complex" => PatternUtils::generate_test_puzzle(TestPuzzleType::ColorMapping),
                            _ => PatternUtils::generate_test_puzzle(TestPuzzleType::Simple),
                        };
                        (ARCSCTTSystem::new(), puzzle)
                    },
                    |(mut system, puzzle)| {
                        black_box(system.solve_puzzle(
                            black_box(&puzzle.examples),
                            black_box(&puzzle.test_input)
                        ).unwrap());
                    }
                );
            },
        );
    }
    
    group.finish();
}

fn benchmark_kernel_transformations(c: &mut Criterion) {
    let mut group = c.benchmark_group("kernel_transformations");
    
    let transforms = vec![
        ("identity", Transform::Identity),
        ("rotate90", Transform::Rotate90),
        ("rotate180", Transform::Rotate180),
        ("flip_horizontal", Transform::FlipHorizontal),
        ("flip_vertical", Transform::FlipVertical),
        ("transpose", Transform::Transpose),
    ];
    
    let grid_sizes = vec![3, 5, 10, 20];
    
    for (name, transform) in transforms {
        for size in &grid_sizes {
            group.throughput(Throughput::Elements(*size as u64 * *size as u64));
            
            group.bench_with_input(
                BenchmarkId::new(format!("{}_{}", name, size), name),
                &(*size, transform.clone()),
                |b, (size, transform)| {
                    let kernel = Kernel::new();
                    let grid = GridUtils::create_pattern(*size, *size, GridPattern::Random(42)).unwrap();
                    
                    b.iter(|| {
                        black_box(kernel.apply_transform(
                            black_box(&grid),
                            black_box(transform)
                        ).unwrap())
                    });
                },
            );
        }
    }
    
    group.finish();
}

fn benchmark_smooth_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("smooth_operations");
    
    let grid_sizes = vec![3, 5, 8, 12];
    
    for size in grid_sizes {
        group.throughput(Throughput::Elements(size as u64 * size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("smooth_rotate_90", size),
            &size,
            |b, &size| {
                let compositor = Compositor::new();
                let grid = GridUtils::create_pattern(size, size, GridPattern::Diagonal).unwrap();
                
                b.iter(|| {
                    black_box(compositor.smooth_rotate_90(black_box(&grid), 2).unwrap())
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("smooth_scale", size),
            &size,
            |b, &size| {
                let compositor = Compositor::new();
                let grid = GridUtils::create_pattern(size, size, GridPattern::Center).unwrap();
                
                b.iter(|| {
                    black_box(compositor.smooth_scale(black_box(&grid), 1.5, 1).unwrap())
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_distance_computations(c: &mut Criterion) {
    let mut group = c.benchmark_group("distance_computations");
    
    let metrics = vec![
        ("euclidean", PatternMetric::Euclidean),
        ("riemannian", PatternMetric::Riemannian),
        ("wasserstein", PatternMetric::Wasserstein),
        ("information_geometric", PatternMetric::InformationGeometric),
    ];
    
    for (name, metric) in metrics {
        group.bench_with_input(
            BenchmarkId::new("pattern_distance", name),
            &metric,
            |b, metric| {
                b.iter_with_setup(
                    || {
                        let mut space = PatternSpace::new();
                        let signatures: Vec<GridSignature> = (0..50).map(|i| {
                            GridSignature {
                                width: 3 + i % 3,
                                height: 3 + i % 3,
                                color_distribution: [(i as u8 % 5, 9)].iter().cloned().collect(),
                                shape_features: vec![i % 5],
                                complexity: i as f64 * 0.1,
                            }
                        }).collect();
                        
                        let morphisms: Vec<Morphism> = signatures.windows(2).map(|pair| {
                            Morphism::new(pair[0].clone(), pair[1].clone(), Transform::Identity)
                        }).collect();
                        
                        space.add_patterns(&morphisms);
                        space
                    },
                    |mut space| {
                        black_box(space.pattern_distance(0, 10, black_box(*metric)));
                    }
                );
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_morphism_extraction,
    benchmark_pattern_space_operations,
    benchmark_composition_learning,
    benchmark_arc_solving,
    benchmark_kernel_transformations,
    benchmark_smooth_operations,
    benchmark_distance_computations
);

criterion_main!(benches);