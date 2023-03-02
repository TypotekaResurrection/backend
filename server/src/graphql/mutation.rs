use entity::async_graphql;

pub mod article;

pub use article::ArticleMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(ArticleMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(ArticleMutation);