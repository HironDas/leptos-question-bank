#[cfg(feature = "ssr")]
use axum_extra::extract::CookieJar;
#[cfg(feature = "ssr")]
use infra::{auth::AuthService, common::ServiceContext};
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos::server;

use crate::error::AppError;

#[server]
pub async fn is_auth() -> Result<Option<bool>, AppError> {
    // use leptos::logging;
    #[cfg(feature = "ssr")]
    {
        // let ctx = expect_context::<ServiceContext>();
        let ctx = use_context::<ServiceContext>().ok_or(AppError::Internal)?;
        let jar = expect_context::<CookieJar>();
        let session = jar.get("session").map(|cookie| cookie.value());
        let auth_service = AuthService::from(ctx);
        let allowed = auth_service.auth(session).await.is_ok();
        Ok(Some(allowed))
    }
}

#[derive(Clone, Copy)]
pub struct AuthContext {
    pub authenticated: RwSignal<Option<bool>>,
}

impl AuthContext {
    pub fn new() -> Self {
        Self {
            authenticated: RwSignal::new(None),
        }
    }

    pub fn set_authenticated(&self, value: bool) {
        self.authenticated.set(Some(value))
    }

    pub fn is_authenticated(&self) -> Option<bool> {
        self.authenticated.get()
    }
}
