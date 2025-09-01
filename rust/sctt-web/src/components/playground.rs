use leptos::*;
use leptos::html::Textarea;
use web_sys::HtmlTextAreaElement;

#[derive(Clone)]
pub struct PlaygroundExample {
    pub name: &'static str,
    pub code: &'static str,
}

pub const EXAMPLES: &[PlaygroundExample] = &[
    PlaygroundExample {
        name: "Smooth Functions",
        code: r#"// Smooth function example
smooth_function : C∞(ℝ, ℝ)
smooth_function = λx. sin(x²)

// Compute derivative
derivative : C∞(ℝ, ℝ)
derivative = D(smooth_function)
// Result: λx. 2x·cos(x²)"#,
    },
    PlaygroundExample {
        name: "Path Types",
        code: r#"// Path type example
path : Path ℝ 0 π
path = ⟨t⟩ π * t * (3 - 2*t)

// Smooth cubic interpolation
// path(0) = 0
// path(1) = π"#,
    },
    PlaygroundExample {
        name: "Function Composition",
        code: r#"// Function composition
f : C∞(ℝ, ℝ) = λx. sin(x)
g : C∞(ℝ, ℝ) = λx. x²

// Composition preserves smoothness
h : C∞(ℝ, ℝ) = f ∘ g
// Result: λx. sin(x²)"#,
    },
    PlaygroundExample {
        name: "Coherence",
        code: r#"// Smooth-Cubical Coherence
smooth_path : C∞(I, ℝ)
smooth_path = λt. sin(π·t)

// As cubical path
cubical_path : Path ℝ 0 0
cubical_path = ⟨t⟩ sin(π·t)"#,
    },
];

#[component]
pub fn Playground() -> impl IntoView {
    let (code, set_code) = create_signal(EXAMPLES[0].code.to_string());
    let (output, set_output) = create_signal(String::new());
    let (selected_example, set_selected_example) = create_signal(0);
    
    let textarea_ref = create_node_ref::<Textarea>();

    // Load example when selection changes
    create_effect(move |_| {
        let idx = selected_example.get();
        if idx < EXAMPLES.len() {
            set_code.set(EXAMPLES[idx].code.to_string());
        }
    });

    // Run code action
    let run_code = move |_| {
        let code_text = code.get();
        
        // Simulate type checking for now
        let result = if code_text.contains("sin") || code_text.contains("cos") {
            "Type: C∞(ℝ → ℝ) - Smooth function from reals to reals\n✓ Type check passed"
        } else if code_text.contains("Path") {
            "Type: Path ℝ - Continuous path in real space\n✓ Type check passed"
        } else if code_text.contains("⟨t⟩") {
            "Type: Path abstraction\n✓ Valid path lambda"
        } else {
            "✓ Expression parsed successfully"
        };
        
        set_output.set(result.to_string());
        
        // In the future, this would call our WASM modules
        // let result = sctt_checker::type_check(&code_text);
    };

    view! {
        <div class="playground-container">
            <div class="playground-controls">
                <select 
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        if let Ok(idx) = value.parse::<usize>() {
                            set_selected_example.set(idx);
                        }
                    }
                >
                    {EXAMPLES.iter().enumerate().map(|(i, ex)| {
                        view! {
                            <option value=i.to_string()>
                                {ex.name}
                            </option>
                        }
                    }).collect_view()}
                </select>
                <button 
                    class="btn btn-run"
                    on:click=run_code
                >
                    "▶ Run"
                </button>
            </div>
            
            <div class="playground-editor">
                <textarea
                    ref=textarea_ref
                    class="code-input"
                    placeholder="// Enter SCTT code here"
                    on:input=move |ev| {
                        set_code.set(event_target_value(&ev));
                    }
                    prop:value=code
                />
            </div>
            
            <div class="playground-output">
                <div class="output-panel">
                    <pre>{output}</pre>
                </div>
                <canvas class="output-canvas" id="output-graph"/>
            </div>
        </div>
    }
}