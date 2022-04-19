use entity::async_graphql;

pub mod user;
pub mod follow;
pub mod article;
pub mod thanks;

pub use user::UserQuery;
pub use follow::FollowQuery;
pub use article::ArticleQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(NoteQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query( UserQuery, ArticleQuery, FollowQuery);
