use std::{borrow::Cow, collections::HashMap};

use chrono::NaiveDateTime;
use log::warn;
use serde::Deserialize;

use crate::file_types::Media;

#[derive(Debug, Clone, Eq, PartialEq)]
/// Describes the time a media file was shot.
pub struct Time {
    inner: NaiveDateTime,
    source: InfoSource,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
/// Source of Time for a media file.
pub enum InfoSource {
    /// A lot of cameras produce files with the datetime in their names.
    ///
    /// This is the most convenient and probably the most reliable
    /// way of fetching datetime.
    FileName {
        /// How to parse the datetime.
        format: Format,
    },

    /// Find datetime in a tag.
    Tag {
        /// Which tag name to use.
        name: String,
        /// How to parse the datetime.
        format: Format,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
/// Format for parsing datetime.
///
/// See the [docs][chrono::format::strftime].
pub struct Format(pub String);

impl Media {
    fn get_value<'s, 't>(
        &'s self,
        source: &InfoSource,
        cached_tags: Option<&'t HashMap<String, String>>,
    ) -> Option<Cow<str>>
    where
        't: 's,
    {
        match source {
            InfoSource::FileName { .. } => self.path().file_name()?.to_str().map(Cow::Borrowed),
            InfoSource::Tag { name, .. } => {
                let all_tags = cached_tags.map(Cow::Borrowed).or_else(|| {
                    self.get_tags()
                        .map_err(|err| {
                            warn!("Failed to get tags for datetime: {:?}", err);
                        })
                        .ok()
                        .map(Cow::Owned)
                })?;
                match all_tags {
                    Cow::Borrowed(cached) => {
                        cached.get(name).map(String::as_ref).map(Cow::Borrowed)
                    }
                    Cow::Owned(mut tags) => tags.remove(name).map(Cow::Owned),
                }
            }
        }
    }

    /// Retrieves DateTime from the media metadata.
    ///
    /// For better performance, the cached tags collection could be provided.
    pub fn get_datetime_from_source(
        &self,
        source: &InfoSource,
        cached_tags: Option<&HashMap<String, String>>,
    ) -> Option<NaiveDateTime> {
        let value = self.get_value(source, cached_tags)?;
        let format = match source {
            InfoSource::FileName { format } | InfoSource::Tag { format, .. } => format,
        };
        NaiveDateTime::parse_from_str(&value, &format.0).ok()
    }

    /// Retrieves DateTime from the media metadata
    /// using multiple sources till success.
    pub fn get_datetime(&self, sources: &[InfoSource]) -> Option<Time> {
        let all_tags = self
            .get_tags()
            .map_err(|err| {
                warn!("Failed to get tags for datetime: {:?}", err);
            })
            .ok();

        sources.iter().find_map(|source| {
            let dt = self.get_datetime_from_source(source, all_tags.as_ref())?;
            Some(Time {
                inner: dt,
                source: source.clone(),
            })
        })
    }
}
