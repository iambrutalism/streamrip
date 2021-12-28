use std::fmt;

pub trait StreamripClient {}

#[derive(Debug)]
pub enum MediaType {
    Album,
    Artist,
    Label,
    Playlist,
    Track,
    Video,
}

impl MediaType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Album => "album".to_string(),
            Self::Artist => "artist".to_string(),
            Self::Label => "label".to_string(),
            Self::Playlist => "playlist".to_string(),
            Self::Track => "track".to_string(),
            Self::Video => "video".to_string(),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Album => "album",
            Self::Artist => "artist",
            Self::Label => "label",
            Self::Playlist => "playlist",
            Self::Track => "track",
            Self::Video => "video",
        }
    }
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
