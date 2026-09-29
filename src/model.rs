use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Track {
    pub path: PathBuf,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<Duration>,
}

impl Track {
    pub fn from_path(path: PathBuf) -> Self{
        let title = path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("Unkown Track")
            .to_string();

        Self {
            path,
            title,
            artist: None,
            album: None,
            duration: None
        }
    }
}

//TODO ALBUM
//pub struct Album {
//}
