#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn insert_chapter_succeeds() {
    use domain::class::{AddClassInput, ChapterInput, SubjectInput};
    use infra::settings::SettingService;

    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let service = SettingService::from(app.ctx);

    // Create class + subject
    let class = service
        .add_class(AddClassInput {
            name: "Class 11".to_string(),
            name_bn: "একাদশ শ্রেণি".to_string(),
        })
        .await
        .expect("Failed to insert class");

    let subject = service
        .add_subject(SubjectInput {
            title: "Biology".to_string(),
            id: class.id,
        })
        .await
        .expect("Failed to insert subject");

    // Insert a chapter
    let chapter = service
        .add_chapter(ChapterInput {
            title: "Cell Structure".to_string(),
            id: subject.id,
        })
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
    use domain::class::{AddClassInput, ChapterInput, SubjectInput};
    use infra::settings::SettingService;

    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let service = SettingService::from(app.ctx.clone());

    let class = service
        .add_class(AddClassInput {
            name: "Class 12".to_string(),
            name_bn: "দ্বাদশ শ্রেণি".to_string(),
        })
        .await
        .expect("Failed to insert class");

    let subject = service
        .add_subject(SubjectInput {
            title: "Chemistry".to_string(),
            id: class.id,
        })
        .await
        .expect("Failed to insert subject");

    // Insert two chapters
    service
        .add_chapter(ChapterInput {
            title: "Atomic Structure".to_string(),
            id: subject.id,
        })
        .await
        .expect("Failed to insert chapter 1");

    service
        .add_chapter(ChapterInput {
            id: subject.id,
            title: "Chemical Bonding".to_string(),
        })
        .await
        .expect("Failed to insert chapter 2");

    let chapters = service
        .chapters(subject.id)
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
    use domain::class::{AddClassInput, ChapterInput, SubjectInput};
    use infra::settings::SettingService;

    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let service = SettingService::from(app.ctx.clone());

    let class = service
        .add_class(AddClassInput {
            name: "Class 12".to_string(),
            name_bn: "দ্বাদশ শ্রেণি".to_string(),
        })
        .await
        .expect("Failed to insert class");

    let subject = service
        .add_subject(SubjectInput {
            title: "Physics".to_string(),
            id: class.id,
        })
        .await
        .expect("Failed to insert subject");

    let chapter = service
        .add_chapter(ChapterInput {
            title: "Old Chapter".to_string(),
            id: subject.id,
        })
        .await
        .expect("Failed to insert chapter");

    // Update chapter
    let updated = service
        .edit_chapter(ChapterInput {
            id: chapter.id,
            title: "New Chapter".to_string(),
        })
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
    use domain::class::AddClassInput;
    use domain::class::{ChapterInput, SubjectInput};
    use infra::settings::SettingService;

    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let service = SettingService::from(app.ctx.clone());

    let class = service
        .add_class(AddClassInput {
            name: "Class 10".to_string(),
            name_bn: "দশম শ্রেণি".to_string(),
        })
        .await
        .expect("Failed to insert class");

    let math = service
        .add_subject(SubjectInput {
            title: "Math".to_string(),
            id: class.id,
        })
        .await
        .expect("Failed to insert Math");

    let english = service
        .add_subject(SubjectInput {
            id: class.id,
            title: "English".to_string(),
        })
        .await
        .expect("Failed to insert English");

    // Insert chapter in Math
    service
        .add_chapter(ChapterInput {
            title: "Algebra".to_string(),
            id: math.id,
        })
        .await
        .expect("Failed to insert Algebra");

    // Insert chapter in English
    service
        .add_chapter(ChapterInput {
            id: english.id,
            title: "Grammar".to_string(),
        })
        .await
        .expect("Failed to insert Grammar");

    // Math should only have Algebra
    let math_chapters = service
        .chapters(math.id)
        .await
        .expect("Failed to fetch Math chapters");
    assert_eq!(math_chapters.len(), 1);
    assert_eq!(math_chapters[0].title, "Algebra");

    // English should only have Grammar
    let english_chapters = service
        .chapters(english.id)
        .await
        .expect("Failed to fetch English chapters");
    assert_eq!(english_chapters.len(), 1);
    assert_eq!(english_chapters[0].title, "Grammar");
}
