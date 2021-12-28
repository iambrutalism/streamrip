pub const AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:83.0) Gecko/20100101 Firefox/83.0";

pub const TIDAL_COVER_URL: &str = "https://resources.tidal.com/images/{uuid}/{width}x{height}.jpg";
pub const SOUNDCLOUD_CLIENT_ID: &str = "qHsjZaNbdTcABbiIQnVfW07cEPGLNjIh";
pub const SOUNDCLOUD_USER_ID: &str = "672320-86895-162383-801513";
pub const SOUNDCLOUD_APP_VERSION: &str = "1630917744";

pub const QUALITY_DESC: [&str; 5] = [
    "128kbps",
    "320kbps",
    "16bit/44.1kHz",
    "24bit/96kHz",
    "24bit/192kHz",
];

pub const QOBUZ_FEATURED_KEYS: [&str; 15] = [
    "most-streamed",
    "recent-releases",
    "best-sellers",
    "press-awards",
    "ideal-discography",
    "editor-picks",
    "most-featured",
    "qobuzissims",
    "new-releases",
    "new-releases-full",
    "harmonia-mundi",
    "universal-classic",
    "universal-jazz",
    "universal-jeunesse",
    "universal-chanson",
];

const __MP4_KEYS: [Option<&str>; 21] = [
    Some("©nam"),
    Some("©ART"),
    Some("©alb"),
    Some("aART"),
    Some("©day"),
    Some("©day"),
    Some("©cmt"),
    Some("desc"),
    Some("purd"),
    Some("©grp"),
    Some("©gen"),
    Some("©lyr"),
    Some("©too"),
    Some("cprt"),
    Some("cpil"),
    Some("covr"),
    Some("trkn"),
    Some("disk"),
    None,
    None,
    None,
];

const __METADATA_TYPES: [&str; 21] = [
    "title",
    "artist",
    "album",
    "albumartist",
    "composer",
    "year",
    "comment",
    "description",
    "purchase_date",
    "grouping",
    "genre",
    "lyrics",
    "encoder",
    "copyright",
    "compilation",
    "cover",
    "tracknumber",
    "discnumber",
    "tracktotal",
    "disctotal",
    "date",
];

pub const COPYRIGHT: char = '℗';
pub const PHON_COPYRIGHT: char = '©';

pub const FLAC_MAX_BLOCKSIZE: u32 = 16777215; // 16.7 MB

pub const TRACK_KEYS: [&str; 7] = [
    "tracknumber",
    "artist",
    "albumartist",
    "composer",
    "title",
    "albumcomposer",
    "explicit",
];
pub const ALBUM_KEYS: [&str; 8] = [
    "albumartist",
    "title",
    "year",
    "bit_depth",
    "sampling_rate",
    "container",
    "albumcomposer",
    "id",
];
pub const FOLDER_FORMAT: &str =
    "{albumartist} - {title} ({year}) [{container}] [{bit_depth}B-{sampling_rate}kHz]";

pub const TRACK_FORMAT: &str = "{tracknumber}. {artist} - {title}";

pub const DEEZER_MAX_Q: u32 = 6;
pub const DEEZER_FEATURED_KEYS: [&str; 3] = ["releases", "charts", "selection"];
// pub const AVAILABLE_QUALITY_IDS = (0, 1, 2, 3, 4);
pub const DEEZER_FORMATS: [&str; 6] = ["AAC_64", "MP3_64", "MP3_128", "MP3_256", "MP3_320", "FLAC"];
// video only for tidal
pub const MEDIA_TYPES: [&str; 6] = ["track", "album", "artist", "label", "playlist", "video"];

// used to homogenize cover size keys
pub const COVER_SIZES: [&str; 4] = ["thumbnail", "small", "large", "original"];

pub const TIDAL_CLIENT_INFO: [&str; 2] = [
    "Pzd0ExNVHkyZLiYN",
    "W7X6UvBaho+XOi1MUeCX6ewv2zTdSOV3Y7qC3p3675I=",
];

pub const QOBUZ_BASE: &str = "https://www.qobuz.com/api.json/0.2/";
pub const TIDAL_BASE: &str = "https://api.tidalhifi.com/v1";
pub const TIDAL_AUTH_URL: &str = "https://auth.tidal.com/v1/oauth2";
pub const DEEZER_BASE: &str = "https://api.deezer.com";
pub const DEEZER_DL: &str = "http://dz.loaderapp.info/deezer";
pub const SOUNDCLOUD_BASE: &str = "https://api-v2.soundcloud.com";

// 1-indexed
pub const QOBUZ_QUALITY_MAP: [u8; 4] = [5, 6, 7, 27];

pub const TIDAL_CLIENT_ID: &str = "Pzd0ExNVHkyZLiYN";
pub const TIDAL_CLIENT_SECRET: &str = "W7X6UvBaho+XOi1MUeCX6ewv2zTdSOV3Y7qC3p3675I=";
