use domain::{Error, Result};
use sqlx::{types::Uuid, Postgres, Transaction};

use crate::common::db;

pub(crate) async fn is_auth(tx: &mut Transaction<'_, Postgres>, token: Uuid) -> Result<()> {
    let is_authencated = sqlx::query!(
        r#"
        SELECT EXISTS(SELECT 1 FROM sessions WHERE session_token = $1)
        "#,
        token
    )
    .fetch_one(&mut **tx)
    .await
    .map_err(db)?
    .exists;

    if let Some(true) = is_authencated {
        Ok(())
    } else {
        Err(Error::NotFound)
    }
}
