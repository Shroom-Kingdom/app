use std::{
    collections::HashMap,
    io::{self, Read},
    path::{Path, PathBuf},
};

use bevy::{
    asset::{AssetIo, AssetIoError, BoxedFuture},
    prelude::*,
};

struct AssetIoTar {
    default_io: Box<dyn AssetIo>,
    archive: HashMap<PathBuf, Vec<u8>>,
}

#[derive(Clone)]
pub struct AssetIoTarConfig(pub Vec<u8>);

impl AssetIo for AssetIoTar {
    fn load_path<'a>(&'a self, path: &'a Path) -> BoxedFuture<'a, Result<Vec<u8>, AssetIoError>> {
        Box::pin(async move {
            if let Some(res) = self.archive.get(path) {
                Ok(res.clone())
            } else {
                self.default_io.load_path(path).await
            }
        })
    }

    fn read_directory(
        &self,
        path: &Path,
    ) -> Result<Box<dyn Iterator<Item = PathBuf>>, AssetIoError> {
        self.default_io.read_directory(path)
    }

    fn is_directory(&self, path: &Path) -> bool {
        self.archive.keys().any(|k| k.starts_with(path))
    }

    fn watch_path_for_changes(&self, path: &Path) -> Result<(), AssetIoError> {
        self.default_io.watch_path_for_changes(path)
    }

    fn watch_for_changes(&self) -> Result<(), AssetIoError> {
        self.default_io.watch_for_changes()
    }
}

#[derive(Default)]
pub struct AssetIoTarPlugin;

impl Plugin for AssetIoTarPlugin {
    fn build(&self, app: &mut App) {
        let task_pool = app
            .world
            .get_resource::<bevy::tasks::IoTaskPool>()
            .expect("`IoTaskPool` resource not found.")
            .0
            .clone();

        let asset_io = {
            let default_io = bevy::asset::create_platform_default_asset_io(app);

            let config = app
                .world
                .get_resource::<AssetIoTarConfig>()
                .map(|x| (*x).clone())
                .unwrap();

            let mut archive = HashMap::new();
            let mut tar = tar::Archive::new(io::Cursor::new(config.0));
            for entry in tar.entries().unwrap() {
                let mut entry = entry.unwrap();
                let mut res = vec![0; entry.size() as usize];
                entry.read_exact(&mut res).unwrap();
                archive.insert(entry.path().unwrap().into(), res);
            }

            AssetIoTar {
                default_io,
                archive,
            }
        };

        app.insert_resource(AssetServer::new(asset_io, task_pool));
    }
}
