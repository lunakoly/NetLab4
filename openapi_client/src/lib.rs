#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &'static str = "";
pub const API_VERSION: &'static str = "1.0.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetMyselfResponse {
    /// Here is your status
    HereIsYourStatus
    (models::InlineResponse2001)
    ,
    /// Your request is junk
    YourRequestIsJunk
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetNewUserResponse {
    /// Here is the default identity
    HereIsTheDefaultIdentity
    (models::InlineResponse200)
    ,
    /// Your request is junk
    YourRequestIsJunk
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PostQueryResponse {
    /// Here is the kill result
    HereIsTheKillResult
    (models::KillResult)
    ,
    /// Here is a new role for you
    HereIsANewRoleForYou
    (models::Role)
    ,
    /// Here are the files
    HereAreTheFiles
    (models::FilesList)
    ,
    /// Here are your crewmates
    HereAreYourCrewmates
    (models::UsersList)
    ,
    /// Some random information
    SomeRandomInformation
    (models::Notification)
    ,
    /// Here is the new location
    HereIsTheNewLocation
    (models::MoveTo)
    ,
    /// Your request is junk
    YourRequestIsJunk
    ,
    /// You are dead
    YouAreDead
}

/// API
#[async_trait]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Returns your status
    async fn get_myself(
        &self,
        context: &C) -> Result<GetMyselfResponse, ApiError>;

    /// Get the initial user context
    async fn get_new_user(
        &self,
        context: &C) -> Result<GetNewUserResponse, ApiError>;

    /// Run a command
    async fn post_query(
        &self,
        request_body: models::Query,
        context: &C) -> Result<PostQueryResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Returns your status
    async fn get_myself(
        &self,
        ) -> Result<GetMyselfResponse, ApiError>;

    /// Get the initial user context
    async fn get_new_user(
        &self,
        ) -> Result<GetNewUserResponse, ApiError>;

    /// Run a command
    async fn post_query(
        &self,
        request_body: models::Query,
        ) -> Result<PostQueryResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self: Self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Returns your status
    async fn get_myself(
        &self,
        ) -> Result<GetMyselfResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_myself(&context).await
    }

    /// Get the initial user context
    async fn get_new_user(
        &self,
        ) -> Result<GetNewUserResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_new_user(&context).await
    }

    /// Run a command
    async fn post_query(
        &self,
        request_body: models::Query,
        ) -> Result<PostQueryResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().post_query(request_body, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
