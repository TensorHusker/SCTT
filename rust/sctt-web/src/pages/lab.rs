use leptos::*;
use leptos::html::Textarea;
use crate::state::AppState;

#[component]
pub fn LabPage() -> impl IntoView {
    let state = use_context::<RwSignal<AppState>>().unwrap();
    let (code, set_code) = create_signal(String::new());
    let (output, set_output) = create_signal(OutputData::default());
    let (is_running, set_is_running) = create_signal(false);
    let (show_visualization, set_show_visualization) = create_signal(true);
    
    // Auto-save to state
    create_effect(move |_| {
        state.update(|s| s.current_code = code.get());
    });
    
    let run_code = move |_| {
        set_is_running.set(true);
        let code_text = code.get();
        
        // Simulate running code
        spawn_local(async move {
            // In reality, this would call WASM modules
            leptos::set_timeout(
                move || {
                    let result = analyze_code(&code_text);
                    set_output.set(result);
                    set_is_running.set(false);
                    
                    // Add to history
                    state.update(|s| {
                        s.add_to_history(code_text.clone(), format!("{:?}", result));
                    });
                },
                std::time::Duration::from_millis(300)
            );
        });
    };
    
    view! {
        <div class="lab-page">
            <div class="lab-header">
                <h1>"SCTT Laboratory"</h1>
                <div class="lab-toolbar">
                    <ExampleSelector set_code=set_code/>
                    <button 
                        class="btn btn-run"
                        on:click=run_code
                        disabled=is_running
                    >
                        {move || if is_running.get() {
                            view! { <span class="spinner">"⟳"</span> " Running..." }
                        } else {
                            view! { <span>"▶"</span> " Run (⌘Enter)" }
                        }}
                    </button>
                    <button 
                        class="btn btn-icon"
                        on:click=move |_| set_show_visualization.update(|v| *v = !*v)
                        title="Toggle visualization"
                    >
                        "📊"
                    </button>
                </div>
            </div>
            
            <div class="lab-workspace">
                <div class="lab-editor">
                    <Editor code=code set_code=set_code is_running=is_running/>
                    <StatusBar output=output/>
                </div>
                
                <div class="lab-output">
                    <OutputPanel output=output/>
                    
                    <Show when=show_visualization>
                        <VisualizationPanel output=output/>
                    </Show>
                </div>
            </div>
            
            <div class="lab-sidebar">
                <HistoryPanel/>
                <SnippetsPanel/>
            </div>
        </div>
    }
}

#[component]
fn Editor(
    code: ReadSignal<String>,
    set_code: WriteSignal<String>,
    is_running: ReadSignal<bool>,
) -> impl IntoView {
    let textarea_ref = create_node_ref::<Textarea>();
    
    // Syntax highlighting would go here
    let highlighted_code = move || {
        let text = code.get();
        // Simple highlighting for demo
        text.replace("λ", "<span class='lambda'>λ</span>")
            .replace("C∞", "<span class='type'>C∞</span>")
            .replace("Path", "<span class='type'>Path</span>")
    };
    
    view! {
        <div class="editor-container">
            <div class="editor-gutter">
                {(1..=30).map(|n| view! { 
                    <div class="line-number">{n}</div> 
                }).collect_view()}
            </div>
            
            <div class="editor-content">
                <textarea
                    ref=textarea_ref
                    class="code-editor"
                    placeholder="// Enter SCTT code here\n// Try: λx. sin(x²)"
                    on:input=move |e| set_code.set(event_target_value(&e))
                    prop:value=code
                    disabled=is_running
                    spellcheck="false"
                />
                
                <div class="editor-overlay" inner_html=highlighted_code/>
            </div>
            
            <AutoComplete code=code set_code=set_code/>
        </div>
    }
}

#[component]
fn AutoComplete(
    code: ReadSignal<String>,
    set_code: WriteSignal<String>,
) -> impl IntoView {
    let (suggestions, set_suggestions) = create_signal(Vec::<String>::new());
    let (selected, set_selected) = create_signal(0);
    
    // Update suggestions based on current text
    create_effect(move |_| {
        let text = code.get();
        if let Some(last_word) = text.split_whitespace().last() {
            let suggs = get_suggestions(last_word);
            set_suggestions.set(suggs);
            set_selected.set(0);
        }
    });
    
    view! {
        <Show when=move || !suggestions.get().is_empty()>
            <div class="autocomplete-popup">
                <For
                    each=move || suggestions.get().into_iter().enumerate()
                    key=|(i, _)| *i
                    children=move |(i, suggestion)| {
                        let is_selected = move || selected.get() == i;
                        view! {
                            <div 
                                class=move || if is_selected() { 
                                    "suggestion selected" 
                                } else { 
                                    "suggestion" 
                                }
                            >
                                {suggestion}
                            </div>
                        }
                    }
                />
            </div>
        </Show>
    }
}

#[component]
fn ExampleSelector(set_code: WriteSignal<String>) -> impl IntoView {
    let examples = vec![
        ("smooth", "λx. sin(x²)", "Smooth function"),
        ("path", "⟨t⟩ π * t * (3 - 2*t)", "Path from 0 to π"),
        ("composition", "f ∘ g where f = sin, g = λx. x²", "Function composition"),
        ("derivative", "∂(λx. x³ + 2x)", "Compute derivative"),
    ];
    
    view! {
        <select 
            class="example-selector"
            on:change=move |e| {
                let value = event_target_value(&e);
                if let Some((_, code, _)) = examples.iter().find(|(id, _, _)| id == &value.as_str()) {
                    set_code.set(code.to_string());
                }
            }
        >
            <option value="">"Load example..."</option>
            {examples.iter().map(|(id, _, name)| {
                view! {
                    <option value=id>{name}</option>
                }
            }).collect_view()}
        </select>
    }
}

#[component]
fn StatusBar(output: ReadSignal<OutputData>) -> impl IntoView {
    view! {
        <div class="status-bar">
            <div class="status-item">
                <span class="status-icon">
                    {move || match output.get().status {
                        Status::Success => "✓",
                        Status::Error => "✗",
                        Status::Warning => "⚠",
                        Status::Idle => "○",
                    }}
                </span>
                <span class="status-text">
                    {move || output.get().status_text}
                </span>
            </div>
            
            <div class="status-item">
                <span class="status-label">"Type:"</span>
                <span class="status-value">
                    {move || output.get().type_info.clone().unwrap_or_else(|| "—".to_string())}
                </span>
            </div>
            
            <div class="status-item">
                <span class="status-label">"Time:"</span>
                <span class="status-value">
                    {move || format!("{}ms", output.get().execution_time)}
                </span>
            </div>
        </div>
    }
}

#[component]
fn OutputPanel(output: ReadSignal<OutputData>) -> impl IntoView {
    view! {
        <div class="output-panel">
            <div class="output-header">
                <h3>"Output"</h3>
                <button class="btn-clear">"Clear"</button>
            </div>
            
            <div class="output-content">
                <Show
                    when=move || !output.get().messages.is_empty()
                    fallback=|| view! { 
                        <div class="output-empty">
                            "No output yet. Run some code to see results."
                        </div> 
                    }
                >
                    <For
                        each=move || output.get().messages.clone()
                        key=|msg| msg.clone()
                        children=move |msg| {
                            view! {
                                <div class="output-message">
                                    <pre>{msg}</pre>
                                </div>
                            }
                        }
                    />
                </Show>
            </div>
        </div>
    }
}

#[component]
fn VisualizationPanel(output: ReadSignal<OutputData>) -> impl IntoView {
    view! {
        <div class="visualization-panel">
            <div class="viz-header">
                <h3>"Visualization"</h3>
                <select class="viz-type">
                    <option>"2D Plot"</option>
                    <option>"3D Surface"</option>
                    <option>"Path Animation"</option>
                </select>
            </div>
            
            <canvas id="viz-canvas" class="viz-canvas"/>
            
            <div class="viz-controls">
                <button class="btn-icon" title="Zoom in">"🔍+"</button>
                <button class="btn-icon" title="Zoom out">"🔍-"</button>
                <button class="btn-icon" title="Reset view">"🔄"</button>
                <button class="btn-icon" title="Export">"💾"</button>
            </div>
        </div>
    }
}

#[component]
fn HistoryPanel() -> impl IntoView {
    let state = use_context::<RwSignal<AppState>>().unwrap();
    
    view! {
        <div class="history-panel">
            <h3>"History"</h3>
            <div class="history-list">
                <For
                    each=move || state.get().history.clone()
                    key=|h| h.timestamp.clone()
                    children=move |entry| {
                        view! {
                            <div class="history-entry">
                                <code>{entry.code}</code>
                                <span class="history-time">{entry.timestamp}</span>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component]
fn SnippetsPanel() -> impl IntoView {
    let state = use_context::<RwSignal<AppState>>().unwrap();
    
    view! {
        <div class="snippets-panel">
            <h3>"Saved Snippets"</h3>
            <button class="btn btn-small">"+ Save Current"</button>
            
            <div class="snippets-list">
                <For
                    each=move || state.get().saved_snippets.clone()
                    key=|s| s.id.clone()
                    children=move |snippet| {
                        view! {
                            <div class="snippet-card">
                                <h4>{snippet.name}</h4>
                                <p>{snippet.description}</p>
                                <div class="snippet-tags">
                                    {snippet.tags.iter().map(|tag| {
                                        view! { <span class="tag">{tag}</span> }
                                    }).collect_view()}
                                </div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

// Helper types and functions
#[derive(Debug, Clone, Default)]
struct OutputData {
    status: Status,
    status_text: String,
    messages: Vec<String>,
    type_info: Option<String>,
    execution_time: u32,
}

#[derive(Debug, Clone, Default)]
enum Status {
    Success,
    Error,
    Warning,
    #[default]
    Idle,
}

fn analyze_code(code: &str) -> OutputData {
    // Mock analysis - would call real WASM modules
    if code.contains("sin") {
        OutputData {
            status: Status::Success,
            status_text: "Type check passed".to_string(),
            messages: vec!["✓ Valid smooth function".to_string()],
            type_info: Some("C∞(ℝ, ℝ)".to_string()),
            execution_time: 42,
        }
    } else {
        OutputData {
            status: Status::Success,
            status_text: "Ready".to_string(),
            messages: vec![],
            type_info: None,
            execution_time: 10,
        }
    }
}

fn get_suggestions(prefix: &str) -> Vec<String> {
    let all = vec![
        "lambda", "Path", "sin", "cos", "exp", "derivative", 
        "smooth", "interval", "composition"
    ];
    all.into_iter()
        .filter(|s| s.starts_with(prefix))
        .map(String::from)
        .collect()
}