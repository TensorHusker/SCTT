use leptos::*;
use leptos_router::*;

#[component]
pub fn LandingPage() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    
    view! {
        <div class="landing-page">
            <section class="hero-section">
                <div class="hero-background">
                    <canvas id="math-canvas" class="animated-background"/>
                </div>
                
                <div class="hero-content">
                    <h1 class="hero-title">
                        <span class="gradient-text">"Smooth Cubical"</span>
                        <br/>
                        <span class="type-text">"Type Theory"</span>
                    </h1>
                    
                    <p class="hero-tagline">
                        "Where calculus meets computation. "
                        "Build verified mathematical software with confidence."
                    </p>
                    
                    <div class="hero-actions">
                        <A href="/lab" class="btn btn-primary btn-large">
                            <span class="btn-icon">"🧪"</span>
                            "Open Laboratory"
                            <span class="btn-hint">"Start experimenting"</span>
                        </A>
                        
                        <A href="/learn" class="btn btn-secondary btn-large">
                            <span class="btn-icon">"📚"</span>
                            "Interactive Tutorial"
                            <span class="btn-hint">"5 min introduction"</span>
                        </A>
                    </div>
                    
                    <div class="quick-example">
                        <pre class="code-preview">
                            <code>
"// Type-safe derivatives
f : C∞(ℝ, ℝ) = λx. sin(x²)
∂f : C∞(ℝ, ℝ) = λx. 2x·cos(x²)  ✓ Verified"
                            </code>
                        </pre>
                    </div>
                </div>
            </section>
            
            <section class="features-section">
                <div class="container">
                    <h2>"Why SCTT?"</h2>
                    
                    <div class="feature-grid">
                        <FeatureCard
                            icon="∂"
                            title="Native Calculus"
                            description="Derivatives, integrals, and limits as first-class citizens"
                            link="/learn#calculus"
                        />
                        
                        <FeatureCard
                            icon="□"
                            title="Computational Paths"
                            description="Equality proofs that compute, not just exist"
                            link="/learn#paths"
                        />
                        
                        <FeatureCard
                            icon="✓"
                            title="Verified Correctness"
                            description="Mathematical guarantees, not just tests"
                            link="/learn#verification"
                        />
                        
                        <FeatureCard
                            icon="⚡"
                            title="Interactive Development"
                            description="Real-time feedback as you type"
                            link="/lab"
                        />
                    </div>
                </div>
            </section>
            
            <section class="demo-section">
                <div class="container">
                    <h2>"See It In Action"</h2>
                    
                    <div class="demo-tabs">
                        <DemoTab 
                            title="Machine Learning"
                            code="// Verified gradient descent\noptimize : (f: C∞(ℝⁿ, ℝ)) → ℝⁿ\noptimize f = fixpoint (λx. x - α·∇f(x))"
                        />
                        
                        <DemoTab 
                            title="Physics"
                            code="// Conservation laws\nenergy : System → ℝ\nproof : ∀t. ∂(energy ∘ evolve)(t) = 0"
                        />
                        
                        <DemoTab 
                            title="Graphics"
                            code="// Smooth interpolation\nspline : Path ℝ³ start end\nspline = ⟨t⟩ bezier(start, c1, c2, end)(t)"
                        />
                    </div>
                </div>
            </section>
            
            <section class="cta-section">
                <div class="container">
                    <div class="cta-card">
                        <h2>"Ready to revolutionize your mathematics?"</h2>
                        <p>"Join researchers worldwide developing the future of verified computation"</p>
                        
                        <div class="cta-actions">
                            <A href="/lab" class="btn btn-primary">
                                "Start Building"
                            </A>
                            
                            <a href="https://github.com/sctt" class="btn btn-outline">
                                "View on GitHub"
                            </a>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}

#[component]
fn FeatureCard(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    link: &'static str,
) -> impl IntoView {
    view! {
        <A href=link class="feature-card">
            <div class="feature-icon">{icon}</div>
            <h3>{title}</h3>
            <p>{description}</p>
            <span class="feature-arrow">"→"</span>
        </A>
    }
}

#[component]
fn DemoTab(title: &'static str, code: &'static str) -> impl IntoView {
    view! {
        <div class="demo-tab">
            <h4>{title}</h4>
            <pre><code>{code}</code></pre>
            <A href="/lab" class="demo-try">"Try this example →"</A>
        </div>
    }
}