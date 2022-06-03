//! Main library entry point for openapi_client implementation.

use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub(crate) struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}