use redmaple::{id::ID, RedMaple};

use super::{post::Post, views::Views, Argument};

/// Adds Content to that redmaple
#[derive(Debug, Clone)]
pub struct PostCreated {
    id: ID,
    redmaple_id: ID,
    post: Post,
}

impl PostCreated {
    /// Creates an event that states that some content has been added to an existing `RedMaple`.
    pub fn new(red_maple: &RedMaple<Argument, Views>, post: Post) -> Self {
        Self {
            id: ID::new(),
            redmaple_id: red_maple.id().clone(),
            post,
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

    /// Gets the inner content that is represented by this event
    pub const fn content(&self) -> &Post {
        &self.post
    }
}
