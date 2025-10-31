pub mod app;
pub mod components;
pub mod configuration;
pub mod domain;
pub mod error;
pub mod pages;
pub mod server_function;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_lazy(App);
}

#[cfg(feature = "ssr")]
use axum_server::Server;

#[cfg(feature = "ssr")]
use sqlx::PgPool;

#[cfg(feature = "ssr")]
use crate::configuration::DatabaseSettings;
use crate::{app::shell, configuration::Settings};

#[cfg(feature = "ssr")]
use axum::routing::IntoMakeService;

#[cfg(feature = "ssr")]
pub struct Application {
    port: u16,
    server: Server,
    service: IntoMakeService<axum::Router>,
}

#[cfg(feature = "ssr")]
impl Application {
    #[cfg(feature = "ssr")]
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let listener = std::net::TcpListener::bind(format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        ))?;

        let port = listener.local_addr().unwrap().port();
        let application_server = run(listener, connection_pool).await?;

        Ok(Self {
            port,
            server: application_server.0,
            service: application_server.1,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    #[cfg(feature = "ssr")]
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.serve(self.service).await
    }
}

#[cfg(feature = "ssr")]
pub async fn run(
    listener: std::net::TcpListener,
    db_pool: PgPool,
) -> Result<(Server, IntoMakeService<axum::Router>), std::io::Error> {
    use std::sync::Arc;

    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    use crate::app::App;

    let conf = get_configuration(None).unwrap();
    let _site_addr = listener.local_addr().expect("Failed to get local address");
    // let leptos_options = LeptosOptions {
    //     site_addr: site_addr,
    //     ..conf.leptos_options
    // };
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    let db_pool = Arc::new(db_pool);

    // let app_state = AppState {
    //     db_pool: db_pool,
    //     leptos_options,
    // };
    //let value = app_state.clone();
    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(db_pool.clone()),
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    // let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    // let listener = std::net::TcpListener::bind(&addr)?;
    // axum::serve(listener, app.into_make_service())
    //     .await
    //     .unwrap()

    let server = Server::from_tcp(listener);
    //server.serve(app.into_make_service()).await?;
    // Ok(server)
    Ok((server, app.into_make_service()))
}

#[cfg(feature = "ssr")]
pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    use sqlx::postgres::PgPoolOptions;

    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}
