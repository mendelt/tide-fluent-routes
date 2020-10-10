use std::fmt::{Debug, Formatter, Result as FmtResult};
use tide::utils::async_trait;
use tide::Endpoint;

pub(crate) struct BoxedEndpoint<State>(Box<dyn Endpoint<State>>);

impl<State> Debug for BoxedEndpoint<State> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.debug_struct("BoxedEndpoint").finish()
    }
}

impl<State: Clone + Send + Sync + 'static> BoxedEndpoint<State> {
    /// Wrap an endpoint in a BoxedEndpoint
    pub(crate) fn new(endpoint: impl Endpoint<State>) -> Self {
        Self(Box::new(endpoint))
    }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Endpoint<State> for BoxedEndpoint<State> {
    async fn call(&self, req: tide::Request<State>) -> tide::Result {
        self.0.call(req).await
    }
}
