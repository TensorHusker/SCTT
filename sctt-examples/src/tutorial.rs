//! # SCTT Tutorial
//!
//! Step-by-step introduction to Smooth Cubical Type Theory.

use sctt_core::prelude::*;
use anyhow::Result;

/// Tutorial: Basic types and terms
pub fn basic_types_tutorial() -> Result<()> {
    println!("=== SCTT Tutorial: Basic Types ===\n");
    
    // Universe hierarchy
    println!("1. Universe Hierarchy:");
    let type0 = Type::universe(0);
    let type1 = Type::universe(1);
    println!("   Type₀ : Type₁");
    println!("   Type₁ : Type₂");
    println!("   ...\n");
    
    // Function types
    println!("2. Function Types (Π-types):");
    let nat = Type::var("ℕ");
    let nat_to_nat = Type::pi("x", nat.clone(), nat.clone());
    println!("   ℕ → ℕ  ≡  (x : ℕ) → ℕ");
    println!("   Type: {:?}\n", nat_to_nat);
    
    // Dependent pairs
    println!("3. Dependent Pairs (Σ-types):");
    let sigma_type = Type::sigma("n", nat.clone(), Type::var("Vec_n"));
    println!("   (n : ℕ) × Vec(n)");
    println!("   Type: {:?}\n", sigma_type);
    
    Ok(())
}

/// Tutorial: Paths and equality
pub fn paths_tutorial() -> Result<()> {
    println!("=== SCTT Tutorial: Paths and Equality ===\n");
    
    println!("1. Path Types:");
    let nat = Type::var("ℕ");
    let zero = Term::var("0");
    let one = Term::var("1");
    
    let path_type = Type::path(nat, zero.clone(), one.clone());
    println!("   Path ℕ 0 1  -- paths from 0 to 1 in ℕ");
    println!("   Type: {:?}\n", path_type);
    
    println!("2. Reflexivity:");
    let refl = Term::path_lambda("i", zero.clone());
    println!("   refl : 0 = 0");
    println!("   refl = λi. 0\n");
    
    println!("3. Path Application:");
    let path_app = Term::path_app(refl, Interval::var("j"));
    println!("   path @ j  -- apply path to interval\n");
    
    Ok(())
}

/// Tutorial: Smooth structure
pub fn smooth_tutorial() -> Result<()> {
    println!("=== SCTT Tutorial: Smooth Structure ===\n");
    
    println!("1. Manifolds as Types:");
    let manifold = Type::var("M");
    println!("   M : Type  -- M is a smooth manifold\n");
    
    println!("2. Tangent Bundles:");
    let tangent_m = Type::TangentBundle {
        base: Box::new(manifold.clone()),
    };
    println!("   T(M) : Type  -- tangent bundle of M");
    println!("   Type: {:?}\n", tangent_m);
    
    println!("3. Differential Forms:");
    let forms = Type::DifferentialForm {
        base: Box::new(manifold),
        degree: 2,
    };
    println!("   Ω²(M) : Type  -- 2-forms on M");
    println!("   Type: {:?}\n", forms);
    
    println!("4. Smooth Maps:");
    let real = Type::var("ℝ");
    let smooth_map = Type::pi("f", 
        Type::pi("x", real.clone(), real.clone()),
        Type::pi("x", real.clone(), Type::TangentBundle {
            base: Box::new(real),
        })
    );
    println!("   Smooth : (ℝ → ℝ) → (x : ℝ) → T(ℝ)");
    println!("   -- predicate for smooth functions\n");
    
    Ok(())
}