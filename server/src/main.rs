use app::shell;
use axum::Router;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use leptos::logging::log;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).ok();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options.clone();

    // SurrealDB: in-memory for template (use Ws for remote in production)
    let db: Surreal<surrealdb::engine::local::Db> =
        Surreal::new::<Mem>(()).await.expect("Failed to create SurrealDB");
    db.use_ns("themepark")
        .use_db("main")
        .await
        .expect("Failed to init SurrealDB namespace");

    let routes = generate_route_list(|| shell(leptos_options.clone()));

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, move || shell(leptos_options.clone()))
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
