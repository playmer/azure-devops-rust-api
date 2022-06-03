// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssociatedWorkItem {
    #[serde(
        rename = "assignedTo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "webUrl", default, skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(
        rename = "workItemType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Change {
    #[serde(
        rename = "changeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_type: Option<change::ChangeType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[serde(
        rename = "newContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_content: Option<ItemContent>,
    #[serde(
        rename = "sourceServerItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_server_item: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
pub mod change {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "add")]
        Add,
        #[serde(rename = "edit")]
        Edit,
        #[serde(rename = "encoding")]
        Encoding,
        #[serde(rename = "rename")]
        Rename,
        #[serde(rename = "delete")]
        Delete,
        #[serde(rename = "undelete")]
        Undelete,
        #[serde(rename = "branch")]
        Branch,
        #[serde(rename = "merge")]
        Merge,
        #[serde(rename = "lock")]
        Lock,
        #[serde(rename = "rollback")]
        Rollback,
        #[serde(rename = "sourceRename")]
        SourceRename,
        #[serde(rename = "targetRename")]
        TargetRename,
        #[serde(rename = "property")]
        Property,
        #[serde(rename = "all")]
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckinNote {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileContentMetadata {
    #[serde(
        rename = "contentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "isBinary", default, skip_serializing_if = "Option::is_none")]
    pub is_binary: Option<bool>,
    #[serde(rename = "isImage", default, skip_serializing_if = "Option::is_none")]
    pub is_image: Option<bool>,
    #[serde(rename = "vsLink", default, skip_serializing_if = "Option::is_none")]
    pub vs_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitRepository {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(
        rename = "defaultBranch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_disabled: Option<bool>,
    #[serde(rename = "isFork", default, skip_serializing_if = "Option::is_none")]
    pub is_fork: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "parentRepository",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_repository: Option<GitRepositoryRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
    #[serde(rename = "remoteUrl", default, skip_serializing_if = "Option::is_none")]
    pub remote_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "sshUrl", default, skip_serializing_if = "Option::is_none")]
    pub ssh_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(
        rename = "validRemoteUrls",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub valid_remote_urls: Vec<String>,
    #[serde(rename = "webUrl", default, skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitRepositoryRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<TeamProjectCollectionReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isFork", default, skip_serializing_if = "Option::is_none")]
    pub is_fork: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
    #[serde(rename = "remoteUrl", default, skip_serializing_if = "Option::is_none")]
    pub remote_url: Option<String>,
    #[serde(rename = "sshUrl", default, skip_serializing_if = "Option::is_none")]
    pub ssh_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphSubjectBase {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityRef {
    #[serde(flatten)]
    pub graph_subject_base: GraphSubjectBase,
    #[serde(
        rename = "directoryAlias",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub directory_alias: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "imageUrl", default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inactive: Option<bool>,
    #[serde(
        rename = "isAadIdentity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_aad_identity: Option<bool>,
    #[serde(
        rename = "isContainer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_container: Option<bool>,
    #[serde(
        rename = "isDeletedInOrigin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_deleted_in_origin: Option<bool>,
    #[serde(
        rename = "profileUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub profile_url: Option<String>,
    #[serde(
        rename = "uniqueName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(
        rename = "contentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<item_content::ContentType>,
}
pub mod item_content {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ContentType {
        #[serde(rename = "rawText")]
        RawText,
        #[serde(rename = "base64Encoded")]
        Base64Encoded,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemModel {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(
        rename = "contentMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_metadata: Option<FileContentMetadata>,
    #[serde(rename = "isFolder", default, skip_serializing_if = "Option::is_none")]
    pub is_folder: Option<bool>,
    #[serde(rename = "isSymLink", default, skip_serializing_if = "Option::is_none")]
    pub is_sym_link: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamProjectCollectionReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamProjectReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[serde(
        rename = "defaultTeamImageUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_team_image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "lastUpdateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<team_project_reference::State>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<team_project_reference::Visibility>,
}
pub mod team_project_reference {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "new")]
        New,
        #[serde(rename = "wellFormed")]
        WellFormed,
        #[serde(rename = "createPending")]
        CreatePending,
        #[serde(rename = "all")]
        All,
        #[serde(rename = "unchanged")]
        Unchanged,
        #[serde(rename = "deleted")]
        Deleted,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Visibility {
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "organization")]
        Organization,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcBranch {
    #[serde(flatten)]
    pub tfvc_branch_ref: TfvcBranchRef,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<TfvcBranch>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mappings: Vec<TfvcBranchMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<TfvcShallowBranchRef>,
    #[serde(
        rename = "relatedBranches",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub related_branches: Vec<TfvcShallowBranchRef>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcBranchMapping {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<String>,
    #[serde(
        rename = "serverItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_item: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcBranchRef {
    #[serde(flatten)]
    pub tfvc_shallow_branch_ref: TfvcShallowBranchRef,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<IdentityRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcChange {
    #[serde(flatten)]
    pub change: Change,
    #[serde(
        rename = "mergeSources",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub merge_sources: Vec<TfvcMergeSource>,
    #[serde(
        rename = "pendingVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_version: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcChangeset {
    #[serde(flatten)]
    pub tfvc_changeset_ref: TfvcChangesetRef,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub changes: Vec<TfvcChange>,
    #[serde(
        rename = "checkinNotes",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub checkin_notes: Vec<CheckinNote>,
    #[serde(
        rename = "collectionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_id: Option<String>,
    #[serde(
        rename = "hasMoreChanges",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_more_changes: Option<bool>,
    #[serde(
        rename = "policyOverride",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub policy_override: Option<TfvcPolicyOverrideInfo>,
    #[serde(
        rename = "teamProjectIds",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub team_project_ids: Vec<String>,
    #[serde(rename = "workItems", default, skip_serializing_if = "Vec::is_empty")]
    pub work_items: Vec<AssociatedWorkItem>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcChangesetRef {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<IdentityRef>,
    #[serde(
        rename = "changesetId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub changeset_id: Option<i32>,
    #[serde(
        rename = "checkedInBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub checked_in_by: Option<IdentityRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(
        rename = "commentTruncated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comment_truncated: Option<bool>,
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcChangesetSearchCriteria {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(
        rename = "followRenames",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub follow_renames: Option<bool>,
    #[serde(rename = "fromDate", default, skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
    #[serde(rename = "fromId", default, skip_serializing_if = "Option::is_none")]
    pub from_id: Option<i32>,
    #[serde(
        rename = "includeLinks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_links: Option<bool>,
    #[serde(rename = "itemPath", default, skip_serializing_if = "Option::is_none")]
    pub item_path: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mappings: Vec<TfvcMappingFilter>,
    #[serde(rename = "toDate", default, skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    #[serde(rename = "toId", default, skip_serializing_if = "Option::is_none")]
    pub to_id: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcChangesetsRequestData {
    #[serde(
        rename = "changesetIds",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub changeset_ids: Vec<i32>,
    #[serde(
        rename = "commentLength",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comment_length: Option<i32>,
    #[serde(
        rename = "includeLinks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_links: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcItem {
    #[serde(flatten)]
    pub item_model: ItemModel,
    #[serde(
        rename = "changeDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_date: Option<String>,
    #[serde(
        rename = "deletionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deletion_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<i32>,
    #[serde(rename = "hashValue", default, skip_serializing_if = "Option::is_none")]
    pub hash_value: Option<String>,
    #[serde(rename = "isBranch", default, skip_serializing_if = "Option::is_none")]
    pub is_branch: Option<bool>,
    #[serde(
        rename = "isPendingChange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_pending_change: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcItemDescriptor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "recursionLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recursion_level: Option<tfvc_item_descriptor::RecursionLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(
        rename = "versionOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_option: Option<tfvc_item_descriptor::VersionOption>,
    #[serde(
        rename = "versionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_type: Option<tfvc_item_descriptor::VersionType>,
}
pub mod tfvc_item_descriptor {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RecursionLevel {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "oneLevel")]
        OneLevel,
        #[serde(rename = "oneLevelPlusNestedEmptyFolders")]
        OneLevelPlusNestedEmptyFolders,
        #[serde(rename = "full")]
        Full,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VersionOption {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "previous")]
        Previous,
        #[serde(rename = "useRename")]
        UseRename,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VersionType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "changeset")]
        Changeset,
        #[serde(rename = "shelveset")]
        Shelveset,
        #[serde(rename = "change")]
        Change,
        #[serde(rename = "date")]
        Date,
        #[serde(rename = "latest")]
        Latest,
        #[serde(rename = "tip")]
        Tip,
        #[serde(rename = "mergeSource")]
        MergeSource,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcItemRequestData {
    #[serde(
        rename = "includeContentMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_content_metadata: Option<bool>,
    #[serde(
        rename = "includeLinks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_links: Option<bool>,
    #[serde(
        rename = "itemDescriptors",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub item_descriptors: Vec<TfvcItemDescriptor>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcLabel {
    #[serde(flatten)]
    pub tfvc_label_ref: TfvcLabelRef,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TfvcItem>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcLabelRef {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(
        rename = "labelScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub label_scope: Option<String>,
    #[serde(
        rename = "modifiedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<IdentityRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcLabelRequestData {
    #[serde(
        rename = "includeLinks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_links: Option<bool>,
    #[serde(
        rename = "itemLabelFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub item_label_filter: Option<String>,
    #[serde(
        rename = "labelScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub label_scope: Option<String>,
    #[serde(
        rename = "maxItemCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_item_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcMappingFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<bool>,
    #[serde(
        rename = "serverPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_path: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcMergeSource {
    #[serde(rename = "isRename", default, skip_serializing_if = "Option::is_none")]
    pub is_rename: Option<bool>,
    #[serde(
        rename = "serverItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_item: Option<String>,
    #[serde(
        rename = "versionFrom",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_from: Option<i32>,
    #[serde(rename = "versionTo", default, skip_serializing_if = "Option::is_none")]
    pub version_to: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcPolicyFailureInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(
        rename = "policyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub policy_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcPolicyOverrideInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(
        rename = "policyFailures",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub policy_failures: Vec<TfvcPolicyFailureInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcShallowBranchRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcShelveset {
    #[serde(flatten)]
    pub tfvc_shelveset_ref: TfvcShelvesetRef,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub changes: Vec<TfvcChange>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<CheckinNote>,
    #[serde(
        rename = "policyOverride",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub policy_override: Option<TfvcPolicyOverrideInfo>,
    #[serde(rename = "workItems", default, skip_serializing_if = "Vec::is_empty")]
    pub work_items: Vec<AssociatedWorkItem>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcShelvesetRef {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(
        rename = "commentTruncated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comment_truncated: Option<bool>,
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<IdentityRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcShelvesetRequestData {
    #[serde(
        rename = "includeDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_details: Option<bool>,
    #[serde(
        rename = "includeLinks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_links: Option<bool>,
    #[serde(
        rename = "includeWorkItems",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_work_items: Option<bool>,
    #[serde(
        rename = "maxChangeCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_change_count: Option<i32>,
    #[serde(
        rename = "maxCommentLength",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_comment_length: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcStatistics {
    #[serde(
        rename = "changesetId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub changeset_id: Option<i32>,
    #[serde(
        rename = "fileCountTotal",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub file_count_total: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcVersionDescriptor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(
        rename = "versionOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_option: Option<tfvc_version_descriptor::VersionOption>,
    #[serde(
        rename = "versionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_type: Option<tfvc_version_descriptor::VersionType>,
}
pub mod tfvc_version_descriptor {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VersionOption {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "previous")]
        Previous,
        #[serde(rename = "useRename")]
        UseRename,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VersionType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "changeset")]
        Changeset,
        #[serde(rename = "shelveset")]
        Shelveset,
        #[serde(rename = "change")]
        Change,
        #[serde(rename = "date")]
        Date,
        #[serde(rename = "latest")]
        Latest,
        #[serde(rename = "tip")]
        Tip,
        #[serde(rename = "mergeSource")]
        MergeSource,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VersionControlProjectInfo {
    #[serde(
        rename = "defaultSourceControlType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_source_control_type: Option<version_control_project_info::DefaultSourceControlType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
    #[serde(
        rename = "supportsGit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_git: Option<bool>,
    #[serde(
        rename = "supportsTFVC",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_tfvc: Option<bool>,
}
pub mod version_control_project_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DefaultSourceControlType {
        #[serde(rename = "tfvc")]
        Tfvc,
        #[serde(rename = "git")]
        Git,
    }
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
pub struct AssociatedWorkItemList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AssociatedWorkItem>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcBranchList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TfvcBranch>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcBranchRefList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TfvcBranchRef>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcChangeList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TfvcChange>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcChangesetRefList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TfvcChangesetRef>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcItemList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TfvcItem>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcLabelRefList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TfvcLabelRef>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TfvcShelvesetRefList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TfvcShelvesetRef>,
}
