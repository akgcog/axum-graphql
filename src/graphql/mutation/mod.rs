use entity::async_graphql;

pub mod user;
pub mod follows;
pub mod article;

pub use user::UserMutation;
// pub use follows::FollowedMutation;
pub use article::ArticleMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(NoteMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation( UserMutation, ArticleMutation);

#[derive(async_graphql::SimpleObject)]
pub struct DeleteResult {
    pub success: bool,
    pub rows_affected: u64,
}