//! Smooth Quest: A Mathematical Adventure in SCTT
//! Learn smooth cubical type theory by solving puzzles and building the theory!

use anyhow::Result;
use colored::*;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use nalgebra as na;
use rand::Rng;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use sctt_core::{Term, Type};
use sctt_smooth::{SmoothFunction, SmoothPath};
use std::{
    io,
    time::{Duration, Instant},
};

// ============================================================================
// GAME STATE
// ============================================================================

#[derive(Debug, Clone)]
struct GameState {
    player: Player,
    current_level: usize,
    levels: Vec<Level>,
    inventory: Vec<MathConcept>,
    knowledge_points: u32,
    smooth_meter: f64,
    cubical_meter: f64,
    coherence_meter: f64,
    messages: Vec<String>,
    current_challenge: Option<Challenge>,
    game_mode: GameMode,
    theory_fragments: Vec<TheoryFragment>,
}

#[derive(Debug, Clone)]
struct Player {
    x: f64,
    y: f64,
    velocity_x: f64,
    velocity_y: f64,
    path_history: Vec<(f64, f64)>,
    is_smooth: bool,
}

#[derive(Debug, Clone)]
enum GameMode {
    Exploration,
    Puzzle,
    TheoryBuilding,
    BossBattle,
}

#[derive(Debug, Clone)]
struct Level {
    name: String,
    description: String,
    challenges: Vec<Challenge>,
    theory_unlock: TheoryFragment,
}

#[derive(Debug, Clone)]
struct Challenge {
    name: String,
    question: String,
    challenge_type: ChallengeType,
    reward: MathConcept,
    hint: String,
}

#[derive(Debug, Clone)]
enum ChallengeType {
    SmoothFunction { target: String, tolerance: f64 },
    PathFinding { start: (f64, f64), end: (f64, f64) },
    Composition { f: String, g: String, expected: String },
    IntervalPuzzle { expression: String, answer: f64 },
    CoherenceCheck { paths: Vec<String> },
}

#[derive(Debug, Clone)]
enum MathConcept {
    Derivative { order: usize },
    SmoothMap { name: String },
    PathType,
    IntervalOperation { op: String },
    CompositionRule,
    ChainRule,
    TaylorSeries { order: usize },
    CoherenceAxiom { number: usize },
}

#[derive(Debug, Clone)]
struct TheoryFragment {
    name: String,
    description: String,
    formal_statement: String,
    unlocks: Vec<MathConcept>,
}

impl GameState {
    fn new() -> Self {
        GameState {
            player: Player::new(),
            current_level: 0,
            levels: create_levels(),
            inventory: vec![],
            knowledge_points: 0,
            smooth_meter: 0.0,
            cubical_meter: 0.0,
            coherence_meter: 0.0,
            messages: vec!["Welcome to Smooth Quest!".to_string()],
            current_challenge: None,
            game_mode: GameMode::Exploration,
            theory_fragments: vec![],
        }
    }

    fn add_message(&mut self, msg: String) {
        self.messages.push(msg);
        if self.messages.len() > 5 {
            self.messages.remove(0);
        }
    }

    fn check_smoothness(&self) -> bool {
        // Check if player's path is smooth (no sharp turns)
        if self.player.path_history.len() < 3 {
            return true;
        }
        
        let n = self.player.path_history.len();
        let p1 = self.player.path_history[n - 3];
        let p2 = self.player.path_history[n - 2];
        let p3 = self.player.path_history[n - 1];
        
        // Check second derivative (acceleration change)
        let v1 = ((p2.0 - p1.0), (p2.1 - p1.1));
        let v2 = ((p3.0 - p2.0), (p3.1 - p2.1));
        let accel_change = ((v2.0 - v1.0).abs() + (v2.1 - v1.1).abs());
        
        accel_change < 0.5  // Threshold for smoothness
    }

    fn update_meters(&mut self) {
        // Update understanding meters based on inventory
        self.smooth_meter = self.inventory.iter()
            .filter(|c| matches!(c, MathConcept::Derivative { .. } | MathConcept::SmoothMap { .. }))
            .count() as f64 * 0.1;
        
        self.cubical_meter = self.inventory.iter()
            .filter(|c| matches!(c, MathConcept::PathType | MathConcept::IntervalOperation { .. }))
            .count() as f64 * 0.1;
        
        self.coherence_meter = self.inventory.iter()
            .filter(|c| matches!(c, MathConcept::CoherenceAxiom { .. }))
            .count() as f64 * 0.15;
    }
}

impl Player {
    fn new() -> Self {
        Player {
            x: 0.5,
            y: 0.5,
            velocity_x: 0.0,
            velocity_y: 0.0,
            path_history: vec![(0.5, 0.5)],
            is_smooth: true,
        }
    }

    fn move_smooth(&mut self, dx: f64, dy: f64) {
        // Smooth movement using acceleration
        let accel = 0.1;
        let friction = 0.9;
        
        self.velocity_x += dx * accel;
        self.velocity_y += dy * accel;
        
        self.velocity_x *= friction;
        self.velocity_y *= friction;
        
        self.x += self.velocity_x;
        self.y += self.velocity_y;
        
        // Keep in bounds [0, 1] (the interval!)
        self.x = self.x.clamp(0.0, 1.0);
        self.y = self.y.clamp(0.0, 1.0);
        
        self.path_history.push((self.x, self.y));
        if self.path_history.len() > 50 {
            self.path_history.remove(0);
        }
    }
}

// ============================================================================
// LEVEL DESIGN
// ============================================================================

fn create_levels() -> Vec<Level> {
    vec![
        Level {
            name: "The Smooth Plains".to_string(),
            description: "Learn what it means to be smooth".to_string(),
            challenges: vec![
                Challenge {
                    name: "First Derivative".to_string(),
                    question: "What is the derivative of x²?".to_string(),
                    challenge_type: ChallengeType::SmoothFunction {
                        target: "2x".to_string(),
                        tolerance: 0.01,
                    },
                    reward: MathConcept::Derivative { order: 1 },
                    hint: "Power rule: d/dx(x^n) = n*x^(n-1)".to_string(),
                },
                Challenge {
                    name: "Smooth Path".to_string(),
                    question: "Navigate smoothly from (0,0) to (1,1)".to_string(),
                    challenge_type: ChallengeType::PathFinding {
                        start: (0.0, 0.0),
                        end: (1.0, 1.0),
                    },
                    reward: MathConcept::PathType,
                    hint: "Avoid sharp turns!".to_string(),
                },
            ],
            theory_unlock: TheoryFragment {
                name: "Smoothness Axiom".to_string(),
                description: "A function is smooth if all derivatives exist".to_string(),
                formal_statement: "f ∈ C^∞ ⟺ ∀n. ∃f^(n)".to_string(),
                unlocks: vec![MathConcept::TaylorSeries { order: 3 }],
            },
        },
        Level {
            name: "The Interval Forest".to_string(),
            description: "Master the unit interval and its operations".to_string(),
            challenges: vec![
                Challenge {
                    name: "Interval Meet".to_string(),
                    question: "Compute 0.3 ∧ 0.7".to_string(),
                    challenge_type: ChallengeType::IntervalPuzzle {
                        expression: "min(0.3, 0.7)".to_string(),
                        answer: 0.3,
                    },
                    reward: MathConcept::IntervalOperation { op: "meet".to_string() },
                    hint: "Meet means minimum!".to_string(),
                },
                Challenge {
                    name: "De Morgan's Law".to_string(),
                    question: "Verify ¬(i ∧ j) = ¬i ∨ ¬j".to_string(),
                    challenge_type: ChallengeType::IntervalPuzzle {
                        expression: "1 - min(i, j) = max(1-i, 1-j)".to_string(),
                        answer: 1.0,  // True
                    },
                    reward: MathConcept::IntervalOperation { op: "demorgan".to_string() },
                    hint: "Test with specific values".to_string(),
                },
            ],
            theory_unlock: TheoryFragment {
                name: "Interval Structure".to_string(),
                description: "The interval I forms a De Morgan algebra".to_string(),
                formal_statement: "(I, ∧, ∨, ¬, 0, 1) is a De Morgan algebra".to_string(),
                unlocks: vec![],
            },
        },
        Level {
            name: "Composition Canyon".to_string(),
            description: "Learn to compose functions and paths".to_string(),
            challenges: vec![
                Challenge {
                    name: "Chain Rule".to_string(),
                    question: "Compose sin(x) with x²".to_string(),
                    challenge_type: ChallengeType::Composition {
                        f: "sin".to_string(),
                        g: "x²".to_string(),
                        expected: "sin(x²)".to_string(),
                    },
                    reward: MathConcept::ChainRule,
                    hint: "(f∘g)(x) = f(g(x))".to_string(),
                },
            ],
            theory_unlock: TheoryFragment {
                name: "Composition Preserves Smoothness".to_string(),
                description: "The composition of smooth functions is smooth".to_string(),
                formal_statement: "f, g ∈ C^∞ ⟹ f∘g ∈ C^∞".to_string(),
                unlocks: vec![MathConcept::CompositionRule],
            },
        },
        Level {
            name: "Coherence Castle".to_string(),
            description: "The final challenge: make smooth and cubical work together!".to_string(),
            challenges: vec![
                Challenge {
                    name: "The Coherence Challenge".to_string(),
                    question: "Find paths that compose smoothly".to_string(),
                    challenge_type: ChallengeType::CoherenceCheck {
                        paths: vec!["t²".to_string(), "1 + t".to_string()],
                    },
                    reward: MathConcept::CoherenceAxiom { number: 1 },
                    hint: "Check derivatives at connection points".to_string(),
                },
            ],
            theory_unlock: TheoryFragment {
                name: "Smooth-Cubical Coherence".to_string(),
                description: "All cubical operations preserve smooth structure".to_string(),
                formal_statement: "comp : Smooth(A) → Smooth(comp(A))".to_string(),
                unlocks: vec![MathConcept::CoherenceAxiom { number: 2 }],
            },
        },
    ]
}

// ============================================================================
// PUZZLE SOLVERS
// ============================================================================

fn check_smooth_function_answer(input: &str, target: &str, tolerance: f64) -> bool {
    // Simplified check - in real game would parse and evaluate
    input.trim().eq_ignore_ascii_case(target) || 
    input.contains(&target)
}

fn check_interval_answer(input: f64, expected: f64, tolerance: f64) -> bool {
    (input - expected).abs() < tolerance
}

fn evaluate_path_smoothness(path: &[(f64, f64)]) -> f64 {
    if path.len() < 3 {
        return 1.0;
    }
    
    let mut smoothness = 1.0;
    for i in 2..path.len() {
        let p1 = path[i - 2];
        let p2 = path[i - 1];
        let p3 = path[i];
        
        // Check curvature change
        let v1 = (p2.0 - p1.0, p2.1 - p1.1);
        let v2 = (p3.0 - p2.0, p3.1 - p2.1);
        let angle_change = ((v2.0 - v1.0).powi(2) + (v2.1 - v1.1).powi(2)).sqrt();
        
        smoothness *= (-angle_change).exp();  // Penalize sharp turns
    }
    
    smoothness
}

// ============================================================================
// RENDERING
// ============================================================================

fn draw_game<B: Backend>(f: &mut Frame<B>, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),      // Title
            Constraint::Min(15),        // Main game area
            Constraint::Length(6),      // Status bars
            Constraint::Length(6),      // Messages
            Constraint::Length(3),      // Input
        ])
        .split(f.size());
    
    // Title
    let title = Paragraph::new("◆ SMOOTH QUEST: A Mathematical Adventure ◆")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);
    
    // Main game area
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(chunks[1]);
    
    // Game field
    draw_game_field(f, main_chunks[0], state);
    
    // Inventory and theory
    draw_inventory(f, main_chunks[1], state);
    
    // Status bars
    draw_status_bars(f, chunks[2], state);
    
    // Messages
    draw_messages(f, chunks[3], state);
    
    // Input area
    draw_input_area(f, chunks[4], state);
}

fn draw_game_field<B: Backend>(f: &mut Frame<B>, area: Rect, state: &GameState) {
    let field_block = Block::default()
        .title(" Game Field ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    
    let inner = field_block.inner(area);
    f.render_widget(field_block, area);
    
    // Create ASCII representation of the game field
    let mut field_text = vec![];
    
    // Draw based on game mode
    match state.game_mode {
        GameMode::Exploration => {
            // Show player position on interval square
            let field_height = 10;
            let field_width = 20;
            
            for y in 0..field_height {
                let mut line = String::new();
                for x in 0..field_width {
                    let fx = x as f64 / field_width as f64;
                    let fy = 1.0 - (y as f64 / field_height as f64);
                    
                    if (fx - state.player.x).abs() < 0.05 && (fy - state.player.y).abs() < 0.1 {
                        line.push_str("◉");  // Player
                    } else if state.player.path_history.iter().any(|(px, py)| 
                        (fx - px).abs() < 0.05 && (fy - py).abs() < 0.1) {
                        line.push_str("·");  // Path trail
                    } else {
                        line.push_str(" ");
                    }
                }
                field_text.push(Line::from(line));
            }
            
            // Add coordinates
            field_text.push(Line::from(format!("Position: ({:.2}, {:.2})", 
                state.player.x, state.player.y)));
            field_text.push(Line::from(format!("Velocity: ({:.2}, {:.2})", 
                state.player.velocity_x, state.player.velocity_y)));
            field_text.push(Line::from(format!("Path smoothness: {}", 
                if state.player.is_smooth { "✓ Smooth" } else { "✗ Not smooth" })));
        }
        GameMode::Puzzle => {
            if let Some(challenge) = &state.current_challenge {
                field_text.push(Line::from(Span::styled(
                    format!("◆ {} ◆", challenge.name),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                )));
                field_text.push(Line::from(""));
                
                // Word wrap the question
                for line in challenge.question.chars().collect::<Vec<_>>().chunks(inner.width as usize - 2) {
                    field_text.push(Line::from(line.iter().collect::<String>()));
                }
                
                field_text.push(Line::from(""));
                field_text.push(Line::from(Span::styled(
                    "Hint: Press 'h' for a hint",
                    Style::default().fg(Color::Gray)
                )));
            }
        }
        GameMode::TheoryBuilding => {
            field_text.push(Line::from(Span::styled(
                "◆ Theory Construction Mode ◆",
                Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)
            )));
            field_text.push(Line::from(""));
            field_text.push(Line::from("Combine concepts to build theorems:"));
            field_text.push(Line::from(""));
            
            for (i, fragment) in state.theory_fragments.iter().enumerate() {
                field_text.push(Line::from(format!("{}. {}", i + 1, fragment.name)));
            }
        }
        GameMode::BossBattle => {
            field_text.push(Line::from(Span::styled(
                "◆ COHERENCE BOSS BATTLE ◆",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK)
            )));
            field_text.push(Line::from(""));
            field_text.push(Line::from("Prove that smooth and cubical structures are compatible!"));
        }
    }
    
    let game_field = Paragraph::new(field_text)
        .wrap(Wrap { trim: false });
    f.render_widget(game_field, inner);
}

fn draw_inventory<B: Backend>(f: &mut Frame<B>, area: Rect, state: &GameState) {
    let inventory_block = Block::default()
        .title(" Inventory & Theory ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Green));
    
    let inner = inventory_block.inner(area);
    f.render_widget(inventory_block, area);
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner);
    
    // Inventory
    let inventory_items: Vec<ListItem> = state.inventory
        .iter()
        .map(|concept| {
            let text = match concept {
                MathConcept::Derivative { order } => format!("∂^{} Derivative", order),
                MathConcept::SmoothMap { name } => format!("⟨{}⟩ Smooth Map", name),
                MathConcept::PathType => "◇ Path Type".to_string(),
                MathConcept::IntervalOperation { op } => format!("□ Interval: {}", op),
                MathConcept::CompositionRule => "∘ Composition".to_string(),
                MathConcept::ChainRule => "⚙ Chain Rule".to_string(),
                MathConcept::TaylorSeries { order } => format!("Σ Taylor (order {})", order),
                MathConcept::CoherenceAxiom { number } => format!("※ Coherence #{}", number),
            };
            ListItem::new(text)
        })
        .collect();
    
    let inventory_list = List::new(inventory_items)
        .block(Block::default().title("Concepts").borders(Borders::TOP))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(inventory_list, chunks[0]);
    
    // Theory fragments
    let theory_items: Vec<ListItem> = state.theory_fragments
        .iter()
        .map(|fragment| ListItem::new(format!("◆ {}", fragment.name)))
        .collect();
    
    let theory_list = List::new(theory_items)
        .block(Block::default().title("Theory").borders(Borders::TOP))
        .style(Style::default().fg(Color::Magenta));
    f.render_widget(theory_list, chunks[1]);
}

fn draw_status_bars<B: Backend>(f: &mut Frame<B>, area: Rect, state: &GameState) {
    let status_block = Block::default()
        .title(" Understanding ")
        .borders(Borders::ALL);
    
    let inner = status_block.inner(area);
    f.render_widget(status_block, area);
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner);
    
    // Smooth meter
    let smooth_gauge = Gauge::default()
        .label(format!("Smooth: {:.0}%", state.smooth_meter * 100.0))
        .percent((state.smooth_meter * 100.0) as u16)
        .style(Style::default().fg(Color::Blue));
    f.render_widget(smooth_gauge, chunks[0]);
    
    // Cubical meter
    let cubical_gauge = Gauge::default()
        .label(format!("Cubical: {:.0}%", state.cubical_meter * 100.0))
        .percent((state.cubical_meter * 100.0) as u16)
        .style(Style::default().fg(Color::Green));
    f.render_widget(cubical_gauge, chunks[1]);
    
    // Coherence meter
    let coherence_gauge = Gauge::default()
        .label(format!("Coherence: {:.0}%", state.coherence_meter * 100.0))
        .percent((state.coherence_meter * 100.0) as u16)
        .style(Style::default().fg(Color::Magenta));
    f.render_widget(coherence_gauge, chunks[2]);
}

fn draw_messages<B: Backend>(f: &mut Frame<B>, area: Rect, state: &GameState) {
    let messages_block = Block::default()
        .title(" Messages ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));
    
    let messages_text: Vec<Line> = state.messages
        .iter()
        .map(|msg| Line::from(msg.clone()))
        .collect();
    
    let messages = Paragraph::new(messages_text)
        .block(messages_block)
        .wrap(Wrap { trim: true });
    
    f.render_widget(messages, area);
}

fn draw_input_area<B: Backend>(f: &mut Frame<B>, area: Rect, state: &GameState) {
    let help_text = match state.game_mode {
        GameMode::Exploration => "Arrow keys: Move | Space: Interact | P: Puzzle | T: Theory | Q: Quit",
        GameMode::Puzzle => "Type answer and press Enter | Esc: Back | H: Hint",
        GameMode::TheoryBuilding => "1-9: Select | C: Combine | Esc: Back",
        GameMode::BossBattle => "Prove the coherence theorem to win!",
    };
    
    let input = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    
    f.render_widget(input, area);
}

// ============================================================================
// GAME LOGIC
// ============================================================================

fn handle_input(state: &mut GameState, key: KeyCode) -> bool {
    match state.game_mode {
        GameMode::Exploration => {
            match key {
                KeyCode::Up => state.player.move_smooth(0.0, 0.1),
                KeyCode::Down => state.player.move_smooth(0.0, -0.1),
                KeyCode::Left => state.player.move_smooth(-0.1, 0.0),
                KeyCode::Right => state.player.move_smooth(0.1, 0.0),
                KeyCode::Char(' ') => {
                    // Check for nearby challenge
                    if let Some(level) = state.levels.get(state.current_level) {
                        if !level.challenges.is_empty() {
                            state.current_challenge = Some(level.challenges[0].clone());
                            state.game_mode = GameMode::Puzzle;
                            state.add_message("Entering puzzle mode!".to_string());
                        }
                    }
                }
                KeyCode::Char('p') | KeyCode::Char('P') => {
                    state.game_mode = GameMode::Puzzle;
                    if let Some(level) = state.levels.get(state.current_level) {
                        if !level.challenges.is_empty() {
                            state.current_challenge = Some(level.challenges[0].clone());
                        }
                    }
                }
                KeyCode::Char('t') | KeyCode::Char('T') => {
                    state.game_mode = GameMode::TheoryBuilding;
                    state.add_message("Entering theory building mode!".to_string());
                }
                KeyCode::Char('q') | KeyCode::Char('Q') => return true,
                _ => {}
            }
            
            // Check smoothness
            state.player.is_smooth = state.check_smoothness();
            
            // Check for level completion
            if state.player.x > 0.9 && state.player.y > 0.9 {
                if state.current_level < state.levels.len() - 1 {
                    state.current_level += 1;
                    state.add_message(format!("Level complete! Entering: {}", 
                        state.levels[state.current_level].name));
                    state.player = Player::new();
                } else {
                    state.game_mode = GameMode::BossBattle;
                    state.add_message("FINAL BOSS: Prove Coherence!".to_string());
                }
            }
        }
        GameMode::Puzzle => {
            match key {
                KeyCode::Esc => {
                    state.game_mode = GameMode::Exploration;
                    state.add_message("Back to exploration".to_string());
                }
                KeyCode::Char('h') | KeyCode::Char('H') => {
                    if let Some(challenge) = &state.current_challenge {
                        state.add_message(format!("Hint: {}", challenge.hint));
                    }
                }
                KeyCode::Enter => {
                    // Check answer (simplified for demo)
                    if let Some(challenge) = &state.current_challenge {
                        state.inventory.push(challenge.reward.clone());
                        state.knowledge_points += 10;
                        state.add_message("Correct! Concept acquired!".to_string());
                        state.update_meters();
                        
                        // Remove challenge from level
                        if let Some(level) = state.levels.get_mut(state.current_level) {
                            level.challenges.retain(|c| c.name != challenge.name);
                            
                            if level.challenges.is_empty() {
                                state.theory_fragments.push(level.theory_unlock.clone());
                                state.add_message(format!("Theory unlocked: {}", 
                                    level.theory_unlock.name));
                            }
                        }
                        
                        state.current_challenge = None;
                        state.game_mode = GameMode::Exploration;
                    }
                }
                _ => {}
            }
        }
        GameMode::TheoryBuilding => {
            match key {
                KeyCode::Esc => {
                    state.game_mode = GameMode::Exploration;
                }
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if state.theory_fragments.len() >= 2 {
                        state.add_message("Combining theories...".to_string());
                        state.coherence_meter += 0.2;
                        state.update_meters();
                        
                        if state.coherence_meter >= 1.0 {
                            state.game_mode = GameMode::BossBattle;
                            state.add_message("Ready for final boss!".to_string());
                        }
                    }
                }
                _ => {}
            }
        }
        GameMode::BossBattle => {
            match key {
                KeyCode::Enter => {
                    if state.coherence_meter >= 1.0 {
                        state.add_message("🎉 VICTORY! You've proven SCTT coherence!".to_string());
                        state.add_message("Smooth and Cubical are united!".to_string());
                        return true;  // End game
                    } else {
                        state.add_message("Not enough coherence understanding!".to_string());
                    }
                }
                KeyCode::Esc => {
                    state.game_mode = GameMode::Exploration;
                }
                _ => {}
            }
        }
    }
    
    false
}

// ============================================================================
// MAIN GAME LOOP
// ============================================================================

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Game state
    let mut game_state = GameState::new();
    game_state.add_message("Welcome to Smooth Quest!".to_string());
    game_state.add_message("Learn SCTT by solving puzzles!".to_string());
    
    // Game loop
    loop {
        terminal.draw(|f| draw_game(f, &game_state))?;
        
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                if handle_input(&mut game_state, code) {
                    break;
                }
            }
        }
    }
    
    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    
    println!("\n{}", "Thanks for playing Smooth Quest!".green().bold());
    println!("{}", "You've learned the foundations of SCTT!".cyan());
    println!("\nFinal Stats:");
    println!("  Knowledge Points: {}", game_state.knowledge_points);
    println!("  Concepts Learned: {}", game_state.inventory.len());
    println!("  Theory Fragments: {}", game_state.theory_fragments.len());
    
    Ok(())
}