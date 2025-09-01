//! SCTT Playground - Interactive REPL for experimenting with SCTT concepts

use anyhow::Result;
use clap::Parser;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RustyResult};
use sctt_core::{Term, Type};
use sctt_smooth::examples::{Exp, Polynomial, Sin};
use sctt_smooth::{SmoothFunction, SmoothPath};
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(name = "sctt-playground")]
#[command(about = "Interactive SCTT playground for learning and experimentation")]
struct Args {
    /// Start with a specific example
    #[arg(short, long)]
    example: Option<String>,
    
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    print_banner();
    
    if let Some(example) = args.example {
        run_example(&example)?;
    } else {
        run_repl(args.verbose)?;
    }
    
    Ok(())
}

fn print_banner() {
    println!("{}", "╔════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║     SCTT Playground - Rust Edition                    ║".cyan());
    println!("{}", "║     Type 'help' for commands, 'quit' to exit          ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════╝".cyan());
    println!();
}

fn run_repl(verbose: bool) -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    
    println!("{}", "Starting interactive session...".green());
    println!("Try: {}", "smooth sin".yellow());
    println!();
    
    loop {
        let readline = rl.readline("sctt> ");
        
        match readline {
            Ok(line) => {
                let line = line.trim();
                
                if line.is_empty() {
                    continue;
                }
                
                rl.add_history_entry(line)?;
                
                match process_command(line, verbose) {
                    Ok(should_quit) => {
                        if should_quit {
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("{}: {}", "Error".red(), e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Use 'quit' to exit");
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
    
    println!("{}", "Goodbye!".green());
    Ok(())
}

fn process_command(input: &str, verbose: bool) -> Result<bool> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    if parts.is_empty() {
        return Ok(false);
    }
    
    match parts[0] {
        "quit" | "exit" => Ok(true),
        "help" => {
            print_help();
            Ok(false)
        }
        "smooth" => {
            if parts.len() < 2 {
                println!("Usage: smooth <function>");
                println!("Functions: sin, exp, poly");
            } else {
                demonstrate_smooth_function(parts[1], verbose)?;
            }
            Ok(false)
        }
        "path" => {
            demonstrate_smooth_path(verbose)?;
            Ok(false)
        }
        "compose" => {
            demonstrate_composition(verbose)?;
            Ok(false)
        }
        "type" => {
            if parts.len() < 2 {
                println!("Usage: type <expression>");
            } else {
                show_type(parts[1])?;
            }
            Ok(false)
        }
        "interval" => {
            demonstrate_interval()?;
            Ok(false)
        }
        "coherence" => {
            demonstrate_coherence_challenge()?;
            Ok(false)
        }
        _ => {
            println!("Unknown command: {}. Type 'help' for available commands.", parts[0]);
            Ok(false)
        }
    }
}

fn print_help() {
    println!("{}", "Available commands:".bold());
    println!("  {}  - Show this help message", "help".cyan());
    println!("  {}  - Exit the playground", "quit".cyan());
    println!("  {} <func> - Explore smooth functions", "smooth".cyan());
    println!("  {}  - Create and analyze smooth paths", "path".cyan());
    println!("  {} - Compose smooth functions", "compose".cyan());
    println!("  {} <expr> - Show type of expression", "type".cyan());
    println!("  {} - Explore interval operations", "interval".cyan());
    println!("  {} - See the coherence challenge", "coherence".cyan());
    println!();
    println!("{}", "Examples:".bold());
    println!("  smooth sin");
    println!("  path");
    println!("  compose");
}

fn demonstrate_smooth_function(func_name: &str, verbose: bool) -> Result<()> {
    println!("{}", format!("Exploring smooth function: {}", func_name).bold());
    
    match func_name {
        "sin" => {
            let sin = Sin;
            println!("Function: sin(x)");
            
            let x = std::f64::consts::PI / 4.0;
            println!("At x = π/4:");
            println!("  f(x) = {:.4}", sin.evaluate(x));
            
            if verbose {
                for order in 0..=4 {
                    println!("  f^({})(x) = {:.4}", order, sin.derivative(x, order));
                }
            } else {
                println!("  f'(x) = {:.4}", sin.derivative(x, 1));
                println!("  (Use --verbose to see higher derivatives)");
            }
            
            println!("✓ {}", "All derivatives exist - function is smooth!".green());
        }
        "exp" => {
            let exp = Exp;
            println!("Function: e^x");
            
            let x = 1.0;
            println!("At x = 1:");
            println!("  f(x) = {:.4}", exp.evaluate(x));
            
            if verbose {
                println!("Special property: all derivatives equal e^x");
                for order in 0..=3 {
                    println!("  f^({})(x) = {:.4}", order, exp.derivative(x, order));
                }
            } else {
                println!("  f'(x) = {:.4}", exp.derivative(x, 1));
            }
            
            println!("✓ {}", "Exponential is the eigenfunction of differentiation!".green());
        }
        "poly" => {
            let poly = Polynomial::new(vec![0.0, 0.0, 1.0]); // x²
            println!("Function: x²");
            
            let x = 2.0;
            println!("At x = 2:");
            println!("  f(x) = {:.4}", poly.evaluate(x));
            println!("  f'(x) = {:.4}", poly.derivative(x, 1));
            println!("  f''(x) = {:.4}", poly.derivative(x, 2));
            println!("  f'''(x) = {:.4}", poly.derivative(x, 3));
            
            println!("✓ {}", "Polynomial - finite derivatives then zero!".green());
        }
        _ => {
            println!("Unknown function: {}", func_name);
            println!("Available: sin, exp, poly");
        }
    }
    
    Ok(())
}

fn demonstrate_smooth_path(verbose: bool) -> Result<()> {
    println!("{}", "Creating smooth paths".bold());
    
    let path1 = SmoothPath::new(0.0, 1.0);
    let path2 = SmoothPath::with_derivatives(0.0, 1.0, 0.5, -0.5);
    
    println!("Path 1: Default smooth path from 0 to 1");
    println!("Path 2: Custom path with specified derivatives");
    
    for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
        println!("At t = {:.2}:", t);
        println!("  Path 1: position = {:.4}, velocity = {:.4}", 
                 path1.at(t), path1.velocity(t));
        if verbose {
            println!("  Path 2: position = {:.4}, velocity = {:.4}", 
                     path2.at(t), path2.velocity(t));
        }
    }
    
    println!();
    println!("Path 1 smooth? {}", 
             if path1.is_smooth() { "✓ Yes".green() } else { "✗ No".red() });
    println!("Path 2 smooth? {}", 
             if path2.is_smooth() { "✓ Yes".green() } else { "✗ No".red() });
    
    Ok(())
}

fn demonstrate_composition(verbose: bool) -> Result<()> {
    use sctt_smooth::SmoothComposition;
    
    println!("{}", "Composing smooth functions".bold());
    println!("Computing: exp(sin(x))");
    
    let sin = Sin;
    let exp = Exp;
    let composition = SmoothComposition::new(sin, exp);
    
    let x = std::f64::consts::PI / 6.0;
    println!("At x = π/6:");
    
    let sin_x = Sin.evaluate(x);
    let exp_sin_x = composition.evaluate(x);
    
    println!("  sin(x) = {:.4}", sin_x);
    println!("  exp(sin(x)) = {:.4}", exp_sin_x);
    
    if verbose {
        println!();
        println!("Chain rule verification:");
        let d_composition = composition.derivative(x, 1);
        let d_sin = Sin.derivative(x, 1);
        let d_exp = Exp.derivative(sin_x, 1);
        let expected = d_exp * d_sin;
        
        println!("  d/dx[exp(sin(x))] = {:.4}", d_composition);
        println!("  exp(sin(x)) * cos(x) = {:.4}", expected);
        println!("  Match? {}", 
                 if (d_composition - expected).abs() < 1e-10 { 
                     "✓ Yes".green() 
                 } else { 
                     "✗ No".red() 
                 });
    }
    
    println!();
    println!("✓ {}", "Composition preserves smoothness!".green());
    
    Ok(())
}

fn show_type(expr: &str) -> Result<()> {
    println!("{}", format!("Type of '{}':", expr).bold());
    
    match expr {
        "Real" | "ℝ" => {
            println!("  {} : Type₀", "ℝ".cyan());
            println!("  (The type of real numbers - inherently smooth)");
        }
        "sin" => {
            println!("  {} : C^∞(ℝ → ℝ)", "sin".cyan());
            println!("  (Smooth function from reals to reals)");
        }
        "I" => {
            println!("  {} : Type₀", "I".cyan());
            println!("  (The interval type [0,1])");
        }
        "Path" => {
            println!("  {} : (A : Type) → A → A → Type", "Path".cyan());
            println!("  (Path type constructor)");
        }
        _ => {
            println!("  Unknown expression");
            println!("  Try: Real, sin, I, Path");
        }
    }
    
    Ok(())
}

fn demonstrate_interval() -> Result<()> {
    println!("{}", "Interval operations".bold());
    println!("The interval I = [0, 1] with De Morgan algebra");
    println!();
    
    let examples = [
        ("0 ∧ 1", "0", "minimum"),
        ("0 ∨ 1", "1", "maximum"),
        ("¬0", "1", "negation"),
        ("¬1", "0", "negation"),
        ("0.3 ∧ 0.7", "0.3", "minimum"),
        ("0.3 ∨ 0.7", "0.7", "maximum"),
    ];
    
    for (expr, result, desc) in &examples {
        println!("  {} = {} ({})", expr.cyan(), result.yellow(), desc);
    }
    
    println!();
    println!("These operations satisfy De Morgan laws:");
    println!("  ¬(i ∧ j) = ¬i ∨ ¬j");
    println!("  ¬(i ∨ j) = ¬i ∧ ¬j");
    
    Ok(())
}

fn demonstrate_coherence_challenge() -> Result<()> {
    println!("{}", "The Coherence Challenge".bold().red());
    println!();
    println!("The key challenge in SCTT: making smooth and cubical work together");
    println!();
    
    println!("{}", "Example Problem:".yellow());
    println!("  Given: Two smooth paths that connect");
    println!("  path1: smooth from 0 to 1");
    println!("  path2: smooth from 1 to 2");
    println!();
    println!("  Composition: path1 ; path2 from 0 to 2");
    println!();
    println!("{}", "Question:".cyan());
    println!("  Is the composition smooth?");
    println!();
    println!("{}", "The Issue:".red());
    println!("  At the connection point (t = 0.5), the velocity might jump!");
    println!("  path1 ends with some velocity v1");
    println!("  path2 starts with some velocity v2");
    println!("  If v1 ≠ v2, we have a discontinuous derivative!");
    println!();
    println!("{}", "The Solution (Your Challenge):".green());
    println!("  Find coherence conditions that guarantee smooth composition");
    println!("  This is what SCTT must solve!");
    println!();
    println!("Hint: What if we required matching derivatives at boundaries?");
    
    Ok(())
}

fn run_example(name: &str) -> Result<()> {
    println!("Running example: {}", name);
    
    match name {
        "smooth" => {
            demonstrate_smooth_function("sin", true)?;
            println!();
            demonstrate_smooth_function("exp", true)?;
        }
        "path" => demonstrate_smooth_path(true)?,
        "compose" => demonstrate_composition(true)?,
        "coherence" => demonstrate_coherence_challenge()?,
        _ => {
            println!("Unknown example: {}", name);
            println!("Available examples: smooth, path, compose, coherence");
        }
    }
    
    Ok(())
}