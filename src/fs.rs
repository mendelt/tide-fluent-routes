//! Extension traits and endpoints for serving content from the file system

use crate::RouteBuilder;
use std::io;
use std::path::Path;
use tide::{Request, Result};

/// Extension method for the routebuilder to serve a directory
pub trait ServeDir<State: Clone + Send + Sync + 'static>: RouteBuilder<State> {
    /// Serve a directory at a location
    fn serve_dir(&mut self, _dir: impl AsRef<Path>) -> io::Result<Self> {
        todo!()
    }
}

impl<State: Clone + Send + Sync + 'static, R: RouteBuilder<State>> ServeDir<State> for R {}

/// Endpoint method for serving directories, prefix is the name of the prefix parameter, path is the path to serve
fn _serve_dir_endpoint<State>(_prefix: &str, _path: &str, _req: Request<State>) -> Result<State> {
    todo!()
}

/// Extension method for the routebuilder to serve a single file
pub trait ServeFile<State: Clone + Send + Sync + 'static>: RouteBuilder<State> {
    /// Same as serve_dir, but a single file
    fn serve_file(&mut self, _file: impl AsRef<Path>) -> io::Result<Self> {
        todo!()
    }
}

impl<State: Clone + Send + Sync + 'static, R: RouteBuilder<State>> ServeFile<State> for R {}

/// Endpoint method for serving files, path is the path to the file to serve
fn _serve_file_endpoint<State>(_path: &str, _req: Request<State>) -> Result<State> {
    todo!()
}
