// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::{models, API_VERSION};
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
}
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
impl ClientBuilder {
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
        }
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self
            .scopes
            .unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(
        &self,
        request: impl Into<azure_core::Request>,
    ) -> Result<azure_core::Response, azure_core::Error> {
        let mut context = azure_core::Context::default();
        let mut request = request.into();
        self.pipeline.send(&mut context, &mut request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn installed_extensions(&self) -> installed_extensions::Client {
        installed_extensions::Client(self.clone())
    }
}
#[non_exhaustive]
#[derive(Debug, thiserror :: Error)]
#[allow(non_camel_case_types)]
pub enum Error {
    #[error(transparent)]
    InstalledExtensions_List(#[from] installed_extensions::list::Error),
    #[error(transparent)]
    InstalledExtensions_Update(#[from] installed_extensions::update::Error),
    #[error(transparent)]
    InstalledExtensions_Get(#[from] installed_extensions::get::Error),
    #[error(transparent)]
    InstalledExtensions_UninstallExtensionByName(
        #[from] installed_extensions::uninstall_extension_by_name::Error,
    ),
    #[error(transparent)]
    InstalledExtensions_InstallExtensionByName(
        #[from] installed_extensions::install_extension_by_name::Error,
    ),
}
pub mod installed_extensions {
    use super::{models, API_VERSION};
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                include_disabled_extensions: None,
                include_errors: None,
                asset_types: None,
                include_installation_issues: None,
            }
        }
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::InstalledExtension>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        pub fn get(
            &self,
            organization: impl Into<String>,
            publisher_name: impl Into<String>,
            extension_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                publisher_name: publisher_name.into(),
                extension_name: extension_name.into(),
                asset_types: None,
            }
        }
        pub fn uninstall_extension_by_name(
            &self,
            organization: impl Into<String>,
            publisher_name: impl Into<String>,
            extension_name: impl Into<String>,
        ) -> uninstall_extension_by_name::Builder {
            uninstall_extension_by_name::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                publisher_name: publisher_name.into(),
                extension_name: extension_name.into(),
                reason: None,
                reason_code: None,
            }
        }
        pub fn install_extension_by_name(
            &self,
            organization: impl Into<String>,
            publisher_name: impl Into<String>,
            extension_name: impl Into<String>,
            version: impl Into<String>,
        ) -> install_extension_by_name::Builder {
            install_extension_by_name::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                publisher_name: publisher_name.into(),
                extension_name: extension_name.into(),
                version: version.into(),
            }
        }
    }
    pub mod list {
        use super::{models, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse {
                status_code: http::StatusCode,
                body: bytes::Bytes,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) include_disabled_extensions: Option<bool>,
            pub(crate) include_errors: Option<bool>,
            pub(crate) asset_types: Option<String>,
            pub(crate) include_installation_issues: Option<bool>,
        }
        impl Builder {
            pub fn include_disabled_extensions(
                mut self,
                include_disabled_extensions: bool,
            ) -> Self {
                self.include_disabled_extensions = Some(include_disabled_extensions);
                self
            }
            pub fn include_errors(mut self, include_errors: bool) -> Self {
                self.include_errors = Some(include_errors);
                self
            }
            pub fn asset_types(mut self, asset_types: impl Into<String>) -> Self {
                self.asset_types = Some(asset_types.into());
                self
            }
            pub fn include_installation_issues(
                mut self,
                include_installation_issues: bool,
            ) -> Self {
                self.include_installation_issues = Some(include_installation_issues);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<
                'static,
                std::result::Result<models::InstalledExtensionList, Error>,
            > {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/{}/_apis/extensionmanagement/installedextensions",
                        self.client.endpoint(),
                        &self.organization
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(
                        http::header::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    url.query_pairs_mut()
                        .append_pair("api-version", super::API_VERSION);
                    if let Some(include_disabled_extensions) = &self.include_disabled_extensions {
                        url.query_pairs_mut().append_pair(
                            "includeDisabledExtensions",
                            &include_disabled_extensions.to_string(),
                        );
                    }
                    if let Some(include_errors) = &self.include_errors {
                        url.query_pairs_mut()
                            .append_pair("includeErrors", &include_errors.to_string());
                    }
                    if let Some(asset_types) = &self.asset_types {
                        url.query_pairs_mut().append_pair("assetTypes", asset_types);
                    }
                    if let Some(include_installation_issues) = &self.include_installation_issues {
                        url.query_pairs_mut().append_pair(
                            "includeInstallationIssues",
                            &include_installation_issues.to_string(),
                        );
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            let rsp_value: models::InstalledExtensionList =
                                serde_json::from_slice(&rsp_body).map_err(|source| {
                                    Error::Deserialize(source, rsp_body.clone())
                                })?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            Err(Error::UnexpectedResponse {
                                status_code,
                                body: rsp_body,
                            })
                        }
                    }
                })
            }
        }
    }
    pub mod update {
        use super::{models, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse {
                status_code: http::StatusCode,
                body: bytes::Bytes,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::InstalledExtension,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<
                'static,
                std::result::Result<models::InstalledExtension, Error>,
            > {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/{}/_apis/extensionmanagement/installedextensions",
                        self.client.endpoint(),
                        &self.organization
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::PATCH);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(
                        http::header::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    url.query_pairs_mut()
                        .append_pair("api-version", super::API_VERSION);
                    req_builder = req_builder.header("content-type", "application/json");
                    let req_body = azure_core::to_json(&self.body).map_err(Error::Serialize)?;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            let rsp_value: models::InstalledExtension =
                                serde_json::from_slice(&rsp_body).map_err(|source| {
                                    Error::Deserialize(source, rsp_body.clone())
                                })?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            Err(Error::UnexpectedResponse {
                                status_code,
                                body: rsp_body,
                            })
                        }
                    }
                })
            }
        }
    }
    pub mod get {
        use super::{models, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse {
                status_code: http::StatusCode,
                body: bytes::Bytes,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) publisher_name: String,
            pub(crate) extension_name: String,
            pub(crate) asset_types: Option<String>,
        }
        impl Builder {
            pub fn asset_types(mut self, asset_types: impl Into<String>) -> Self {
                self.asset_types = Some(asset_types.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<
                'static,
                std::result::Result<models::InstalledExtension, Error>,
            > {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/{}/_apis/extensionmanagement/installedextensionsbyname/{}/{}",
                        self.client.endpoint(),
                        &self.organization,
                        &self.publisher_name,
                        &self.extension_name
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(
                        http::header::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    url.query_pairs_mut()
                        .append_pair("api-version", super::API_VERSION);
                    if let Some(asset_types) = &self.asset_types {
                        url.query_pairs_mut().append_pair("assetTypes", asset_types);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            let rsp_value: models::InstalledExtension =
                                serde_json::from_slice(&rsp_body).map_err(|source| {
                                    Error::Deserialize(source, rsp_body.clone())
                                })?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            Err(Error::UnexpectedResponse {
                                status_code,
                                body: rsp_body,
                            })
                        }
                    }
                })
            }
        }
    }
    pub mod uninstall_extension_by_name {
        use super::{models, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse {
                status_code: http::StatusCode,
                body: bytes::Bytes,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) publisher_name: String,
            pub(crate) extension_name: String,
            pub(crate) reason: Option<String>,
            pub(crate) reason_code: Option<String>,
        }
        impl Builder {
            pub fn reason(mut self, reason: impl Into<String>) -> Self {
                self.reason = Some(reason.into());
                self
            }
            pub fn reason_code(mut self, reason_code: impl Into<String>) -> Self {
                self.reason_code = Some(reason_code.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, std::result::Result<(), Error>> {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/{}/_apis/extensionmanagement/installedextensionsbyname/{}/{}",
                        self.client.endpoint(),
                        &self.organization,
                        &self.publisher_name,
                        &self.extension_name
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::DELETE);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(
                        http::header::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    url.query_pairs_mut()
                        .append_pair("api-version", super::API_VERSION);
                    if let Some(reason) = &self.reason {
                        url.query_pairs_mut().append_pair("reason", reason);
                    }
                    if let Some(reason_code) = &self.reason_code {
                        url.query_pairs_mut().append_pair("reasonCode", reason_code);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => Ok(()),
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            Err(Error::UnexpectedResponse {
                                status_code,
                                body: rsp_body,
                            })
                        }
                    }
                })
            }
        }
    }
    pub mod install_extension_by_name {
        use super::{models, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse {
                status_code: http::StatusCode,
                body: bytes::Bytes,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) publisher_name: String,
            pub(crate) extension_name: String,
            pub(crate) version: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<
                'static,
                std::result::Result<models::InstalledExtension, Error>,
            > {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/{}/_apis/extensionmanagement/installedextensionsbyname/{}/{}/{}",
                        self.client.endpoint(),
                        &self.organization,
                        &self.publisher_name,
                        &self.extension_name,
                        &self.version
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::POST);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(
                        http::header::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    url.query_pairs_mut()
                        .append_pair("api-version", super::API_VERSION);
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            let rsp_value: models::InstalledExtension =
                                serde_json::from_slice(&rsp_body).map_err(|source| {
                                    Error::Deserialize(source, rsp_body.clone())
                                })?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            Err(Error::UnexpectedResponse {
                                status_code,
                                body: rsp_body,
                            })
                        }
                    }
                })
            }
        }
    }
}
