use leptos::*;
use leptos_router::*;

#[derive(Clone)]
struct BlogPost {
    id: &'static str,
    date: &'static str,
    title: &'static str,
    summary: &'static str,
    content: &'static str,
}

const BLOG_POSTS: &[BlogPost] = &[
    BlogPost {
        id: "coherence-breakthrough",
        date: "2024-01-15",
        title: "The Coherence Breakthrough",
        summary: "After months of research, we've finally solved the smooth-cubical coherence problem...",
        content: r#"
# The Coherence Breakthrough

After months of intensive research, we've achieved a major milestone in SCTT development: 
full coherence between smooth and cubical structures.

## The Problem

The central challenge was ensuring that:
1. Differentiation respects path composition
2. Path operations preserve smoothness
3. Both structures interact predictably

## Our Solution

We introduced a new coherence axiom that ensures:
```
D(path_compose(p, q)) = path_compose(D(p), D(q))
```

This seemingly simple equation has profound implications...
        "#,
    },
    BlogPost {
        id: "wasm-implementation",
        date: "2024-01-10",
        title: "Implementing Smooth Functions in WASM",
        summary: "How we compiled symbolic differentiation to WebAssembly for browser execution...",
        content: r#"
# Implementing Smooth Functions in WASM

Today we're excited to share how we brought SCTT to the browser using Rust and WebAssembly.

## Architecture

Our implementation consists of four core modules:
- `sctt-core`: Type system fundamentals
- `sctt-smooth`: Differentiation and integration
- `sctt-cubical`: Path and interval operations
- `sctt-checker`: Bidirectional type checking

Each module compiles to a separate WASM binary, allowing modular loading...
        "#,
    },
    BlogPost {
        id: "ml-applications",
        date: "2024-01-05",
        title: "Why SCTT Matters for Machine Learning",
        summary: "Verified gradients, guaranteed convergence, and the future of differentiable programming...",
        content: r#"
# Why SCTT Matters for Machine Learning

Machine learning is fundamentally about optimization over smooth functions. 
SCTT provides the mathematical foundation to verify these operations.

## Verified Gradients

With SCTT, gradient computations are not just automatic—they're provably correct...
        "#,
    },
];

#[component]
pub fn BlogPage() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"Development Blog"</h1>
            <p>"Follow our journey building Smooth Cubical Type Theory"</p>
            
            <div class="posts-grid">
                {BLOG_POSTS.iter().map(|post| {
                    view! {
                        <article class="post-card">
                            <span class="post-date">{post.date}</span>
                            <h3>{post.title}</h3>
                            <p>{post.summary}</p>
                            <A 
                                href=format!("/blog/{}", post.id)
                                class="read-more"
                            >
                                "Read more →"
                            </A>
                        </article>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}