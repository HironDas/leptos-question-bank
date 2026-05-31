#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn insert_chapter_succeeds() {
    use std::sync::Arc;

    use leptos_question_bank::{
        domain::class::AddClassInput,
        server_function::academic_helper::{
            chapter::insert_chapter, class::insert_class, subject::insert_subject,
        },
    };

    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let pool = Arc::new(app.db_pool.clone());

    // Create class + subject
    let class = insert_class(
        pool.clone(),
        AddClassInput {
            name: "Class 11".to_string(),
            name_bn: "একাদশ শ্রেণি".to_string(),
        },
    )
    .await
    .expect("Failed to insert class");

    let subject = insert_subject(pool.clone(), "Biology".to_string(), class.id)
        .await
        .expect("Failed to insert subject");

    // Insert a chapter
    let chapter = insert_chapter(pool, "Cell Structure".to_string(), subject.id)
        .await
        .expect("Failed to insert chapter");

    assert!(chapter.id > 0);
    assert_eq!(chapter.title, "Cell Structure");
    assert_eq!(chapter.subject_id, subject.id);
    assert_eq!(chapter.class_id, class.id);
    assert_eq!(chapter.order, 1);
}

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn fetch_chapters_by_subject_returns_correct_data() {
    use std::sync::Arc;

    use leptos_question_bank::{
        domain::class::AddClassInput,
        server_function::academic_helper::{
            chapter::{insert_chapter, subject_chapter},
            class::insert_class,
            subject::insert_subject,
        },
    };

    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let pool = Arc::new(app.db_pool.clone());

    let class = insert_class(
        pool.clone(),
        AddClassInput {
            name: "Class 12".to_string(),
            name_bn: "দ্বাদশ শ্রেণি".to_string(),
        },
    )
    .await
    .expect("Failed to insert class");

    let subject = insert_subject(pool.clone(), "Chemistry".to_string(), class.id)
        .await
        .expect("Failed to insert subject");

    // Insert two chapters
    insert_chapter(pool.clone(), "Atomic Structure".to_string(), subject.id)
        .await
        .expect("Failed to insert chapter 1");
    insert_chapter(pool.clone(), "Chemical Bonding".to_string(), subject.id)
        .await
        .expect("Failed to insert chapter 2");

    let chapters = subject_chapter(subject.id)
        .await
        .expect("Failed to fetch chapters");

    assert_eq!(chapters.len(), 2);
    assert_eq!(chapters[0].order, 1);
    assert_eq!(chapters[0].title, "Atomic Structure");
    assert_eq!(chapters[1].order, 2);
    assert_eq!(chapters[1].title, "Chemical Bonding");
}

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn update_chapter_succeeds() {
    use std::sync::Arc;

    use leptos_question_bank::{
        domain::class::AddClassInput,
        server_function::academic_helper::{
            chapter::{edit_chapter, insert_chapter},
            class::insert_class,
            subject::insert_subject,
        },
    };

    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let pool = Arc::new(app.db_pool.clone());

    let class = insert_class(
        pool.clone(),
        AddClassInput {
            name: "Class 12".to_string(),
            name_bn: "দ্বাদশ শ্রেণি".to_string(),
        },
    )
    .await
    .expect("Failed to insert class");

    let subject = insert_subject(pool.clone(), "Physics".to_string(), class.id)
        .await
        .expect("Failed to insert subject");

    let chapter = insert_chapter(pool.clone(), "Old Chapter".to_string(), subject.id)
        .await
        .expect("Failed to insert chapter");

    // Update chapter
    let updated = edit_chapter(pool, "New Chapter".to_string(), chapter.id)
        .await
        .expect("Failed to update chapter");

    assert_eq!(updated.id, chapter.id);
    assert_eq!(updated.title, "New Chapter");
    assert_eq!(updated.subject_id, subject.id);
    assert_eq!(updated.class_id, class.id);
    assert_eq!(updated.order, chapter.order);
}

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn chapters_are_scoped_to_subject() {
    use std::sync::Arc;

    use leptos_question_bank::{
        domain::class::AddClassInput,
        server_function::academic_helper::{
            chapter::{insert_chapter, subject_chapter},
            class::insert_class,
            subject::insert_subject,
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

    let math = insert_subject(pool.clone(), "Math".to_string(), class.id)
        .await
        .expect("Failed to insert Math");
    let english = insert_subject(pool.clone(), "English".to_string(), class.id)
        .await
        .expect("Failed to insert English");

    // Insert chapter in Math
    insert_chapter(pool.clone(), "Algebra".to_string(), math.id)
        .await
        .expect("Failed to insert Algebra");
    // Insert chapter in English
    insert_chapter(pool.clone(), "Grammar".to_string(), english.id)
        .await
        .expect("Failed to insert Grammar");

    // Math should only have Algebra
    let math_chapters = subject_chapter(math.id)
        .await
        .expect("Failed to fetch Math chapters");
    assert_eq!(math_chapters.len(), 1);
    assert_eq!(math_chapters[0].title, "Algebra");

    // English should only have Grammar
    let english_chapters = subject_chapter(english.id)
        .await
        .expect("Failed to fetch English chapters");
    assert_eq!(english_chapters.len(), 1);
    assert_eq!(english_chapters[0].title, "Grammar");
}
