use leptos::*;
use leptos_router::*;
use crate::state::AppState;

#[component]
pub fn LearnPage() -> impl IntoView {
    let state = use_context::<RwSignal<AppState>>().unwrap();
    let (current_lesson, set_current_lesson) = create_signal(0);
    let (code_input, set_code_input) = create_signal(String::new());
    let (feedback, set_feedback) = create_signal(String::new());
    
    let lessons = get_lessons();
    let lesson = move || lessons.get(current_lesson.get()).cloned().unwrap_or_default();
    
    let check_answer = move |_| {
        let input = code_input.get();
        let correct = lesson().check(&input);
        
        if correct {
            set_feedback.set("✅ Correct! Well done!".to_string());
            
            // Unlock achievement for first lesson
            if current_lesson.get() == 0 {
                state.update(|s| {
                    s.unlock_achievement(
                        "first_steps".to_string(),
                        "First Steps".to_string(),
                        "Completed your first SCTT lesson".to_string()
                    );
                });
            }
            
            // Mark as completed
            state.update(|s| {
                s.session.completed_tutorials.push(lesson().id.clone());
            });
        } else {
            set_feedback.set("🤔 Not quite. Check the hint below.".to_string());
        }
    };
    
    let next_lesson = move |_| {
        if current_lesson.get() < lessons.len() - 1 {
            set_current_lesson.update(|n| *n += 1);
            set_code_input.set(String::new());
            set_feedback.set(String::new());
        }
    };
    
    let prev_lesson = move |_| {
        if current_lesson.get() > 0 {
            set_current_lesson.update(|n| *n -= 1);
            set_code_input.set(String::new());
            set_feedback.set(String::new());
        }
    };
    
    view! {
        <div class="learn-page">
            <div class="learn-header">
                <h1>"Interactive SCTT Tutorial"</h1>
                <div class="progress-bar">
                    <div 
                        class="progress-fill"
                        style=move || format!("width: {}%", (current_lesson.get() + 1) * 100 / lessons.len())
                    />
                    <span class="progress-text">
                        {move || format!("Lesson {} of {}", current_lesson.get() + 1, lessons.len())}
                    </span>
                </div>
            </div>
            
            <div class="learn-content">
                <aside class="lesson-sidebar">
                    <h3>"Lessons"</h3>
                    <nav class="lesson-nav">
                        {lessons.iter().enumerate().map(|(i, lesson)| {
                            let is_current = move || current_lesson.get() == i;
                            let is_completed = move || {
                                state.get().session.completed_tutorials.contains(&lesson.id)
                            };
                            
                            view! {
                                <button
                                    class=move || {
                                        let mut classes = "lesson-link".to_string();
                                        if is_current() { classes.push_str(" current"); }
                                        if is_completed() { classes.push_str(" completed"); }
                                        classes
                                    }
                                    on:click=move |_| {
                                        set_current_lesson.set(i);
                                        set_code_input.set(String::new());
                                        set_feedback.set(String::new());
                                    }
                                >
                                    <span class="lesson-number">{i + 1}</span>
                                    <span class="lesson-title">{lesson.title}</span>
                                    {move || if is_completed() {
                                        view! { <span class="lesson-check">"✓"</span> }
                                    } else {
                                        view! { <span/> }
                                    }}
                                </button>
                            }
                        }).collect_view()}
                    </nav>
                </aside>
                
                <main class="lesson-main">
                    <article class="lesson-content">
                        <h2>{move || lesson().title}</h2>
                        
                        <div class="lesson-explanation" inner_html=move || lesson().content/>
                        
                        <div class="lesson-example">
                            <h3>"Example"</h3>
                            <pre class="code-example">
                                <code>{move || lesson().example}</code>
                            </pre>
                        </div>
                        
                        <div class="lesson-exercise">
                            <h3>"Your Turn"</h3>
                            <p>{move || lesson().prompt}</p>
                            
                            <div class="exercise-editor">
                                <textarea
                                    class="exercise-input"
                                    placeholder="Type your answer here..."
                                    on:input=move |e| set_code_input.set(event_target_value(&e))
                                    prop:value=code_input
                                />
                                
                                <div class="exercise-actions">
                                    <button 
                                        class="btn btn-primary"
                                        on:click=check_answer
                                    >
                                        "Check Answer"
                                    </button>
                                    
                                    <button 
                                        class="btn btn-secondary"
                                        on:click=move |_| set_code_input.set(lesson().solution.clone())
                                    >
                                        "Show Solution"
                                    </button>
                                </div>
                            </div>
                            
                            <Show when=move || !feedback.get().is_empty()>
                                <div class=move || {
                                    if feedback.get().contains("✅") {
                                        "feedback success"
                                    } else {
                                        "feedback hint"
                                    }
                                }>
                                    {feedback}
                                </div>
                            </Show>
                            
                            <details class="hint-box">
                                <summary>"Need a hint?"</summary>
                                <p>{move || lesson().hint}</p>
                            </details>
                        </div>
                    </article>
                    
                    <nav class="lesson-navigation">
                        <button 
                            class="btn btn-nav"
                            on:click=prev_lesson
                            disabled=move || current_lesson.get() == 0
                        >
                            "← Previous"
                        </button>
                        
                        <button 
                            class="btn btn-nav btn-primary"
                            on:click=next_lesson
                            disabled=move || current_lesson.get() >= lessons.len() - 1
                        >
                            "Next →"
                        </button>
                    </nav>
                </main>
                
                <aside class="lesson-resources">
                    <h3>"Quick Reference"</h3>
                    
                    <div class="reference-card">
                        <h4>"Symbols"</h4>
                        <dl>
                            <dt>"λ"</dt><dd>"Lambda (function)"</dd>
                            <dt>"∂"</dt><dd>"Derivative"</dd>
                            <dt>"∞"</dt><dd>"Infinity (smooth)"</dd>
                            <dt>"⟨⟩"</dt><dd>"Path brackets"</dd>
                        </dl>
                    </div>
                    
                    <div class="reference-card">
                        <h4>"Types"</h4>
                        <dl>
                            <dt>"ℝ"</dt><dd>"Real numbers"</dd>
                            <dt>"I"</dt><dd>"Interval [0,1]"</dd>
                            <dt>"C∞"</dt><dd>"Smooth functions"</dd>
                            <dt>"Path"</dt><dd>"Path type"</dd>
                        </dl>
                    </div>
                    
                    <div class="tips-card">
                        <h4>"💡 Pro Tip"</h4>
                        <p>{move || lesson().pro_tip}</p>
                    </div>
                </aside>
            </div>
        </div>
    }
}

#[derive(Clone, Default)]
struct Lesson {
    id: String,
    title: String,
    content: String,
    example: String,
    prompt: String,
    solution: String,
    hint: String,
    pro_tip: String,
}

impl Lesson {
    fn check(&self, input: &str) -> bool {
        // Normalize and compare
        input.trim().replace(" ", "") == self.solution.replace(" ", "")
    }
}

fn get_lessons() -> Vec<Lesson> {
    vec![
        Lesson {
            id: "functions".to_string(),
            title: "Functions in SCTT".to_string(),
            content: r#"
                <p>In SCTT, functions are first-class citizens. We write them using lambda notation:</p>
                <p><code>λx. expression</code></p>
                <p>This reads as "a function that takes x and returns expression".</p>
                <p>All functions in SCTT can be smooth, meaning they have derivatives of all orders!</p>
            "#.to_string(),
            example: "λx. x² + 2*x + 1".to_string(),
            prompt: "Write a function that squares its input:".to_string(),
            solution: "λx. x²".to_string(),
            hint: "Use λx. followed by x squared (x²)".to_string(),
            pro_tip: "You can type x^2 and it will be displayed as x²".to_string(),
        },
        Lesson {
            id: "derivatives".to_string(),
            title: "Taking Derivatives".to_string(),
            content: r#"
                <p>SCTT can automatically compute derivatives of smooth functions!</p>
                <p>Use the ∂ operator (or D) to take a derivative:</p>
                <p><code>∂(function)</code></p>
                <p>The result is always another smooth function.</p>
            "#.to_string(),
            example: "∂(λx. x³) = λx. 3*x²".to_string(),
            prompt: "Take the derivative of λx. sin(x):".to_string(),
            solution: "∂(λx. sin(x))".to_string(),
            hint: "Wrap the function in ∂(...)".to_string(),
            pro_tip: "SCTT verifies derivatives are correct at the type level!".to_string(),
        },
        Lesson {
            id: "paths".to_string(),
            title: "Path Types".to_string(),
            content: r#"
                <p>Paths represent continuous transformations between values.</p>
                <p>A path from a to b is written: <code>Path A a b</code></p>
                <p>We construct paths using: <code>⟨t⟩ expression</code></p>
                <p>where t varies from 0 to 1.</p>
            "#.to_string(),
            example: "⟨t⟩ t * π".to_string(),
            prompt: "Create a path from 0 to 1 using linear interpolation:".to_string(),
            solution: "⟨t⟩ t".to_string(),
            hint: "The simplest path is just ⟨t⟩ t".to_string(),
            pro_tip: "Paths in SCTT compute! They're not just proofs.".to_string(),
        },
        Lesson {
            id: "composition".to_string(),
            title: "Function Composition".to_string(),
            content: r#"
                <p>Compose functions with the ∘ operator:</p>
                <p><code>f ∘ g</code> means "f after g"</p>
                <p>SCTT ensures smoothness is preserved through composition!</p>
            "#.to_string(),
            example: "sin ∘ (λx. x²) = λx. sin(x²)".to_string(),
            prompt: "Compose cos with the squaring function:".to_string(),
            solution: "cos ∘ (λx. x²)".to_string(),
            hint: "Put cos first, then ∘, then the square function".to_string(),
            pro_tip: "The chain rule is built into SCTT's type system!".to_string(),
        },
        Lesson {
            id: "types".to_string(),
            title: "Type Annotations".to_string(),
            content: r#"
                <p>We can annotate expressions with types using :</p>
                <p><code>expression : Type</code></p>
                <p>Common types include:</p>
                <ul>
                    <li>ℝ - real numbers</li>
                    <li>C∞(A, B) - smooth functions from A to B</li>
                    <li>Path A a b - paths in A from a to b</li>
                </ul>
            "#.to_string(),
            example: "f : C∞(ℝ, ℝ) = λx. exp(x)".to_string(),
            prompt: "Annotate a sine function with its type:".to_string(),
            solution: "sin : C∞(ℝ, ℝ)".to_string(),
            hint: "sine maps real numbers to real numbers smoothly".to_string(),
            pro_tip: "Types help SCTT optimize and verify your code!".to_string(),
        },
    ]
}