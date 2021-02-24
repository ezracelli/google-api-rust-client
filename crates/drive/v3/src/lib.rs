#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the user, the user's Drive, and system capabilities."]
pub struct About {
    #[serde(rename = "appInstalled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the user has installed the requesting app."]
    pub app_installed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canCreateDrives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the user can create shared drives."]
    pub can_create_drives: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canCreateTeamDrives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use canCreateDrives instead."]
    pub can_create_team_drives: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "driveThemes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of themes that are supported for shared drives."]
    pub drive_themes: ::std::option::Option<::std::vec::Vec<AboutDriveThemes>>,
    #[serde(rename = "exportFormats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A map of source MIME type to possible targets for all supported exports."]
    pub export_formats: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::vec::Vec<::std::string::String>>,
    >,
    #[serde(rename = "folderColorPalette")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The currently supported folder colors as RGB hex strings."]
    pub folder_color_palette: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "importFormats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A map of source MIME type to possible targets for all supported imports."]
    pub import_formats: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::vec::Vec<::std::string::String>>,
    >,
    #[serde(rename = "kind")]
    #[serde(default = "about_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#about\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "maxImportSizes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A map of maximum import sizes by MIME type, in bytes."]
    pub max_import_sizes:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "maxUploadSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum upload size in bytes."]
    pub max_upload_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storageQuota")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's storage quota limits and usage. All fields are measured in bytes."]
    pub storage_quota: ::std::option::Option<AboutStorageQuota>,
    #[serde(rename = "teamDriveThemes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use driveThemes instead."]
    pub team_drive_themes: ::std::option::Option<::std::vec::Vec<AboutTeamDriveThemes>>,
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The authenticated user."]
    pub user: ::std::option::Option<::std::boxed::Box<User>>,
}
mod about_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#about")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AboutDriveThemes {
    #[serde(rename = "backgroundImageLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to this theme's background image."]
    pub background_image_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "colorRgb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The color of this theme as an RGB hex string."]
    pub color_rgb: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the theme."]
    pub id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The user's storage quota limits and usage. All fields are measured in bytes."]
pub struct AboutStorageQuota {
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The usage limit, if applicable. This will not be present if the user has unlimited storage."]
    pub limit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "usage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total usage across all services."]
    pub usage: ::std::option::Option<::std::string::String>,
    #[serde(rename = "usageInDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The usage by all files in Google Drive."]
    pub usage_in_drive: ::std::option::Option<::std::string::String>,
    #[serde(rename = "usageInDriveTrash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The usage by trashed files in Google Drive."]
    pub usage_in_drive_trash: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AboutTeamDriveThemes {
    #[serde(rename = "backgroundImageLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use driveThemes/backgroundImageLink instead."]
    pub background_image_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "colorRgb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use driveThemes/colorRgb instead."]
    pub color_rgb: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use driveThemes/id instead."]
    pub id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A change to a file or shared drive."]
pub struct Change {
    #[serde(rename = "changeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the change. Possible values are file and drive."]
    pub change_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "drive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The updated state of the shared drive. Present if the changeType is drive, the user is still a member of the shared drive, and the shared drive has not been deleted."]
    pub drive: ::std::option::Option<::std::boxed::Box<Drive>>,
    #[serde(rename = "driveId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the shared drive associated with this change."]
    pub drive_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The updated state of the file. Present if the type is file and the file has not been removed from this list of changes."]
    pub file: ::std::option::Option<::std::boxed::Box<File>>,
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the file which has changed."]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "change_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#change\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "removed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the file or shared drive has been removed from this list of changes, for example by deletion or loss of access."]
    pub removed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "teamDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use drive instead."]
    pub team_drive: ::std::option::Option<::std::boxed::Box<TeamDrive>>,
    #[serde(rename = "teamDriveId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use driveId instead."]
    pub team_drive_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time of this change (RFC 3339 date-time)."]
    pub time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use changeType instead."]
    pub _type: ::std::option::Option<::std::string::String>,
}
mod change_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#change")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of changes for a user."]
pub struct ChangeList {
    #[serde(rename = "changes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of changes. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
    pub changes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Change>>>,
    #[serde(rename = "kind")]
    #[serde(default = "change_list_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#changeList\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "newStartPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The starting page token for future changes. This will be present only if the end of the current changes list has been reached."]
    pub new_start_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token for the next page of changes. This will be absent if the end of the changes list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod change_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#changeList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An notification channel used to watch for resource changes."]
pub struct Channel {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The address where notifications are delivered for this channel."]
    pub address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expiration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional."]
    pub expiration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A UUID or similar unique string that identifies this channel."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "channel_defaults :: kind")]
    #[doc = "Identifies this as a notification channel used to watch for changes to a resource, which is \"api#channel\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "params")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional parameters controlling delivery channel behavior. Optional."]
    pub params: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Boolean value to indicate whether payload is wanted. Optional."]
    pub payload: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An opaque ID that identifies the resource being watched on this channel. Stable across different API versions."]
    pub resource_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A version-specific identifier for the watched resource."]
    pub resource_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An arbitrary string delivered to the target address with each notification delivered over this channel. Optional."]
    pub token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of delivery mechanism used for this channel."]
    pub _type: ::std::option::Option<::std::string::String>,
}
mod channel_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("api#channel")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A comment on a file."]
pub struct Comment {
    #[serde(rename = "anchor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A region of the document represented as a JSON string. See anchor documentation for details on how to define and interpret anchor properties."]
    pub anchor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The author of the comment. The author's email address and permission ID will not be populated."]
    pub author: ::std::option::Option<::std::boxed::Box<User>>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The plain text content of the comment. This field is used for setting the content, while htmlContent should be displayed."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the comment was created (RFC 3339 date-time)."]
    pub created_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the comment has been deleted. A deleted comment has no content."]
    pub deleted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "htmlContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content of the comment with HTML formatting."]
    pub html_content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the comment."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "comment_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#comment\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "modifiedTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last time the comment or any of its replies was modified (RFC 3339 date-time)."]
    pub modified_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quotedFileContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment."]
    pub quoted_file_content: ::std::option::Option<CommentQuotedFileContent>,
    #[serde(rename = "replies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full list of replies to the comment in chronological order."]
    pub replies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Reply>>>,
    #[serde(rename = "resolved")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the comment has been resolved by one of its replies."]
    pub resolved: ::std::option::Option<::std::primitive::bool>,
}
mod comment_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#comment")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment."]
pub struct CommentQuotedFileContent {
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MIME type of the quoted content."]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quoted content itself. This is interpreted as plain text if set through the API."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of comments on a file."]
pub struct CommentList {
    #[serde(rename = "comments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of comments. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
    pub comments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Comment>>>,
    #[serde(rename = "kind")]
    #[serde(default = "comment_list_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#commentList\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token for the next page of comments. This will be absent if the end of the comments list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod comment_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#commentList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A restriction for accessing the content of the file."]
pub struct ContentRestriction {
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the content of the file is read-only. If a file is read-only, a new revision of the file may not be added, comments may not be added or modified, and the title of the file may not be modified."]
    pub read_only: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reason for why the content of the file is restricted. This is only mutable on requests that also set readOnly=true."]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "restrictingUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user who set the content restriction. Only populated if readOnly is true."]
    pub restricting_user: ::std::option::Option<::std::boxed::Box<User>>,
    #[serde(rename = "restrictionTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the content restriction was set (formatted RFC 3339 timestamp). Only populated if readOnly is true."]
    pub restriction_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the content restriction. Currently the only possible value is globalContentRestriction."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Representation of a shared drive."]
pub struct Drive {
    #[serde(rename = "backgroundImageFile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on drive.drives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set."]
    pub background_image_file: ::std::option::Option<DriveBackgroundImageFile>,
    #[serde(rename = "backgroundImageLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short-lived link to this shared drive's background image."]
    pub background_image_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Capabilities the current user has on this shared drive."]
    pub capabilities: ::std::option::Option<DriveCapabilities>,
    #[serde(rename = "colorRgb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The color of this shared drive as an RGB hex string. It can only be set on a drive.drives.update request that does not set themeId."]
    pub color_rgb: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the shared drive was created (RFC 3339 date-time)."]
    pub created_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hidden")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the shared drive is hidden from default view."]
    pub hidden: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of this shared drive which is also the ID of the top level folder of this shared drive."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "drive_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#drive\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this shared drive."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "restrictions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of restrictions that apply to this shared drive or items inside this shared drive."]
    pub restrictions: ::std::option::Option<DriveRestrictions>,
    #[serde(rename = "themeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the theme from which the background image and color will be set. The set of possible driveThemes can be retrieved from a drive.about.get response. When not specified on a drive.drives.create request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile."]
    pub theme_id: ::std::option::Option<::std::string::String>,
}
mod drive_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#drive")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on drive.drives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set."]
pub struct DriveBackgroundImageFile {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of an image file in Google Drive to use for the background image."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width of the cropped image in the closed range of 0 to 1. This value represents the width of the cropped image divided by the width of the entire image. The height is computed by applying a width to height aspect ratio of 80 to 9. The resulting image must be at least 1280 pixels wide and 144 pixels high."]
    pub width: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "xCoordinate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The X coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the horizontal distance from the left side of the entire image to the left side of the cropping area divided by the width of the entire image."]
    pub x_coordinate: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "yCoordinate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Y coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the vertical distance from the top side of the entire image to the top side of the cropping area divided by the height of the entire image."]
    pub y_coordinate: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Capabilities the current user has on this shared drive."]
pub struct DriveCapabilities {
    #[serde(rename = "canAddChildren")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can add children to folders in this shared drive."]
    pub can_add_children: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canChangeCopyRequiresWriterPermissionRestriction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can change the copyRequiresWriterPermission restriction of this shared drive."]
    pub can_change_copy_requires_writer_permission_restriction:
        ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canChangeDomainUsersOnlyRestriction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can change the domainUsersOnly restriction of this shared drive."]
    pub can_change_domain_users_only_restriction: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canChangeDriveBackground")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can change the background of this shared drive."]
    pub can_change_drive_background: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canChangeDriveMembersOnlyRestriction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can change the driveMembersOnly restriction of this shared drive."]
    pub can_change_drive_members_only_restriction: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canComment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can comment on files in this shared drive."]
    pub can_comment: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canCopy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can copy files in this shared drive."]
    pub can_copy: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canDeleteChildren")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can delete children from folders in this shared drive."]
    pub can_delete_children: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canDeleteDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can delete this shared drive. Attempting to delete the shared drive may still fail if there are untrashed items inside the shared drive."]
    pub can_delete_drive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canDownload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can download files in this shared drive."]
    pub can_download: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canEdit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can edit files in this shared drive"]
    pub can_edit: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canListChildren")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can list the children of folders in this shared drive."]
    pub can_list_children: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canManageMembers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can add members to this shared drive or remove them or change their role."]
    pub can_manage_members: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canReadRevisions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can read the revisions resource of files in this shared drive."]
    pub can_read_revisions: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canRename")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can rename files or folders in this shared drive."]
    pub can_rename: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canRenameDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can rename this shared drive."]
    pub can_rename_drive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canShare")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can share files or folders in this shared drive."]
    pub can_share: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canTrashChildren")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can trash children from folders in this shared drive."]
    pub can_trash_children: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A set of restrictions that apply to this shared drive or items inside this shared drive."]
pub struct DriveRestrictions {
    #[serde(rename = "adminManagedRestrictions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether administrative privileges on this shared drive are required to modify restrictions."]
    pub admin_managed_restrictions: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "copyRequiresWriterPermission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the options to copy, print, or download files inside this shared drive, should be disabled for readers and commenters. When this restriction is set to true, it will override the similarly named field to true for any file inside this shared drive."]
    pub copy_requires_writer_permission: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "domainUsersOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether access to this shared drive and items inside this shared drive is restricted to users of the domain to which this shared drive belongs. This restriction may be overridden by other sharing policies controlled outside of this shared drive."]
    pub domain_users_only: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "driveMembersOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether access to items inside this shared drive is restricted to its members."]
    pub drive_members_only: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of shared drives."]
pub struct DriveList {
    #[serde(rename = "drives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of shared drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
    pub drives: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Drive>>>,
    #[serde(rename = "kind")]
    #[serde(default = "drive_list_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#driveList\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token for the next page of shared drives. This will be absent if the end of the list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod drive_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#driveList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The metadata for a file."]
pub struct File {
    #[serde(rename = "appProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A collection of arbitrary key-value pairs which are private to the requesting app.\nEntries with null values are cleared in update and copy requests. These properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties."]
    pub app_properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take."]
    pub capabilities: ::std::option::Option<FileCapabilities>,
    #[serde(rename = "contentHints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional information about the content of the file. These fields are never populated in responses."]
    pub content_hints: ::std::option::Option<FileContentHints>,
    #[serde(rename = "contentRestrictions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restrictions for accessing the content of the file. Only populated if such a restriction exists."]
    pub content_restrictions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContentRestriction>>>,
    #[serde(rename = "copyRequiresWriterPermission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the options to copy, print, or download this file, should be disabled for readers and commenters."]
    pub copy_requires_writer_permission: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the file was created (RFC 3339 date-time)."]
    pub created_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short description of the file."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "driveId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the shared drive the file resides in. Only populated for items in shared drives."]
    pub drive_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "explicitlyTrashed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the file has been explicitly trashed, as opposed to recursively trashed from a parent folder."]
    pub explicitly_trashed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "exportLinks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Links for exporting Docs Editors files to specific formats."]
    pub export_links:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "fileExtension")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The final component of fullFileExtension. This is only available for files with binary content in Google Drive."]
    pub file_extension: ::std::option::Option<::std::string::String>,
    #[serde(rename = "folderColorRgb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The color for a folder as an RGB hex string. The supported colors are published in the folderColorPalette field of the About resource.\nIf an unsupported color is specified, the closest color in the palette will be used instead."]
    pub folder_color_rgb: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fullFileExtension")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full file extension extracted from the name field. May contain multiple concatenated extensions, such as \"tar.gz\". This is only available for files with binary content in Google Drive.\nThis is automatically updated when the name field changes, however it is not cleared if the new name does not contain a valid extension."]
    pub full_file_extension: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hasAugmentedPermissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether there are permissions directly on this file. This field is only populated for items in shared drives."]
    pub has_augmented_permissions: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "hasThumbnail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this file has a thumbnail. This does not indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field."]
    pub has_thumbnail: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "headRevisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the file's head revision. This is currently only available for files with binary content in Google Drive."]
    pub head_revision_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "iconLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A static, unauthenticated link to the file's icon."]
    pub icon_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the file."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imageMediaMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional metadata about image media, if available."]
    pub image_media_metadata: ::std::option::Option<FileImageMediaMetadata>,
    #[serde(rename = "isAppAuthorized")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the file was created or opened by the requesting app."]
    pub is_app_authorized: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(default = "file_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#file\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "lastModifyingUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last user to modify the file."]
    pub last_modifying_user: ::std::option::Option<::std::boxed::Box<User>>,
    #[serde(rename = "md5Checksum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MD5 checksum for the content of the file. This is only applicable to files with binary content in Google Drive."]
    pub md5_checksum: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MIME type of the file.\nGoogle Drive will attempt to automatically detect an appropriate value from uploaded content if no value is provided. The value cannot be changed unless a new revision is uploaded.\nIf a file is created with a Google Doc MIME type, the uploaded content will be imported if possible. The supported import formats are published in the About resource."]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "modifiedByMe")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the file has been modified by this user."]
    pub modified_by_me: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "modifiedByMeTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last time the file was modified by the user (RFC 3339 date-time)."]
    pub modified_by_me_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "modifiedTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last time the file was modified by anyone (RFC 3339 date-time).\nNote that setting modifiedTime will also update modifiedByMeTime for the user."]
    pub modified_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the file. This is not necessarily unique within a folder. Note that for immutable items such as the top level folders of shared drives, My Drive root folder, and Application Data folder the name is constant."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "originalFilename")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The original filename of the uploaded content if available, or else the original value of the name field. This is only available for files with binary content in Google Drive."]
    pub original_filename: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ownedByMe")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the user owns the file. Not populated for items in shared drives."]
    pub owned_by_me: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "owners")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The owners of the file. Currently, only certain legacy files may have more than one owner. Not populated for items in shared drives."]
    pub owners: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<User>>>,
    #[serde(rename = "parents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IDs of the parent folders which contain the file.\nIf not specified as part of a create request, the file will be placed directly in the user's My Drive folder. If not specified as part of a copy request, the file will inherit any discoverable parents of the source file. Update requests must use the addParents and removeParents parameters to modify the parents list."]
    pub parents: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "permissionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of permission IDs for users with access to this file."]
    pub permission_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full list of permissions for the file. This is only available if the requesting user can share the file. Not populated for items in shared drives."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Permission>>>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A collection of arbitrary key-value pairs which are visible to all apps.\nEntries with null values are cleared in update and copy requests."]
    pub properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "quotaBytesUsed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of storage quota bytes used by the file. This includes the head revision as well as previous revisions with keepForever enabled."]
    pub quota_bytes_used: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shared")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the file has been shared. Not populated for items in shared drives."]
    pub shared: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "sharedWithMeTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the file was shared with the user, if applicable (RFC 3339 date-time)."]
    pub shared_with_me_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sharingUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user who shared the file with the requesting user, if applicable."]
    pub sharing_user: ::std::option::Option<::std::boxed::Box<User>>,
    #[serde(rename = "shortcutDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shortcut file details. Only populated for shortcut files, which have the mimeType field set to application/vnd.google-apps.shortcut."]
    pub shortcut_details: ::std::option::Option<FileShortcutDetails>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of the file's content in bytes. This is applicable to binary files in Google Drive and Google Docs files."]
    pub size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "spaces")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of spaces which contain the file. The currently supported values are 'drive', 'appDataFolder' and 'photos'."]
    pub spaces: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "starred")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the user has starred the file."]
    pub starred: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "teamDriveId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use driveId instead."]
    pub team_drive_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thumbnailLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short-lived link to the file's thumbnail, if available. Typically lasts on the order of hours. Only populated when the requesting app can access the file's content. If the file isn't shared publicly, the URL returned in Files.thumbnailLink must be fetched using a credentialed request."]
    pub thumbnail_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thumbnailVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The thumbnail version for use in thumbnail cache invalidation."]
    pub thumbnail_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trashed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file. The trashed item is excluded from all files.list responses returned for any user who does not own the file. However, all users with access to the file can see the trashed item metadata in an API response. All users with access can copy, download, export, and share the file."]
    pub trashed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "trashedTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time that the item was trashed (RFC 3339 date-time). Only populated for items in shared drives."]
    pub trashed_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trashingUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives."]
    pub trashing_user: ::std::option::Option<::std::boxed::Box<User>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the user."]
    pub version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "videoMediaMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional metadata about video media. This may not be available immediately upon upload."]
    pub video_media_metadata: ::std::option::Option<FileVideoMediaMetadata>,
    #[serde(rename = "viewedByMe")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the file has been viewed by this user."]
    pub viewed_by_me: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "viewedByMeTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last time the file was viewed by the user (RFC 3339 date-time)."]
    pub viewed_by_me_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "viewersCanCopyContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use copyRequiresWriterPermission instead."]
    pub viewers_can_copy_content: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "webContentLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link for downloading the content of the file in a browser. This is only available for files with binary content in Google Drive."]
    pub web_content_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "webViewLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link for opening the file in a relevant Google editor or viewer in a browser."]
    pub web_view_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "writersCanShare")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether users with only writer permission can modify the file's permissions. Not populated for items in shared drives."]
    pub writers_can_share: ::std::option::Option<::std::primitive::bool>,
}
mod file_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#file")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take."]
pub struct FileCapabilities {
    #[serde(rename = "canAddChildren")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can add children to this folder. This is always false when the item is not a folder."]
    pub can_add_children: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canAddFolderFromAnotherDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can add a folder from another drive (different shared drive or My Drive) to this folder. This is false when the item is not a folder. Only populated for items in shared drives."]
    pub can_add_folder_from_another_drive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canAddMyDriveParent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can add a parent for the item without removing an existing parent in the same request. Not populated for shared drive files."]
    pub can_add_my_drive_parent: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canChangeCopyRequiresWriterPermission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can change the copyRequiresWriterPermission restriction of this file."]
    pub can_change_copy_requires_writer_permission: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canChangeViewersCanCopyContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated"]
    pub can_change_viewers_can_copy_content: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canComment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can comment on this file."]
    pub can_comment: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canCopy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can copy this file. For an item in a shared drive, whether the current user can copy non-folder descendants of this item, or this item itself if it is not a folder."]
    pub can_copy: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canDelete")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can delete this file."]
    pub can_delete: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canDeleteChildren")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can delete children of this folder. This is false when the item is not a folder. Only populated for items in shared drives."]
    pub can_delete_children: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canDownload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can download this file."]
    pub can_download: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canEdit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can edit this file. Other factors may limit the type of changes a user can make to a file. For example, see canChangeCopyRequiresWriterPermission or canModifyContent."]
    pub can_edit: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canListChildren")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can list the children of this folder. This is always false when the item is not a folder."]
    pub can_list_children: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canModifyContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can modify the content of this file."]
    pub can_modify_content: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canModifyContentRestriction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can modify restrictions on content of this file."]
    pub can_modify_content_restriction: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canMoveChildrenOutOfDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can move children of this folder outside of the shared drive. This is false when the item is not a folder. Only populated for items in shared drives."]
    pub can_move_children_out_of_drive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canMoveChildrenOutOfTeamDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use canMoveChildrenOutOfDrive instead."]
    pub can_move_children_out_of_team_drive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canMoveChildrenWithinDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can move children of this folder within this drive. This is false when the item is not a folder. Note that a request to move the child may still fail depending on the current user's access to the child and to the destination folder."]
    pub can_move_children_within_drive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canMoveChildrenWithinTeamDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use canMoveChildrenWithinDrive instead."]
    pub can_move_children_within_team_drive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canMoveItemIntoTeamDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use canMoveItemOutOfDrive instead."]
    pub can_move_item_into_team_drive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canMoveItemOutOfDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can move this item outside of this drive by changing its parent. Note that a request to change the parent of the item may still fail depending on the new parent that is being added."]
    pub can_move_item_out_of_drive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canMoveItemOutOfTeamDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use canMoveItemOutOfDrive instead."]
    pub can_move_item_out_of_team_drive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canMoveItemWithinDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can move this item within this drive. Note that a request to change the parent of the item may still fail depending on the new parent that is being added and the parent that is being removed."]
    pub can_move_item_within_drive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canMoveItemWithinTeamDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use canMoveItemWithinDrive instead."]
    pub can_move_item_within_team_drive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canMoveTeamDriveItem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use canMoveItemWithinDrive or canMoveItemOutOfDrive instead."]
    pub can_move_team_drive_item: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canReadDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can read the shared drive to which this file belongs. Only populated for items in shared drives."]
    pub can_read_drive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canReadRevisions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can read the revisions resource of this file. For a shared drive item, whether revisions of non-folder descendants of this item, or this item itself if it is not a folder, can be read."]
    pub can_read_revisions: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canReadTeamDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use canReadDrive instead."]
    pub can_read_team_drive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canRemoveChildren")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can remove children from this folder. This is always false when the item is not a folder. For a folder in a shared drive, use canDeleteChildren or canTrashChildren instead."]
    pub can_remove_children: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canRemoveMyDriveParent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can remove a parent from the item without adding another parent in the same request. Not populated for shared drive files."]
    pub can_remove_my_drive_parent: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canRename")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can rename this file."]
    pub can_rename: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canShare")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can modify the sharing settings for this file."]
    pub can_share: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canTrash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can move this file to trash."]
    pub can_trash: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canTrashChildren")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can trash children of this folder. This is false when the item is not a folder. Only populated for items in shared drives."]
    pub can_trash_children: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canUntrash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can restore this file from trash."]
    pub can_untrash: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional information about the content of the file. These fields are never populated in responses."]
pub struct FileContentHints {
    #[serde(rename = "indexableText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Text to be indexed for the file to improve fullText queries. This is limited to 128KB in length and may contain HTML elements."]
    pub indexable_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thumbnail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A thumbnail for the file. This will only be used if Google Drive cannot generate a standard thumbnail."]
    pub thumbnail: ::std::option::Option<FileContentHintsThumbnail>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A thumbnail for the file. This will only be used if Google Drive cannot generate a standard thumbnail."]
pub struct FileContentHintsThumbnail {
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The thumbnail data encoded with URL-safe Base64 (RFC 4648 section 5)."]
    pub image: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MIME type of the thumbnail."]
    pub mime_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional metadata about image media, if available."]
pub struct FileImageMediaMetadata {
    #[serde(rename = "aperture")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The aperture used to create the photo (f-number)."]
    pub aperture: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "cameraMake")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The make of the camera used to create the photo."]
    pub camera_make: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cameraModel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The model of the camera used to create the photo."]
    pub camera_model: ::std::option::Option<::std::string::String>,
    #[serde(rename = "colorSpace")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The color space of the photo."]
    pub color_space: ::std::option::Option<::std::string::String>,
    #[serde(rename = "exposureBias")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The exposure bias of the photo (APEX value)."]
    pub exposure_bias: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "exposureMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The exposure mode used to create the photo."]
    pub exposure_mode: ::std::option::Option<::std::string::String>,
    #[serde(rename = "exposureTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The length of the exposure, in seconds."]
    pub exposure_time: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "flashUsed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether a flash was used to create the photo."]
    pub flash_used: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "focalLength")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The focal length used to create the photo, in millimeters."]
    pub focal_length: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The height of the image in pixels."]
    pub height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "isoSpeed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ISO speed used to create the photo."]
    pub iso_speed: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "lens")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The lens used to create the photo."]
    pub lens: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Geographic location information stored in the image."]
    pub location: ::std::option::Option<FileImageMediaMetadataLocation>,
    #[serde(rename = "maxApertureValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The smallest f-number of the lens at the focal length used to create the photo (APEX value)."]
    pub max_aperture_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "meteringMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The metering mode used to create the photo."]
    pub metering_mode: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rotation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of clockwise 90 degree rotations applied from the image's original orientation."]
    pub rotation: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "sensor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of sensor used to create the photo."]
    pub sensor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subjectDistance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The distance to the subject of the photo, in meters."]
    pub subject_distance: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date and time the photo was taken (EXIF DateTime)."]
    pub time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "whiteBalance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The white balance mode used to create the photo."]
    pub white_balance: ::std::option::Option<::std::string::String>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width of the image in pixels."]
    pub width: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Geographic location information stored in the image."]
pub struct FileImageMediaMetadataLocation {
    #[serde(rename = "altitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The altitude stored in the image."]
    pub altitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "latitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The latitude stored in the image."]
    pub latitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "longitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The longitude stored in the image."]
    pub longitude: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Shortcut file details. Only populated for shortcut files, which have the mimeType field set to application/vnd.google-apps.shortcut."]
pub struct FileShortcutDetails {
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the file that this shortcut points to."]
    pub target_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetMimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MIME type of the file that this shortcut points to. The value of this field is a snapshot of the target's MIME type, captured when the shortcut is created."]
    pub target_mime_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional metadata about video media. This may not be available immediately upon upload."]
pub struct FileVideoMediaMetadata {
    #[serde(rename = "durationMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The duration of the video in milliseconds."]
    pub duration_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The height of the video in pixels."]
    pub height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width of the video in pixels."]
    pub width: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of files."]
pub struct FileList {
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of files. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
    pub files: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<File>>>,
    #[serde(rename = "incompleteSearch")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the search process was incomplete. If true, then some search results may be missing, since all documents were not searched. This may occur when searching multiple drives with the \"allDrives\" corpora, but all corpora could not be searched. When this happens, it is suggested that clients narrow their query by choosing a different corpus such as \"user\" or \"drive\"."]
    pub incomplete_search: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(default = "file_list_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#fileList\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token for the next page of files. This will be absent if the end of the files list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod file_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#fileList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of generated file IDs which can be provided in create requests."]
pub struct GeneratedIds {
    #[serde(rename = "ids")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IDs generated for the requesting user in the specified space."]
    pub ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "kind")]
    #[serde(default = "generated_ids_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#generatedIds\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "space")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of file that can be created with these IDs."]
    pub space: ::std::option::Option<::std::string::String>,
}
mod generated_ids_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#generatedIds")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy."]
pub struct Permission {
    #[serde(rename = "allowFileDiscovery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the permission allows the file to be discovered through search. This is only applicable for permissions of type domain or anyone."]
    pub allow_file_discovery: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the account associated with this permission has been deleted. This field only pertains to user and group permissions."]
    pub deleted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The \"pretty\" name of the value of the permission. The following is a list of examples for each type of permission:  \n- user - User's full name, as defined for their Google account, such as \"Joe Smith.\" \n- group - Name of the Google Group, such as \"The Company Administrators.\" \n- domain - String domain name, such as \"thecompany.com.\" \n- anyone - No displayName is present."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The domain to which this permission refers."]
    pub domain: ::std::option::Option<::std::string::String>,
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of the user or group to which this permission refers."]
    pub email_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expirationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which this permission will expire (RFC 3339 date-time). Expiration times have the following restrictions:  \n- They can only be set on user and group permissions \n- The time must be in the future \n- The time cannot be more than a year in the future"]
    pub expiration_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of this permission. This is a unique identifier for the grantee, and is published in User resources as permissionId. IDs should be treated as opaque values."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "permission_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#permission\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "permissionDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of whether the permissions on this shared drive item are inherited or directly on this item. This is an output-only field which is present only for shared drive items."]
    pub permission_details: ::std::option::Option<::std::vec::Vec<PermissionPermissionDetails>>,
    #[serde(rename = "photoLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the user's profile photo, if available."]
    pub photo_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The role granted by this permission. While new values may be supported in the future, the following are currently allowed:  \n- owner \n- organizer \n- fileOrganizer \n- writer \n- commenter \n- reader"]
    pub role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "teamDrivePermissionDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use permissionDetails instead."]
    pub team_drive_permission_details:
        ::std::option::Option<::std::vec::Vec<PermissionTeamDrivePermissionDetails>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the grantee. Valid values are:  \n- user \n- group \n- domain \n- anyone  When creating a permission, if type is user or group, you must provide an emailAddress for the user or group. When type is domain, you must provide a domain. There isn't extra information required for a anyone type."]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "view")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the view for this permission. Only populated for permissions that belong to a view. published is the only supported value."]
    pub view: ::std::option::Option<::std::string::String>,
}
mod permission_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#permission")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PermissionPermissionDetails {
    #[serde(rename = "inherited")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this permission is inherited. This field is always populated. This is an output-only field."]
    pub inherited: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "inheritedFrom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the item from which this permission is inherited. This is an output-only field."]
    pub inherited_from: ::std::option::Option<::std::string::String>,
    #[serde(rename = "permissionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The permission type for this user. While new values may be added in future, the following are currently possible:  \n- file \n- member"]
    pub permission_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The primary role for this user. While new values may be added in the future, the following are currently possible:  \n- organizer \n- fileOrganizer \n- writer \n- commenter \n- reader"]
    pub role: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PermissionTeamDrivePermissionDetails {
    #[serde(rename = "inherited")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use permissionDetails/inherited instead."]
    pub inherited: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "inheritedFrom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use permissionDetails/inheritedFrom instead."]
    pub inherited_from: ::std::option::Option<::std::string::String>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use permissionDetails/role instead."]
    pub role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "teamDrivePermissionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use permissionDetails/permissionType instead."]
    pub team_drive_permission_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of permissions for a file."]
pub struct PermissionList {
    #[serde(rename = "kind")]
    #[serde(default = "permission_list_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#permissionList\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token for the next page of permissions. This field will be absent if the end of the permissions list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of permissions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Permission>>>,
}
mod permission_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#permissionList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A reply to a comment on a file."]
pub struct Reply {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The action the reply performed to the parent comment. Valid values are:  \n- resolve \n- reopen"]
    pub action: ::std::option::Option<::std::string::String>,
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The author of the reply. The author's email address and permission ID will not be populated."]
    pub author: ::std::option::Option<::std::boxed::Box<User>>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The plain text content of the reply. This field is used for setting the content, while htmlContent should be displayed. This is required on creates if no action is specified."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the reply was created (RFC 3339 date-time)."]
    pub created_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the reply has been deleted. A deleted reply has no content."]
    pub deleted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "htmlContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content of the reply with HTML formatting."]
    pub html_content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the reply."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "reply_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#reply\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "modifiedTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last time the reply was modified (RFC 3339 date-time)."]
    pub modified_time: ::std::option::Option<::std::string::String>,
}
mod reply_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#reply")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of replies to a comment on a file."]
pub struct ReplyList {
    #[serde(rename = "kind")]
    #[serde(default = "reply_list_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#replyList\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token for the next page of replies. This will be absent if the end of the replies list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "replies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of replies. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
    pub replies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Reply>>>,
}
mod reply_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#replyList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The metadata for a revision to a file."]
pub struct Revision {
    #[serde(rename = "exportLinks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Links for exporting Docs Editors files to specific formats."]
    pub export_links:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the revision."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keepForever")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to keep this revision forever, even if it is no longer the head revision. If not set, the revision will be automatically purged 30 days after newer content is uploaded. This can be set on a maximum of 200 revisions for a file.\nThis field is only applicable to files with binary content in Drive."]
    pub keep_forever: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(default = "revision_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#revision\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "lastModifyingUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last user to modify this revision."]
    pub last_modifying_user: ::std::option::Option<::std::boxed::Box<User>>,
    #[serde(rename = "md5Checksum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MD5 checksum of the revision's content. This is only applicable to files with binary content in Drive."]
    pub md5_checksum: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MIME type of the revision."]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "modifiedTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last time the revision was modified (RFC 3339 date-time)."]
    pub modified_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "originalFilename")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The original filename used to create this revision. This is only applicable to files with binary content in Drive."]
    pub original_filename: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publishAuto")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether subsequent revisions will be automatically republished. This is only applicable to Docs Editors files."]
    pub publish_auto: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "published")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this revision is published. This is only applicable to Docs Editors files."]
    pub published: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "publishedLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the published revision. This is only populated for Google Sites files."]
    pub published_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publishedOutsideDomain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this revision is published outside the domain. This is only applicable to Docs Editors files."]
    pub published_outside_domain: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of the revision's content in bytes. This is only applicable to files with binary content in Drive."]
    pub size: ::std::option::Option<::std::string::String>,
}
mod revision_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#revision")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of revisions of a file."]
pub struct RevisionList {
    #[serde(rename = "kind")]
    #[serde(default = "revision_list_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#revisionList\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token for the next page of revisions. This will be absent if the end of the revisions list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revisions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of revisions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
    pub revisions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Revision>>>,
}
mod revision_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#revisionList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StartPageToken {
    #[serde(rename = "kind")]
    #[serde(default = "start_page_token_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#startPageToken\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "startPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The starting page token for listing changes."]
    pub start_page_token: ::std::option::Option<::std::string::String>,
}
mod start_page_token_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#startPageToken")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deprecated: use the drive collection instead."]
pub struct TeamDrive {
    #[serde(rename = "backgroundImageFile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on drive.teamdrives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set."]
    pub background_image_file: ::std::option::Option<TeamDriveBackgroundImageFile>,
    #[serde(rename = "backgroundImageLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short-lived link to this Team Drive's background image."]
    pub background_image_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Capabilities the current user has on this Team Drive."]
    pub capabilities: ::std::option::Option<TeamDriveCapabilities>,
    #[serde(rename = "colorRgb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The color of this Team Drive as an RGB hex string. It can only be set on a drive.teamdrives.update request that does not set themeId."]
    pub color_rgb: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the Team Drive was created (RFC 3339 date-time)."]
    pub created_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of this Team Drive which is also the ID of the top level folder of this Team Drive."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "team_drive_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#teamDrive\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this Team Drive."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "restrictions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of restrictions that apply to this Team Drive or items inside this Team Drive."]
    pub restrictions: ::std::option::Option<TeamDriveRestrictions>,
    #[serde(rename = "themeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the theme from which the background image and color will be set. The set of possible teamDriveThemes can be retrieved from a drive.about.get response. When not specified on a drive.teamdrives.create request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile."]
    pub theme_id: ::std::option::Option<::std::string::String>,
}
mod team_drive_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#teamDrive")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on drive.teamdrives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set."]
pub struct TeamDriveBackgroundImageFile {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of an image file in Drive to use for the background image."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width of the cropped image in the closed range of 0 to 1. This value represents the width of the cropped image divided by the width of the entire image. The height is computed by applying a width to height aspect ratio of 80 to 9. The resulting image must be at least 1280 pixels wide and 144 pixels high."]
    pub width: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "xCoordinate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The X coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the horizontal distance from the left side of the entire image to the left side of the cropping area divided by the width of the entire image."]
    pub x_coordinate: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "yCoordinate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Y coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the vertical distance from the top side of the entire image to the top side of the cropping area divided by the height of the entire image."]
    pub y_coordinate: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Capabilities the current user has on this Team Drive."]
pub struct TeamDriveCapabilities {
    #[serde(rename = "canAddChildren")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can add children to folders in this Team Drive."]
    pub can_add_children: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canChangeCopyRequiresWriterPermissionRestriction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can change the copyRequiresWriterPermission restriction of this Team Drive."]
    pub can_change_copy_requires_writer_permission_restriction:
        ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canChangeDomainUsersOnlyRestriction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can change the domainUsersOnly restriction of this Team Drive."]
    pub can_change_domain_users_only_restriction: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canChangeTeamDriveBackground")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can change the background of this Team Drive."]
    pub can_change_team_drive_background: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canChangeTeamMembersOnlyRestriction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can change the teamMembersOnly restriction of this Team Drive."]
    pub can_change_team_members_only_restriction: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canComment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can comment on files in this Team Drive."]
    pub can_comment: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canCopy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can copy files in this Team Drive."]
    pub can_copy: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canDeleteChildren")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can delete children from folders in this Team Drive."]
    pub can_delete_children: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canDeleteTeamDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can delete this Team Drive. Attempting to delete the Team Drive may still fail if there are untrashed items inside the Team Drive."]
    pub can_delete_team_drive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canDownload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can download files in this Team Drive."]
    pub can_download: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canEdit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can edit files in this Team Drive"]
    pub can_edit: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canListChildren")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can list the children of folders in this Team Drive."]
    pub can_list_children: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canManageMembers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can add members to this Team Drive or remove them or change their role."]
    pub can_manage_members: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canReadRevisions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can read the revisions resource of files in this Team Drive."]
    pub can_read_revisions: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canRemoveChildren")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use canDeleteChildren or canTrashChildren instead."]
    pub can_remove_children: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canRename")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can rename files or folders in this Team Drive."]
    pub can_rename: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canRenameTeamDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can rename this Team Drive."]
    pub can_rename_team_drive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canShare")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can share files or folders in this Team Drive."]
    pub can_share: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canTrashChildren")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current user can trash children from folders in this Team Drive."]
    pub can_trash_children: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A set of restrictions that apply to this Team Drive or items inside this Team Drive."]
pub struct TeamDriveRestrictions {
    #[serde(rename = "adminManagedRestrictions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether administrative privileges on this Team Drive are required to modify restrictions."]
    pub admin_managed_restrictions: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "copyRequiresWriterPermission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the options to copy, print, or download files inside this Team Drive, should be disabled for readers and commenters. When this restriction is set to true, it will override the similarly named field to true for any file inside this Team Drive."]
    pub copy_requires_writer_permission: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "domainUsersOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether access to this Team Drive and items inside this Team Drive is restricted to users of the domain to which this Team Drive belongs. This restriction may be overridden by other sharing policies controlled outside of this Team Drive."]
    pub domain_users_only: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "teamMembersOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether access to items inside this Team Drive is restricted to members of this Team Drive."]
    pub team_members_only: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of Team Drives."]
pub struct TeamDriveList {
    #[serde(rename = "kind")]
    #[serde(default = "team_drive_list_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#teamDriveList\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token for the next page of Team Drives. This will be absent if the end of the Team Drives list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "teamDrives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of Team Drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
    pub team_drives: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TeamDrive>>>,
}
mod team_drive_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#teamDriveList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about a Drive user."]
pub struct User {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A plain text displayable name for this user."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of the user. This may not be present in certain contexts if the user has not made their email address visible to the requester."]
    pub email_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "user_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#user\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "me")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this user is the requesting user."]
    pub me: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "permissionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's ID as visible in Permission resources."]
    pub permission_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "photoLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the user's profile photo, if available."]
    pub photo_link: ::std::option::Option<::std::string::String>,
}
mod user_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#user")
    }
}
