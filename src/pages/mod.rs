mod add_edit_quesiton;
mod class_setting;
mod home;
mod login;
mod page_not_found;
mod signup;
mod unauthorized;
mod view_questions;

pub use class_setting::ClassSettingRoute;

pub use add_edit_quesiton::AddEditQuestion;
pub use home::HomePage;
pub use login::Login;
pub use page_not_found::NotFound;
pub use signup::Signup;
pub use unauthorized::UnAuthorized;
pub use view_questions::ViewQuestions;
