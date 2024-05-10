use std::{env, fmt, fs, path::PathBuf};

use log::info;
use serde::{Deserialize, Serialize};

use foton::{TimeFormat, TimeSource};

use super::AnyError;

/// Configuration values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub library: Vec<PathBuf>,
    pub foton_tagged_dir: Option<PathBuf>,
    pub metadata: Option<MetadataExtractorConfig>,
}

/// Settings for getting relevant metadata from media.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataExtractorConfig {
    pub time_source: Vec<TimeSource>,
}

const CONFIG_NAME: &str = "foton.toml";

impl Config {
    /// List paths to locate a config file.
    ///
    /// The first one that gets found overwrites
    /// all the others.
    pub fn locations() -> impl Iterator<Item = PathBuf> {
        env::current_dir()
            .map(|cd| cd.join(CONFIG_NAME))
            .into_iter()
            .chain(home::home_dir().map(|hd| hd.join(CONFIG_NAME)))
    }

    /// Generate an example [`Config`]
    /// to create stub config file.
    pub fn stub() -> Self {
        Self {
            library: home::home_dir()
                .map(|hd| hd.join("Photos"))
                .into_iter()
                .collect(),
            foton_tagged_dir: home::home_dir().map(|hd| hd.join("Photos").join("tagged")),
            metadata: Some(MetadataExtractorConfig {
                time_source: vec![
                    TimeSource::FileName {
                        format: TimeFormat {
                            fmt: "IMG_%Y%m%d_%H%M%S".into(),
                            take_prefix: Some(19),
                            only_date: false,
                        },
                    },
                    TimeSource::FileName {
                        format: TimeFormat {
                            fmt: "GIF_%Y%m%d_%H%M%S".into(),
                            take_prefix: Some(19),
                            only_date: false,
                        },
                    },
                    TimeSource::FileName {
                        format: TimeFormat::from("VID_%Y%m%d_%H%M%S.mp4"),
                    },
                    TimeSource::FileName {
                        format: TimeFormat::from("%Y%m%d_%H%M%S.jpg"),
                    },
                    TimeSource::Tag {
                        name: "Date and time of original data generation".into(),
                        format: TimeFormat::from("%F %T"),
                    },
                    TimeSource::Tag {
                        name: "creation_time".into(),
                        format: TimeFormat::from("%+"),
                    },
                    TimeSource::Tag {
                        name: "Date and time of digital data generation".into(),
                        format: TimeFormat::from("%F %T"),
                    },
                    TimeSource::Tag {
                        name: "File change date and time".into(),
                        format: TimeFormat::from("%F %T"),
                    },
                    TimeSource::Tag {
                        name: "GPS date".into(),
                        format: TimeFormat {
                            fmt: "%F".into(),
                            take_prefix: None,
                            only_date: true,
                        },
                    },
                    TimeSource::FileName {
                        format: TimeFormat {
                            fmt: "IMG_%Y%m%d".into(),
                            take_prefix: Some(12),
                            only_date: true,
                        },
                    },
                    TimeSource::FileName {
                        format: TimeFormat {
                            fmt: "IMG-%Y%m%d".into(),
                            take_prefix: Some(12),
                            only_date: true,
                        },
                    },
                ],
            }),
        }
    }

    /// Load the config file from
    /// the listed [locations][Self::locations].
    pub fn load() -> Result<Option<Self>, AnyError> {
        for path in Self::locations() {
            if path.exists() {
                info!("Loading config from {}", path.display());
                let conf_file = fs::read_to_string(path)?;
                return Ok(Some(toml::from_str(&conf_file)?));
            }
        }
        Ok(None)
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = toml::to_string(self).map_err(|_| fmt::Error)?;
        f.write_str(&s)
    }
}
