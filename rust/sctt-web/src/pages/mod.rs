pub mod landing;
pub mod lab;
pub mod learn;
pub mod reference;

pub use landing::LandingPage;
pub use lab::LabPage;
pub use learn::LearnPage;
pub use reference::ReferencePage;

use leptos::*;
use leptos_router::*;

// Example page for specific examples
#[component]
pub fn ExamplePage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.get().get("id").cloned().unwrap_or_default();
    
    view! {
        <div class="example-page">
            <h1>"Example: " {id}</h1>
            // Load specific example based on ID
        </div>
    }
}