use std::{collections::HashMap as Map, fs::File, io::BufReader, path::Path};

use exif::{Reader, Value};
use log::{info, warn};
use once_cell::sync::Lazy;

use crate::AnyError;

/// Get tags of an image by its path.
pub fn get_image_tags<P>(path: &P) -> Result<Map<String, Value>, AnyError>
where
    P: AsRef<Path>,
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
pub fn get_tags_distribution<P>(it: impl Iterator<Item = P>) -> Map<String, u32>
where
    P: AsRef<Path>,
{
    let mut all_tags = Map::new();
    for path in it {
        match get_image_tags(&path) {
            Ok(map) => {
                for (k, _) in map {
                    all_tags.entry(k).and_modify(|x| *x += 1).or_insert(1);
                }
            }
            Err(err) => {
                let path = path.as_ref();
                info!("{}: {:?}", path.display(), err);
                match get_video_tags(path) {
                    Ok(map) => {
                        for (k, _) in map {
                            all_tags.entry(k).and_modify(|x| *x += 1).or_insert(1);
                        }
                    }
                    Err(err) => {
                        warn!("{}: {:?}", path.display(), err);
                    }
                }
            }
        }
    }
    all_tags
}

/// Get distribution of a specific tag's values on a number of files.
pub fn get_tag_values_distribution<P>(tag: &str, it: impl Iterator<Item = P>) -> Map<String, u32>
where
    P: AsRef<Path>,
{
    let mut all_tags = Map::new();
    for path in it {
        match get_image_tags(&path) {
            Ok(map) => {
                if let Some(val) = map.get(tag) {
                    let val = format!("{:?}", val);
                    all_tags.entry(val).and_modify(|x| *x += 1).or_insert(1);
                }
            }
            Err(err) => {
                let path = path.as_ref();
                info!("{}: {:?}", path.display(), err);
                match get_video_tags(path) {
                    Ok(map) => {
                        if let Some(val) = map.get(tag) {
                            let val = format!("{:?}", val);
                            all_tags.entry(val).and_modify(|x| *x += 1).or_insert(1);
                        }
                    }
                    Err(err) => {
                        warn!("{}: {:?}", path.display(), err);
                    }
                }
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

/// Get tags of a video by its path.
pub fn get_video_tags(file: &Path) -> Result<Map<String, String>, AnyError> {
    let md = FFMPEG.get_metadata(file)?;
    Ok(md)
}
