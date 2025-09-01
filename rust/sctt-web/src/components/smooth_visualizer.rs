use leptos::*;
use leptos::html::Canvas;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::f64::consts::PI;

#[component]
pub fn SmoothVisualizer() -> impl IntoView {
    let canvas_ref = create_node_ref::<Canvas>();
    let (function_type, set_function_type) = create_signal("sin".to_string());
    
    // Draw function on canvas
    let draw_function = move || {
        if let Some(canvas) = canvas_ref.get() {
            let canvas_element: HtmlCanvasElement = canvas.into();
            let ctx = canvas_element
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();
            
            let width = canvas_element.width() as f64;
            let height = canvas_element.height() as f64;
            
            // Clear canvas
            ctx.clear_rect(0.0, 0.0, width, height);
            
            // Draw axes
            ctx.set_stroke_style(&"#64748b".into());
            ctx.set_line_width(1.0);
            ctx.begin_path();
            ctx.move_to(0.0, height / 2.0);
            ctx.line_to(width, height / 2.0);
            ctx.move_to(width / 2.0, 0.0);
            ctx.line_to(width / 2.0, height);
            ctx.stroke();
            
            // Draw function
            ctx.set_stroke_style(&"#6366f1".into());
            ctx.set_line_width(2.0);
            ctx.begin_path();
            
            let func = function_type.get();
            let scale = 4.0; // x range from -2π to 2π
            
            for px in 0..(width as i32) {
                let x = (px as f64 - width / 2.0) * scale * PI / width;
                let y = match func.as_str() {
                    "sin" => x.sin(),
                    "cos" => x.cos(),
                    "sin(x²)" => (x * x).sin(),
                    "exp" => x.exp().min(10.0).max(-10.0) / 10.0,
                    _ => x / 2.0,
                };
                
                let py = height / 2.0 - y * height / 4.0;
                
                if px == 0 {
                    ctx.move_to(px as f64, py);
                } else {
                    ctx.line_to(px as f64, py);
                }
            }
            
            ctx.stroke();
        }
    };
    
    // Draw when component mounts and when function changes
    create_effect(move |_| {
        function_type.get();
        request_animation_frame(move || draw_function());
    });

    view! {
        <div class="smooth-visualizer">
            <h3>"Smooth Function Visualizer"</h3>
            <div class="controls">
                <select on:change=move |ev| {
                    set_function_type.set(event_target_value(&ev));
                }>
                    <option value="sin">"sin(x)"</option>
                    <option value="cos">"cos(x)"</option>
                    <option value="sin(x²)">"sin(x²)"</option>
                    <option value="exp">"exp(x)"</option>
                </select>
            </div>
            <canvas
                ref=canvas_ref
                width="600"
                height="400"
                class="visualization-canvas"
            />
        </div>
    }
}