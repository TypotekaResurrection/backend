use entity::async_graphql;

pub mod article;
pub use article::ArticleQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(ArticleQuery, OtherArticle, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(ArticleQuery);