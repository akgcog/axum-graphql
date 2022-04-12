use entity::async_graphql;

pub mod user;
pub mod followed;

pub use note::NoteMutation;
pub use user::UserMutation;
pub use followed::FollowedMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(NoteMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(NoteMutation, UserMutation, FollowedMutation);

#[derive(async_graphql::SimpleObject)]
pub struct DeleteResult {
    pub success: bool,
    pub rows_affected: u64,
}