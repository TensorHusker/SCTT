use leptos::*;
use leptos_router::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="hero">
            <div class="hero-content">
                <h1 class="hero-title">"Smooth Cubical Type Theory"</h1>
                <p class="hero-subtitle">
                    "Building a revolutionary type theory where calculus meets computation"
                </p>
                <div class="hero-buttons">
                    <A href="/playground" class="btn btn-primary">"Try Interactive Demo"</A>
                    <A href="/blog" class="btn btn-secondary">"Read Latest Post"</A>
                </div>
            </div>
        </div>

        <section class="container">
            <div class="intro-grid">
                <div class="intro-card">
                    <div class="card-icon">"∂"</div>
                    <h3>"Smooth Mathematics"</h3>
                    <p>"Native support for derivatives, integrals, and differential geometry"</p>
                </div>
                <div class="intro-card">
                    <div class="card-icon">"□"</div>
                    <h3>"Cubical Structure"</h3>
                    <p>"Computational interpretation of paths, homotopies, and higher dimensions"</p>
                </div>
                <div class="intro-card">
                    <div class="card-icon">"⚡"</div>
                    <h3>"Pure Rust + WASM"</h3>
                    <p>"Run SCTT directly in your browser with WebAssembly"</p>
                </div>
            </div>
        </section>

        <section class="container">
            <h2>"Quick Start"</h2>
            <div class="quick-start">
                <p>"Try these SCTT expressions in the " <A href="/playground">"playground"</A> ":"</p>
                <pre class="code-block">
"// Define a smooth function
smooth_function : C∞(ℝ, ℝ)
smooth_function = λx. sin(x²)

// Compute its derivative automatically
D(smooth_function) = λx. 2x·cos(x²)

// Define a path
path : Path ℝ 0 π
path = ⟨t⟩ π * t * (3 - 2*t)"
                </pre>
            </div>
        </section>

        <section class="container">
            <h2>"Recent Development"</h2>
            <div class="status-grid">
                <StatusCard 
                    title="Type System" 
                    status="Active" 
                    description="Core type checking with bidirectional inference"
                />
                <StatusCard 
                    title="Smooth Module" 
                    status="Active" 
                    description="Symbolic differentiation and Taylor series"
                />
                <StatusCard 
                    title="Cubical Module" 
                    status="Active" 
                    description="Path types and interval operations"
                />
                <StatusCard 
                    title="Coherence" 
                    status="Research" 
                    description="Ensuring smooth and cubical structures align"
                />
            </div>
        </section>
    }
}

#[component]
fn StatusCard(
    title: &'static str,
    status: &'static str,
    description: &'static str,
) -> impl IntoView {
    let status_class = match status {
        "Active" => "status-active",
        "Research" => "status-research",
        _ => "status-planned",
    };

    view! {
        <div class="status-card">
            <h4>{title}</h4>
            <span class=format!("status-badge {}", status_class)>{status}</span>
            <p>{description}</p>
        </div>
    }
}