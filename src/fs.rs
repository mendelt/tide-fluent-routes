//! Extension traits and endpoints for serving content from the file system

use tide::Response;
use tide::Body;
use std::ffi::OsStr;
use crate::prelude::*;
use std::io;
use std::path::{Path, PathBuf};
use tide::{Request, Result, StatusCode};
use async_std::path::PathBuf as AsyncPathBuf;
use log;

/// Extension method for the routebuilder to serve a directory
pub trait ServeDir<State: Clone + Send + Sync + 'static>: RouteBuilder<State> {
    /// Serve a directory at a location
    fn serve_dir(self, dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir = dir.as_ref().to_owned().canonicalize()?;

        Ok(self.at("*path", |route| route.get( move |req| async { serve_dir_endpoint("*path", dir.clone(), req).await})))
    }
}

impl<State: Clone + Send + Sync + 'static, R: RouteBuilder<State>> ServeDir<State> for R {}

/// Endpoint method for serving directories, prefix is the name of the prefix parameter, path is the path to serve
async fn serve_dir_endpoint<State>(prefix: &str, dir_path: PathBuf, req: Request<State>) -> Result {
    // let path = req.url().path();
    // let path = path.trim_start_matches(&self.prefix);

    let path = req.param(prefix)?.trim_start_matches('/');

    let mut dir_path = dir_path.clone();
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

/// Extension method for the routebuilder to serve a single file
pub trait ServeFile<State: Clone + Send + Sync + 'static>: RouteBuilder<State> {
    /// Same as serve_dir, but a single file
    fn serve_file(&mut self, file: impl AsRef<Path>) -> io::Result<Self> {
        let file_path = file.as_ref().to_owned().canonicalize()?;

        Ok(self.at("*path", |route| route.get( move |req| async { serve_file_endpoint(file_path.clone(), req).await})))
    }
}

impl<State: Clone + Send + Sync + 'static, R: RouteBuilder<State>> ServeFile<State> for R {}

/// Endpoint method for serving files, path is the path to the file to serve
async fn serve_file_endpoint<State>(file_path: PathBuf, _req: Request<State>) -> Result {
    let file_path = AsyncPathBuf::from(file_path);

    if !file_path.exists().await {
        log::warn!("File not found: {:?}", file_path);
        Ok(Response::new(StatusCode::NotFound))
    }
    else {
        let body = Body::from_file(&file_path).await?;
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(body);
        Ok(res)
    }
}
