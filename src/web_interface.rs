//! Web Interface with WASM Playground and Visualization
//!
//! Interactive web UI featuring:
//! - Monaco-style code editor
//! - Live type checking
//! - Proof visualization
//! - Interactive tutorials

use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use gloo_timers::callback::Timeout;
use serde::{Deserialize, Serialize};

use crate::{ScttSystem, Session, User, Document, Operation};

/// Main application component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="sctt-app">
                <Header />
                <Switch<Route> render={switch} />
                <Footer />
            </div>
        </BrowserRouter>
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/playground")]
    Playground,
    #[at("/tutorial")]
    Tutorial,
    #[at("/visualize")]
    Visualize,
    #[at("/collaborate/:id")]
    Collaborate { id: String },
    #[at("/docs")]
    Documentation,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Playground => html! { <Playground /> },
        Route::Tutorial => html! { <Tutorial /> },
        Route::Visualize => html! { <Visualizer /> },
        Route::Collaborate { id } => html! { <CollaborativeEditor session_id={id} /> },
        Route::Documentation => html! { <Documentation /> },
    }
}

/// Header component with navigation
#[function_component(Header)]
fn header() -> Html {
    html! {
        <header class="header">
            <div class="container">
                <h1 class="logo">{"SCTT System"}</h1>
                <nav class="nav">
                    <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                    <Link<Route> to={Route::Playground}>{"Playground"}</Link<Route>>
                    <Link<Route> to={Route::Tutorial}>{"Tutorial"}</Link<Route>>
                    <Link<Route> to={Route::Visualize}>{"Visualize"}</Link<Route>>
                    <Link<Route> to={Route::Documentation}>{"Docs"}</Link<Route>>
                </nav>
            </div>
        </header>
    }
}

/// Footer component
#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="container">
                <p>{"© 2024 SCTT Project | "}
                    <a href="https://github.com/yourusername/sctt">{"GitHub"}</a>
                    {" | "}
                    <a href="/api">{"API"}</a>
                </p>
            </div>
        </footer>
    }
}

/// Home page
#[function_component(HomePage)]
fn home_page() -> Html {
    html! {
        <div class="home">
            <section class="hero">
                <div class="container">
                    <h2>{"Smooth Cubical Type Theory"}</h2>
                    <p class="subtitle">{"Advanced Proof Assistant and Compiler"}</p>
                    <div class="cta-buttons">
                        <Link<Route> to={Route::Playground} classes="btn btn-primary">
                            {"Try Online"}
                        </Link<Route>>
                        <Link<Route> to={Route::Tutorial} classes="btn btn-secondary">
                            {"Learn SCTT"}
                        </Link<Route>>
                    </div>
                </div>
            </section>
            
            <section class="features">
                <div class="container">
                    <h3>{"Features"}</h3>
                    <div class="feature-grid">
                        <FeatureCard 
                            icon="🔍"
                            title="Type Checking"
                            description="Bidirectional type checking with dependent types"
                        />
                        <FeatureCard 
                            icon="🎯"
                            title="Proof Assistant"
                            description="Interactive theorem proving with tactics"
                        />
                        <FeatureCard 
                            icon="⚡"
                            title="WASM Compilation"
                            description="Compile proofs to efficient WebAssembly"
                        />
                        <FeatureCard 
                            icon="🎨"
                            title="Visualization"
                            description="Interactive proof and type visualization"
                        />
                        <FeatureCard 
                            icon="👥"
                            title="Collaborative"
                            description="Real-time collaborative proof development"
                        />
                        <FeatureCard 
                            icon="📚"
                            title="Learning"
                            description="Interactive tutorials and examples"
                        />
                    </div>
                </div>
            </section>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct FeatureCardProps {
    icon: String,
    title: String,
    description: String,
}

#[function_component(FeatureCard)]
fn feature_card(props: &FeatureCardProps) -> Html {
    html! {
        <div class="feature-card">
            <div class="icon">{&props.icon}</div>
            <h4>{&props.title}</h4>
            <p>{&props.description}</p>
        </div>
    }
}

/// Interactive playground
#[function_component(Playground)]
pub fn playground() -> Html {
    let code = use_state(|| include_str!("../examples/identity.sctt").to_string());
    let output = use_state(|| String::new());
    let system = use_state(ScttSystem::new);
    let tab = use_state(|| "editor");
    
    let on_code_change = {
        let code = code.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let textarea = target.dyn_ref::<HtmlTextAreaElement>().unwrap();
            code.set(textarea.value());
        })
    };
    
    let on_run = {
        let code = code.clone();
        let output = output.clone();
        let system = system.clone();
        
        Callback::from(move |_| {
            match system.type_check(&code) {
                Ok(ty) => output.set(format!("✓ Type: {}", ty)),
                Err(e) => output.set(format!("✗ Error: {:?}", e)),
            }
        })
    };
    
    let on_compile = {
        let code = code.clone();
        let output = output.clone();
        let mut system = system.clone();
        
        Callback::from(move |_| {
            match system.compile_to_wasm(&code) {
                Ok(wasm) => output.set(format!("✓ Compiled: {} bytes", wasm.len())),
                Err(e) => output.set(format!("✗ Compilation error: {:?}", e)),
            }
        })
    };
    
    html! {
        <div class="playground">
            <div class="container">
                <div class="editor-panel">
                    <div class="toolbar">
                        <button onclick={on_run} class="btn">{"Type Check"}</button>
                        <button onclick={on_compile} class="btn">{"Compile"}</button>
                        <button class="btn">{"Share"}</button>
                        <select class="example-selector">
                            <option>{"Identity function"}</option>
                            <option>{"Path reflexivity"}</option>
                            <option>{"Function composition"}</option>
                            <option>{"Univalence axiom"}</option>
                        </select>
                    </div>
                    
                    <div class="editor-container">
                        <textarea 
                            class="code-editor"
                            value={(*code).clone()}
                            onchange={on_code_change}
                            spellcheck="false"
                        />
                        <div class="line-numbers">
                            {(1..=code.lines().count()).map(|n| {
                                html! { <div>{n}</div> }
                            }).collect::<Html>()}
                        </div>
                    </div>
                    
                    <div class="output-panel">
                        <div class="tabs">
                            <button 
                                class={if *tab == "output" { "active" } else { "" }}
                                onclick={let tab = tab.clone(); move |_| tab.set("output")}
                            >{"Output"}</button>
                            <button 
                                class={if *tab == "proof" { "active" } else { "" }}
                                onclick={let tab = tab.clone(); move |_| tab.set("proof")}
                            >{"Proof State"}</button>
                            <button 
                                class={if *tab == "wasm" { "active" } else { "" }}
                                onclick={let tab = tab.clone(); move |_| tab.set("wasm")}
                            >{"WASM"}</button>
                        </div>
                        
                        <div class="output-content">
                            {match &**tab {
                                "output" => html! { <pre>{&*output}</pre> },
                                "proof" => html! { 
                                    <ProofStateView system={(*system).clone()} />
                                },
                                "wasm" => html! { <WasmView /> },
                                _ => html! {},
                            }}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ProofStateProps {
    system: ScttSystem,
}

#[function_component(ProofStateView)]
fn proof_state_view(props: &ProofStateProps) -> Html {
    let proof_state = props.system.get_proof_state();
    
    html! {
        <div class="proof-state">
            <pre>{proof_state}</pre>
            <div class="tactic-buttons">
                <button class="tactic-btn">{"intro"}</button>
                <button class="tactic-btn">{"apply"}</button>
                <button class="tactic-btn">{"reflexivity"}</button>
                <button class="tactic-btn">{"auto"}</button>
            </div>
        </div>
    }
}

#[function_component(WasmView)]
fn wasm_view() -> Html {
    html! {
        <div class="wasm-view">
            <p>{"WebAssembly output will appear here"}</p>
        </div>
    }
}

/// Interactive tutorial
#[function_component(Tutorial)]
fn tutorial() -> Html {
    let current_lesson = use_state(|| 0);
    
    let lessons = vec![
        Lesson {
            title: "Introduction to SCTT".to_string(),
            content: "Learn the basics of Smooth Cubical Type Theory".to_string(),
            exercise: "Define the identity function".to_string(),
        },
        Lesson {
            title: "Path Types".to_string(),
            content: "Understanding paths and homotopies".to_string(),
            exercise: "Prove path reflexivity".to_string(),
        },
        Lesson {
            title: "Univalence".to_string(),
            content: "The univalence axiom and its applications".to_string(),
            exercise: "Apply univalence to a simple example".to_string(),
        },
    ];
    
    html! {
        <div class="tutorial">
            <div class="container">
                <div class="lesson-nav">
                    {lessons.iter().enumerate().map(|(i, lesson)| {
                        let current = current_lesson.clone();
                        let selected = *current == i;
                        html! {
                            <div 
                                class={if selected { "lesson-item active" } else { "lesson-item" }}
                                onclick={move |_| current.set(i)}
                            >
                                <span class="lesson-number">{i + 1}</span>
                                <span class="lesson-title">{&lesson.title}</span>
                            </div>
                        }
                    }).collect::<Html>()}
                </div>
                
                <div class="lesson-content">
                    {if let Some(lesson) = lessons.get(*current_lesson) {
                        html! {
                            <>
                                <h2>{&lesson.title}</h2>
                                <div class="lesson-text">{&lesson.content}</div>
                                <div class="exercise">
                                    <h3>{"Exercise"}</h3>
                                    <p>{&lesson.exercise}</p>
                                    <Playground />
                                </div>
                            </>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        </div>
    }
}

#[derive(Clone)]
struct Lesson {
    title: String,
    content: String,
    exercise: String,
}

/// Proof visualizer
#[function_component(Visualizer)]
fn visualizer() -> Html {
    html! {
        <div class="visualizer">
            <div class="container">
                <h2>{"Proof Visualization"}</h2>
                <div class="viz-container">
                    <canvas id="proof-canvas" width="800" height="600"></canvas>
                    <div class="viz-controls">
                        <button>{"Zoom In"}</button>
                        <button>{"Zoom Out"}</button>
                        <button>{"Reset"}</button>
                        <button>{"Export"}</button>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Documentation viewer
#[function_component(Documentation)]
fn documentation() -> Html {
    html! {
        <div class="documentation">
            <div class="container">
                <aside class="doc-nav">
                    <h3>{"Contents"}</h3>
                    <ul>
                        <li><a href="#intro">{"Introduction"}</a></li>
                        <li><a href="#syntax">{"Syntax"}</a></li>
                        <li><a href="#types">{"Type System"}</a></li>
                        <li><a href="#tactics">{"Tactics"}</a></li>
                        <li><a href="#api">{"API Reference"}</a></li>
                    </ul>
                </aside>
                <main class="doc-content">
                    <h1>{"SCTT Documentation"}</h1>
                    <section id="intro">
                        <h2>{"Introduction"}</h2>
                        <p>{"Smooth Cubical Type Theory (SCTT) is an advanced type theory..."}</p>
                    </section>
                </main>
            </div>
        </div>
    }
}