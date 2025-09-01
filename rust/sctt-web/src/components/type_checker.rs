use leptos::*;

#[component]
pub fn TypeChecker() -> impl IntoView {
    let (input, set_input) = create_signal(String::new());
    let (type_result, set_type_result) = create_signal(String::new());
    
    let check_type = move |_| {
        let expr = input.get();
        
        // Eventually this will call into our SCTT type checker WASM module
        // For now, simple pattern matching
        let result = if expr.contains("sin") || expr.contains("cos") {
            format!("C∞(ℝ, ℝ)")
        } else if expr.contains("Path") {
            format!("Path Type")
        } else if expr.contains("λ") {
            format!("Function Type")
        } else if expr == "0" || expr == "1" {
            format!("I (Interval)")
        } else if expr.parse::<f64>().is_ok() {
            format!("ℝ (Real)")
        } else {
            format!("Unknown")
        };
        
        set_type_result.set(result);
    };

    view! {
        <div class="type-checker">
            <h3>"Type Checker"</h3>
            <div class="type-checker-input">
                <input
                    type="text"
                    placeholder="Enter expression to type check"
                    on:input=move |ev| set_input.set(event_target_value(&ev))
                    prop:value=input
                />
                <button on:click=check_type>"Check"</button>
            </div>
            <div class="type-result">
                <Show
                    when=move || !type_result.get().is_empty()
                    fallback=|| view! { <span class="muted">"No type result yet"</span> }
                >
                    <strong>"Type: "</strong> {type_result}
                </Show>
            </div>
        </div>
    }
}