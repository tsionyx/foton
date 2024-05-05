use std::{collections::HashMap as Map, fs::File, io::BufReader, path::Path};

use exif::{Reader, Value};
use log::warn;
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

/// Get distribution of tags presence on a number of files.
pub fn get_tags_distribution(it: impl Iterator<Item = Media>) -> Map<String, u32> {
    let mut all_tags = Map::new();
    for resource in it {
        match resource.get_tags() {
            Ok(map) => {
                for (k, _) in map {
                    all_tags.entry(k).and_modify(|x| *x += 1).or_insert(1);
                }
            }
            Err(err) => {
                warn!("{}: {:?}", resource, err);
            }
        }
    }
    all_tags
}

/// Get distribution of a specific tag's values on a number of files.
pub fn get_tag_values_distribution(tag: &str, it: impl Iterator<Item = Media>) -> Map<String, u32> {
    let mut all_tags = Map::new();
    for resource in it {
        match resource.get_tags() {
            Ok(map) => {
                if let Some(val) = map.get(tag) {
                    all_tags
                        .entry(val.clone())
                        .and_modify(|x| *x += 1)
                        .or_insert(1);
                }
            }
            Err(err) => {
                warn!("{}: {:?}", resource, err);
            }
        }
    }
    all_tags
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
