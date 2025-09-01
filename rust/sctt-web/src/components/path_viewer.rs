use leptos::*;

#[component]
pub fn PathViewer() -> impl IntoView {
    let (t_value, set_t_value) = create_signal(0.5_f64);
    let (path_expr, set_path_expr) = create_signal("π * t * (3 - 2*t)".to_string());
    
    // Evaluate path at parameter t
    let path_value = move || {
        let t = t_value.get();
        let expr = path_expr.get();
        
        // Simple evaluation for demo
        if expr.contains("π * t * (3 - 2*t)") {
            std::f64::consts::PI * t * (3.0 - 2.0 * t)
        } else if expr.contains("sin") {
            (std::f64::consts::PI * t).sin()
        } else if expr.contains("t²") {
            t * t
        } else {
            t
        }
    };
    
    // Check boundary conditions
    let boundary_check = move || {
        let expr = path_expr.get();
        
        // Evaluate at t=0 and t=1
        let at_0 = if expr.contains("π * t * (3 - 2*t)") {
            0.0
        } else if expr.contains("sin") {
            0.0
        } else if expr.contains("t²") {
            0.0
        } else {
            0.0
        };
        
        let at_1 = if expr.contains("π * t * (3 - 2*t)") {
            std::f64::consts::PI
        } else if expr.contains("sin") {
            0.0
        } else if expr.contains("t²") {
            1.0
        } else {
            1.0
        };
        
        format!("path(0) = {:.3}, path(1) = {:.3}", at_0, at_1)
    };

    view! {
        <div class="path-viewer">
            <h3>"Path Explorer"</h3>
            
            <div class="path-input">
                <label>"Path expression: "</label>
                <input
                    type="text"
                    value=path_expr
                    on:input=move |ev| set_path_expr.set(event_target_value(&ev))
                />
            </div>
            
            <div class="parameter-slider">
                <label>"t = " {move || format!("{:.2}", t_value.get())}</label>
                <input
                    type="range"
                    min="0"
                    max="1"
                    step="0.01"
                    value=move || t_value.get().to_string()
                    on:input=move |ev| {
                        if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                            set_t_value.set(v);
                        }
                    }
                />
            </div>
            
            <div class="path-output">
                <div class="value">
                    <strong>"path(" {move || format!("{:.2}", t_value.get())} ") = "</strong>
                    {move || format!("{:.4}", path_value())}
                </div>
                <div class="boundaries">
                    <strong>"Boundaries: "</strong>
                    {boundary_check}
                </div>
            </div>
        </div>
    }
}