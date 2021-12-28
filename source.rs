pub enum Source {
    Qobuz,
    Deezer,
    Tidal,
    SoundCloud,
    Deezloader,
}

impl Source {
    pub fn from_name(name: &str) {
        match name {
            "qobuz" => Self::Qobuz,
            "deezer" => Self::Deezer,
            "tidal" => Self::Tidal,
            "soundcloud" => Self::SoundCloud,
            "deezloader" => Self::Deezloader,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Qobuz => "qobuz",
            Self::Deezer => "deezer",
            Self::Tidal => "tidal",
            Self::SoundCloud => "soundcloud",
            Self::Deezloader => "deezloader",
        }
    }
}
