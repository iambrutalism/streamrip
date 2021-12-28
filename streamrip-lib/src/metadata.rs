use crate::JsonMap;

pub struct AlbumMetadata {
    // Included in tags
    // pub album: String,
    pub albumartist: String,
    // pub artist: String,
    pub albumcomposer: Option<String>,
    pub comment: Option<String>,
    pub compilation: Option<String>,
    pub composer: Option<String>,
    pub copyright: Option<String>,
    pub cover: Option<String>,
    pub date: Option<String>,
    pub description: Option<String>,
    pub disctotal: Option<u8>,
    // pub encoder: Option<String>,
    // pub grouping: Option<String>,
    pub lyrics: Option<String>,
    // pub purchase_date: Option<String>,
    pub title: String,
    pub tracktotal: u16,
    pub isrc: Option<String>,
    pub genre: Option<String>,

    // Other information
    /// In Hz
    pub sampling_rate: Option<u32>,
    /// In bits per sample
    pub bit_depth: Option<u8>,
    pub booklets: Option<String>,
    /// Sizes: thumbnail
    pub cover_urls: [Option<String>; 4],
    pub work: Option<String>,
    pub id: Option<String>,
}

impl AlbumMetadata {
    pub fn from_qobuz(meta: JsonMap) {}
}

/// Inherits all the information from it's album's
/// `AlbumMetadata`. This is track specific metadata.
pub struct TrackMetadata {
    pub title: String,
    pub artist: String,
    pub tracknumber: u16,
    pub discnumber: u8,
    pub lyrics: Option<String>,

    /// False if there is no explicit tag
    pub explicit: bool,
    pub id: String,
}
