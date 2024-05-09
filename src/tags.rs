use std::{collections::HashMap as Map, fs::File, io::BufReader, path::Path};

use exif::{Reader, Value};
use once_cell::sync::Lazy;

use crate::{file_types::Media, AnyError, MediaType};

/// Get tags of an image by its path.
pub fn get_image_tags<P>(path: &P) -> Result<Map<String, Value>, AnyError>
where
    P: AsRef<Path> + ?Sized,
{
    let mut file = BufReader::new(File::open(path)?);
    let exif = Reader::new().read_from_container(&mut file)?;
    Ok(exif
        .fields()
        .filter_map(|f| {
            f.tag
                .description()
                .map(|desc| (desc.to_string(), f.value.clone()))
        })
        .collect())
}

#[derive(Debug, Copy, Clone)]
struct FFMpeg;

impl FFMpeg {
    fn get_metadata<P>(self, path: &P) -> Result<Map<String, String>, ffmpeg_next::Error>
    where
        P: AsRef<Path> + ?Sized,
    {
        ffmpeg_next::format::input(path).map(|ctx| {
            ctx.metadata()
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect()
        })
    }
}

static FFMPEG: Lazy<FFMpeg> = Lazy::new(|| {
    ffmpeg_next::init().expect("could not initialize ffmpeg");
    FFMpeg
});

impl Media {
    /// Fetch the tags' collection from a media.
    pub fn get_tags(&self) -> Result<Map<String, String>, AnyError> {
        match self.type_ {
            MediaType::Photo | MediaType::Animation => get_image_tags(self.path()).map(|tags| {
                tags.into_iter()
                    .map(|(k, v)| (k, format!("{:?}", v)))
                    .collect()
            }),
            MediaType::Video => {
                let md = FFMPEG.get_metadata(self.path())?;
                Ok(md)
            }
        }
    }
}
