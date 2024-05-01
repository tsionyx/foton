//! Operations with filesystem.
use std::{path::PathBuf, rc::Rc};

use walkdir::WalkDir;

use crate::file_types::MediaType;

#[derive(Debug)]
/// Filesystem entry point(s) for your photo collection.
pub struct Library {
    paths: Vec<PathBuf>,
}

impl Library {
    /// Create a [`Library`] given a path.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self::with_paths(vec![root.into()])
    }

    /// Create a [`Library`] given multiple paths.
    pub fn with_paths(paths: Vec<PathBuf>) -> Self {
        Self { paths }
    }

    /// Iter all files with given extensions in a [`Library`].
    fn iter_extensions(&self, extensions: Vec<&'static str>) -> impl Iterator<Item = PathBuf> + '_ {
        let extensions_shared = Rc::new(extensions);
        self.paths.iter().flat_map(move |root| {
            let extensions = Rc::clone(&extensions_shared);
            WalkDir::new(root).into_iter().filter_map(move |entry| {
                let entry = entry.ok()?.into_path();
                if entry.is_dir() {
                    return None;
                }
                let ext = entry.extension()?.to_str()?.to_ascii_lowercase();
                extensions.contains(&ext.as_str()).then_some(entry)
            })
        })
    }

    /// Iter all files of a given [`MediaType`] in a [`Library`].
    pub fn iter(&self, resource_type: MediaType) -> impl Iterator<Item = PathBuf> + '_ {
        let extensions = resource_type.supported_extensions();
        self.iter_extensions(extensions)
    }

    /// Iter files of all supported [`MediaType`]s in a [`Library`].
    pub fn iter_all(&self) -> impl Iterator<Item = PathBuf> + '_ {
        let extensions: Vec<_> = enum_iterator::all::<MediaType>()
            .flat_map(|resource_type| resource_type.supported_extensions())
            .collect();
        self.iter_extensions(extensions)
    }
}
