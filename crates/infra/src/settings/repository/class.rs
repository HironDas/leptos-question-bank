use domain::class::{AddClassInput, Class};
use sqlx::{prelude::FromRow, PgPool, Postgres, Transaction};

use crate::common::db;

#[derive(FromRow)]
pub(crate) struct ClassRow {
    id: i32,
    name: String,
    name_bn: String,
    order: i32,
}

impl From<ClassRow> for Class {
    fn from(row: ClassRow) -> Self {
        Self {
            id: row.id as u32,
            name: row.name,
            name_bn: row.name_bn,
            order: row.order as u32,
        }
    }
}

pub(crate) async fn list_class(pool: &PgPool) -> domain::Result<Vec<Class>> {
    let classes = sqlx::query_as!(
        ClassRow,
        r#"
        SELECT id, name, name_bn, "order"  FROM classes ORDER BY "order" ASC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(db)?
    .into_iter()
    .map(Class::from)
    .collect();

    Ok(classes)
}

pub(crate) async fn insert_class(pool: &PgPool, class: AddClassInput) -> Result<Class> {
    let max_order: Option<i32> = sqlx::query_scalar!(
        r#"
                SELECT MAX("order") FROM classes
                "#
    )
    .fetch_one(pool)
    .await
    .map_err(db)?;

    let new_order = max_order.unwrap_or(0) + 10;

    sqlx::query_as!(
        ClassRow,
        r#"
        INSERT INTO classes (name, name_bn, "order") VALUES ($1, $2, $3) RETURNING id, name, name_bn, "order"
        "#,
        class.name,
        class.name_bn,
        new_order as i32
    )
    .fetch_one(&*pool)
    .await
    .map(Class::from)
    .map_err(db)?
}
