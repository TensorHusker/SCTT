use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer, HttpResponse, Result};
use leptos::*;
use leptos_actix::{generate_route_list, LeptosRoutes};
use sctt_web::App as SCTTApp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting SCTT server on http://localhost:8080");

    // Get Leptos configuration
    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(SCTTApp);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            // Enable CORS for WASM
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .expose_headers(vec![
                        "Cross-Origin-Opener-Policy",
                        "Cross-Origin-Embedder-Policy",
                    ])
            )
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            
            // Serve WASM files with proper headers
            .service(
                web::scope("/wasm")
                    .wrap(
                        middleware::DefaultHeaders::new()
                            .header("Cross-Origin-Opener-Policy", "same-origin")
                            .header("Cross-Origin-Embedder-Policy", "require-corp")
                            .header("Content-Type", "application/wasm")
                    )
                    .service(Files::new("", "./wasm"))
            )
            
            // Serve static assets
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", "./assets"))
            
            // API endpoints for SCTT operations
            .route("/api/typecheck", web::post(typecheck_handler))
            .route("/api/evaluate", web::post(evaluate_handler))
            .route("/api/health", web::get(health_check))
            
            // Leptos routes
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                SCTTApp,
            )
            .service(Files::new("/", site_root.clone()).index_file("index.html"))
    })
    .bind(&addr)?
    .run()
    .await
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "SCTT Server",
        "version": env!("CARGO_PKG_VERSION")
    })))
}

#[derive(serde::Deserialize)]
struct TypeCheckRequest {
    code: String,
}

async fn typecheck_handler(req: web::Json<TypeCheckRequest>) -> Result<HttpResponse> {
    // This would call into our SCTT type checker
    // For now, return a mock response
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "type": "C∞(ℝ, ℝ)",
        "expression": req.code
    })))
}

#[derive(serde::Deserialize)]
struct EvaluateRequest {
    expression: String,
    value: f64,
}

async fn evaluate_handler(req: web::Json<EvaluateRequest>) -> Result<HttpResponse> {
    // This would evaluate smooth functions
    // For now, return a mock response
    let result = if req.expression.contains("sin") {
        req.value.sin()
    } else if req.expression.contains("cos") {
        req.value.cos()
    } else {
        req.value
    };
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "result": result,
        "expression": req.expression,
        "input": req.value
    })))
}