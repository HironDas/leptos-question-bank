use infra::settings::SettingService;

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn insert_subject_succeeds() {
    use domain::class::{AddClassInput, SubjectInput};
    use infra::settings::SettingService;

    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let ctx = app.ctx;

    let service = SettingService::from(ctx);
    // Create a class first
    let class = service
        .add_class(AddClassInput {
            name: "Class 8".to_string(),
            name_bn: "অষ্টম শ্রেণি".to_string(),
        })
        .await
        .expect("Failed to insert class");

    // Insert a subject under that class
    let subject = service
        .add_subject(SubjectInput {
            title: "Mathematics".to_string(),
            id: class.id,
        })
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
    use domain::class::{AddClassInput, SubjectInput};
    use infra::settings::SettingService;
    use leptos_question_bank::server_function::academic_setting::subjects;

    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let service = SettingService::from(app.ctx);

    // Create a class
    let class = service
        .add_class(AddClassInput {
            name: "Class 9".to_string(),
            name_bn: "নবম শ্রেণি".to_string(),
        })
        .await
        .expect("Failed to insert class");

    // Insert two subjects
    service
        .add_subject(SubjectInput {
            title: "Mathematics".to_string(),
            id: class.id,
        })
        .await
        .expect("Failed to insert Math");

    service
        .add_subject(SubjectInput {
            title: "Physics".to_string(),
            id: class.id,
        })
        .await
        .expect("Failed to insert Physics");

    // Fetch subjects for this class
    let subjects = subjects(class.id).await.expect("Failed to fetch subjects");

    assert_eq!(subjects.len(), 2);
    assert_eq!(subjects[0].order, 1);
    assert_eq!(subjects[1].order, 2);
    assert_eq!(subjects[0].title, "Mathematics");
    assert_eq!(subjects[1].title, "Physics");
}

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn update_subject_succeeds() {
    use domain::class::{AddClassInput, SubjectInput};

    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let service = SettingService::from(app.ctx);

    let class = service
        .add_class(AddClassInput {
            name: "Class 10".to_string(),
            name_bn: "দশম শ্রেণি".to_string(),
        })
        .await
        .expect("Failed to insert class");

    let subject = service
        .add_subject(SubjectInput {
            title: "Old Subject".to_string(),
            id: class.id,
        })
        .await
        .expect("Failed to insert subject");

    // Update the subject
    let updated = service
        .edit_subject(SubjectInput {
            id: subject.id,
            title: "New Subject".to_string(),
        })
        .await
        .expect("Failed to update subject");

    assert_eq!(updated.id, subject.id);
    assert_eq!(updated.title, "New Subject");
    assert_eq!(updated.class_id, class.id);
}

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn subjects_are_scoped_to_class() {
    use domain::class::{AddClassInput, SubjectInput};

    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let service = SettingService::from(app.ctx);

    // Create two classes
    let class_a = service
        .add_class(AddClassInput {
            name: "Class A".to_string(),
            name_bn: "ক্লাস ক".to_string(),
        })
        .await
        .expect("Failed to insert class A");

    let class_b = service
        .add_class(AddClassInput {
            name: "Class B".to_string(),
            name_bn: "ক্লাস খ".to_string(),
        })
        .await
        .expect("Failed to insert class B");

    // Insert subject in class A
    service
        .add_subject(SubjectInput {
            title: "Subject A".to_string(),
            id: class_a.id,
        })
        .await
        .expect("Failed to insert subject in A");

    // Insert subject in class B
    service
        .add_subject(SubjectInput {
            title: "Subject B".to_string(),
            id: class_b.id,
        })
        .await
        .expect("Failed to insert subject in B");

    // Class A should only have its own subject
    let subjects_a = service
        .subjects(class_a.id)
        .await
        .expect("Failed to fetch subjects for A");
    assert_eq!(subjects_a.len(), 1);
    assert_eq!(subjects_a[0].title, "Subject A");

    // Class B should only have its own subject
    let subjects_b = service
        .subjects(class_b.id)
        .await
        .expect("Failed to fetch subjects for B");
    assert_eq!(subjects_b.len(), 1);
    assert_eq!(subjects_b[0].title, "Subject B");
}
