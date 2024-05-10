use std::{borrow::Cow, collections::HashMap};

use chrono::{NaiveDate, NaiveDateTime};
use log::warn;
use serde::{Deserialize, Serialize};

use crate::file_types::Media;

#[derive(Debug, Clone, Eq, PartialEq)]
/// Describes the time a media file was shot.
pub struct Time {
    inner: NaiveDateTime,
    source: InfoSource,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
/// How to extract the datetime from the string.
pub struct Format {
    /// Format for parsing datetime.
    ///
    /// See the [docs][chrono::format::strftime].
    pub fmt: String,

    /// If specified, only the prefix of the given string
    /// would be used for matching against the format.
    pub take_prefix: Option<usize>,

    #[serde(default)]
    /// If set, do not try to parse the whole datetime,
    /// but only the date (00:00:00 will be assumed).
    pub only_date: bool,
}

impl From<String> for Format {
    fn from(value: String) -> Self {
        Self {
            fmt: value,
            take_prefix: None,
            only_date: false,
        }
    }
}

impl From<&str> for Format {
    fn from(value: &str) -> Self {
        Self::from(value.to_owned())
    }
}

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
                            warn!("Failed to get datetime tags for {}: {:?}", self, err);
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
        let value = if let Some(n) = format.take_prefix {
            value.chars().take(n).collect()
        } else {
            value
        };

        if format.only_date {
            let date = NaiveDate::parse_from_str(&value, &format.fmt).ok()?;
            date.and_hms_opt(0, 0, 0)
        } else {
            NaiveDateTime::parse_from_str(&value, &format.fmt).ok()
        }
    }

    /// Retrieves DateTime from the media metadata
    /// using multiple sources till success.
    pub fn get_datetime(&self, sources: &[InfoSource]) -> Option<Time> {
        let all_tags = self
            .get_tags()
            .map_err(|err| {
                warn!("Failed to get datetime tags for {}: {:?}", self, err);
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
