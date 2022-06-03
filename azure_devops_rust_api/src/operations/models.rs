// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(flatten)]
    pub operation_reference: OperationReference,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(
        rename = "detailedMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub detailed_message: Option<String>,
    #[serde(
        rename = "resultMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_message: Option<String>,
    #[serde(rename = "resultUrl", default, skip_serializing_if = "Option::is_none")]
    pub result_url: Option<OperationResultReference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "pluginId", default, skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_reference::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
pub mod operation_reference {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "notSet")]
        NotSet,
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "cancelled")]
        Cancelled,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResultReference {
    #[serde(rename = "resultUrl", default, skip_serializing_if = "Option::is_none")]
    pub result_url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
}
