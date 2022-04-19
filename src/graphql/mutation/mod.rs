use entity::async_graphql;

pub mod user;
pub mod follow;
pub mod article;
pub mod thanks;

pub use user::UserMutation;
pub use follow::FollowMutation;
pub use article::ArticleMutation;
pub use thanks::ThanksMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(NoteMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation( UserMutation, ArticleMutation, FollowMutation, ThanksMutation);

#[derive(async_graphql::SimpleObject)]
pub struct DeleteResult {
    pub success: bool,
    pub rows_affected: u64,
}