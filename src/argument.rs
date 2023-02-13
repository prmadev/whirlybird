//! This is an implementation of the `EventGroup` used as an exmaple of the basic logic neccessary,
//! to create your own event type

use redmaple::{event_group::EventGroup, id::ID};

use self::{maple_created::Created, views::BlogMode};

mod maple_created;
mod post;
mod post_added;
mod post_deleted;
mod post_moded;
mod post_published;
pub mod views;

/// Event hold all the events that could happened to a `RedMaple`
#[derive(Debug, Clone)]
pub enum Argument {
    /// States that a RedMaple is created
    Created(maple_created::Created<views::Views>),
    /// When a content is added. It does not neccessarily means that it is published
    PostAdded(post_added::PostCreated),
    /// Happens When a content is visible by all those that can view the RedMaple
    PostPublished(post_published::ContentPublished),
    /// Happens when the view mod of the post changes
    PostModed(post_moded::ContentModed),
    /// Happens when the content is no longer visible
    PostDeleted(post_deleted::PostDeleted),
}

impl EventGroup for Argument {
    fn id(&self) -> &ID {
        match *self {
            Self::Created(ref e) => e.id(),
            Self::PostAdded(ref e) => e.id(),
            Self::PostPublished(ref e) => e.id(),
            Self::PostModed(ref e) => e.id(),
            Self::PostDeleted(ref e) => e.id(),
        }
    }

    fn redmaple_id(&self) -> &ID {
        match *self {
            Self::Created(ref e) => e.redmaple_id(),
            Self::PostAdded(ref e) => e.redmaple_id(),
            Self::PostPublished(ref e) => e.redmaple_id(),
            Self::PostModed(ref e) => e.redmaple_id(),
            Self::PostDeleted(ref e) => e.redmaple_id(),
        }
    }

    fn time(&self) -> &std::time::SystemTime {
        match *self {
            Self::Created(ref e) => e.created(),
            Self::PostAdded(ref e) => e.created(),
            Self::PostPublished(ref e) => e.created(),
            Self::PostModed(ref e) => e.created(),
            Self::PostDeleted(ref e) => e.created(),
        }
    }

    fn has_the_same_contents(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Created(a), Self::Created(b)) => a.redmaple_id() == b.redmaple_id(),
            (Self::PostAdded(a), Self::PostAdded(b)) => a.post() == b.post(),
            (Self::PostDeleted(a), Self::PostDeleted(b)) => a.post_id() == b.post_id(),
            (Self::PostModed(a), Self::PostModed(b)) => {
                a.post_id() == b.post_id() && a.new_mod() == b.new_mod()
            }
            (Self::PostPublished(a), Self::PostPublished(b)) => a.post_id() == b.post_id(),
            (_, _) => false,
        }
    }
}

impl Argument {
    /// Creates a new instance of `Argument::Created`
    #[must_use]
    pub fn new_create_event() -> Self {
        Self::Created(Created::new(views::Views::Blog(BlogMode::Text), ID::new()))
    }
}
