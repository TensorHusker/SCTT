//! # SCTT ARC-AGI Solver
//!
//! Standalone CLI that reads ARC challenge JSON, infers transforms via
//! type-theoretic reasoning, and writes a submission file.
//!
//! Usage:
//!   sctt-arc-solve --input challenges.json --output submission.json
//!   sctt-arc-solve --input challenges.json --solutions solutions.json  # evaluate

use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

use serde_json::{Map, Value};
use sctt_cartesian::arc::{Grid, solve_task};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut input_path = None;
    let mut output_path = None;
    let mut solutions_path = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--input" | "-i" => {
                i += 1;
                input_path = Some(args[i].clone());
            }
            "--output" | "-o" => {
                i += 1;
                output_path = Some(args[i].clone());
            }
            "--solutions" | "-s" => {
                i += 1;
                solutions_path = Some(args[i].clone());
            }
            "--help" | "-h" => {
                eprintln!("sctt-arc-solve: SCTT-based ARC-AGI solver");
                eprintln!("Usage: sctt-arc-solve --input challenges.json --output submission.json");
                eprintln!("       sctt-arc-solve --input challenges.json --solutions solutions.json");
                process::exit(0);
            }
            other => {
                eprintln!("Unknown argument: {}", other);
                process::exit(1);
            }
        }
        i += 1;
    }

    let input_path = input_path.unwrap_or_else(|| {
        eprintln!("Error: --input is required");
        process::exit(1);
    });

    // Load challenges
    let challenges_raw = fs::read_to_string(&input_path).unwrap_or_else(|e| {
        eprintln!("Error reading {}: {}", input_path, e);
        process::exit(1);
    });
    let challenges: HashMap<String, Value> = serde_json::from_str(&challenges_raw).unwrap_or_else(|e| {
        eprintln!("Error parsing JSON: {}", e);
        process::exit(1);
    });

    eprintln!("Loaded {} tasks from {}", challenges.len(), input_path);

    // Solve each task
    let mut submission: Map<String, Value> = Map::new();
    let mut solved = 0;
    let mut attempted = 0;

    for (task_id, task_val) in &challenges {
        attempted += 1;

        let train = parse_examples(task_val.get("train"));
        let test_inputs = parse_test_inputs(task_val.get("test"));

        if train.is_empty() || test_inputs.is_empty() {
            // Emit empty attempts
            let test_results = make_empty_attempts(test_inputs.len());
            submission.insert(task_id.clone(), Value::Array(test_results));
            continue;
        }

        match solve_task(&train, &test_inputs) {
            Some(outputs) => {
                solved += 1;
                let test_results: Vec<Value> = outputs.iter().map(|grid| {
                    let grid_2d = grid_to_2d(grid);
                    Value::Object({
                        let mut m = Map::new();
                        m.insert("attempt_1".to_string(), grid_2d.clone());
                        m.insert("attempt_2".to_string(), grid_2d);
                        m
                    })
                }).collect();
                submission.insert(task_id.clone(), Value::Array(test_results));
            }
            None => {
                let test_results = make_empty_attempts(test_inputs.len());
                submission.insert(task_id.clone(), Value::Array(test_results));
            }
        }

        if attempted % 100 == 0 {
            eprintln!("  Progress: {}/{} tasks, {} solved", attempted, challenges.len(), solved);
        }
    }

    eprintln!("Solved {}/{} tasks ({:.1}%)", solved, attempted,
        100.0 * solved as f64 / attempted.max(1) as f64);

    // Write output or evaluate
    if let Some(ref out_path) = output_path {
        let json = serde_json::to_string_pretty(&submission).unwrap();
        fs::write(out_path, json).unwrap_or_else(|e| {
            eprintln!("Error writing {}: {}", out_path, e);
            process::exit(1);
        });
        eprintln!("Submission written to {}", out_path);
    }

    if let Some(ref sol_path) = solutions_path {
        evaluate(&submission, sol_path, &challenges);
    }

    if output_path.is_none() && solutions_path.is_none() {
        // Print to stdout
        let json = serde_json::to_string_pretty(&submission).unwrap();
        println!("{}", json);
    }
}

// ─── ARC JSON Parsing ───────────────────────────────────────────────────────

fn parse_grid(val: &Value) -> Option<Grid> {
    let rows_val = val.as_array()?;
    if rows_val.is_empty() {
        return None;
    }
    let rows = rows_val.len();
    let cols = rows_val[0].as_array()?.len();
    let mut data = Vec::with_capacity(rows * cols);
    for row in rows_val {
        let row_arr = row.as_array()?;
        if row_arr.len() != cols {
            return None;
        }
        for cell in row_arr {
            data.push(cell.as_u64()? as u8);
        }
    }
    Some(Grid::new(rows, cols, data))
}

fn parse_examples(train: Option<&Value>) -> Vec<(Grid, Grid)> {
    let train = match train {
        Some(Value::Array(arr)) => arr,
        _ => return vec![],
    };
    train.iter().filter_map(|ex| {
        let input = parse_grid(ex.get("input")?)?;
        let output = parse_grid(ex.get("output")?)?;
        Some((input, output))
    }).collect()
}

fn parse_test_inputs(test: Option<&Value>) -> Vec<Grid> {
    let test = match test {
        Some(Value::Array(arr)) => arr,
        _ => return vec![],
    };
    test.iter().filter_map(|ex| {
        parse_grid(ex.get("input")?)
    }).collect()
}

fn grid_to_2d(grid: &Grid) -> Value {
    let rows: Vec<Value> = (0..grid.rows).map(|r| {
        let cols: Vec<Value> = (0..grid.cols).map(|c| {
            Value::Number(serde_json::Number::from(grid.get(r, c) as u64))
        }).collect();
        Value::Array(cols)
    }).collect();
    Value::Array(rows)
}

fn make_empty_attempts(n: usize) -> Vec<Value> {
    (0..n).map(|_| {
        let empty_grid = Value::Array(vec![Value::Array(vec![Value::Number(0.into())])]);
        Value::Object({
            let mut m = Map::new();
            m.insert("attempt_1".to_string(), empty_grid.clone());
            m.insert("attempt_2".to_string(), empty_grid);
            m
        })
    }).collect()
}

// ─── Evaluation ─────────────────────────────────────────────────────────────

fn evaluate(submission: &Map<String, Value>, solutions_path: &str, _challenges: &HashMap<String, Value>) {
    let sol_raw = fs::read_to_string(solutions_path).unwrap_or_else(|e| {
        eprintln!("Error reading solutions: {}", e);
        process::exit(1);
    });
    let solutions: HashMap<String, Value> = serde_json::from_str(&sol_raw).unwrap_or_else(|e| {
        eprintln!("Error parsing solutions: {}", e);
        process::exit(1);
    });

    let mut correct = 0;
    let mut total = 0;

    for (task_id, sol_val) in &solutions {
        total += 1;

        let expected_grids: Vec<Value> = match sol_val.as_array() {
            Some(arr) => arr.clone(),
            None => continue,
        };

        let submitted = match submission.get(task_id) {
            Some(Value::Array(arr)) => arr.clone(),
            _ => continue,
        };

        if submitted.len() != expected_grids.len() {
            continue;
        }

        let mut task_correct = true;
        for (sub, exp) in submitted.iter().zip(expected_grids.iter()) {
            let attempt1 = sub.get("attempt_1");
            let attempt2 = sub.get("attempt_2");
            let matches = attempt1 == Some(exp) || attempt2 == Some(exp);
            if !matches {
                task_correct = false;
                break;
            }
        }

        if task_correct {
            correct += 1;
        }
    }

    eprintln!("\nEvaluation: {}/{} correct ({:.1}%)",
        correct, total, 100.0 * correct as f64 / total.max(1) as f64);
}
