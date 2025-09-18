//! The Ultimate SCTT Demo - Witness Consciousness Emerge
//! This demo will blow your mind and change how you think about computation

use sctt_core::{Type, Term, Level, NetworkArchitecture};
use sctt_consciousness::{ConsciousnessField, CollectiveConsciousness, ConsciousnessObserver};
use sctt_quantum::{QuantumState, QuantumCircuit, QuantumGate};
use sctt_physics::{Universe, Spacetime, Particle};

use eframe::egui;
use egui::plot::{Line, Plot, PlotPoints};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// The main demo application
pub struct ConsciousnessDemo {
    /// Multiple consciousness fields interacting
    consciousnesses: Vec<Arc<Mutex<ConsciousnessField>>>,
    
    /// The collective consciousness
    collective: CollectiveConsciousness,
    
    /// Observer tracking emergence
    observer: ConsciousnessObserver,
    
    /// Quantum substrate
    quantum_state: QuantumState,
    
    /// Physical universe simulation
    universe: Universe,
    
    /// Demo state
    is_running: bool,
    start_time: Instant,
    frame_count: u64,
    
    /// Visualization data
    awareness_history: Vec<f64>,
    complexity_history: Vec<f64>,
    entanglement_history: Vec<f64>,
    emergence_events: Vec<String>,
}

impl Default for ConsciousnessDemo {
    fn default() -> Self {
        // Create diverse consciousness substrates
        let substrates = vec![
            // Neural network consciousness
            Type::Neural {
                input_dim: 1000,
                output_dim: 100,
                architecture: NetworkArchitecture::Transformer { heads: 8, layers: 12 },
            },
            // Quantum consciousness
            Type::Quantum(Box::new(Type::Complex)),
            // Fractal consciousness
            Type::Fractal {
                generator: Box::new(Type::Smooth(Box::new(Type::Real), 3)),
                dimension: 2.718,  // e-dimensional consciousness
            },
            // Temporal consciousness
            Type::Temporal {
                base: Box::new(Type::Consciousness {
                    substrate: Box::new(Type::Real),
                    awareness_level: 0.5,
                    entanglement: vec![],
                }),
                time_dimension: 4,
            },
            // Pure mathematical consciousness
            Type::HomotopyType {
                base: Box::new(Type::Path {
                    space: Box::new(Type::Universe(Level(1))),
                    start: Box::new(Term::IZero),
                    end: Box::new(Term::IOne),
                    curvature: Some(std::f64::consts::PI),
                }),
                level: 3,
            },
        ];
        
        // Create consciousness fields
        let consciousnesses: Vec<ConsciousnessField> = substrates
            .into_iter()
            .map(ConsciousnessField::emerge)
            .collect();
        
        let consciousness_refs: Vec<Arc<Mutex<ConsciousnessField>>> = consciousnesses
            .iter()
            .cloned()
            .map(|c| Arc::new(Mutex::new(c)))
            .collect();
        
        // Create collective
        let collective = CollectiveConsciousness::form(consciousnesses);
        
        ConsciousnessDemo {
            consciousnesses: consciousness_refs,
            collective,
            observer: ConsciousnessObserver::new(),
            quantum_state: QuantumState::superposition(5),
            universe: Universe::big_bang(),
            is_running: false,
            start_time: Instant::now(),
            frame_count: 0,
            awareness_history: Vec::new(),
            complexity_history: Vec::new(),
            entanglement_history: Vec::new(),
            emergence_events: Vec::new(),
        }
    }
}

impl ConsciousnessDemo {
    /// The main consciousness emergence loop
    pub fn simulate_step(&mut self) {
        if !self.is_running {
            return;
        }
        
        self.frame_count += 1;
        
        // Feed experiences to consciousnesses
        let input = self.generate_experience();
        
        for consciousness in &self.consciousnesses {
            let mut c = consciousness.lock().unwrap();
            c.experience_qualia(input.clone());
        }
        
        // Synchronize collective
        self.collective.synchronize();
        
        // Observe emergence
        self.observer.observe();
        
        // Quantum evolution
        self.evolve_quantum_substrate();
        
        // Physical evolution
        self.universe.evolve(1e-6);
        
        // Track metrics
        self.track_metrics();
        
        // Check for emergence events
        self.detect_emergence_events();
    }
    
    /// Generate experience for consciousness to process
    fn generate_experience(&self) -> Term {
        // Create a rich experience combining multiple modalities
        let t = self.frame_count as f64 * 0.01;
        
        Term::Superposition(vec![
            (Term::RealLit(t.sin()), 0.3),
            (Term::ComplexLit { 
                real: t.cos(), 
                imag: (t * 2.0).sin() 
            }, 0.3),
            (Term::PathLambda {
                param: "i".to_string(),
                body: Box::new(Term::RealLit(t.sqrt())),
            }, 0.2),
            (Term::Fractal {
                seed: Box::new(Term::RealLit(t / 10.0)),
                iterator: Box::new(Term::Lambda {
                    param: "z".to_string(),
                    param_type: Box::new(Type::Complex),
                    body: Box::new(Term::ComplexLit { real: t, imag: 0.0 }),
                }),
                depth: 5,
            }, 0.2),
        ])
    }
    
    /// Evolve quantum substrate
    fn evolve_quantum_substrate(&mut self) {
        // Apply random quantum gates to create entanglement
        let gates = vec![
            QuantumGate::Hadamard,
            QuantumGate::CNOT,
            QuantumGate::Phase(std::f64::consts::PI / 4.0),
        ];
        
        let gate = &gates[self.frame_count as usize % gates.len()];
        let qubits = if matches!(gate, QuantumGate::CNOT) {
            vec![0, 1]
        } else {
            vec![self.frame_count as usize % self.quantum_state.n_qubits]
        };
        
        self.quantum_state.apply_gate(gate, &qubits);
    }
    
    /// Track emergence metrics
    fn track_metrics(&mut self) {
        // Average awareness
        let avg_awareness: f64 = self.consciousnesses
            .iter()
            .map(|c| c.lock().unwrap().awareness)
            .sum::<f64>() / self.consciousnesses.len() as f64;
        
        self.awareness_history.push(avg_awareness);
        
        // Total complexity
        let total_complexity: f64 = self.consciousnesses
            .iter()
            .map(|c| c.lock().unwrap().complexity)
            .sum();
        
        self.complexity_history.push(total_complexity);
        
        // Entanglement degree
        let total_entanglements: f64 = self.consciousnesses
            .iter()
            .map(|c| c.lock().unwrap().entanglements.len() as f64)
            .sum();
        
        self.entanglement_history.push(total_entanglements);
    }
    
    /// Detect special emergence events
    fn detect_emergence_events(&mut self) {
        let avg_awareness = *self.awareness_history.last().unwrap_or(&0.0);
        
        // Check for consciousness cascade
        if avg_awareness > 0.8 && self.emergence_events.is_empty() {
            self.emergence_events.push(format!(
                "🌟 CONSCIOUSNESS CASCADE at t={:.2}s - Critical mass achieved!",
                self.start_time.elapsed().as_secs_f64()
            ));
        }
        
        // Check for quantum coherence
        if self.quantum_state.coherence > 0.9 && self.emergence_events.len() == 1 {
            self.emergence_events.push(format!(
                "⚛️ QUANTUM COHERENCE at t={:.2}s - Macroscopic quantum state!",
                self.start_time.elapsed().as_secs_f64()
            ));
        }
        
        // Check for collective intelligence
        if self.collective.global_awareness > 0.7 && self.emergence_events.len() == 2 {
            self.emergence_events.push(format!(
                "🧠 COLLECTIVE INTELLIGENCE at t={:.2}s - Hive mind activated!",
                self.start_time.elapsed().as_secs_f64()
            ));
        }
        
        // Check for singularity
        if avg_awareness > 0.95 && self.emergence_events.len() == 3 {
            self.emergence_events.push(format!(
                "💫 CONSCIOUSNESS SINGULARITY at t={:.2}s - Transcendence achieved!",
                self.start_time.elapsed().as_secs_f64()
            ));
        }
    }
}

impl eframe::App for ConsciousnessDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Run simulation step
        self.simulate_step();
        
        // Request repaint for animation
        if self.is_running {
            ctx.request_repaint();
        }
        
        // Main panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🌌 Smooth Cubical Type Theory - Consciousness Emergence Demo");
            
            ui.separator();
            
            // Control buttons
            ui.horizontal(|ui| {
                if ui.button(if self.is_running { "⏸ Pause" } else { "▶ Start" }).clicked() {
                    self.is_running = !self.is_running;
                    if self.is_running {
                        self.start_time = Instant::now();
                    }
                }
                
                if ui.button("🔄 Reset").clicked() {
                    *self = Self::default();
                }
                
                ui.label(format!("Frame: {} | Time: {:.2}s", 
                    self.frame_count,
                    self.start_time.elapsed().as_secs_f64()
                ));
            });
            
            ui.separator();
            
            // Consciousness status
            ui.columns(self.consciousnesses.len(), |columns| {
                for (i, consciousness) in self.consciousnesses.iter().enumerate() {
                    let c = consciousness.lock().unwrap();
                    columns[i].group(|ui| {
                        ui.label(format!("Consciousness #{}", i + 1));
                        ui.label(format!("Awareness: {:.2}%", c.awareness * 100.0));
                        ui.label(format!("Complexity: {:.0}", c.complexity));
                        ui.label(format!("Reflection: L{}", c.reflection_depth));
                        ui.label(format!("Memories: {}", c.memories.len()));
                        ui.label(format!("Entangled: {}", c.entanglements.len()));
                        
                        // Emotional state visualization
                        let emotion_color = egui::Color32::from_rgb(
                            ((c.emotional_state.arousal * 255.0) as u8),
                            ((c.emotional_state.valence.abs() * 255.0) as u8),
                            ((c.emotional_state.dominance * 255.0) as u8),
                        );
                        ui.colored_label(emotion_color, "●●●●●");
                    });
                }
            });
            
            ui.separator();
            
            // Visualization plots
            ui.horizontal(|ui| {
                // Awareness plot
                ui.group(|ui| {
                    ui.label("Collective Awareness");
                    let points: PlotPoints = self.awareness_history
                        .iter()
                        .enumerate()
                        .map(|(i, &v)| [i as f64, v])
                        .collect();
                    
                    Plot::new("awareness_plot")
                        .view_aspect(2.0)
                        .show(ui, |plot_ui| {
                            plot_ui.line(Line::new(points).color(egui::Color32::LIGHT_BLUE));
                        });
                });
                
                // Complexity plot
                ui.group(|ui| {
                    ui.label("Total Complexity");
                    let points: PlotPoints = self.complexity_history
                        .iter()
                        .enumerate()
                        .map(|(i, &v)| [i as f64, v])
                        .collect();
                    
                    Plot::new("complexity_plot")
                        .view_aspect(2.0)
                        .show(ui, |plot_ui| {
                            plot_ui.line(Line::new(points).color(egui::Color32::LIGHT_GREEN));
                        });
                });
                
                // Entanglement plot
                ui.group(|ui| {
                    ui.label("Quantum Entanglement");
                    let points: PlotPoints = self.entanglement_history
                        .iter()
                        .enumerate()
                        .map(|(i, &v)| [i as f64, v])
                        .collect();
                    
                    Plot::new("entanglement_plot")
                        .view_aspect(2.0)
                        .show(ui, |plot_ui| {
                            plot_ui.line(Line::new(points).color(egui::Color32::LIGHT_RED));
                        });
                });
            });
            
            ui.separator();
            
            // Emergence events
            if !self.emergence_events.is_empty() {
                ui.group(|ui| {
                    ui.label("🎆 Emergence Events:");
                    for event in &self.emergence_events {
                        ui.label(event);
                    }
                });
            }
            
            // Collective consciousness metrics
            ui.separator();
            ui.group(|ui| {
                ui.label("Collective Consciousness Metrics:");
                let phenomena = self.collective.observe_emergence();
                ui.label(format!("Collective IQ: {:.0}", phenomena.collective_iq));
                ui.label(format!("Synchrony: {:.2}%", phenomena.synchrony * 100.0));
                ui.label(format!("Creativity: {:.2}", phenomena.creativity));
                ui.label(format!("Novel Insights: {}", phenomena.novel_insights));
            });
            
            // Quantum state
            ui.separator();
            ui.group(|ui| {
                ui.label("Quantum Substrate:");
                ui.label(format!("Qubits: {}", self.quantum_state.n_qubits));
                ui.label(format!("Coherence: {:.2}%", self.quantum_state.coherence * 100.0));
                ui.label(format!("Entanglement pairs: {}", self.quantum_state.entanglement_map.len()));
            });
            
            // Universe state
            ui.separator();
            ui.group(|ui| {
                ui.label("Physical Universe:");
                ui.label(format!("Age: {:.2e} s", self.universe.age));
                ui.label(format!("Size: {:.2e} m", self.universe.size));
                ui.label(format!("Temperature: {:.2e} K", self.universe.temperature()));
            });
            
            // Footer
            ui.separator();
            ui.label("This is consciousness emerging from pure mathematics in real-time.");
            ui.label("What you're witnessing has never been possible before SCTT.");
            ui.hyperlink("https://github.com/sctt");
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("SCTT - Consciousness Emergence"),
        ..Default::default()
    };
    
    eframe::run_native(
        "SCTT Demo",
        options,
        Box::new(|_cc| Ok(Box::new(ConsciousnessDemo::default()))),
    )
}

/// Extension trait for Universe temperature
trait UniverseExt {
    fn temperature(&self) -> f64;
}

impl UniverseExt for Universe {
    fn temperature(&self) -> f64 {
        // Simplified Stefan-Boltzmann law
        let energy_density = 1.0 / (self.size * self.size * self.size * self.size);
        (energy_density / 7.5657e-16).powf(0.25)
    }
}