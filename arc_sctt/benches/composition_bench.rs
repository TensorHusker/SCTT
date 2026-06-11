//! Benchmarks for composition engine performance
//! 
//! Specialized benchmarks focusing on the smooth composition capabilities
//! of the morphological intelligence system.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use arc_sctt::*;
use arc_sctt::utils::*;
use std::collections::HashMap;

fn benchmark_morphism_composition(c: &mut Criterion) {
    let mut group = c.benchmark_group("morphism_composition");
    
    let chain_lengths = vec![2, 5, 10, 20];
    
    for length in chain_lengths {
        group.throughput(Throughput::Elements(length as u64));
        
        group.bench_with_input(
            BenchmarkId::new("sequential_composition", length),
            &length,
            |b, &length| {
                b.iter_with_setup(
                    || {
                        let mut kernel = Kernel::new();
                        let sig = GridSignature {
                            width: 4, height: 4,
                            color_distribution: HashMap::new(),
                            shape_features: vec![],
                            complexity: 1.0,
                        };
                        
                        let base_morphism = Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90);
                        (kernel, base_morphism, length)
                    },
                    |(mut kernel, base_morphism, length)| {
                        let mut current = base_morphism.clone();
                        
                        for _ in 1..length {
                            let next = Morphism::new(
                                current.target.clone(),
                                current.target.clone(),
                                Transform::Rotate90,
                            );
                            current = black_box(kernel.compose_morphisms(&current, &next).unwrap());
                        }
                        
                        black_box(current);
                    }
                );
            },
        );
    }
    
    group.finish();
}

fn benchmark_smooth_interpolation(c: &mut Criterion) {
    let mut group = c.benchmark_group("smooth_interpolation");
    
    let interpolation_points = vec![5, 10, 20, 50];
    
    for points in interpolation_points {
        group.throughput(Throughput::Elements(points as u64));
        
        group.bench_with_input(
            BenchmarkId::new("pattern_interpolation", points),
            &points,
            |b, &points| {
                b.iter_with_setup(
                    || {
                        let mut space = PatternSpace::new();
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
                        
                        let morphism = Morphism::new(sig1, sig2, Transform::Rotate90);
                        space.add_patterns(&[morphism]);
                        (space, points)
                    },
                    |(space, points)| {
                        for i in 0..points {
                            let t = i as f64 / (points - 1) as f64;
                            black_box(space.interpolate_patterns(0, 1, t).unwrap());
                        }
                    }
                );
            },
        );
    }
    
    group.finish();
}

fn benchmark_composition_strategies(c: &mut Criterion) {
    let mut group = c.benchmark_group("composition_strategies");
    
    let strategies = vec![
        "sequential",
        "parallel", 
        "hierarchical",
        "smooth",
    ];
    
    for strategy in strategies {
        group.bench_with_input(
            BenchmarkId::new("learn_composition_strategy", strategy),
            &strategy,
            |b, &strategy| {
                b.iter_with_setup(
                    || {
                        let mut compositor = Compositor::new();
                        let sig = GridSignature {
                            width: 3, height: 3,
                            color_distribution: HashMap::new(),
                            shape_features: vec![],
                            complexity: 1.0,
                        };
                        
                        let morphisms = match strategy {
                            "sequential" => vec![
                                Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90),
                                Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90),
                                Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90),
                            ],
                            "parallel" => vec![
                                Morphism::new(sig.clone(), sig.clone(), Transform::FlipHorizontal),
                                Morphism::new(sig.clone(), sig.clone(), Transform::FlipVertical),
                                Morphism::new(sig.clone(), sig.clone(), Transform::Transpose),
                            ],
                            "hierarchical" => vec![
                                Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90),
                                Morphism::new(sig.clone(), sig.clone(), Transform::FlipHorizontal),
                                Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90),
                                Morphism::new(sig.clone(), sig.clone(), Transform::FlipVertical),
                            ],
                            "smooth" => vec![
                                Morphism::new(sig.clone(), sig.clone(), Transform::Scale { factor: 1.1 }),
                                Morphism::new(sig.clone(), sig.clone(), Transform::Translate { dx: 1, dy: 1 }),
                                Morphism::new(sig.clone(), sig.clone(), Transform::Scale { factor: 0.9 }),
                            ],
                            _ => vec![Morphism::new(sig.clone(), sig.clone(), Transform::Identity)],
                        };
                        
                        (compositor, morphisms)
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

fn benchmark_smooth_transformations(c: &mut Criterion) {
    let mut group = c.benchmark_group("smooth_transformations");
    
    let grid_sizes = vec![3, 5, 8, 12];
    let smoothness_orders = vec![1, 2, 3];
    
    for size in &grid_sizes {
        for order in &smoothness_orders {
            group.throughput(Throughput::Elements(*size as u64 * *size as u64));
            
            group.bench_with_input(
                BenchmarkId::new(format!("smooth_rotate_{}x{}_C{}", size, size, order), *size),
                &(*size, *order),
                |b, &(size, order)| {
                    let compositor = Compositor::new();
                    let grid = GridUtils::create_pattern(size, size, GridPattern::Random(123)).unwrap();
                    
                    b.iter(|| {
                        black_box(compositor.smooth_rotate_90(black_box(&grid), order).unwrap())
                    });
                },
            );
            
            group.bench_with_input(
                BenchmarkId::new(format!("smooth_scale_{}x{}_C{}", size, size, order), *size),
                &(*size, *order),
                |b, &(size, order)| {
                    let compositor = Compositor::new();
                    let grid = GridUtils::create_pattern(size, size, GridPattern::Border).unwrap();
                    
                    b.iter(|| {
                        black_box(compositor.smooth_scale(black_box(&grid), 1.5, order).unwrap())
                    });
                },
            );
        }
    }
    
    group.finish();
}

fn benchmark_bilinear_sampling(c: &mut Criterion) {
    let mut group = c.benchmark_group("bilinear_sampling");
    
    let sample_counts = vec![100, 500, 1000, 5000];
    
    for count in sample_counts {
        group.throughput(Throughput::Elements(count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("bilinear_sample", count),
            &count,
            |b, &count| {
                let compositor = Compositor::new();
                let grid = GridUtils::create_pattern(10, 10, GridPattern::Checkerboard).unwrap();
                
                b.iter(|| {
                    for i in 0..count {
                        let x = (i as f64 * 0.137) % 9.0; // Prime-like step for good coverage
                        let y = (i as f64 * 0.239) % 9.0;
                        black_box(compositor.bilinear_sample(black_box(&grid), x, y));
                    }
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_composition_caching(c: &mut Criterion) {
    let mut group = c.benchmark_group("composition_caching");
    
    let cache_sizes = vec![10, 50, 100, 500];
    
    for cache_size in cache_sizes {
        group.bench_with_input(
            BenchmarkId::new("cached_composition", cache_size),
            &cache_size,
            |b, &cache_size| {
                b.iter_with_setup(
                    || {
                        let mut kernel = Kernel::new();
                        let sig = GridSignature {
                            width: 2, height: 2,
                            color_distribution: HashMap::new(),
                            shape_features: vec![],
                            complexity: 0.5,
                        };
                        
                        // Pre-populate cache with compositions
                        let transforms = vec![
                            Transform::Rotate90,
                            Transform::Rotate180,
                            Transform::FlipHorizontal,
                            Transform::FlipVertical,
                        ];
                        
                        for i in 0..cache_size {
                            let t1 = &transforms[i % transforms.len()];
                            let t2 = &transforms[(i + 1) % transforms.len()];
                            
                            let m1 = Morphism::new(sig.clone(), sig.clone(), t1.clone());
                            let m2 = Morphism::new(sig.clone(), sig.clone(), t2.clone());
                            
                            let _ = kernel.compose_morphisms(&m1, &m2);
                        }
                        
                        // Create test morphisms
                        let m1 = Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90);
                        let m2 = Morphism::new(sig.clone(), sig.clone(), Transform::FlipHorizontal);
                        
                        (kernel, m1, m2)
                    },
                    |(mut kernel, m1, m2)| {
                        // This should hit the cache
                        black_box(kernel.compose_morphisms(black_box(&m1), black_box(&m2)).unwrap());
                    }
                );
            },
        );
    }
    
    group.finish();
}

fn benchmark_learning_adaptation(c: &mut Criterion) {
    let mut group = c.benchmark_group("learning_adaptation");
    
    let training_iterations = vec![1, 5, 10, 25];
    
    for iterations in training_iterations {
        group.throughput(Throughput::Elements(iterations as u64));
        
        group.bench_with_input(
            BenchmarkId::new("adaptive_learning", iterations),
            &iterations,
            |b, &iterations| {
                b.iter_with_setup(
                    || {
                        let mut compositor = Compositor::new();
                        let sig = GridSignature {
                            width: 3, height: 3,
                            color_distribution: HashMap::new(),
                            shape_features: vec![],
                            complexity: 1.0,
                        };
                        
                        let base_morphisms = vec![
                            Morphism::new(sig.clone(), sig.clone(), Transform::Rotate90),
                            Morphism::new(sig.clone(), sig.clone(), Transform::FlipHorizontal),
                            Morphism::new(sig.clone(), sig.clone(), Transform::Transpose),
                        ];
                        
                        (compositor, base_morphisms, iterations)
                    },
                    |(mut compositor, base_morphisms, iterations)| {
                        for _ in 0..iterations {
                            black_box(compositor.learn_from_patterns(black_box(&base_morphisms)).unwrap());
                        }
                    }
                );
            },
        );
    }
    
    group.finish();
}

fn benchmark_vector_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector_operations");
    
    let vector_dimensions = vec![16, 32, 64, 128];
    
    for dim in vector_dimensions {
        group.throughput(Throughput::Elements(dim as u64));
        
        group.bench_with_input(
            BenchmarkId::new("vector_distance", dim),
            &dim,
            |b, &dim| {
                let space = PatternSpace::new();
                let v1: Vec<f64> = (0..dim).map(|i| (i as f64).sin()).collect();
                let v2: Vec<f64> = (0..dim).map(|i| (i as f64).cos()).collect();
                
                b.iter(|| {
                    black_box(space.vector_distance(black_box(&v1), black_box(&v2)))
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("vector_curvature", dim),
            &dim,
            |b, &dim| {
                let compositor = Compositor::new();
                let v1: Vec<f64> = (0..dim).map(|i| (i as f64 * 0.1).sin()).collect();
                let v2: Vec<f64> = (0..dim).map(|i| (i as f64 * 0.1).cos()).collect();
                
                b.iter(|| {
                    black_box(compositor.vector_curvature(black_box(&v1), black_box(&v2)))
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    composition_benches,
    benchmark_morphism_composition,
    benchmark_smooth_interpolation,
    benchmark_composition_strategies,
    benchmark_smooth_transformations,
    benchmark_bilinear_sampling,
    benchmark_composition_caching,
    benchmark_learning_adaptation,
    benchmark_vector_operations
);

criterion_main!(composition_benches);