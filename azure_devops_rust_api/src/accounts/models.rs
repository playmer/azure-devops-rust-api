// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(
        rename = "accountName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_name: Option<String>,
    #[serde(
        rename = "accountOwner",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_owner: Option<String>,
    #[serde(
        rename = "accountStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_status: Option<account::AccountStatus>,
    #[serde(
        rename = "accountType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_type: Option<account::AccountType>,
    #[serde(
        rename = "accountUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_uri: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_date: Option<String>,
    #[serde(rename = "hasMoved", default, skip_serializing_if = "Option::is_none")]
    pub has_moved: Option<bool>,
    #[serde(
        rename = "lastUpdatedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_updated_by: Option<String>,
    #[serde(
        rename = "lastUpdatedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_updated_date: Option<String>,
    #[serde(
        rename = "namespaceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub namespace_id: Option<String>,
    #[serde(
        rename = "newCollectionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_collection_id: Option<String>,
    #[serde(
        rename = "organizationName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[serde(
        rename = "statusReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_reason: Option<String>,
}
pub mod account {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccountStatus {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "deleted")]
        Deleted,
        #[serde(rename = "moved")]
        Moved,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccountType {
        #[serde(rename = "personal")]
        Personal,
        #[serde(rename = "organization")]
        Organization,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountCreateInfoInternal {
    #[serde(
        rename = "accountName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<AccountPreferencesInternal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[serde(
        rename = "serviceDefinitions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub service_definitions: Vec<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountPreferencesInternal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub culture: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertiesCollection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VssJsonCollectionWrapper {
    #[serde(flatten)]
    pub vss_json_collection_wrapper_base: VssJsonCollectionWrapperBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VssJsonCollectionWrapperBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Account>,
}
