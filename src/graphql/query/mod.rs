use entity::async_graphql;

pub mod user;
pub mod follows;
pub mod article;

pub use user::UserQuery;
// pub use follows::FollowedQuery;
pub use article::ArticleQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(NoteQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query( UserQuery, ArticleQuery);
