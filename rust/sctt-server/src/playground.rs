use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CodeInput {
    pub code: String,
    pub mode: Option<String>, // "typecheck" | "evaluate" | "visualize"
}

#[derive(Debug, Serialize)]
pub struct CodeOutput {
    pub success: bool,
    pub result: Option<String>,
    pub error: Option<String>,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Serialize)]
pub struct Diagnostic {
    pub severity: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
}

pub async fn run_code(input: web::Json<CodeInput>) -> Result<HttpResponse> {
    let code = &input.code;
    let mode = input.mode.as_deref().unwrap_or("typecheck");

    let output = match mode {
        "typecheck" => typecheck_sctt(code),
        "evaluate" => evaluate_sctt(code),
        "visualize" => visualize_sctt(code),
        _ => CodeOutput {
            success: false,
            result: None,
            error: Some("Unknown mode".to_string()),
            diagnostics: vec![],
        }
    };

    Ok(HttpResponse::Ok().json(output))
}

fn typecheck_sctt(code: &str) -> CodeOutput {
    // TODO: Integrate with sctt-checker
    if code.contains("smooth") {
        CodeOutput {
            success: true,
            result: Some("Type: C∞(ℝ, ℝ)".to_string()),
            error: None,
            diagnostics: vec![],
        }
    } else {
        CodeOutput {
            success: true,
            result: Some("Type: Type".to_string()),
            error: None,
            diagnostics: vec![],
        }
    }
}

fn evaluate_sctt(code: &str) -> CodeOutput {
    // TODO: Integrate with sctt-core evaluator
    CodeOutput {
        success: true,
        result: Some("42".to_string()),
        error: None,
        diagnostics: vec![],
    }
}

fn visualize_sctt(code: &str) -> CodeOutput {
    // TODO: Generate visualization data
    CodeOutput {
        success: true,
        result: Some(r#"{"type": "path", "dimensions": 2}"#.to_string()),
        error: None,
        diagnostics: vec![],
    }
}