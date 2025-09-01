use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppState {
    pub current_code: String,
    pub saved_snippets: Vec<SavedSnippet>,
    pub history: Vec<HistoryEntry>,
    pub preferences: UserPreferences,
    pub session: SessionData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSnippet {
    pub id: String,
    pub name: String,
    pub code: String,
    pub description: String,
    pub tags: Vec<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub code: String,
    pub result: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub theme: String,
    pub font_size: u8,
    pub auto_run: bool,
    pub show_types: bool,
    pub vim_mode: bool,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            font_size: 14,
            auto_run: false,
            show_types: true,
            vim_mode: false,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionData {
    pub last_visited: String,
    pub completed_tutorials: Vec<String>,
    pub achievements: Vec<Achievement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub unlocked_at: String,
}

impl AppState {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
    
    pub fn from_json(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }
    
    pub fn add_to_history(&mut self, code: String, result: String) {
        self.history.push(HistoryEntry {
            code,
            result,
            timestamp: chrono::Utc::now().to_rfc3339(),
        });
        
        // Keep only last 100 entries
        if self.history.len() > 100 {
            self.history.remove(0);
        }
    }
    
    pub fn save_snippet(&mut self, name: String, description: String, tags: Vec<String>) {
        let snippet = SavedSnippet {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            code: self.current_code.clone(),
            description,
            tags,
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        self.saved_snippets.push(snippet);
    }
    
    pub fn unlock_achievement(&mut self, id: String, name: String, description: String) {
        if !self.session.achievements.iter().any(|a| a.id == id) {
            self.session.achievements.push(Achievement {
                id,
                name,
                description,
                unlocked_at: chrono::Utc::now().to_rfc3339(),
            });
        }
    }
}