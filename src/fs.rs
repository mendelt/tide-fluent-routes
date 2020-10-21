//! Extension traits and endpoints for serving content from the file system

use crate::prelude::*;
use async_std::path::PathBuf as AsyncPathBuf;
use log;
use std::ffi::OsStr;
use std::io;
use std::path::Path;
use tide::Body;
use tide::Response;
use tide::{utils::async_trait, Endpoint};
use tide::{Request, Result, StatusCode};

/// Extension methods for the routebuilder to serving files and directories
pub trait ServeFs<State: Clone + Send + Sync + 'static>: RouteBuilder<State> {
    /// Serve a directory at a location
    fn serve_dir(self, dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir_path = AsyncPathBuf::from(dir.as_ref().to_owned().canonicalize()?);

        Ok(self.at("*path", |route| {
            route.get(ServeDirEndpoint {
                dir_path,
                prefix: "path".to_string(),
            })
        }))
    }

    /// Same as serve_dir, but for a single file
    fn serve_file(self, file: impl AsRef<Path>) -> io::Result<Self> {
        let file_path = AsyncPathBuf::from(file.as_ref().to_owned().canonicalize()?);

        Ok(self.get(ServeFileEndpoint { file_path }))
    }
}

impl<State: Clone + Send + Sync + 'static, R: RouteBuilder<State>> ServeFs<State> for R {}

struct ServeDirEndpoint {
    dir_path: AsyncPathBuf,
    prefix: String,
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Endpoint<State> for ServeDirEndpoint {
    async fn call(&self, req: Request<State>) -> Result {
        let path = req.param(&self.prefix)?.trim_start_matches('/');

        let mut file_path = self.dir_path.clone();
        for p in Path::new(path) {
            if p == OsStr::new(".") {
                continue;
            } else if p == OsStr::new("..") {
                file_path.pop();
            } else {
                file_path.push(&p);
            }
        }

        log::info!("Requested file: {:?}", file_path);

        if !file_path.starts_with(&self.dir_path) {
            log::warn!("Unauthorized attempt to read: {:?}", file_path);
            Ok(Response::new(StatusCode::Forbidden))
        } else if !file_path.exists().await {
            log::warn!("File not found: {:?}", file_path);
            Ok(Response::new(StatusCode::NotFound))
        } else {
            Ok(Response::builder(StatusCode::Ok)
                .body(Body::from_file(&file_path).await?)
                .build())
        }
    }
}

/// Endpoint method for serving files, path is the path to the file to serve
struct ServeFileEndpoint {
    file_path: AsyncPathBuf,
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Endpoint<State> for ServeFileEndpoint {
    async fn call(&self, _req: Request<State>) -> Result {
        if !self.file_path.exists().await {
            log::warn!("File not found: {:?}", self.file_path);
            Ok(Response::new(StatusCode::NotFound))
        } else {
            Ok(Response::builder(StatusCode::Ok)
                .body(Body::from_file(&self.file_path).await?)
                .build())
        }
    }
}
