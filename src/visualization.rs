//! Advanced Visualization for Proofs and Type Theory
//!
//! Interactive visualizations including:
//! - Proof trees
//! - Type dependency graphs
//! - Homotopy diagrams
//! - Performance profiling

use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::proof_assistant::{Goal, ProofStep};
use crate::sctt_typechecker::Term;

/// Main visualization engine
pub struct Visualizer {
    canvas: HtmlCanvasElement,
    proof_tree: ProofTree,
    type_graph: TypeDependencyGraph,
    homotopy_viewer: HomotopyViewer,
}

/// Proof tree visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofTree {
    pub root: ProofNode,
    pub layout: TreeLayout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofNode {
    pub id: usize,
    pub goal: String,
    pub tactic: Option<String>,
    pub children: Vec<ProofNode>,
    pub status: NodeStatus,
    pub metadata: NodeMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeStatus {
    Open,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetadata {
    pub time_spent: f64,
    pub complexity: usize,
    pub dependencies: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct TreeLayout {
    pub node_positions: HashMap<usize, (f32, f32)>,
    pub edges: Vec<((f32, f32), (f32, f32))>,
    pub bounds: (f32, f32, f32, f32), // (min_x, min_y, max_x, max_y)
}

impl ProofTree {
    pub fn from_proof_state(goals: &[Goal], history: &[ProofStep]) -> Self {
        let root = Self::build_tree(goals, history);
        let layout = Self::calculate_layout(&root);
        
        ProofTree { root, layout }
    }
    
    fn build_tree(goals: &[Goal], history: &[ProofStep]) -> ProofNode {
        // Build tree from proof history
        ProofNode {
            id: 0,
            goal: if goals.is_empty() { 
                "Proof Complete".to_string() 
            } else { 
                goals[0].conclusion.clone() 
            },
            tactic: None,
            children: Vec::new(),
            status: if goals.is_empty() { 
                NodeStatus::Completed 
            } else { 
                NodeStatus::Open 
            },
            metadata: NodeMetadata {
                time_spent: 0.0,
                complexity: 1,
                dependencies: Vec::new(),
            },
        }
    }
    
    fn calculate_layout(root: &ProofNode) -> TreeLayout {
        let mut layout = TreeLayout {
            node_positions: HashMap::new(),
            edges: Vec::new(),
            bounds: (0.0, 0.0, 0.0, 0.0),
        };
        
        Self::layout_recursive(root, 0.0, 0.0, 100.0, &mut layout);
        layout
    }
    
    fn layout_recursive(
        node: &ProofNode,
        x: f32,
        y: f32,
        spacing: f32,
        layout: &mut TreeLayout,
    ) {
        layout.node_positions.insert(node.id, (x, y));
        
        let child_count = node.children.len() as f32;
        if child_count > 0.0 {
            let total_width = (child_count - 1.0) * spacing;
            let start_x = x - total_width / 2.0;
            
            for (i, child) in node.children.iter().enumerate() {
                let child_x = start_x + i as f32 * spacing;
                let child_y = y + 100.0;
                
                layout.edges.push(((x, y), (child_x, child_y)));
                Self::layout_recursive(child, child_x, child_y, spacing * 0.8, layout);
            }
        }
        
        // Update bounds
        layout.bounds.0 = layout.bounds.0.min(x - 50.0);
        layout.bounds.1 = layout.bounds.1.min(y - 50.0);
        layout.bounds.2 = layout.bounds.2.max(x + 50.0);
        layout.bounds.3 = layout.bounds.3.max(y + 50.0);
    }
}

/// Type dependency graph
pub struct TypeDependencyGraph {
    nodes: Vec<TypeNode>,
    edges: Vec<TypeEdge>,
}

#[derive(Debug, Clone)]
pub struct TypeNode {
    pub id: usize,
    pub name: String,
    pub kind: TypeKind,
    pub term: Option<Term>,
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Universe,
    Pi,
    Path,
    Inductive,
    Definition,
}

#[derive(Debug, Clone)]
pub struct TypeEdge {
    pub from: usize,
    pub to: usize,
    pub kind: EdgeKind,
}

#[derive(Debug, Clone)]
pub enum EdgeKind {
    Dependency,
    Application,
    Instantiation,
}

/// Homotopy viewer for path types
pub struct HomotopyViewer {
    paths: Vec<HomotopyPath>,
    dimension: usize,
}

#[derive(Debug, Clone)]
pub struct HomotopyPath {
    pub start: Point,
    pub end: Point,
    pub interpolation: Box<dyn Fn(f64) -> Point>,
    pub dimension: usize,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub coords: Vec<f64>,
}

impl Visualizer {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        Visualizer {
            canvas,
            proof_tree: ProofTree {
                root: ProofNode {
                    id: 0,
                    goal: "Initial".to_string(),
                    tactic: None,
                    children: Vec::new(),
                    status: NodeStatus::Open,
                    metadata: NodeMetadata {
                        time_spent: 0.0,
                        complexity: 1,
                        dependencies: Vec::new(),
                    },
                },
                layout: TreeLayout {
                    node_positions: HashMap::new(),
                    edges: Vec::new(),
                    bounds: (0.0, 0.0, 0.0, 0.0),
                },
            },
            type_graph: TypeDependencyGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            },
            homotopy_viewer: HomotopyViewer {
                paths: Vec::new(),
                dimension: 2,
            },
        }
    }
    
    /// Render proof tree to canvas
    pub fn render_proof_tree(&self) -> Result<(), JsValue> {
        let backend = CanvasBackend::with_canvas_object(self.canvas.clone())
            .ok_or_else(|| JsValue::from_str("Failed to create canvas backend"))?;
        
        let root = backend.into_drawing_area();
        root.fill(&WHITE)?;
        
        let mut chart = ChartBuilder::on(&root)
            .caption("Proof Tree", ("sans-serif", 30))
            .margin(10)
            .build_cartesian_2d(
                self.proof_tree.layout.bounds.0..self.proof_tree.layout.bounds.2,
                self.proof_tree.layout.bounds.1..self.proof_tree.layout.bounds.3,
            )?;
        
        // Draw edges
        for ((x1, y1), (x2, y2)) in &self.proof_tree.layout.edges {
            chart.draw_series(LineSeries::new(
                vec![(*x1, *y1), (*x2, *y2)],
                &BLACK,
            ))?;
        }
        
        // Draw nodes
        for (id, (x, y)) in &self.proof_tree.layout.node_positions {
            let node = self.find_node(*id, &self.proof_tree.root);
            if let Some(node) = node {
                let color = match node.status {
                    NodeStatus::Completed => &GREEN,
                    NodeStatus::InProgress => &YELLOW,
                    NodeStatus::Failed => &RED,
                    NodeStatus::Open => &BLUE,
                };
                
                chart.draw_series(PointSeries::of_element(
                    vec![(*x, *y)],
                    20,
                    color,
                    &|c, s, st| {
                        Circle::new(c, s, st.filled())
                    },
                ))?;
                
                // Draw label
                chart.draw_series(std::iter::once(Text::new(
                    node.goal.clone(),
                    (*x, *y - 30.0),
                    ("sans-serif", 12).into_font(),
                )))?;
            }
        }
        
        root.present()?;
        Ok(())
    }
    
    /// Render type dependency graph
    pub fn render_type_graph(&self) -> Result<(), JsValue> {
        let backend = CanvasBackend::with_canvas_object(self.canvas.clone())
            .ok_or_else(|| JsValue::from_str("Failed to create canvas backend"))?;
        
        let root = backend.into_drawing_area();
        root.fill(&WHITE)?;
        
        // Force-directed layout
        let positions = self.calculate_force_layout();
        
        let mut chart = ChartBuilder::on(&root)
            .caption("Type Dependencies", ("sans-serif", 30))
            .margin(10)
            .build_cartesian_2d(-100f32..100f32, -100f32..100f32)?;
        
        // Draw edges
        for edge in &self.type_graph.edges {
            if let (Some(from_pos), Some(to_pos)) = (
                positions.get(&edge.from),
                positions.get(&edge.to),
            ) {
                let style = match edge.kind {
                    EdgeKind::Dependency => ShapeStyle::from(&RED),
                    EdgeKind::Application => ShapeStyle::from(&BLUE),
                    EdgeKind::Instantiation => ShapeStyle::from(&GREEN).stroke_width(2),
                };
                
                chart.draw_series(LineSeries::new(
                    vec![*from_pos, *to_pos],
                    style,
                ))?;
            }
        }
        
        // Draw nodes
        for node in &self.type_graph.nodes {
            if let Some(pos) = positions.get(&node.id) {
                let color = match node.kind {
                    TypeKind::Universe => &MAGENTA,
                    TypeKind::Pi => &BLUE,
                    TypeKind::Path => &GREEN,
                    TypeKind::Inductive => &RED,
                    TypeKind::Definition => &BLACK,
                };
                
                chart.draw_series(PointSeries::of_element(
                    vec![*pos],
                    15,
                    color,
                    &|c, s, st| Rectangle::new([c, c], s, st.filled()),
                ))?;
                
                chart.draw_series(std::iter::once(Text::new(
                    node.name.clone(),
                    (pos.0, pos.1 - 20.0),
                    ("sans-serif", 10).into_font(),
                )))?;
            }
        }
        
        root.present()?;
        Ok(())
    }
    
    /// Render homotopy diagram
    pub fn render_homotopy(&self) -> Result<(), JsValue> {
        let backend = CanvasBackend::with_canvas_object(self.canvas.clone())
            .ok_or_else(|| JsValue::from_str("Failed to create canvas backend"))?;
        
        let root = backend.into_drawing_area();
        root.fill(&WHITE)?;
        
        if self.homotopy_viewer.dimension == 2 {
            let mut chart = ChartBuilder::on(&root)
                .caption("Homotopy Paths", ("sans-serif", 30))
                .margin(10)
                .build_cartesian_2d(-1f32..1f32, -1f32..1f32)?;
            
            chart.configure_mesh().draw()?;
            
            // Draw paths
            for path in &self.homotopy_viewer.paths {
                let points: Vec<(f32, f32)> = (0..=100)
                    .map(|i| {
                        let t = i as f64 / 100.0;
                        let p = (path.interpolation)(t);
                        (p.coords[0] as f32, p.coords[1] as f32)
                    })
                    .collect();
                
                chart.draw_series(LineSeries::new(points, &BLUE))?;
                
                // Draw endpoints
                chart.draw_series(PointSeries::of_element(
                    vec![
                        (path.start.coords[0] as f32, path.start.coords[1] as f32),
                        (path.end.coords[0] as f32, path.end.coords[1] as f32),
                    ],
                    5,
                    &RED,
                    &|c, s, st| Circle::new(c, s, st.filled()),
                ))?;
            }
        }
        
        root.present()?;
        Ok(())
    }
    
    /// Calculate force-directed layout for graph
    fn calculate_force_layout(&self) -> HashMap<usize, (f32, f32)> {
        let mut positions = HashMap::new();
        let node_count = self.type_graph.nodes.len();
        
        // Initialize with circle layout
        for (i, node) in self.type_graph.nodes.iter().enumerate() {
            let angle = 2.0 * std::f32::consts::PI * i as f32 / node_count as f32;
            positions.insert(node.id, (50.0 * angle.cos(), 50.0 * angle.sin()));
        }
        
        // Force simulation (simplified)
        for _ in 0..100 {
            let mut forces: HashMap<usize, (f32, f32)> = HashMap::new();
            
            // Repulsion between all nodes
            for n1 in &self.type_graph.nodes {
                let mut force = (0.0, 0.0);
                let p1 = positions[&n1.id];
                
                for n2 in &self.type_graph.nodes {
                    if n1.id != n2.id {
                        let p2 = positions[&n2.id];
                        let dx = p1.0 - p2.0;
                        let dy = p1.1 - p2.1;
                        let dist = (dx * dx + dy * dy).sqrt().max(1.0);
                        
                        force.0 += 100.0 * dx / (dist * dist);
                        force.1 += 100.0 * dy / (dist * dist);
                    }
                }
                
                forces.insert(n1.id, force);
            }
            
            // Attraction along edges
            for edge in &self.type_graph.edges {
                let p1 = positions[&edge.from];
                let p2 = positions[&edge.to];
                let dx = p2.0 - p1.0;
                let dy = p2.1 - p1.1;
                
                forces.entry(edge.from).or_insert((0.0, 0.0)).0 += dx * 0.01;
                forces.entry(edge.from).or_insert((0.0, 0.0)).1 += dy * 0.01;
                forces.entry(edge.to).or_insert((0.0, 0.0)).0 -= dx * 0.01;
                forces.entry(edge.to).or_insert((0.0, 0.0)).1 -= dy * 0.01;
            }
            
            // Apply forces
            for (id, force) in forces {
                if let Some(pos) = positions.get_mut(&id) {
                    pos.0 += force.0.max(-5.0).min(5.0);
                    pos.1 += force.1.max(-5.0).min(5.0);
                }
            }
        }
        
        positions
    }
    
    fn find_node(&self, id: usize, node: &ProofNode) -> Option<&ProofNode> {
        if node.id == id {
            Some(node)
        } else {
            node.children.iter().find_map(|child| self.find_node(id, child))
        }
    }
}

/// Export visualization as SVG
pub fn export_svg(viz: &Visualizer) -> String {
    let mut svg = String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 600">"#);
    
    // Add proof tree elements
    for ((x1, y1), (x2, y2)) in &viz.proof_tree.layout.edges {
        svg.push_str(&format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" />"#,
            x1 + 400.0, y1 + 300.0, x2 + 400.0, y2 + 300.0
        ));
    }
    
    for (_id, (x, y)) in &viz.proof_tree.layout.node_positions {
        svg.push_str(&format!(
            r#"<circle cx="{}" cy="{}" r="20" fill="blue" />"#,
            x + 400.0, y + 300.0
        ));
    }
    
    svg.push_str("</svg>");
    svg
}

/// Performance profiling visualization
#[wasm_bindgen]
pub struct PerformanceProfiler {
    samples: Vec<PerformanceSample>,
}

#[derive(Debug, Clone)]
pub struct PerformanceSample {
    pub timestamp: f64,
    pub operation: String,
    pub duration: f64,
    pub memory: usize,
}

#[wasm_bindgen]
impl PerformanceProfiler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        PerformanceProfiler {
            samples: Vec::new(),
        }
    }
    
    pub fn record(&mut self, operation: String, duration: f64, memory: usize) {
        self.samples.push(PerformanceSample {
            timestamp: js_sys::Date::now(),
            operation,
            duration,
            memory,
        });
    }
    
    pub fn render_timeline(&self, canvas_id: &str) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: HtmlCanvasElement = canvas.dyn_into()?;
        
        let backend = CanvasBackend::with_canvas_object(canvas)?;
        let root = backend.into_drawing_area();
        root.fill(&WHITE)?;
        
        if !self.samples.is_empty() {
            let min_time = self.samples.first().unwrap().timestamp;
            let max_time = self.samples.last().unwrap().timestamp;
            let max_duration = self.samples.iter()
                .map(|s| s.duration)
                .fold(0.0, f64::max);
            
            let mut chart = ChartBuilder::on(&root)
                .caption("Performance Timeline", ("sans-serif", 20))
                .margin(10)
                .x_label_area_size(30)
                .y_label_area_size(40)
                .build_cartesian_2d(min_time..max_time, 0f64..max_duration)?;
            
            chart.configure_mesh().draw()?;
            
            chart.draw_series(
                self.samples.iter().map(|s| {
                    Rectangle::new(
                        [(s.timestamp, 0.0), (s.timestamp + s.duration, s.duration)],
                        BLUE.mix(0.5).filled(),
                    )
                })
            )?;
        }
        
        root.present()?;
        Ok(())
    }
}