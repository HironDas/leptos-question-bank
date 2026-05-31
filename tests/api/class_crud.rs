#[cfg(feature = "test-fullstack")]
use crate::helpers::{spawn_app, TestApp};

// #[cfg(feature = "test-fullstack")]
// async fn create_user_and_class(app: &TestApp) -> (u32, String) {
//     use std::sync::Arc;

//     use leptos_question_bank::{
//         domain::class::AddClassInput,
//         server_function::{
//             academic_helper::class::insert_class,
//             signup::{insert_new_user, User},
//         },
//     };

//     let pool = Arc::new(app.db_pool.clone());

//     // Create a user first (may be needed for auth context)
//     let user = User {
//         username: "class_test".to_string(),
//         email: "class_test@example.com".to_string(),
//         password: "Hiron@123".to_string(),
//         confirm_password: "Hiron@123".to_string(),
//     };
//     let new_user = user.try_into().unwrap();
//     insert_new_user(new_user, pool.clone())
//         .await
//         .expect("Failed to create test user");

//     // Insert a class
//     let class_input = AddClassInput {
//         name: "Class 9".to_string(),
//         name_bn: "নবম শ্রেণি".to_string(),
//     };
//     let class = insert_class(pool.clone(), class_input)
//         .await
//         .expect("Failed to insert class");

//     (class.id, class.name)
// }

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn insert_class_succeeds() {
    let app = spawn_app().await;
    use std::sync::Arc;

    use leptos_question_bank::{
        domain::class::AddClassInput, server_function::academic_helper::class::insert_class,
    };

    let pool = Arc::new(app.db_pool.clone());
    let class_input = AddClassInput {
        name: "Class 10".to_string(),
        name_bn: "দশম শ্রেণি".to_string(),
    };

    let class = insert_class(pool, class_input)
        .await
        .expect("Failed to insert class");

    assert!(class.id > 0);
    assert_eq!(class.name, "Class 10");
    assert_eq!(class.name_bn, "দশম শ্রেণি");
    assert_eq!(class.order, 1);
}

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn fetch_classes_returns_inserted_classes() {
    let app = spawn_app().await;
    use std::sync::Arc;

    use leptos_question_bank::{
        domain::class::AddClassInput,
        server_function::academic_helper::class::{class, insert_class},
    };

    let pool = Arc::new(app.db_pool.clone());

    // Insert two classes
    let class1 = AddClassInput {
        name: "Class 6".to_string(),
        name_bn: "ষষ্ঠ শ্রেণি".to_string(),
    };
    insert_class(pool.clone(), class1)
        .await
        .expect("Failed to insert class 1");

    let class2 = AddClassInput {
        name: "Class 7".to_string(),
        name_bn: "সপ্তম শ্রেণি".to_string(),
    };
    insert_class(pool.clone(), class2)
        .await
        .expect("Failed to insert class 2");

    // Fetch all classes
    let classes = class(pool.clone()).await.expect("Failed to fetch classes");

    assert_eq!(classes.len(), 2);
    assert_eq!(classes[0].order, 1);
    assert_eq!(classes[1].order, 2);
    // Classes should be ordered by "order" ASC
    assert_eq!(classes[0].name, "Class 6");
    assert_eq!(classes[1].name, "Class 7");
}

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn update_class_succeeds() {
    let app = spawn_app().await;
    use std::sync::Arc;

    use leptos_question_bank::{
        domain::class::{AddClassInput, UpdateClassInput},
        server_function::academic_helper::class::{edit_class, insert_class},
    };

    let pool = Arc::new(app.db_pool.clone());

    // Insert initial class
    let class_input = AddClassInput {
        name: "Old Name".to_string(),
        name_bn: "পুরাতন নাম".to_string(),
    };
    let class = insert_class(pool.clone(), class_input)
        .await
        .expect("Failed to insert class");

    // Update the class
    let update = UpdateClassInput {
        class_id: class.id,
        name: "New Name".to_string(),
        name_bn: "নতুন নাম".to_string(),
    };

    let updated = edit_class(pool, update)
        .await
        .expect("Failed to update class");

    assert_eq!(updated.id, class.id);
    assert_eq!(updated.name, "New Name");
    assert_eq!(updated.name_bn, "নতুন নাম");
    // Order should remain unchanged
    assert_eq!(updated.order, class.order);
}

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn insert_multiple_classes_auto_increments_order() {
    let app = spawn_app().await;
    use std::sync::Arc;

    use leptos_question_bank::{
        domain::class::AddClassInput, server_function::academic_helper::class::insert_class,
    };

    let pool = Arc::new(app.db_pool.clone());

    for i in 1..=5 {
        let class_input = AddClassInput {
            name: format!("Class {}", i),
            name_bn: format!("ক্লাস {}", i),
        };
        let class = insert_class(pool.clone(), class_input)
            .await
            .expect("Failed to insert class");
        assert_eq!(class.order, i);
    }
}
