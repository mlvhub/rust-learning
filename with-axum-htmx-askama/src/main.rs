use anyhow::Context;
use askama::Template;
use axum::{
    extract::Form,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    routing::post,
    Router,
};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct AppState {
    todos: Mutex<Vec<String>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_state = Arc::new(AppState {
        todos: Mutex::new(vec![]),
    });

    info!("initializing router...");

    let api_router = Router::new()
        .route("/hello", get(hello_from_the_server))
        .route("/todos", post(add_todo))
        .with_state(app_state);

    let assets_path = std::env::current_dir().unwrap();
    let router = Router::new()
        .nest("/api", api_router)
        .route("/", get(hello))
        .route("/another", get(another))
        .nest_service::<ServeDir>(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );
    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    info!("router initialized, now listening on port {}", port);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .context("error while starting server")?;

    Ok(())
}

#[derive(Template)]
#[template(path = "todo-list.html")]
struct TodoList {
    todos: Vec<String>,
}

#[derive(Deserialize)]
struct TodoRequest {
    todo: String,
}

async fn add_todo(
    State(state): State<Arc<AppState>>,
    Form(todo): Form<TodoRequest>,
) -> impl IntoResponse {
    let mut lock = state.todos.lock().unwrap();
    lock.push(todo.todo);

    let template = TodoList {
        todos: lock.clone(),
    };

    HtmlTemplate(template)
}

async fn hello_from_the_server() -> &'static str {
    "Hello!"
}

async fn another() -> impl IntoResponse {
    let template = AnotherTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "another.html")]
struct AnotherTemplate;

async fn hello() -> impl IntoResponse {
    let template = HelloTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate;

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
struct HtmlTemplate<T>(T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
