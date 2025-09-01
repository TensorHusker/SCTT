use leptos::*;

#[component]
pub fn ReferencePage() -> impl IntoView {
    let (search_query, set_search_query) = create_signal(String::new());
    let (selected_category, set_selected_category) = create_signal("all".to_string());
    
    let filtered_items = move || {
        let query = search_query.get().to_lowercase();
        let category = selected_category.get();
        
        get_reference_items()
            .into_iter()
            .filter(|item| {
                (category == "all" || item.category == category) &&
                (query.is_empty() || 
                 item.name.to_lowercase().contains(&query) ||
                 item.description.to_lowercase().contains(&query))
            })
            .collect::<Vec<_>>()
    };
    
    view! {
        <div class="reference-page">
            <header class="reference-header">
                <h1>"SCTT Reference"</h1>
                <p>"Complete documentation for Smooth Cubical Type Theory"</p>
                
                <div class="reference-controls">
                    <input
                        type="search"
                        class="reference-search"
                        placeholder="Search reference..."
                        on:input=move |e| set_search_query.set(event_target_value(&e))
                    />
                    
                    <div class="category-filters">
                        <button 
                            class=move || if selected_category.get() == "all" { "filter active" } else { "filter" }
                            on:click=move |_| set_selected_category.set("all".to_string())
                        >
                            "All"
                        </button>
                        <button 
                            class=move || if selected_category.get() == "types" { "filter active" } else { "filter" }
                            on:click=move |_| set_selected_category.set("types".to_string())
                        >
                            "Types"
                        </button>
                        <button 
                            class=move || if selected_category.get() == "operators" { "filter active" } else { "filter" }
                            on:click=move |_| set_selected_category.set("operators".to_string())
                        >
                            "Operators"
                        </button>
                        <button 
                            class=move || if selected_category.get() == "functions" { "filter active" } else { "filter" }
                            on:click=move |_| set_selected_category.set("functions".to_string())
                        >
                            "Functions"
                        </button>
                    </div>
                </div>
            </header>
            
            <div class="reference-content">
                <div class="reference-grid">
                    <For
                        each=filtered_items
                        key=|item| item.name
                        children=move |item| {
                            view! {
                                <ReferenceCard item=item/>
                            }
                        }
                    />
                </div>
            </div>
            
            <footer class="reference-footer">
                <p>"Can't find what you're looking for? "</p>
                <a href="https://github.com/sctt/docs" class="link">
                    "Contribute to documentation →"
                </a>
            </footer>
        </div>
    }
}

#[component]
fn ReferenceCard(item: ReferenceItem) -> impl IntoView {
    let (expanded, set_expanded) = create_signal(false);
    
    view! {
        <article class="reference-card">
            <header 
                class="reference-card-header"
                on:click=move |_| set_expanded.update(|e| *e = !*e)
            >
                <div class="reference-title">
                    <code class="reference-name">{&item.name}</code>
                    <span class="reference-type">{&item.type_signature}</span>
                </div>
                <span class="reference-category-badge">{&item.category}</span>
            </header>
            
            <div class="reference-summary">
                <p>{&item.description}</p>
            </div>
            
            <Show when=expanded>
                <div class="reference-details">
                    <div class="reference-example">
                        <h4>"Example"</h4>
                        <pre><code>{&item.example}</code></pre>
                    </div>
                    
                    <div class="reference-properties">
                        <h4>"Properties"</h4>
                        <ul>
                            {item.properties.iter().map(|prop| {
                                view! { <li>{prop}</li> }
                            }).collect_view()}
                        </ul>
                    </div>
                    
                    <div class="reference-related">
                        <h4>"See Also"</h4>
                        <div class="related-links">
                            {item.related.iter().map(|link| {
                                view! { 
                                    <a href=format!("#{}",link) class="related-link">
                                        {link}
                                    </a> 
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </div>
            </Show>
        </article>
    }
}

#[derive(Clone)]
struct ReferenceItem {
    name: &'static str,
    category: &'static str,
    type_signature: &'static str,
    description: &'static str,
    example: &'static str,
    properties: Vec<&'static str>,
    related: Vec<&'static str>,
}

fn get_reference_items() -> Vec<ReferenceItem> {
    vec![
        ReferenceItem {
            name: "C∞",
            category: "types",
            type_signature: "(A : Type) → (B : Type) → Type",
            description: "The type of smooth functions from A to B with derivatives of all orders",
            example: "f : C∞(ℝ, ℝ) = λx. sin(x²)",
            properties: vec![
                "All derivatives exist",
                "Closed under composition",
                "Forms a category",
            ],
            related: vec!["∂", "compose", "ℝ"],
        },
        ReferenceItem {
            name: "Path",
            category: "types",
            type_signature: "(A : Type) → A → A → Type",
            description: "A continuous path in type A from one point to another",
            example: "p : Path ℝ 0 π = ⟨t⟩ π * t",
            properties: vec![
                "p(0) = start point",
                "p(1) = end point",
                "Continuous transformation",
            ],
            related: vec!["⟨⟩", "I", "transport"],
        },
        ReferenceItem {
            name: "∂",
            category: "operators",
            type_signature: "C∞(A, B) → C∞(A, TB)",
            description: "Differentiation operator for smooth functions",
            example: "∂(λx. x³) = λx. 3*x²",
            properties: vec![
                "Linear operator",
                "Leibniz rule holds",
                "Chain rule built-in",
            ],
            related: vec!["C∞", "∇", "D"],
        },
        ReferenceItem {
            name: "∘",
            category: "operators",
            type_signature: "C∞(B, C) → C∞(A, B) → C∞(A, C)",
            description: "Function composition that preserves smoothness",
            example: "sin ∘ (λx. x²) = λx. sin(x²)",
            properties: vec![
                "Associative",
                "Preserves smoothness",
                "Identity is λx. x",
            ],
            related: vec!["C∞", "id", "chain rule"],
        },
        ReferenceItem {
            name: "I",
            category: "types",
            type_signature: "Type",
            description: "The interval type [0,1] with De Morgan algebra structure",
            example: "i : I, i ∧ j, i ∨ j, 1 - i",
            properties: vec![
                "0 and 1 are endpoints",
                "De Morgan laws hold",
                "Used for path parameters",
            ],
            related: vec!["Path", "⟨⟩", "interval algebra"],
        },
        ReferenceItem {
            name: "ℝ",
            category: "types",
            type_signature: "Type",
            description: "The type of real numbers with smooth structure",
            example: "x : ℝ = 3.14159",
            properties: vec![
                "Complete ordered field",
                "Smooth manifold structure",
                "Standard topology",
            ],
            related: vec!["C∞", "ℂ", "ℚ"],
        },
        ReferenceItem {
            name: "transport",
            category: "functions",
            type_signature: "Path Type A B → A → B",
            description: "Transport a value along a path (coercion along equality)",
            example: "transport (⟨t⟩ Vec ℝ (2+t)) [1, 2]",
            properties: vec![
                "Preserves structure",
                "Computational content",
                "Respects composition",
            ],
            related: vec!["Path", "ap", "subst"],
        },
        ReferenceItem {
            name: "λ",
            category: "operators",
            type_signature: "(x : A) → B(x)",
            description: "Lambda abstraction for creating functions",
            example: "λx. λy. x² + y²",
            properties: vec![
                "Variable binding",
                "Can be nested",
                "β-reduction: (λx.e) a = e[x:=a]",
            ],
            related: vec!["→", "application", "η-expansion"],
        },
    ]
}