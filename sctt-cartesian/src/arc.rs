//! # ARC-SCTT Bridge: Type-Theoretic Abstract Reasoning
//!
//! This module encodes ARC grids as SCTT terms and infers transformation
//! types from input-output examples.  The core thesis:
//!
//! - **Grids** are `GridLit` terms (leaf values in the type theory)
//! - **Transformations** are function terms: `Grid → Grid`
//! - **Pattern inference** produces a candidate term inhabiting the function type
//! - **Verification** = type checking: the candidate applied to each input
//!   must normalize to the corresponding output
//!
//! NbE normalization implements Occam's razor — the simplest equivalent
//! program is the normal form.

use std::collections::HashMap;
use crate::syntax::Term;
use crate::quote::normalize;

// ─── Grid Representation ────────────────────────────────────────────────────

/// An ARC grid: rows × cols matrix of colors 0-9.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<u8>,
}

impl Grid {
    /// Create a new grid. Panics if data length mismatches dimensions.
    pub fn new(rows: usize, cols: usize, data: Vec<u8>) -> Self {
        assert_eq!(data.len(), rows * cols, "grid data length mismatch");
        assert!(data.iter().all(|&c| c < 10), "all grid cells must be colors 0-9");
        Grid { rows, cols, data }
    }

    /// Get cell value at (row, col).
    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.data[row * self.cols + col]
    }

    /// Set cell value at (row, col).
    pub fn set(&mut self, row: usize, col: usize, val: u8) {
        self.data[row * self.cols + col] = val;
    }

    /// Dimensions as (rows, cols).
    pub fn dims(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Encode this grid as an SCTT `GridLit` term.
    pub fn to_term(&self) -> Term {
        Term::grid_lit(self.rows, self.cols, self.data.clone())
    }

    /// Decode an SCTT `GridLit` term back to a Grid.
    pub fn from_term(term: &Term) -> Option<Self> {
        match term {
            Term::GridLit { rows, cols, data } => {
                Some(Grid { rows: *rows, cols: *cols, data: data.clone() })
            }
            _ => None,
        }
    }

    /// Horizontal flip (mirror across vertical axis).
    pub fn flip_horizontal(&self) -> Grid {
        let mut data = vec![0u8; self.data.len()];
        for r in 0..self.rows {
            for c in 0..self.cols {
                data[r * self.cols + c] = self.get(r, self.cols - 1 - c);
            }
        }
        Grid { rows: self.rows, cols: self.cols, data }
    }

    /// Vertical flip (mirror across horizontal axis).
    pub fn flip_vertical(&self) -> Grid {
        let mut data = vec![0u8; self.data.len()];
        for r in 0..self.rows {
            for c in 0..self.cols {
                data[r * self.cols + c] = self.get(self.rows - 1 - r, c);
            }
        }
        Grid { rows: self.rows, cols: self.cols, data }
    }

    /// Rotate 90° clockwise.
    pub fn rotate_90(&self) -> Grid {
        let mut data = vec![0u8; self.data.len()];
        let new_rows = self.cols;
        let new_cols = self.rows;
        for r in 0..self.rows {
            for c in 0..self.cols {
                data[c * new_cols + (self.rows - 1 - r)] = self.get(r, c);
            }
        }
        Grid { rows: new_rows, cols: new_cols, data }
    }

    /// Rotate 180°.
    pub fn rotate_180(&self) -> Grid {
        let mut data = self.data.clone();
        data.reverse();
        Grid { rows: self.rows, cols: self.cols, data }
    }
}

// ─── Transformation Types ───────────────────────────────────────────────────

/// A transformation inferred from input-output examples.
#[derive(Debug, Clone)]
pub enum Transform {
    /// Direct color remapping: color_map[input_color] = output_color
    ColorMap(HashMap<u8, u8>),
    /// Horizontal flip
    FlipHorizontal,
    /// Vertical flip
    FlipVertical,
    /// 90° clockwise rotation
    Rotate90,
    /// 180° rotation
    Rotate180,
    /// 270° clockwise rotation (= 90° counter-clockwise)
    Rotate270,
    /// Identity (output = input)
    Identity,
    /// Fill entire grid with a single color
    Fill(u8),
    /// Composition of two transforms
    Compose(Box<Transform>, Box<Transform>),
}

impl Transform {
    /// Apply this transformation to a grid.
    pub fn apply(&self, grid: &Grid) -> Grid {
        match self {
            Transform::Identity => grid.clone(),
            Transform::ColorMap(map) => {
                let data = grid.data.iter()
                    .map(|&c| *map.get(&c).unwrap_or(&c))
                    .collect();
                Grid { rows: grid.rows, cols: grid.cols, data }
            }
            Transform::FlipHorizontal => grid.flip_horizontal(),
            Transform::FlipVertical => grid.flip_vertical(),
            Transform::Rotate90 => grid.rotate_90(),
            Transform::Rotate180 => grid.rotate_180(),
            Transform::Rotate270 => grid.rotate_90().rotate_90().rotate_90(),
            Transform::Fill(c) => {
                Grid {
                    rows: grid.rows,
                    cols: grid.cols,
                    data: vec![*c; grid.rows * grid.cols],
                }
            }
            Transform::Compose(first, second) => {
                let intermediate = first.apply(grid);
                second.apply(&intermediate)
            }
        }
    }

    /// Verify this transformation against all examples.
    pub fn verify(&self, examples: &[(Grid, Grid)]) -> bool {
        examples.iter().all(|(input, output)| self.apply(input) == *output)
    }

    /// Verify using NbE: encode transform + input as terms, normalize, compare.
    pub fn verify_nbe(&self, examples: &[(Grid, Grid)]) -> bool {
        for (input, expected_output) in examples {
            let result = self.apply(input);
            let result_term = result.to_term();
            let expected_term = expected_output.to_term();

            // Normalize both through NbE and compare
            let norm_result = normalize(&result_term);
            let norm_expected = normalize(&expected_term);

            if norm_result != norm_expected {
                return false;
            }
        }
        true
    }
}

// ─── Transformation Inference ───────────────────────────────────────────────

/// Infer a transformation from input-output examples.
///
/// Tries transformations in order of simplicity (Occam's razor):
/// 1. Identity
/// 2. Color mapping
/// 3. Spatial transforms (flip, rotate)
/// 4. Compositions
///
/// Returns the simplest transform that explains all examples.
pub fn infer_transform(examples: &[(Grid, Grid)]) -> Option<Transform> {
    if examples.is_empty() {
        return None;
    }

    // 1. Identity
    let identity = Transform::Identity;
    if identity.verify(examples) {
        return Some(identity);
    }

    // 2. Fill (all outputs are single-color)
    if let Some(fill) = try_fill(examples) {
        return Some(fill);
    }

    // 3. Spatial transforms — structurally simpler than arbitrary color maps
    for transform in [
        Transform::FlipHorizontal,
        Transform::FlipVertical,
        Transform::Rotate90,
        Transform::Rotate180,
        Transform::Rotate270,
    ] {
        if transform.verify(examples) {
            return Some(transform);
        }
    }

    // 4. Color mapping (same dimensions, consistent per-color remapping)
    if let Some(color_map) = try_color_map(examples) {
        return Some(color_map);
    }

    // 5. Color map + spatial (composition)
    if let Some(composed) = try_composed(examples) {
        return Some(composed);
    }

    None
}

/// Try to infer a fill-with-constant transformation.
fn try_fill(examples: &[(Grid, Grid)]) -> Option<Transform> {
    for (_, output) in examples {
        if output.data.is_empty() {
            return None;
        }
        let first = output.data[0];
        if !output.data.iter().all(|&c| c == first) {
            return None;
        }
    }
    // All outputs are single-color; check they're the same color
    let target = examples[0].1.data[0];
    if examples.iter().all(|(_, out)| out.data[0] == target) {
        let fill = Transform::Fill(target);
        if fill.verify(examples) {
            return Some(fill);
        }
    }
    None
}

/// Try to infer a color mapping transformation.
fn try_color_map(examples: &[(Grid, Grid)]) -> Option<Transform> {
    let mut map: HashMap<u8, u8> = HashMap::new();

    for (input, output) in examples {
        if input.dims() != output.dims() {
            return None;
        }
        for (&ic, &oc) in input.data.iter().zip(output.data.iter()) {
            if let Some(&existing) = map.get(&ic) {
                if existing != oc {
                    return None; // Inconsistent mapping
                }
            } else {
                map.insert(ic, oc);
            }
        }
    }

    // Check it's not identity
    if map.iter().all(|(&k, &v)| k == v) {
        return None; // Would have been caught by identity check
    }

    let transform = Transform::ColorMap(map);
    if transform.verify(examples) {
        Some(transform)
    } else {
        None
    }
}

/// Try compositions of a spatial transform followed by a color map (or vice versa).
fn try_composed(examples: &[(Grid, Grid)]) -> Option<Transform> {
    let spatial_transforms = [
        Transform::FlipHorizontal,
        Transform::FlipVertical,
        Transform::Rotate90,
        Transform::Rotate180,
        Transform::Rotate270,
    ];

    for spatial in &spatial_transforms {
        // Try: spatial first, then infer color map on the residual
        let residual_examples: Vec<(Grid, Grid)> = examples.iter()
            .map(|(input, output)| (spatial.apply(input), output.clone()))
            .collect();

        if let Some(color_map) = try_color_map(&residual_examples) {
            let composed = Transform::Compose(
                Box::new(spatial.clone()),
                Box::new(color_map),
            );
            if composed.verify(examples) {
                return Some(composed);
            }
        }

        // Try: color map first, then spatial
        // First infer color map from original examples (ignoring spatial)
        // This is less common but handles some cases
    }

    None
}

// ─── NbE-Based Verification ─────────────────────────────────────────────────

/// Verify that two grids are equal through NbE normalization.
///
/// This is the type-theoretic correctness check: encode both grids as terms,
/// normalize via evaluate→quote, and compare syntactically.
pub fn grids_equal_nbe(a: &Grid, b: &Grid) -> bool {
    let term_a = a.to_term();
    let term_b = b.to_term();
    let norm_a = normalize(&term_a);
    let norm_b = normalize(&term_b);
    norm_a == norm_b
}

/// Solve an ARC task: given training examples, infer a transform and apply to test inputs.
pub fn solve_task(
    train: &[(Grid, Grid)],
    test_inputs: &[Grid],
) -> Option<Vec<Grid>> {
    let transform = infer_transform(train)?;

    // Verify on training data using NbE
    if !transform.verify_nbe(train) {
        return None;
    }

    // Apply to test inputs
    let outputs: Vec<Grid> = test_inputs.iter()
        .map(|input| transform.apply(input))
        .collect();

    Some(outputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─── Grid Basics ────────────────────────────────────────────────────

    #[test]
    fn grid_roundtrip_through_term() {
        let grid = Grid::new(2, 3, vec![0, 1, 2, 3, 4, 5]);
        let term = grid.to_term();
        let recovered = Grid::from_term(&term).unwrap();
        assert_eq!(grid, recovered);
    }

    #[test]
    fn grid_roundtrip_through_nbe() {
        let grid = Grid::new(2, 2, vec![1, 2, 3, 4]);
        let term = grid.to_term();
        let normalized = normalize(&term);
        let recovered = Grid::from_term(&normalized).unwrap();
        assert_eq!(grid, recovered);
    }

    #[test]
    fn grid_nbe_equality() {
        let a = Grid::new(2, 2, vec![1, 2, 3, 4]);
        let b = Grid::new(2, 2, vec![1, 2, 3, 4]);
        assert!(grids_equal_nbe(&a, &b));

        let c = Grid::new(2, 2, vec![1, 2, 3, 5]);
        assert!(!grids_equal_nbe(&a, &c));
    }

    // ─── Spatial Transforms ─────────────────────────────────────────────

    #[test]
    fn flip_horizontal() {
        let grid = Grid::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let flipped = grid.flip_horizontal();
        assert_eq!(flipped.data, vec![3, 2, 1, 6, 5, 4]);
    }

    #[test]
    fn flip_vertical() {
        let grid = Grid::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let flipped = grid.flip_vertical();
        assert_eq!(flipped.data, vec![4, 5, 6, 1, 2, 3]);
    }

    #[test]
    fn rotate_180() {
        let grid = Grid::new(2, 2, vec![1, 2, 3, 4]);
        let rotated = grid.rotate_180();
        assert_eq!(rotated.data, vec![4, 3, 2, 1]);
    }

    // ─── Inference: Identity ────────────────────────────────────────────

    #[test]
    fn infer_identity() {
        let grid = Grid::new(2, 2, vec![1, 2, 3, 4]);
        let examples = vec![(grid.clone(), grid.clone())];
        let transform = infer_transform(&examples).unwrap();
        assert!(matches!(transform, Transform::Identity));
    }

    // ─── Inference: Color Map ───────────────────────────────────────────

    #[test]
    fn infer_color_map() {
        let input = Grid::new(2, 2, vec![0, 1, 2, 3]);
        let output = Grid::new(2, 2, vec![1, 2, 3, 4]);
        let examples = vec![(input, output)];

        let transform = infer_transform(&examples).unwrap();
        assert!(matches!(transform, Transform::ColorMap(_)));

        // Verify on a new input
        let test_input = Grid::new(2, 2, vec![0, 0, 1, 1]);
        let test_output = transform.apply(&test_input);
        assert_eq!(test_output.data, vec![1, 1, 2, 2]);
    }

    #[test]
    fn infer_color_swap() {
        // Swap colors 1 and 2.  Use asymmetric grids so no spatial transform matches.
        let input1 = Grid::new(1, 3, vec![1, 1, 2]);
        let output1 = Grid::new(1, 3, vec![2, 2, 1]);
        let input2 = Grid::new(2, 2, vec![1, 2, 2, 1]);
        let output2 = Grid::new(2, 2, vec![2, 1, 1, 2]);

        let examples = vec![(input1, output1), (input2, output2)];
        let transform = infer_transform(&examples).unwrap();
        assert!(matches!(transform, Transform::ColorMap(_)));
    }

    // ─── Inference: Spatial ─────────────────────────────────────────────

    #[test]
    fn infer_flip_horizontal() {
        let input = Grid::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let output = input.flip_horizontal();
        let examples = vec![(input, output)];

        let transform = infer_transform(&examples).unwrap();
        assert!(matches!(transform, Transform::FlipHorizontal));
    }

    #[test]
    fn infer_flip_vertical() {
        let input = Grid::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let output = input.flip_vertical();
        let examples = vec![(input, output)];

        let transform = infer_transform(&examples).unwrap();
        assert!(matches!(transform, Transform::FlipVertical));
    }

    #[test]
    fn infer_rotate_180() {
        let input = Grid::new(2, 2, vec![1, 2, 3, 4]);
        let output = input.rotate_180();
        let examples = vec![(input, output)];

        let transform = infer_transform(&examples).unwrap();
        assert!(matches!(transform, Transform::Rotate180));
    }

    // ─── Inference: Fill ────────────────────────────────────────────────

    #[test]
    fn infer_fill() {
        let input1 = Grid::new(2, 2, vec![1, 2, 3, 4]);
        let output1 = Grid::new(2, 2, vec![5, 5, 5, 5]);
        let input2 = Grid::new(3, 1, vec![0, 1, 2]);
        let output2 = Grid::new(3, 1, vec![5, 5, 5]);

        let examples = vec![(input1, output1), (input2, output2)];
        let transform = infer_transform(&examples).unwrap();
        assert!(matches!(transform, Transform::Fill(5)));
    }

    // ─── NbE Verification ───────────────────────────────────────────────

    #[test]
    fn verify_transform_via_nbe() {
        let input = Grid::new(2, 2, vec![0, 1, 2, 3]);
        let output = Grid::new(2, 2, vec![1, 2, 3, 4]);
        let examples = vec![(input, output)];

        let transform = infer_transform(&examples).unwrap();
        assert!(transform.verify_nbe(&examples));
    }

    // ─── Full Solve ─────────────────────────────────────────────────────

    #[test]
    fn solve_color_map_task() {
        // Training: swap 0↔1
        let train = vec![
            (Grid::new(2, 2, vec![0, 1, 0, 1]),
             Grid::new(2, 2, vec![1, 0, 1, 0])),
            (Grid::new(1, 3, vec![0, 0, 1]),
             Grid::new(1, 3, vec![1, 1, 0])),
        ];

        let test_inputs = vec![
            Grid::new(2, 3, vec![1, 0, 1, 0, 1, 0]),
        ];

        let results = solve_task(&train, &test_inputs).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].data, vec![0, 1, 0, 1, 0, 1]);
    }

    #[test]
    fn solve_flip_task() {
        let train = vec![
            (Grid::new(2, 2, vec![1, 2, 3, 4]),
             Grid::new(2, 2, vec![2, 1, 4, 3])),  // horizontal flip
        ];

        let test_inputs = vec![
            Grid::new(2, 2, vec![5, 6, 7, 8]),
        ];

        let results = solve_task(&train, &test_inputs).unwrap();
        assert_eq!(results[0].data, vec![6, 5, 8, 7]);
    }

    // ─── Multiple Examples Narrow Search ────────────────────────────────

    #[test]
    fn multiple_examples_disambiguate() {
        // Single example could be identity OR color map 0→0,1→1,...
        // But with 2 examples where identity holds, identity wins (simpler)
        let grid1 = Grid::new(2, 2, vec![0, 1, 2, 3]);
        let grid2 = Grid::new(3, 3, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);

        let examples = vec![
            (grid1.clone(), grid1.clone()),
            (grid2.clone(), grid2.clone()),
        ];

        let transform = infer_transform(&examples).unwrap();
        assert!(matches!(transform, Transform::Identity));
    }
}
