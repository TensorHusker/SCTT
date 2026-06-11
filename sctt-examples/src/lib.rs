//! # SCTT Examples and Tutorials
//!
//! This crate contains comprehensive examples demonstrating
//! the capabilities of Smooth Cubical Type Theory.

pub mod tutorial;
pub mod advanced;

use sctt_core::prelude::*;
use anyhow::Result;

/// Basic SCTT examples
pub fn run_basic_examples() -> Result<()> {
    println!("Running basic SCTT examples...");
    
    // Identity function
    let id_type = Type::pi("A", Type::universe(0), 
                          Type::pi("x", Type::var("A"), Type::var("A")));
    let id_term = Term::lambda("A", Term::lambda("x", Term::var("x")));
    
    println!("Identity function: {:?} : {:?}", id_term, id_type);
    
    // Boolean type (as example HIT)
    let bool_type = Type::var("Bool");
    println!("Boolean type: {:?}", bool_type);
    
    Ok(())
}

/// Smooth mathematics examples
pub fn run_smooth_examples() -> Result<()> {
    println!("Running smooth mathematics examples...");
    
    // Real numbers
    let real = Type::var("ℝ");
    
    // Tangent bundle
    let tangent_real = Type::TangentBundle {
        base: Box::new(real.clone()),
    };
    
    println!("Tangent bundle T(ℝ): {:?}", tangent_real);
    
    // Differential 1-forms
    let omega_1 = Type::DifferentialForm {
        base: Box::new(real),
        degree: 1,
    };
    
    println!("1-forms Ω¹(ℝ): {:?}", omega_1);
    
    Ok(())
}

/// Higher Inductive Types examples
pub fn run_hit_examples() -> Result<()> {
    println!("Running HIT examples...");
    
    use sctt_core::hit::HIT;
    
    // Circle
    let circle = HIT::circle();
    println!("Circle S¹: {:?}", circle);
    
    // Sphere
    let sphere = HIT::sphere();
    println!("Sphere S²: {:?}", sphere);
    
    Ok(())
}