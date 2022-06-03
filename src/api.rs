//! Main library entry point for openapi_client implementation.

use async_trait::async_trait;
use log::info;
use swagger::{Has, XSpanIdString};
use swagger::ApiError;
use openapi_client::models;
use crate::server::Server;

use openapi_client::{
    Api,
    AddCommentResponse,
    GetCommentResponse,
};

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Add a new comment to the podcast
    async fn add_comment(
        &self,
        comment: Option<models::Comment>,
        context: &C) -> Result<AddCommentResponse, ApiError>
    {
        let context = context.clone();
        info!("add_comment({:?}) - X-Span-ID: {:?}", comment, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get all comments
    async fn get_comment(
        &self,
        context: &C) -> Result<GetCommentResponse, ApiError>
    {
        let context = context.clone();
        info!("get_comment() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

}
