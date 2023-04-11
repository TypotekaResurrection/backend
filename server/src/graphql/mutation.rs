use entity::async_graphql;

pub mod article;
mod registration;
mod comment;
mod category;
mod delete_result;
mod category_article;

pub use article::ArticleMutation;
pub use registration::UserMutation;
pub use comment::CommentMutation;
pub use category::CategoryMutation;
pub use category_article::CategoryArticleMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(ArticleMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(ArticleMutation, UserMutation, CommentMutation, CategoryMutation, CategoryArticleMutation);
