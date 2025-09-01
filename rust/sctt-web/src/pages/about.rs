use leptos::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"About SCTT"</h1>
            
            <section>
                <h2>"What is Smooth Cubical Type Theory?"</h2>
                <p>
                    "SCTT is a new type theory that combines the computational power of "
                    "cubical type theory with the mathematical elegance of smooth mathematics. "
                    "It allows us to reason about continuous functions, derivatives, and paths "
                    "in a unified, type-safe framework."
                </p>
            </section>

            <section>
                <h2>"Why Build SCTT?"</h2>
                <p>
                    "Modern mathematics and computer science increasingly need to reason about "
                    "continuous and smooth structures. From machine learning to physics simulations, "
                    "we need type systems that can express and verify properties of smooth functions. "
                    "SCTT bridges this gap."
                </p>
            </section>

            <section>
                <h2>"Key Innovations"</h2>
                <ul>
                    <li>"<strong>Native Smoothness:</strong> First type theory with built-in smooth structure"</li>
                    <li>"<strong>Computational Paths:</strong> Paths are computable, not just abstract"</li>
                    <li>"<strong>Coherence:</strong> Smooth and cubical operations work together seamlessly"</li>
                    <li>"<strong>Practical:</strong> Runs in the browser via WebAssembly"</li>
                </ul>
            </section>

            <section>
                <h2>"Technology Stack"</h2>
                <p>"SCTT is built entirely in Rust, leveraging:"</p>
                <ul>
                    <li>"<strong>Rust:</strong> For safety and performance"</li>
                    <li>"<strong>WebAssembly:</strong> For browser execution"</li>
                    <li>"<strong>Leptos:</strong> For reactive UI without JavaScript"</li>
                    <li>"<strong>Actix-Web:</strong> For serving the application"</li>
                </ul>
                <p>
                    "No JavaScript frameworks, no npm dependencies—just pure Rust compiled to WASM."
                </p>
            </section>

            <section>
                <h2>"Get Involved"</h2>
                <p>
                    "SCTT is an open-source research project. We welcome contributions from "
                    "mathematicians, type theorists, and developers interested in pushing the "
                    "boundaries of formal mathematics."
                </p>
                <div class="cta-buttons">
                    <a href="https://github.com/yourusername/SCTT" class="btn btn-primary">
                        "View on GitHub"
                    </a>
                    <a href="/docs" class="btn btn-secondary">
                        "Read Documentation"
                    </a>
                </div>
            </section>
        </div>
    }
}