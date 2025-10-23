pub mod app;
pub mod components;
pub mod domain;
pub mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

#[cfg(feature = "ssr")]
use axum_server::Server;

#[cfg(feature = "ssr")]
pub async fn run(listener: std::net::TcpListener) -> Result<(), std::io::Error> {
    use crate::app::*;
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
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
    server.serve(app.into_make_service()).await?;
    // Ok(server)
    Ok(())
}
