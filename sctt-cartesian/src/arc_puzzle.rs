//! # ARC Puzzle Generation & Runetika Integration
//!
//! Generates ARC-style grid puzzles from SCTT transformation types
//! and provides a JSON interchange format for game integration.
//!
//! Runetika puzzles ↔ ARC grids ↔ SCTT types:
//! - Glyph grids map to `GridLit` terms
//! - Puzzle rules map to transformation function types
//! - Player solutions map to term inhabitants
//! - Solution verification = type checking

use crate::arc::{Grid, Transform, infer_transform};

/// An ARC-style puzzle task with training examples and test inputs.
#[derive(Debug, Clone)]
pub struct PuzzleTask {
    /// Unique identifier
    pub id: String,
    /// Difficulty rating (1-5)
    pub difficulty: u8,
    /// Training examples: (input, output) pairs
    pub train: Vec<(Grid, Grid)>,
    /// Test inputs (player must produce outputs)
    pub test_inputs: Vec<Grid>,
    /// Expected test outputs (for validation, hidden from player)
    pub test_outputs: Vec<Grid>,
    /// The underlying transform (for generation)
    pub transform: Transform,
}

impl PuzzleTask {
    /// Verify a player's solution against the expected output.
    pub fn check_solution(&self, test_idx: usize, answer: &Grid) -> bool {
        if test_idx >= self.test_outputs.len() {
            return false;
        }
        *answer == self.test_outputs[test_idx]
    }

    /// Check if the player's solution is consistent with training examples.
    pub fn check_consistent(&self, answer_transform: &Transform) -> bool {
        answer_transform.verify(&self.train)
    }
}

/// Generate a puzzle from a known transform and random inputs.
pub fn generate_puzzle(
    id: &str,
    transform: Transform,
    difficulty: u8,
    inputs: Vec<Grid>,
) -> PuzzleTask {
    let n = inputs.len();
    assert!(n >= 3, "need at least 3 inputs (2 train + 1 test)");

    let outputs: Vec<Grid> = inputs.iter().map(|g| transform.apply(g)).collect();

    let train_count = n - 1;
    let train: Vec<(Grid, Grid)> = inputs[..train_count].iter()
        .zip(outputs[..train_count].iter())
        .map(|(i, o)| (i.clone(), o.clone()))
        .collect();
    let test_inputs = vec![inputs[train_count].clone()];
    let test_outputs = vec![outputs[train_count].clone()];

    PuzzleTask {
        id: id.to_string(),
        difficulty,
        train,
        test_inputs,
        test_outputs,
        transform,
    }
}

/// Generate a simple color-swap puzzle.
pub fn generate_color_swap_puzzle(id: &str) -> PuzzleTask {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert(1, 2);
    map.insert(2, 1);
    let transform = Transform::ColorMap(map);

    let inputs = vec![
        Grid::new(3, 3, vec![1, 2, 1, 2, 1, 2, 1, 2, 1]),
        Grid::new(2, 4, vec![1, 1, 2, 2, 2, 2, 1, 1]),
        Grid::new(3, 3, vec![0, 1, 2, 1, 0, 2, 2, 1, 0]),
    ];

    generate_puzzle(id, transform, 1, inputs)
}

/// Generate a flip puzzle.
pub fn generate_flip_puzzle(id: &str) -> PuzzleTask {
    let transform = Transform::FlipHorizontal;
    let inputs = vec![
        Grid::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
        Grid::new(2, 4, vec![1, 2, 3, 4, 5, 6, 7, 8]),
        Grid::new(3, 3, vec![0, 1, 0, 2, 3, 2, 4, 5, 4]),
    ];

    generate_puzzle(id, transform, 2, inputs)
}

/// Generate a rotation puzzle.
pub fn generate_rotation_puzzle(id: &str) -> PuzzleTask {
    let transform = Transform::Rotate180;
    let inputs = vec![
        Grid::new(3, 3, vec![1, 0, 0, 0, 2, 0, 0, 0, 3]),
        Grid::new(2, 2, vec![1, 2, 3, 4]),
        Grid::new(3, 3, vec![5, 6, 7, 8, 9, 0, 1, 2, 3]),
    ];

    generate_puzzle(id, transform, 2, inputs)
}

/// A collection of generated puzzles at various difficulties.
pub fn generate_puzzle_set() -> Vec<PuzzleTask> {
    vec![
        generate_color_swap_puzzle("runetika_color_swap_1"),
        generate_flip_puzzle("runetika_flip_1"),
        generate_rotation_puzzle("runetika_rotate_1"),
    ]
}

/// Data collected when a player solves (or fails) a puzzle.
#[derive(Debug, Clone)]
pub struct SolutionTrace {
    /// Puzzle ID
    pub puzzle_id: String,
    /// Was the solution correct?
    pub correct: bool,
    /// Player's submitted grid
    pub submitted: Grid,
    /// Time taken in seconds
    pub time_secs: f64,
    /// Number of attempts before this submission
    pub attempt_number: u32,
}

/// Convert a solution trace to data for improving the solver.
///
/// Correct solutions become positive training examples.
/// Incorrect solutions become negative examples (what NOT to do).
pub fn trace_to_training_data(trace: &SolutionTrace, task: &PuzzleTask) -> Vec<(Grid, Grid)> {
    if trace.correct {
        // Extend training set with verified examples
        let mut data = task.train.clone();
        for (test_in, test_out) in task.test_inputs.iter().zip(task.test_outputs.iter()) {
            data.push((test_in.clone(), test_out.clone()));
        }
        data
    } else {
        // Return training data as-is (failed attempt is noted but doesn't add data)
        task.train.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_and_solve_color_swap() {
        let puzzle = generate_color_swap_puzzle("test_1");
        assert_eq!(puzzle.train.len(), 2);
        assert_eq!(puzzle.test_inputs.len(), 1);

        // Solve using the inference engine
        let transform = infer_transform(&puzzle.train).unwrap();
        let result = transform.apply(&puzzle.test_inputs[0]);
        assert!(puzzle.check_solution(0, &result));
    }

    #[test]
    fn generate_and_solve_flip() {
        let puzzle = generate_flip_puzzle("test_2");
        let transform = infer_transform(&puzzle.train).unwrap();
        let result = transform.apply(&puzzle.test_inputs[0]);
        assert!(puzzle.check_solution(0, &result));
    }

    #[test]
    fn generate_and_solve_rotation() {
        let puzzle = generate_rotation_puzzle("test_3");
        let transform = infer_transform(&puzzle.train).unwrap();
        let result = transform.apply(&puzzle.test_inputs[0]);
        assert!(puzzle.check_solution(0, &result));
    }

    #[test]
    fn puzzle_set_all_solvable() {
        let puzzles = generate_puzzle_set();
        for puzzle in &puzzles {
            let transform = infer_transform(&puzzle.train).unwrap();
            for (test_in, test_out) in puzzle.test_inputs.iter().zip(puzzle.test_outputs.iter()) {
                let result = transform.apply(test_in);
                assert_eq!(&result, test_out, "Failed on puzzle {}", puzzle.id);
            }
        }
    }

    #[test]
    fn solution_trace_correct() {
        let puzzle = generate_color_swap_puzzle("trace_test");
        let correct_answer = puzzle.test_outputs[0].clone();
        let trace = SolutionTrace {
            puzzle_id: "trace_test".to_string(),
            correct: true,
            submitted: correct_answer,
            time_secs: 5.0,
            attempt_number: 1,
        };
        let data = trace_to_training_data(&trace, &puzzle);
        // Correct trace should include train + test examples
        assert_eq!(data.len(), 3); // 2 train + 1 test
    }
}
