use app::shell;
use axum::{
    body::Body,
    extract::FromRef,
    extract::State,
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
    Router,
};
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use leptos::logging::log;
use std::path::PathBuf;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

#[derive(Clone)]
struct AppState {
    leptos: LeptosOptions,
    site_root: PathBuf,
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos.clone()
    }
}

fn static_mime(path: &str) -> &'static str {
    if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".js") {
        "application/javascript"
    } else if path.ends_with(".wasm") {
        "application/wasm"
    } else {
        "application/octet-stream"
    }
}

async fn serve_pkg(
    axum::extract::Path(path): axum::extract::Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    if path.contains("..") || path.contains('\\') {
        return (StatusCode::BAD_REQUEST, Body::empty()).into_response();
    }
    let file_path = state.site_root.join("pkg").join(path);
    match tokio::fs::read(&file_path).await {
        Ok(bytes) => {
            let mime = static_mime(&path);
            (
                [(header::CONTENT_TYPE, HeaderValue::from_static(mime))],
                bytes,
            )
                .into_response()
        }
        Err(_) => (StatusCode::NOT_FOUND, Body::empty()).into_response(),
    }
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).ok();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options.clone();
    let site_root = PathBuf::from(&*leptos_options.site_root);

    // SurrealDB: in-memory for template (use Ws for remote in production)
    let db: Surreal<surrealdb::engine::local::Db> =
        Surreal::new::<Mem>(()).await.expect("Failed to create SurrealDB");
    db.use_ns("themepark")
        .use_db("main")
        .await
        .expect("Failed to init SurrealDB namespace");

    let shell_for_routes = leptos_options.clone();
    let routes = generate_route_list(move || shell(shell_for_routes.clone()));

    let shell_for_app = leptos_options.clone();
    let app_state = AppState {
        leptos: leptos_options,
        site_root,
    };
    let app = Router::new()
        .route("/favicon.ico", axum::routing::get(|| async {
            axum::response::Redirect::permanent("/favicon.svg")
        }))
        .route("/pkg/{*path}", axum::routing::get(serve_pkg))
        .leptos_routes(&app_state, routes, move || shell(shell_for_app.clone()))
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
