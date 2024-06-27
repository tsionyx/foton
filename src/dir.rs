//! Operations with filesystem.
use std::{path::PathBuf, rc::Rc};

use walkdir::WalkDir;

use crate::file_types::{Media, MediaType};

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
    fn iter_extensions(
        &self,
        extensions: Vec<(MediaType, &'static str)>,
    ) -> impl Iterator<Item = Media> + '_ {
        let extensions_shared = Rc::new(extensions);
        self.paths.iter().flat_map(move |root| {
            let extensions = Rc::clone(&extensions_shared);
            WalkDir::new(root).into_iter().filter_map(move |entry| {
                let entry = entry.ok()?.into_path();
                if entry.is_dir() {
                    return None;
                }
                let ext = entry.extension()?.to_str()?.to_ascii_lowercase();
                let type_ = extensions
                    .iter()
                    .find_map(|(type_, extension)| (extension == &ext).then_some(*type_));
                type_.map(|type_| Media { type_, path: entry })
            })
        })
    }

    /// Iter all files of a given [`MediaType`] in a [`Library`].
    pub fn iter_type(&self, resource_type: MediaType) -> impl Iterator<Item = Media> + '_ {
        let extensions = resource_type
            .supported_extensions()
            .into_iter()
            .map(|ext| (resource_type, ext))
            .collect();
        self.iter_extensions(extensions)
    }

    /// Iter files of all supported [`MediaType`]s in a [`Library`].
    pub fn iter_all(&self) -> impl Iterator<Item = Media> + '_ {
        let extensions: Vec<_> = enum_iterator::all::<MediaType>()
            .flat_map(|resource_type| {
                resource_type
                    .supported_extensions()
                    .into_iter()
                    .map(move |ext| (resource_type, ext))
            })
            .collect();
        self.iter_extensions(extensions)
    }

    /// Iter files of a particular [`MediaType`]s or all supported [`MediaType`]s in a [`Library`].
    pub fn iter(
        &self,
        resource_type: impl Into<Option<MediaType>>,
    ) -> Box<dyn Iterator<Item = Media> + '_> {
        if let Some(resource_type) = resource_type.into() {
            Box::new(self.iter_type(resource_type))
        } else {
            Box::new(self.iter_all())
        }
    }
}
