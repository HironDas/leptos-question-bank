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

use std::sync::Arc;

#[cfg(feature = "ssr")]
use axum::body::Body;

#[cfg(feature = "ssr")]
use axum::http::Uri;
#[cfg(feature = "ssr")]
use axum::response::IntoResponse;
#[cfg(feature = "ssr")]
use axum::{extract::State, http::Request};
#[cfg(feature = "ssr")]
use axum_extra::extract::CookieJar;
#[cfg(feature = "ssr")]
use axum_server::Server;

use leptos::prelude::*;

#[cfg(feature = "ssr")]
use sqlx::PgPool;

#[cfg(feature = "ssr")]
use crate::configuration::DatabaseSettings;
#[cfg(feature = "ssr")]
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
#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub db_pool: Arc<PgPool>,
}

#[cfg(feature = "ssr")]
impl axum::extract::FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

#[cfg(feature = "ssr")]
pub async fn run(
    listener: std::net::TcpListener,
    db_pool: PgPool,
) -> Result<(Server, IntoMakeService<axum::Router>), std::io::Error> {
    use std::sync::Arc;

    use crate::app::App;
    use axum::routing::get;
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let mut conf = get_configuration(Some("Cargo.toml")).unwrap();
    let site_addr = listener.local_addr().expect("Failed to get local address");
    conf.leptos_options.site_addr = site_addr;
    log!("Configuration: {:?}", conf);
    let leptos_options = conf.leptos_options;
    //let req = leptos_options
    let addr = leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    let db_pool = Arc::new(db_pool);

    let app_state = AppState {
        leptos_options,
        db_pool,
    };

    let app = Router::new()
        .route(
            "/api/{*fn_name}",
            get(leptos_server_handler).post(leptos_server_handler),
        )
        //.nest("/", leptos_axum::leptos_routes_with_context())
        // .leptos_routes_with_context(
        //     &leptos_options,
        //     routes,
        //     move || {
        //         use leptos_axum::ResponseOptions;
        //         provide_context(db_pool.clone());
        //         provide_context(ResponseOptions::default());
        //         //provide_context(jar.clone());
        //     },
        //     {
        //         let leptos_options = leptos_options.clone();
        //         move || shell(leptos_options.clone())
        //     },
        // )
        // .fallback(leptos_axum::render_app_to_stream({
        //     let leptos_options = leptos_options.clone();
        //     move || shell(leptos_options.clone())
        // }))
        //.fallback(leptos_handler)
        .leptos_routes_with_handler(routes, leptos_handler)
        .fallback(leptos_error_handler)
        .with_state(app_state);

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
    use leptos::logging::log;
    use sqlx::postgres::PgPoolOptions;
    log!("Database configuration: {:?}", configuration);
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(10))
        .connect_lazy_with(configuration.with_db())
}

#[cfg(feature = "ssr")]
async fn leptos_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    req: Request<Body>,
) -> impl IntoResponse {
    use leptos_axum::render_app_to_stream_with_context;
    let handler = render_app_to_stream_with_context(
        move || {
            provide_context(jar.clone());
        },
        {
            let options = state.leptos_options.clone();
            move || shell(options.clone())
        },
    );
    handler(req).await.into_response()
}

#[cfg(feature = "ssr")]
async fn leptos_server_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    req: Request<Body>,
) -> impl IntoResponse {
    use leptos::prelude::*;
    use leptos_axum::handle_server_fns_with_context;
    let handler = handle_server_fns_with_context(
        move || {
            provide_context(state.db_pool.clone());
            provide_context(jar.clone());
        },
        req,
    );
    handler.await
}

#[cfg(feature = "ssr")]
async fn leptos_error_handler(
    uri: Uri,
    State(state): State<AppState>,
    req: Request<Body>,
) -> impl IntoResponse {
    let handler = leptos_axum::file_and_error_handler(move |options| shell(options.clone()));

    handler(uri, State(state.leptos_options), req)
        .await
        .into_response()
}
