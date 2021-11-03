use std::{
    // collections::HashMap,
    io::{self, Read},
    path::{Path, PathBuf},
    sync::RwLock,
};

use bevy::{
    asset::{AssetIo, AssetIoError, BoxedFuture},
    prelude::*,
};

struct AssetIoTar {
    default_io: Box<dyn AssetIo>,
    archive: RwLock<tar::Archive<io::Cursor<Vec<u8>>>>,
    // cached_name_map: HashMap<PathBuf, usize>,
}

#[derive(Clone)]
pub struct AssetIoTarConfig(pub Vec<u8>);

impl AssetIo for AssetIoTar {
    fn load_path<'a>(&'a self, path: &'a Path) -> BoxedFuture<'a, Result<Vec<u8>, AssetIoError>> {
        Box::pin(async move {
            // TODO use resource for caching?
            // if let Some(index) = self.cached_name_map.get(&path.to_path_buf()) {
            //     if let Some(Ok(mut entry)) =
            //         self.archive.write().unwrap().entries().unwrap().nth(*index)
            //     {
            //         let mut res = vec![0; entry.size() as usize];
            //         entry.read_exact(&mut res).unwrap();
            //         return Ok(res);
            //     }
            // }
            if let Some((_index, mut entry)) = self
                .archive
                .write()
                .unwrap()
                .entries()
                .unwrap()
                .enumerate()
                .filter_map(|(index, entry)| entry.ok().map(|e| (index, e)))
                .find(|(_index, entry)| entry.path().unwrap() == path)
            {
                let mut res = vec![0; entry.size() as usize];
                entry.read_exact(&mut res).unwrap();
                // self.cached_name_map.insert(path.to_path_buf(), index);
                Ok(res)
            } else {
                Err(AssetIoError::NotFound(path.to_owned()))
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
        self.default_io.is_directory(path)
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

            AssetIoTar {
                default_io,
                archive: RwLock::new(tar::Archive::new(io::Cursor::new(config.0))),
                // cached_name_map: HashMap::new(),
            }
        };

        app.insert_resource(AssetServer::new(asset_io, task_pool));
    }
}
