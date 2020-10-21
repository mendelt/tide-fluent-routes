//! Extension traits and endpoints for serving content from the file system

use crate::prelude::*;
use async_std::path::PathBuf as AsyncPathBuf;
use log;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use tide::Body;
use tide::Response;
use tide::{utils::async_trait, Endpoint};
use tide::{Request, Result, StatusCode};

/// Extension method for the routebuilder to serve a directory
pub trait ServeDir<State: Clone + Send + Sync + 'static>: RouteBuilder<State> {
    /// Serve a directory at a location
    fn serve_dir(self, dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir_path = dir.as_ref().to_owned().canonicalize()?;

        Ok(self.at("*path", |route| {
            route.get(ServeDirEndpoint {
                dir_path,
                prefix: "*path".to_string(),
            })
        }))
    }
}

impl<State: Clone + Send + Sync + 'static, R: RouteBuilder<State>> ServeDir<State> for R {}

struct ServeDirEndpoint {
    dir_path: PathBuf,
    prefix: String,
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Endpoint<State> for ServeDirEndpoint {
    async fn call(&self, req: Request<State>) -> Result {
        let path = req.param(&self.prefix)?.trim_start_matches('/');

        let mut dir_path = self.dir_path.clone();
        for p in Path::new(path) {
            if p == OsStr::new(".") {
                continue;
            } else if p == OsStr::new("..") {
                dir_path.pop();
            } else {
                dir_path.push(&p);
            }
        }

        log::info!("Requested file: {:?}", dir_path);

        let file_path = AsyncPathBuf::from(dir_path);
        // if !file_path.starts_with(&self.dir) {
        //     log::warn!("Unauthorized attempt to read: {:?}", file_path);
        //     return Ok(Response::new(StatusCode::Forbidden));
        // }
        if !file_path.exists().await {
            log::warn!("File not found: {:?}", file_path);
            return Ok(Response::new(StatusCode::NotFound));
        }
        let body = Body::from_file(&file_path).await?;
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(body);
        Ok(res)
    }
}

/// Extension method for the routebuilder to serve a single file
pub trait ServeFile<State: Clone + Send + Sync + 'static>: RouteBuilder<State> {
    /// Same as serve_dir, but a single file
    fn serve_file(self, file: impl AsRef<Path>) -> io::Result<Self> {
        let file_path = file.as_ref().to_owned().canonicalize()?;

        Ok(self.get(ServeFileEndpoint { file_path }))
    }
}

impl<State: Clone + Send + Sync + 'static, R: RouteBuilder<State>> ServeFile<State> for R {}

/// Endpoint method for serving files, path is the path to the file to serve
struct ServeFileEndpoint {
    file_path: PathBuf,
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Endpoint<State> for ServeFileEndpoint {
    async fn call(&self, _req: Request<State>) -> Result {
        let file_path = AsyncPathBuf::from(self.file_path.clone());

        if !file_path.exists().await {
            log::warn!("File not found: {:?}", self.file_path);
            Ok(Response::new(StatusCode::NotFound))
        } else {
            let body = Body::from_file(&file_path).await?;
            let mut res = Response::new(StatusCode::Ok);
            res.set_body(body);
            Ok(res)
        }
    }
}
