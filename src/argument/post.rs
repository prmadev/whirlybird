use std::fmt::Debug;

use redmaple::redmaple::id::ID;

/// Content type sets the mode of each content.
#[derive(Debug, Clone)]
pub enum Mode {
    /// The main post of the redmaple
    HeadPost,
    /// Answer the redmaple, or optionally, answer to another content of the same redmaple
    Conversation(Option<ID>),
    /// A new edition for the headpost
    Edition,
}

/// Content holds the different forms of content
#[derive(Debug, Clone)]
pub enum Post {
    /// A Text content is a string.
    Text { id: ID },
    /// A Text content is a subscription to a picture stream.
    Picture { id: ID },
}
impl Post {
    pub const fn id(&self) -> &ID {
        match self {
            Self::Text { id } | Self::Picture { id } => id,
        }
    }
}
