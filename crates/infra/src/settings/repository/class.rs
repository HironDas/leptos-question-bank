use domain::class::Class;
use sqlx::{Postgres, Transaction};

pub(crate) async fn list_classes(tx: &mut Transaction<'_, Postgres>) -> domain::Result<Vec<Class>> {
    todo!()
}
