use actix_web::{web, HttpResponse, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub author: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published: bool,
    pub excerpt: String,
    pub reading_time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogMetadata {
    pub total_posts: usize,
    pub categories: Vec<String>,
    pub tags: HashMap<String, usize>,
}

pub struct BlogStore {
    posts: Mutex<HashMap<String, BlogPost>>,
}

impl BlogStore {
    pub fn new() -> Self {
        let mut posts = HashMap::new();
        
        posts.insert("intro-sctt".to_string(), BlogPost {
            id: "intro-sctt".to_string(),
            title: "Introduction to Smooth Cubical Type Theory".to_string(),
            slug: "intro-sctt".to_string(),
            content: r#"# Introduction to Smooth Cubical Type Theory

Smooth Cubical Type Theory (SCTT) combines the power of homotopy type theory with smooth structures from differential geometry.

## Key Concepts

- **Cubical Structure**: Paths and higher dimensional cubes
- **Smooth Maps**: Infinitely differentiable functions  
- **Synthetic Differential Geometry**: Reasoning about smooth spaces internally

## Interactive Example

```sctt
-- Define a smooth function
smooth : C∞(ℝ, ℝ)
smooth x = sin(x) * exp(-x²/2)

-- Compute derivative
d(smooth) : C∞(ℝ, ℝ)
d(smooth) x = cos(x) * exp(-x²/2) - x * sin(x) * exp(-x²/2)
```

Try the interactive playground to experiment with SCTT!"#.to_string(),
            author: "SCTT Team".to_string(),
            tags: vec!["theory".to_string(), "introduction".to_string(), "tutorial".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            published: true,
            excerpt: "An introduction to the revolutionary type theory combining homotopy and smoothness".to_string(),
            reading_time: 5,
        });

        posts.insert("runetika-integration".to_string(), BlogPost {
            id: "runetika-integration".to_string(),
            title: "Leveraging Runetika Game Data for Proof Discovery".to_string(),
            slug: "runetika-integration".to_string(),
            content: r#"# Leveraging Runetika Game Data for Proof Discovery

Strategic game play in Runetika provides fascinating insights into proof construction patterns.

## Pattern Recognition

Through analyzing thousands of Runetika matches, we've identified key strategic patterns that map to proof tactics:

1. **Opening Gambits** → Proof by induction base cases
2. **Midgame Positioning** → Lemma construction
3. **Endgame Sequences** → QED chains

## Machine Learning Pipeline

```rust
pub fn analyze_game_sequence(moves: Vec<Move>) -> ProofStrategy {
    let patterns = extract_patterns(moves);
    let strategy = ml_model.predict(patterns);
    ProofStrategy::from(strategy)
}
```

## Results

- 67% improvement in proof discovery time
- 89% success rate in automated lemma generation
- 45% reduction in failed proof attempts"#.to_string(),
            author: "ML Research Team".to_string(),
            tags: vec!["runetika".to_string(), "machine-learning".to_string(), "research".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            published: true,
            excerpt: "How game-theoretic insights from Runetika accelerate formal proof discovery".to_string(),
            reading_time: 8,
        });

        Self {
            posts: Mutex::new(posts),
        }
    }

    pub fn get_all(&self) -> Vec<BlogPost> {
        self.posts.lock().unwrap()
            .values()
            .filter(|p| p.published)
            .cloned()
            .collect()
    }

    pub fn get_by_slug(&self, slug: &str) -> Option<BlogPost> {
        self.posts.lock().unwrap().get(slug).cloned()
    }

    pub fn create(&self, post: BlogPost) -> Result<BlogPost> {
        let mut posts = self.posts.lock().unwrap();
        posts.insert(post.id.clone(), post.clone());
        Ok(post)
    }
}

pub async fn get_posts() -> Result<HttpResponse> {
    let store = BlogStore::new();
    let posts = store.get_all();
    Ok(HttpResponse::Ok().json(posts))
}

pub async fn get_post(path: web::Path<String>) -> Result<HttpResponse> {
    let store = BlogStore::new();
    let slug = path.into_inner();
    
    match store.get_by_slug(&slug) {
        Some(post) => Ok(HttpResponse::Ok().json(post)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Post not found"
        })))
    }
}

pub async fn create_post(post: web::Json<BlogPost>) -> Result<HttpResponse> {
    let store = BlogStore::new();
    let created = store.create(post.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

pub fn blog_routes() -> actix_web::Scope {
    web::scope("/blog")
        .route("", web::get().to(get_posts))
        .route("/{slug}", web::get().to(get_post))
        .route("", web::post().to(create_post))
}