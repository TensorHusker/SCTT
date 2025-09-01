use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

mod components;
mod pages;
mod state;

use components::*;
use pages::*;
use state::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    // Global state for the app
    provide_context(create_rw_signal(AppState::default()));
    
    view! {
        <Stylesheet id="leptos" href="/pkg/sctt-web.css"/>
        <Title text="SCTT Lab - Smooth Cubical Type Theory"/>
        <Meta name="description" content="Interactive laboratory for developing Smooth Cubical Type Theory"/>
        
        // Keyboard shortcuts handler
        <KeyboardShortcuts/>
        
        <Router>
            <div class="app-container">
                <SmartNav/>
                <main class="main-content">
                    <Routes>
                        <Route path="" view=LandingPage/>
                        <Route path="/lab" view=LabPage/>
                        <Route path="/learn" view=LearnPage/>
                        <Route path="/reference" view=ReferencePage/>
                        <Route path="/examples/:id" view=ExamplePage/>
                    </Routes>
                </main>
                <HelpPanel/>
            </div>
        </Router>
    }
}

#[component]
fn SmartNav() -> impl IntoView {
    let route = use_route();
    let path = move || route.path();
    
    view! {
        <nav class="smart-nav">
            <div class="nav-brand">
                <div class="logo">
                    <span class="logo-icon">"∂∞"</span>
                    <span class="logo-text">"SCTT"</span>
                    <span class="logo-badge">"Lab"</span>
                </div>
            </div>
            
            <div class="nav-center">
                <NavLink href="/" label="Home" icon="🏠"/>
                <NavLink href="/lab" label="Laboratory" icon="🧪" kbd="⌘L"/>
                <NavLink href="/learn" label="Learn" icon="📚" kbd="⌘K"/>
                <NavLink href="/reference" label="Reference" icon="📖" kbd="⌘R"/>
            </div>
            
            <div class="nav-actions">
                <SearchBar/>
                <ThemeToggle/>
                <SaveButton/>
            </div>
        </nav>
    }
}

#[component]
fn NavLink(
    href: &'static str,
    label: &'static str,
    icon: &'static str,
    #[prop(optional)] kbd: Option<&'static str>,
) -> impl IntoView {
    let route = use_route();
    let is_active = move || route.path() == href;
    
    view! {
        <A 
            href=href
            class=move || if is_active() { "nav-link active" } else { "nav-link" }
        >
            <span class="nav-icon">{icon}</span>
            <span class="nav-label">{label}</span>
            {kbd.map(|k| view! { <kbd class="nav-kbd">{k}</kbd> })}
        </A>
    }
}

#[component]
fn SearchBar() -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let (is_open, set_is_open) = create_signal(false);
    
    view! {
        <div class="search-container">
            <button 
                class="search-trigger"
                on:click=move |_| set_is_open.set(true)
            >
                <span>"🔍"</span>
                <kbd>"⌘K"</kbd>
            </button>
            
            <Show when=is_open>
                <div class="search-modal" on:click=move |_| set_is_open.set(false)>
                    <div class="search-box" on:click=|e| e.stop_propagation()>
                        <input
                            type="text"
                            class="search-input"
                            placeholder="Search documentation, examples, or type expressions..."
                            on:input=move |e| set_query.set(event_target_value(&e))
                            autofocus
                        />
                        <SearchResults query=query/>
                    </div>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn SearchResults(query: ReadSignal<String>) -> impl IntoView {
    let results = move || {
        let q = query.get();
        if q.is_empty() {
            vec![]
        } else {
            // Mock search results - would be real search in production
            vec![
                ("λx. sin(x²)", "Smooth function example", "/lab?example=smooth"),
                ("Path ℝ 0 π", "Path type definition", "/lab?example=path"),
                ("Differentiation", "Learn about derivatives", "/learn#derivatives"),
            ]
        }
    };
    
    view! {
        <div class="search-results">
            <For
                each=results
                key=|r| r.0
                children=move |(code, desc, link)| {
                    view! {
                        <a href=link class="search-result">
                            <code>{code}</code>
                            <span>{desc}</span>
                        </a>
                    }
                }
            />
        </div>
    }
}

#[component]
fn ThemeToggle() -> impl IntoView {
    let (theme, set_theme) = create_signal("dark");
    
    let toggle = move |_| {
        let new_theme = if theme.get() == "dark" { "light" } else { "dark" };
        set_theme.set(new_theme);
        // Apply theme to document
        if let Some(doc) = document() {
            let _ = doc.document_element()
                .unwrap()
                .set_attribute("data-theme", new_theme);
        }
    };
    
    view! {
        <button class="theme-toggle" on:click=toggle>
            {move || if theme.get() == "dark" { "🌙" } else { "☀️" }}
        </button>
    }
}

#[component]
fn SaveButton() -> impl IntoView {
    let state = use_context::<RwSignal<AppState>>().unwrap();
    
    let save = move |_| {
        // Save current work to localStorage
        if let Some(storage) = window().local_storage().ok().flatten() {
            let _ = storage.set_item("sctt_work", &state.get().to_json());
        }
        // Show toast notification
        spawn_local(async {
            // Would show toast here
        });
    };
    
    view! {
        <button class="save-button" on:click=save title="Save work (⌘S)">
            "💾"
        </button>
    }
}

#[component]
fn HelpPanel() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);
    
    view! {
        <div class=move || if is_open.get() { "help-panel open" } else { "help-panel" }>
            <button 
                class="help-toggle"
                on:click=move |_| set_is_open.update(|b| *b = !*b)
                title="Help (⌘?)"
            >
                "?"
            </button>
            
            <Show when=is_open>
                <div class="help-content">
                    <h3>"Quick Help"</h3>
                    
                    <section>
                        <h4>"Keyboard Shortcuts"</h4>
                        <dl>
                            <dt><kbd>"⌘L"</kbd></dt><dd>"Open Laboratory"</dd>
                            <dt><kbd>"⌘K"</kbd></dt><dd>"Search"</dd>
                            <dt><kbd>"⌘Enter"</kbd></dt><dd>"Run code"</dd>
                            <dt><kbd>"⌘S"</kbd></dt><dd>"Save work"</dd>
                            <dt><kbd>"⌘/"</kbd></dt><dd>"Toggle help"</dd>
                        </dl>
                    </section>
                    
                    <section>
                        <h4>"Syntax Guide"</h4>
                        <pre class="syntax-guide">
"λx. expr     - Lambda
⟨t⟩ expr     - Path lambda  
C∞(A, B)     - Smooth function
Path A x y   - Path type
∂ f          - Derivative"
                        </pre>
                    </section>
                    
                    <section>
                        <h4>"Tips"</h4>
                        <ul>
                            <li>"Click on any example to load it"</li>
                            <li>"Hover over types for explanations"</li>
                            <li>"Use Tab for autocomplete"</li>
                        </ul>
                    </section>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn KeyboardShortcuts() -> impl IntoView {
    use leptos::ev::keydown;
    use web_sys::KeyboardEvent;
    
    let navigate = use_navigate();
    let state = use_context::<RwSignal<AppState>>().unwrap();
    
    let handle_shortcut = move |e: KeyboardEvent| {
        let key = e.key();
        let cmd = e.meta_key() || e.ctrl_key();
        
        if cmd {
            match key.as_str() {
                "l" | "L" => {
                    e.prevent_default();
                    navigate("/lab", NavigateOptions::default());
                },
                "k" | "K" => {
                    e.prevent_default();
                    // Trigger search
                },
                "s" | "S" => {
                    e.prevent_default();
                    // Save work
                    if let Some(storage) = window().local_storage().ok().flatten() {
                        let _ = storage.set_item("sctt_work", &state.get().to_json());
                    }
                },
                "/" | "?" => {
                    e.prevent_default();
                    // Toggle help
                },
                _ => {}
            }
        }
    };
    
    view! {
        <div on:keydown=window_event_listener(keydown, handle_shortcut)/>
    }
}

// Helper functions
fn document() -> Option<web_sys::Document> {
    web_sys::window()?.document()
}

fn window() -> web_sys::Window {
    web_sys::window().expect("Window should exist")
}

// WASM entry point
#[cfg(feature = "csr")]
#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}