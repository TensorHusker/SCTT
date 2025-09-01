use leptos::*;
use crate::components::{Playground as PlaygroundComponent, TypeChecker, SmoothVisualizer, PathViewer};

#[component]
pub fn PlaygroundPage() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"SCTT Playground"</h1>
            <p>"Experiment with Smooth Cubical Type Theory in your browser"</p>
            
            <PlaygroundComponent/>
            
            <div class="tools-grid">
                <TypeChecker/>
                <PathViewer/>
            </div>
            
            <SmoothVisualizer/>
        </div>
    }
}