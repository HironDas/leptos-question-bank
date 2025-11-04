#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use leptos_question_bank::{configuration::get_configuration, Application};

    // let conf = get_configuration(None).unwrap();
    // let addr = conf.leptos_options.site_addr;
    // let listener = std::net::TcpListener::bind(&addr).unwrap();
    //run(listener).await.unwrap();
    //
    let configuration = get_configuration().expect("Failed to get configuration");
    let application = Application::build(configuration)
        .await
        .expect("Failed to build application");
    application
        .run_until_stopped()
        .await
        .expect("Application not running");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
