//! `views` implement `ViewMode`s that are suitable for `Argument` implementation of `RedMaple`

use redmaple::view_mode::ViewMode;

/// Holds the different view modes that the `RedMaple` could present
#[derive(Clone, Debug)]
#[must_use]
pub enum Views {
    /// means one big post up, and editions for that post + comments and replies
    Blog(BlogMode),
    /// conversation means a series of talks that two or more people could have, responding to
    /// each other
    Conversation,
    /// a series of links that reflect a replied conversations. reflecting and responding to each
    /// other.
    ResponseLinks,
    /// a curated list of links that hold the same theme
    CuratedList,
    /// a list of todo items representing a task progress
    TodoList,
}

impl ViewMode for Views {
    fn name(&self) -> String {
        match self {
            Self::Blog(m) => match m {
                BlogMode::Text => "BlogText".to_owned(),
                BlogMode::PhotoSlide => "BlogPhotoSlide".to_owned(),
                BlogMode::Video => "BlogVideo".to_owned(),
            },
            Self::Conversation => "Conversation".to_owned(),
            Self::ResponseLinks => "ResponseLinks".to_owned(),
            Self::CuratedList => "CuratedList".to_owned(),
            Self::TodoList => "TodoList".to_owned(),
        }
    }
}

/// blogs could form different views, on could stress on the text while the other could present a
/// series of photos or a video
#[derive(Clone, Debug)]
pub enum BlogMode {
    /// Text is the traditional essay blogging form
    Text,
    /// PhotoSlide does not neccessarily means that the post should have an slider, rather, the
    /// focus is the photos taken.
    PhotoSlide,
    /// Video represent a video stream as the main post
    Video,
}
