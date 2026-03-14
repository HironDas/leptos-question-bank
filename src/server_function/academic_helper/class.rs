use leptos::{logging::log, prelude::*};
#[cfg(feature = "ssr")]
use sqlx::PgPool;
use std::sync::Arc;

#[cfg(feature = "ssr")]
use crate::error::QuestionBankError;
use crate::{
    domain::class::{AddClassInput, UpdateClassInput},
    server_function::academic_setting::{Class, Subject},
};

#[cfg(feature = "ssr")]
pub async fn class(pool: Arc<PgPool>) -> Result<Vec<Class>, ServerFnError> {
    let classes = sqlx::query!(
        r#"
        SELECT id, name, name_bn, "order"  FROM classes ORDER BY "order" ASC
        "#
    )
    .try_map(|record| {
        Ok(Class {
            id: record.id as u32,
            name: record.name,
            name_bn: record.name_bn,
            order: record.order as u32,
        })
    })
    .fetch_all(&*pool)
    .await?;

    Ok(classes)
}

#[server]
pub async fn add_class(class: AddClassInput) -> Result<(), ServerFnError> {
    use validator::Validate;
    log!("Adding class: {:?}", class);
    if let Err(errors) = class.validate() {
        log!("Validation errors: {:?}", errors);
        return Err(ServerFnError::ServerError("Invalid input data".into()));
    } else {
        log!("Input data is valid.");
        #[cfg(feature = "ssr")]
        {
            use sqlx::PgPool;
            use std::sync::Arc;
            let pool = expect_context::<Arc<PgPool>>();

            insert_class(pool, class).await.map_err(|e| e.into()).map(|_| ())
        }
        
    }
}

#[cfg(feature = "ssr")]
pub async fn insert_class(
    pool: Arc<PgPool>,
    class: AddClassInput,
) -> Result<(), QuestionBankError> {
    use anyhow::Context;
    log!("Inserting class: {:?}", class);
    let max_order = sqlx::query!(
        r#"
                SELECT MAX("order") FROM classes
                "#
    )
    .fetch_one(&*pool)
    .await
    .context("Somethig went wrong")?;

    log!("Current max order is: {:?}", max_order);

    let new_order = max_order.max.unwrap_or(0) + 1;
    log!("New order for the class will be: {}", new_order);

    sqlx::query!(
        r#"
        INSERT INTO classes (name, name_bn, "order") VALUES ($1, $2, $3)
        "#,
        class.name,
        class.name_bn,
        new_order as i32
    )
    .execute(&*pool)
    .await
    .map(|result| {
        log!("Class Inserted Successfully! and the id is {:?}", result);
        Ok(())
    })
    .map_err(|e| {
        log!("Failed to insert class: {}", e);
        e
    })
    .context("Something Went wrong on saving class")?
}

#[server]
pub async fn update_class(class: UpdateClassInput) -> Result<(), ServerFnError> {
    use validator::Validate;
    if let Err(errors) = class.validate() {
        return Err(errors.into());
    }else {
        
        Ok(())
    }
}
