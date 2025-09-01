use leptos::*;

#[component]
pub fn TheoryPage() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"Core Theory"</h1>
            
            <section class="theory-section">
                <h2>"Smooth Structure"</h2>
                <div class="math-block">
                    <p>"Every smooth function has a well-defined derivative:"</p>
                    <pre>"D : C∞(M, N) → C∞(M, TN)"</pre>
                </div>
                <p>
                    "In SCTT, smooth functions are first-class citizens. We can compute "
                    "derivatives symbolically and verify smoothness properties at the type level."
                </p>
            </section>

            <section class="theory-section">
                <h2>"Cubical Operations"</h2>
                <div class="math-block">
                    <p>"Composition operations respect both smooth and cubical structure:"</p>
                    <pre>"comp : (A : I → 𝒰) → (φ : 𝔽) → Partial φ A → A(0) → A(1)"</pre>
                </div>
                <p>
                    "Paths in SCTT are continuous functions from the interval [0,1]. "
                    "We use De Morgan algebra for interval operations, ensuring computational behavior."
                </p>
            </section>

            <section class="theory-section">
                <h2>"The Coherence Theorem"</h2>
                <div class="math-block">
                    <pre>"smooth(f ∘ g) = smooth(f) ∘ smooth(g)"</pre>
                </div>
                <p>
                    "The key innovation of SCTT: all operations preserve smoothness coherently. "
                    "This means differentiation commutes with path operations, enabling new forms "
                    "of verified mathematical computation."
                </p>
            </section>

            <section class="theory-section">
                <h2>"Type System Features"</h2>
                <ul>
                    <li>"<strong>Dependent Types:</strong> Types can depend on values"</li>
                    <li>"<strong>Universe Hierarchy:</strong> Type : Type₁ : Type₂ : ..."</li>
                    <li>"<strong>Smooth Modality:</strong> Distinguish smooth from discrete"</li>
                    <li>"<strong>Path Types:</strong> Equality as paths in space"</li>
                    <li>"<strong>Interval Type:</strong> I = [0,1] with De Morgan algebra"</li>
                </ul>
            </section>

            <section class="theory-section">
                <h2>"Applications"</h2>
                <div class="applications-grid">
                    <div class="application-card">
                        <h3>"Machine Learning"</h3>
                        <p>"Verified gradients and guaranteed convergence properties"</p>
                    </div>
                    <div class="application-card">
                        <h3>"Physics Simulation"</h3>
                        <p>"Type-safe differential equations and conservation laws"</p>
                    </div>
                    <div class="application-card">
                        <h3>"Computer Graphics"</h3>
                        <p>"Smooth interpolation with mathematical guarantees"</p>
                    </div>
                    <div class="application-card">
                        <h3>"Formal Verification"</h3>
                        <p>"Prove properties about continuous systems"</p>
                    </div>
                </div>
            </section>
        </div>
    }
}