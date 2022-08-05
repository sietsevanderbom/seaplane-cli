//! The `/restrict` endpoint APIs which allows working with [`Restriction`]s
pub mod models;
use std::str::FromStr;

pub use models::*;

use reqwest::Url;

use crate::{
    api::{
        map_api_error,
        v1::{ApiRequest, RangeQueryContext, RequestBuilder},
        METADATA_API_URL,
    },
    error::{Result, SeaplaneError},
};

/// A builder struct for creating a [`RestrictRequest`] which will then be used for making a
/// request against the `/restrict` APIs
#[derive(Debug)]
pub struct RestrictRequestBuilder {
    builder: RequestBuilder<RequestTarget>,
}

impl From<RequestBuilder<RequestTarget>> for RestrictRequestBuilder {
    fn from(builder: RequestBuilder<RequestTarget>) -> Self {
        Self { builder }
    }
}

impl Default for RestrictRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RestrictRequestBuilder {
    /// Create a new RestrictRequestBuilder
    pub fn new() -> Self {
        RequestBuilder::new(METADATA_API_URL, "v1/restrict/").into()
    }

    /// Build a RestrictRequest from the given parameters
    pub fn build(self) -> Result<RestrictRequest> {
        Ok(self.builder.build()?.into())
    }

    /// Set the token used in Bearer Authorization
    ///
    /// **NOTE:** This is required for all endpoints
    #[must_use]
    pub fn token<U: Into<String>>(self, token: U) -> Self {
        self.builder.token(token).into()
    }

    // Used in testing and development to manually set the URL
    #[doc(hidden)]
    pub fn base_url<U: AsRef<str>>(self, url: U) -> Self {
        self.builder.base_url(url).into()
    }

    /// The restricted directory, encoded in url-safe base64.
    ///
    /// **NOTE:** This is not required for all endpoints
    #[must_use]
    pub fn single_restriction<S: Into<String>>(mut self, api: S, directory: S) -> Self {
        self.builder.target = Some(RequestTarget::Single {
            api: api.into(),
            directory: RestrictedDirectory::from_encoded(directory.into()),
        });
        self
    }

    /// The context with which to perform a range query within an API
    ///
    /// **NOTE:** This is not required for all endpoints
    #[must_use]
    pub fn api_range<S: Into<String>>(
        mut self,
        api: S,
        context: RangeQueryContext<RestrictedDirectory>,
    ) -> Self {
        self.builder.target = Some(RequestTarget::ApiRange {
            api: api.into(),
            context,
        });
        self
    }

    /// The context with which to perform a range query across all APIs
    ///
    /// **NOTE:** This is not required for all endpoints
    #[must_use]
    pub fn all_range<S: Into<String>>(
        mut self,
        from_api: Option<S>,
        context: RangeQueryContext<RestrictedDirectory>,
    ) -> Self {
        self.builder.target = Some(RequestTarget::AllRange {
            from_api: from_api.map(|a| a.into()),
            context,
        });
        self
    }
}

/// For making requests against the `/request` APIs.
#[derive(Debug)]
pub struct RestrictRequest {
    request: ApiRequest<RequestTarget>,
}

impl From<ApiRequest<RequestTarget>> for RestrictRequest {
    fn from(request: ApiRequest<RequestTarget>) -> Self {
        Self { request }
    }
}

impl RestrictRequest {
    /// Create a new request builder
    pub fn builder() -> RestrictRequestBuilder {
        RestrictRequestBuilder::new()
    }

    // Internal method creating the URL for single key endpoints
    fn single_url(&self) -> Result<Url> {
        match &self.request.target {
            Some(RequestTarget::Single { api, directory }) => Ok(self
                .request
                .endpoint_url
                .join(&format!("{}/base64:{}/", api, directory.encoded()))?),
            _ => Err(SeaplaneError::IncorrectRestrictRequestTarget),
        }
    }

    #[allow(dead_code)]
    // Internal method creating the URL for all range endpoints
    fn range_url(&self) -> Result<Url> {
        match &self.request.target {
            Some(RequestTarget::AllRange { from_api, context }) => {
                let mut url = self.request.endpoint_url.clone();

                match (from_api, context.from()) {
                    (None, None) => Ok(url),
                    (Some(api), Some(from)) => {
                        url.set_query(Some(&format!(
                            "from_api={}&from=base64:{}",
                            api,
                            from.encoded()
                        )));
                        Ok(url)
                    }
                    (..) => Err(SeaplaneError::IncorrectRestrictRequestTarget),
                }
            }

            Some(RequestTarget::ApiRange { api, context }) => {
                let api = Api::from_str(api)
                    .map_err(|_| SeaplaneError::IncorrectRestrictRequestTarget)?;

                let mut url = self.request.endpoint_url.join(&format!("{}/", api))?;

                match context.from() {
                    None => Ok(url),
                    Some(from) => {
                        url.set_query(Some(&format!("from=base64:{}", from.encoded())));
                        Ok(url)
                    }
                }
            }
            _ => Err(SeaplaneError::IncorrectRestrictRequestTarget),
        }
    }

    /// Returns restriction details for an API-directory combination
    ///
    /// **NOTE:** This endpoint requires the `RequestTarget` be a `Single`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use seaplane::api::v1::{RestrictRequestBuilder,RestrictRequest};
    ///
    /// let req = RestrictRequestBuilder::new()
    ///     .token("abc123_token")
    ///     .single_restriction("config", "bW9ieQo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.get_restriction().unwrap();
    /// dbg!(resp);
    /// ```
    pub fn get_restriction(&self) -> Result<Restriction> {
        let url = self.single_url()?;
        let resp = self
            .request
            .client
            .get(url)
            .bearer_auth(&self.request.token)
            .send()?;
        map_api_error(resp)?
            .json::<Restriction>()
            .map_err(Into::into)
    }
}