use redmaple::redmaple::{id::ID, RedMaple};

use super::{post::Post, views::Views, Argument};

/// Sets a Content as published
#[derive(Debug, Clone)]
pub struct ContentPublished {
    id: ID,
    redmaple_id: ID,
    post_id: ID,
}

impl ContentPublished {
    pub fn new(red_maple: &RedMaple<Argument, Views>, post: &Post) -> Self {
        Self {
            id: ID::new(),
            redmaple_id: red_maple.id().clone(),
            post_id: post.id().clone(),
        }
    }

    pub const fn redmaple_id(&self) -> &ID {
        &self.redmaple_id
    }

    pub const fn post_id(&self) -> &ID {
        &self.post_id
    }

    pub const fn id(&self) -> &ID {
        &self.id
    }
}
