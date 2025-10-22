
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
use leptos::config::get_configuration;
use leptos_question_bank::run;

let conf = get_configuration(None).unwrap();
let addr = conf.leptos_options.site_addr;
let listener = std::net::TcpListener::bind(&addr).unwrap();
run(listener).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
