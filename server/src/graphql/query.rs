use entity::async_graphql;

pub mod article;
pub mod user;
pub mod comment;
pub mod category;

pub use article::ArticleQuery;
pub use user::UserQuery;
pub use comment::CommentQuery;
pub use category::CategoryQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(ArticleQuery, OtherArticle, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(ArticleQuery, UserQuery, CommentQuery, CategoryQuery);
