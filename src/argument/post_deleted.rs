use redmaple::redmaple::{id::ID, RedMaple};

use super::{post::Post, views::Views, Argument};

/// Sets a Content as published
#[derive(Debug, Clone)]
pub struct PostDeleted {
    id: ID,
    redmaple_id: ID,
    post_id: ID,
}

impl PostDeleted {
    /// Creates an event that states that some content has been deleted (invisible) to users.
    pub fn new(red_maple: &RedMaple<Argument, Views>, post: &Post) -> Self {
        Self {
            id: ID::new(),
            redmaple_id: red_maple.id().clone(),
            post_id: post.id().clone(),
        }
    }

    /// Gets the ID of the entity
    pub const fn id(&self) -> &ID {
        &self.id
    }

    /// Gets the ID of the redmaple that holds this event
    pub const fn redmaple_id(&self) -> &ID {
        &self.redmaple_id
    }

    /// Gets the inner content ID that this event is effecting on
    pub const fn post_id(&self) -> &ID {
        &self.post_id
    }
}
