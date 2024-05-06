use std::{collections::HashMap as Map, fs::File, io::BufReader, path::Path};

use exif::{Reader, Tag, Value};
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

/// Find the [EXIF tag][Tag] by a description.
///
/// TODO: find a better way to enumerate well-known constants
pub fn find_exif_tag(description: &str) -> Option<Tag> {
    let lower_description = description.to_lowercase();
    let contexts = [
        exif::Context::Exif,
        exif::Context::Gps,
        exif::Context::Tiff,
        exif::Context::Interop,
    ];
    for ctx in contexts {
        for i in 0..=u16::MAX {
            let tag = Tag(ctx, i);
            if let Some(desc) = tag.description() {
                let desc = desc.to_lowercase();
                if desc == lower_description {
                    return Some(tag);
                }
            }
        }
    }
    None
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
