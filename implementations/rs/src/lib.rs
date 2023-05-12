use std::{fs, path::Path, sync::Arc};
use crate::wrap::wrap_info::get_manifest;

use polywrap_core::invoker::Invoker;
use polywrap_plugin::{error::PluginError, implementor::plugin_impl, JSON};
use wrap::module::{
    ArgsExists, ArgsMkdir, ArgsReadFile, ArgsReadFileAsString, ArgsRm, ArgsRmdir, ArgsWriteFile,
    Module,
};
pub mod wrap;

#[derive(Debug)]
pub struct FileSystemPlugin {}

#[plugin_impl]
impl Module for FileSystemPlugin {
    fn read_file(
        &mut self,
        args: &ArgsReadFile,
        _: Arc<dyn Invoker>,
    ) -> Result<Vec<u8>, PluginError> {
        fs::read(&args.path).map_err(|e| PluginError::ModuleError(e.to_string()))
    }

    fn read_file_as_string(
        &mut self,
        args: &ArgsReadFileAsString,
        _: Arc<dyn Invoker>,
    ) -> Result<String, PluginError> {
        // TODO: Add encoding
        fs::read_to_string(&args.path).map_err(|e| PluginError::ModuleError(e.to_string()))
    }

    fn exists(&mut self, args: &ArgsExists, _: Arc<dyn Invoker>) -> Result<bool, PluginError> {
        Ok(Path::new(&args.path).exists())
    }

    fn write_file(
        &mut self,
        args: &ArgsWriteFile,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<bool>, PluginError> {
        fs::write(Path::new(&args.path), &args.data).unwrap();

        Ok(Some(true))
    }

    fn mkdir(
        &mut self,
        args: &ArgsMkdir,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<bool>, PluginError> {
        let recursive = if let Some(recursive) = args.recursive {
            recursive
        } else {
            false
        };

        let path = Path::new(&args.path);

        if recursive {
            fs::create_dir_all(path).unwrap();
        } else {
            fs::create_dir(path).unwrap();
        }

        Ok(Some(true))
    }

    fn rm(&mut self, args: &ArgsRm, _: Arc<dyn Invoker>) -> Result<Option<bool>, PluginError> {
        let recursive = if let Some(recursive) = args.recursive {
            recursive
        } else {
            false
        };

        let force = if let Some(force) = args.force {
            force
        } else {
            false
        };

        let path = Path::new(&args.path);

        if path.is_dir() {
            if force {
                rm_rf::ensure_removed(path).unwrap();
            } else if recursive {
                fs::remove_dir_all(path).unwrap();
            } else {
                fs::remove_dir(path).unwrap();
            }
        } else {
            fs::remove_file(path).unwrap();
        }

        Ok(Some(true))
    }

    fn rmdir(
        &mut self,
        args: &ArgsRmdir,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<bool>, PluginError> {
        fs::remove_dir(&args.path).unwrap();

        Ok(Some(true))
    }
}
