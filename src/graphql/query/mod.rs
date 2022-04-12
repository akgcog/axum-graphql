use entity::async_graphql;

pub mod user;
pub mod followed;

pub use note::NoteQuery;
pub use user::UserQuery;
pub use followed::FollowedQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(NoteQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(NoteQuery, UserQuery, FollowedQuery);
