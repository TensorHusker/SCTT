use actix_web::{web, HttpResponse, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPost {
    pub slug: String,
    pub title: String,
    pub date: DateTime<Utc>,
    pub tags: Vec<String>,
    pub content: String,
    pub reading_time: u32,
}

pub struct Blog {
    posts_dir: String,
}

impl Blog {
    pub fn new() -> Self {
        Self {
            posts_dir: "./content/posts".to_string(),
        }
    }

    pub fn load_post(&self, slug: &str) -> Option<BlogPost> {
        let path = format!("{}/{}.md", self.posts_dir, slug);
        if !Path::new(&path).exists() {
            return None;
        }

        let content = fs::read_to_string(&path).ok()?;
        let (frontmatter, body) = self.parse_frontmatter(&content)?;
        
        Some(BlogPost {
            slug: slug.to_string(),
            title: frontmatter.get("title")?.to_string(),
            date: frontmatter.get("date")
                .and_then(|d| DateTime::parse_from_rfc3339(d).ok())
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(Utc::now),
            tags: frontmatter.get("tags")
                .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default(),
            content: body,
            reading_time: self.calculate_reading_time(&body),
        })
    }

    pub fn list_posts(&self) -> Vec<BlogPost> {
        let mut posts = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&self.posts_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".md") {
                        let slug = name.trim_end_matches(".md");
                        if let Some(post) = self.load_post(slug) {
                            posts.push(post);
                        }
                    }
                }
            }
        }
        
        posts.sort_by(|a, b| b.date.cmp(&a.date));
        posts
    }

    fn parse_frontmatter(&self, content: &str) -> Option<(std::collections::HashMap<String, String>, String)> {
        if !content.starts_with("---") {
            return Some((std::collections::HashMap::new(), content.to_string()));
        }

        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            return None;
        }

        let mut frontmatter = std::collections::HashMap::new();
        for line in parts[1].lines() {
            if let Some((key, value)) = line.split_once(':') {
                frontmatter.insert(
                    key.trim().to_string(),
                    value.trim().trim_matches('"').to_string()
                );
            }
        }

        Some((frontmatter, parts[2].trim().to_string()))
    }

    fn calculate_reading_time(&self, content: &str) -> u32 {
        let words = content.split_whitespace().count();
        ((words as f32) / 200.0).ceil() as u32
    }
}

pub async fn get_posts() -> Result<HttpResponse> {
    let blog = Blog::new();
    let posts = blog.list_posts();
    Ok(HttpResponse::Ok().json(posts))
}

pub async fn get_post(path: web::Path<String>) -> Result<HttpResponse> {
    let blog = Blog::new();
    let slug = path.into_inner();
    
    match blog.load_post(&slug) {
        Some(post) => Ok(HttpResponse::Ok().json(post)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Post not found"
        })))
    }
}