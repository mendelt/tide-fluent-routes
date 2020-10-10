use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    sync::Arc,
};
use tide::Endpoint;
use tide::{utils::async_trait, Middleware};

/// Implement some useful stuff around Box<dyn Endpoint>
pub(crate) struct BoxedEndpoint<State>(Box<dyn Endpoint<State>>);

impl<State: Clone + Send + Sync + 'static> BoxedEndpoint<State> {
    /// Wrap an endpoint in a BoxedEndpoint
    pub(crate) fn new(endpoint: impl Endpoint<State>) -> Self {
        Self(Box::new(endpoint))
    }
}

impl<State> Debug for BoxedEndpoint<State> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.debug_struct("BoxedEndpoint").finish()
    }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Endpoint<State> for BoxedEndpoint<State> {
    async fn call(&self, req: tide::Request<State>) -> tide::Result {
        self.0.call(req).await
    }
}

/// Implement some useful stuff around Arc<dyn Middleware>
pub(crate) struct ArcMiddleware<State>(Arc<dyn Middleware<State>>);

impl<State: Clone + Send + Sync + 'static> ArcMiddleware<State> {
    /// Wrap an endpoint in a BoxedEndpoint
    pub(crate) fn new(ware: impl Middleware<State>) -> Self {
        Self(Arc::new(ware))
    }
}

impl<State> Debug for ArcMiddleware<State> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.debug_struct("ArcMiddleware").finish()
    }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for ArcMiddleware<State> {
    async fn handle(
        &self,
        request: tide::Request<State>,
        next: tide::Next<'_, State>,
    ) -> tide::Result {
        self.0.handle(request, next).await
    }
}
