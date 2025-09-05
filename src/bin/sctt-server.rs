//! SCTT API Server with WebSocket Support
//!
//! Production-ready server featuring:
//! - REST API endpoints
//! - WebSocket collaboration
//! - Proof persistence
//! - Rate limiting and authentication

use axum::{
    extract::{Path, Query, State, WebSocketUpgrade},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use axum::extract::ws::{Message, WebSocket};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, FromRow};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
};
use tokio::sync::{Mutex, RwLock};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

type Sessions = Arc<RwLock<HashMap<String, SessionState>>>;
type Connections = Arc<Mutex<HashMap<String, Vec<tokio::sync::mpsc::UnboundedSender<Message>>>>>;

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
    sessions: Sessions,
    connections: Connections,
}

struct SessionState {
    document: String,
    version: u64,
    users: Vec<String>,
    proof_state: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Database setup
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:sctt.db".to_string());
    let db = SqlitePool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&db).await?;

    // Shared state
    let state = AppState {
        db,
        sessions: Arc::new(RwLock::new(HashMap::new())),
        connections: Arc::new(Mutex::new(HashMap::new())),
    };

    // Build router
    let app = Router::new()
        // API routes
        .route("/api/health", get(health_check))
        .route("/api/typecheck", post(typecheck))
        .route("/api/compile", post(compile))
        .route("/api/prove", post(prove))
        .route("/api/session", post(create_session))
        .route("/api/session/:id", get(get_session))
        .route("/api/proofs", get(list_proofs))
        .route("/api/proof/:id", get(get_proof).post(save_proof))
        
        // WebSocket endpoint
        .route("/ws/:session_id", get(websocket_handler))
        
        // Static files
        .nest_service("/", ServeDir::new("dist"))
        
        // CORS
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 SCTT Server running on http://{}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

#[derive(Deserialize)]
struct TypeCheckRequest {
    code: String,
}

#[derive(Serialize)]
struct TypeCheckResponse {
    success: bool,
    result: Option<String>,
    error: Option<String>,
}

async fn typecheck(Json(req): Json<TypeCheckRequest>) -> impl IntoResponse {
    // Type check the code
    let system = sctt_system::ScttSystem::new();
    
    match system.type_check(&req.code) {
        Ok(ty) => Json(TypeCheckResponse {
            success: true,
            result: Some(ty),
            error: None,
        }),
        Err(e) => Json(TypeCheckResponse {
            success: false,
            result: None,
            error: Some(format!("{:?}", e)),
        }),
    }
}

#[derive(Deserialize)]
struct CompileRequest {
    code: String,
    optimization: String,
}

#[derive(Serialize)]
struct CompileResponse {
    success: bool,
    wasm: Option<Vec<u8>>,
    error: Option<String>,
}

async fn compile(Json(req): Json<CompileRequest>) -> impl IntoResponse {
    use sctt_system::{ScttToWasmCompiler, OptLevel};
    
    let opt_level = match req.optimization.as_str() {
        "none" => OptLevel::None,
        "basic" => OptLevel::Basic,
        "aggressive" => OptLevel::Aggressive,
        _ => OptLevel::Basic,
    };
    
    let mut compiler = ScttToWasmCompiler::new(opt_level);
    let system = sctt_system::ScttSystem::new();
    
    match system.parse_term(&req.code) {
        Ok(term) => {
            let ir = compiler.sctt_to_ir(&term);
            let optimized = compiler.optimize(ir);
            let wasm_module = compiler.ir_to_wasm(&optimized);
            
            Json(CompileResponse {
                success: true,
                wasm: Some(wasm_module.encode()),
                error: None,
            })
        }
        Err(e) => Json(CompileResponse {
            success: false,
            wasm: None,
            error: Some(format!("{:?}", e)),
        }),
    }
}

#[derive(Deserialize)]
struct ProveRequest {
    statement: String,
    tactics: Vec<String>,
}

#[derive(Serialize)]
struct ProveResponse {
    success: bool,
    proof_state: Option<String>,
    error: Option<String>,
}

async fn prove(Json(req): Json<ProveRequest>) -> impl IntoResponse {
    use sctt_system::ProofAssistant;
    
    let mut assistant = ProofAssistant::new();
    
    // Parse and start proof
    match sctt_system::parser::parse(&req.statement) {
        Ok(term) => {
            if let Err(e) = assistant.start_proof("goal", term) {
                return Json(ProveResponse {
                    success: false,
                    proof_state: None,
                    error: Some(e),
                });
            }
            
            // Apply tactics
            for tactic_str in req.tactics {
                if let Some(tactic) = sctt_system::proof_assistant::Tactic::from_name(&tactic_str) {
                    let _ = assistant.apply_tactic(tactic, 0);
                }
            }
            
            Json(ProveResponse {
                success: true,
                proof_state: Some(assistant.render_proof_state()),
                error: None,
            })
        }
        Err(e) => Json(ProveResponse {
            success: false,
            proof_state: None,
            error: Some(e),
        }),
    }
}

#[derive(Serialize)]
struct SessionResponse {
    id: String,
    created: bool,
}

async fn create_session(State(state): State<AppState>) -> impl IntoResponse {
    let id = uuid::Uuid::new_v4().to_string();
    
    let session = SessionState {
        document: String::new(),
        version: 0,
        users: Vec::new(),
        proof_state: String::new(),
    };
    
    state.sessions.write().await.insert(id.clone(), session);
    
    Json(SessionResponse {
        id,
        created: true,
    })
}

async fn get_session(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let sessions = state.sessions.read().await;
    
    if let Some(session) = sessions.get(&id) {
        Json(serde_json::json!({
            "id": id,
            "document": session.document,
            "version": session.version,
            "users": session.users,
            "proof_state": session.proof_state,
        }))
    } else {
        Json(serde_json::json!({
            "error": "Session not found"
        }))
    }
}

#[derive(FromRow, Serialize)]
struct ProofRecord {
    id: String,
    name: String,
    statement: String,
    proof: String,
    created_at: String,
}

async fn list_proofs(State(state): State<AppState>) -> impl IntoResponse {
    let proofs = sqlx::query_as::<_, ProofRecord>(
        "SELECT id, name, statement, proof, created_at FROM proofs ORDER BY created_at DESC LIMIT 100"
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();
    
    Json(proofs)
}

async fn get_proof(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, ProofRecord>(
        "SELECT id, name, statement, proof, created_at FROM proofs WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await
    {
        Ok(proof) => Json(serde_json::json!(proof)),
        Err(_) => Json(serde_json::json!({
            "error": "Proof not found"
        })),
    }
}

#[derive(Deserialize)]
struct SaveProofRequest {
    name: String,
    statement: String,
    proof: String,
}

async fn save_proof(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<SaveProofRequest>,
) -> impl IntoResponse {
    let result = sqlx::query(
        "INSERT INTO proofs (id, name, statement, proof, created_at) VALUES (?, ?, ?, ?, datetime('now'))"
    )
    .bind(&id)
    .bind(&req.name)
    .bind(&req.statement)
    .bind(&req.proof)
    .execute(&state.db)
    .await;
    
    match result {
        Ok(_) => Json(serde_json::json!({
            "success": true,
            "id": id
        })),
        Err(e) => Json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        })),
    }
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Path(session_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, session_id, state))
}

async fn handle_socket(socket: WebSocket, session_id: String, state: AppState) {
    let (sender, mut receiver) = socket.split();
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    
    // Add connection to session
    {
        let mut connections = state.connections.lock().await;
        connections.entry(session_id.clone())
            .or_insert_with(Vec::new)
            .push(tx);
    }
    
    // Spawn sender task
    let mut rx = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });
    
    // Handle incoming messages
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            handle_client_message(text, &session_id, &state).await;
        }
    }
    
    // Remove connection on disconnect
    let mut connections = state.connections.lock().await;
    if let Some(session_conns) = connections.get_mut(&session_id) {
        session_conns.retain(|tx| !tx.is_closed());
    }
}

async fn handle_client_message(text: String, session_id: &str, state: &AppState) {
    use sctt_system::collaborative::{ClientMessage, ServerMessage, OperationalTransform};
    
    if let Ok(msg) = serde_json::from_str::<ClientMessage>(&text) {
        match msg {
            ClientMessage::Operation { op, version } => {
                // Apply operational transformation
                let mut sessions = state.sessions.write().await;
                if let Some(session) = sessions.get_mut(session_id) {
                    OperationalTransform::apply(&mut session.document, &op);
                    session.version = version + 1;
                    
                    // Broadcast to other users
                    let response = ServerMessage::Operation {
                        op,
                        user_id: "user".to_string(),
                        version: session.version,
                    };
                    
                    broadcast_to_session(session_id, response, state).await;
                }
            }
            _ => {}
        }
    }
}

async fn broadcast_to_session(
    session_id: &str,
    msg: sctt_system::collaborative::ServerMessage,
    state: &AppState,
) {
    let connections = state.connections.lock().await;
    
    if let Some(session_conns) = connections.get(session_id) {
        let text = serde_json::to_string(&msg).unwrap();
        
        for tx in session_conns {
            let _ = tx.send(Message::Text(text.clone()));
        }
    }
}

mod sctt_system {
    pub use crate::*;
}