#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn insert_subject_succeeds() {
    use std::sync::Arc;

    use leptos_question_bank::{
        domain::class::AddClassInput,
        server_function::academic_helper::{class::insert_class, subject::insert_subject},
    };

    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let pool = Arc::new(app.db_pool.clone());

    // Create a class first
    let class = insert_class(
        pool.clone(),
        AddClassInput {
            name: "Class 8".to_string(),
            name_bn: "অষ্টম শ্রেণি".to_string(),
        },
    )
    .await
    .expect("Failed to insert class");

    // Insert a subject under that class
    let subject = insert_subject(pool, "Mathematics".to_string(), class.id)
        .await
        .expect("Failed to insert subject");

    assert!(subject.id > 0);
    assert_eq!(subject.title, "Mathematics");
    assert_eq!(subject.class_id, class.id);
    assert_eq!(subject.order, 1);
}

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn fetch_subjects_by_class_returns_correct_data() {
    use std::sync::Arc;

    use leptos_question_bank::{
        domain::class::AddClassInput,
        server_function::academic_helper::{
            class::insert_class,
            subject::{insert_subject, subject},
        },
    };

    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let pool = Arc::new(app.db_pool.clone());

    // Create a class
    let class = insert_class(
        pool.clone(),
        AddClassInput {
            name: "Class 9".to_string(),
            name_bn: "নবম শ্রেণি".to_string(),
        },
    )
    .await
    .expect("Failed to insert class");

    // Insert two subjects
    insert_subject(pool.clone(), "Mathematics".to_string(), class.id)
        .await
        .expect("Failed to insert Math");
    insert_subject(pool.clone(), "Physics".to_string(), class.id)
        .await
        .expect("Failed to insert Physics");

    // Fetch subjects for this class
    let subjects = subject(class.id).await.expect("Failed to fetch subjects");

    assert_eq!(subjects.len(), 2);
    assert_eq!(subjects[0].order, 1);
    assert_eq!(subjects[1].order, 2);
    assert_eq!(subjects[0].title, "Mathematics");
    assert_eq!(subjects[1].title, "Physics");
}

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn update_subject_succeeds() {
    use std::sync::Arc;

    use leptos_question_bank::{
        domain::class::AddClassInput,
        server_function::academic_helper::{
            class::insert_class,
            subject::{edit_subject, insert_subject},
        },
    };

    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let pool = Arc::new(app.db_pool.clone());

    let class = insert_class(
        pool.clone(),
        AddClassInput {
            name: "Class 10".to_string(),
            name_bn: "দশম শ্রেণি".to_string(),
        },
    )
    .await
    .expect("Failed to insert class");

    let subject = insert_subject(pool.clone(), "Old Subject".to_string(), class.id)
        .await
        .expect("Failed to insert subject");

    // Update the subject
    let updated = edit_subject(pool, "New Subject".to_string(), subject.id)
        .await
        .expect("Failed to update subject");

    assert_eq!(updated.id, subject.id);
    assert_eq!(updated.title, "New Subject");
    assert_eq!(updated.class_id, class.id);
}

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn subjects_are_scoped_to_class() {
    use std::sync::Arc;

    use leptos_question_bank::{
        domain::class::AddClassInput,
        server_function::academic_helper::{
            class::insert_class,
            subject::{insert_subject, subject},
        },
    };

    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let pool = Arc::new(app.db_pool.clone());

    // Create two classes
    let class_a = insert_class(
        pool.clone(),
        AddClassInput {
            name: "Class A".to_string(),
            name_bn: "ক্লাস ক".to_string(),
        },
    )
    .await
    .expect("Failed to insert class A");

    let class_b = insert_class(
        pool.clone(),
        AddClassInput {
            name: "Class B".to_string(),
            name_bn: "ক্লাস খ".to_string(),
        },
    )
    .await
    .expect("Failed to insert class B");

    // Insert subject in class A
    insert_subject(pool.clone(), "Subject A".to_string(), class_a.id)
        .await
        .expect("Failed to insert subject in A");

    // Insert subject in class B
    insert_subject(pool.clone(), "Subject B".to_string(), class_b.id)
        .await
        .expect("Failed to insert subject in B");

    // Class A should only have its own subject
    let subjects_a = subject(class_a.id)
        .await
        .expect("Failed to fetch subjects for A");
    assert_eq!(subjects_a.len(), 1);
    assert_eq!(subjects_a[0].title, "Subject A");

    // Class B should only have its own subject
    let subjects_b = subject(class_b.id)
        .await
        .expect("Failed to fetch subjects for B");
    assert_eq!(subjects_b.len(), 1);
    assert_eq!(subjects_b[0].title, "Subject B");
}
