#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An item with user information and settings."]
pub struct About {
    #[serde(rename = "additionalRoleInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about supported additional roles per file type. The most specific type takes precedence."]
    pub additional_role_info: ::std::option::Option<::std::vec::Vec<AboutAdditionalRoleInfo>>,
    #[serde(rename = "canCreateDrives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the user can create shared drives."]
    pub can_create_drives: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canCreateTeamDrives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use canCreateDrives instead."]
    pub can_create_team_drives: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "domainSharingPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The domain sharing policy for the current user. Possible values are:  \n- allowed \n- allowedWithWarning \n- incomingOnly \n- disallowed"]
    pub domain_sharing_policy: ::std::option::Option<::std::string::String>,
    #[serde(rename = "driveThemes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of themes that are supported for shared drives."]
    pub drive_themes: ::std::option::Option<::std::vec::Vec<AboutDriveThemes>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ETag of the item."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "exportFormats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The allowable export formats."]
    pub export_formats: ::std::option::Option<::std::vec::Vec<AboutExportFormats>>,
    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of additional features enabled on this account."]
    pub features: ::std::option::Option<::std::vec::Vec<AboutFeatures>>,
    #[serde(rename = "folderColorPalette")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The palette of allowable folder colors as RGB hex strings."]
    pub folder_color_palette: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "importFormats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The allowable import formats."]
    pub import_formats: ::std::option::Option<::std::vec::Vec<AboutImportFormats>>,
    #[serde(rename = "isCurrentAppInstalled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A boolean indicating whether the authenticated app is installed by the authenticated user."]
    pub is_current_app_installed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(default = "about_defaults :: kind")]
    #[doc = "This is always drive#about."]
    pub kind: ::std::string::String,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's language or locale code, as defined by BCP 47, with some extensions from Unicode's LDML format (http://www.unicode.org/reports/tr35/)."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "largestChangeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The largest change id."]
    pub largest_change_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxUploadSizes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of max upload sizes for each file type. The most specific type takes precedence."]
    pub max_upload_sizes: ::std::option::Option<::std::vec::Vec<AboutMaxUploadSizes>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the current user."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "permissionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current user's ID as visible in the permissions collection."]
    pub permission_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quotaBytesByService")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of storage quota used by different Google services."]
    pub quota_bytes_by_service: ::std::option::Option<::std::vec::Vec<AboutQuotaBytesByService>>,
    #[serde(rename = "quotaBytesTotal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of quota bytes. This is only relevant when quotaType is LIMITED."]
    pub quota_bytes_total: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quotaBytesUsed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of quota bytes used by Google Drive."]
    pub quota_bytes_used: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quotaBytesUsedAggregate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of quota bytes used by all Google apps (Drive, Picasa, etc.)."]
    pub quota_bytes_used_aggregate: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quotaBytesUsedInTrash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of quota bytes used by trashed items."]
    pub quota_bytes_used_in_trash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quotaType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the user's storage quota. Possible values are:  \n- LIMITED \n- UNLIMITED"]
    pub quota_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "remainingChangeIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of remaining change ids, limited to no more than 2500."]
    pub remaining_change_ids: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rootFolderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the root folder."]
    pub root_folder_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link back to this item."]
    pub self_link: ::std::option::Option<::std::string::String>,
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
pub struct AboutAdditionalRoleInfo {
    #[serde(rename = "roleSets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The supported additional roles per primary role."]
    pub role_sets: ::std::option::Option<::std::vec::Vec<AboutAdditionalRoleInfoRoleSets>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content type that this additional role info applies to."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AboutAdditionalRoleInfoRoleSets {
    #[serde(rename = "additionalRoles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The supported additional roles with the primary role."]
    pub additional_roles: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "primaryRole")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A primary permission role."]
    pub primary_role: ::std::option::Option<::std::string::String>,
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
pub struct AboutExportFormats {
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content type to convert from."]
    pub source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The possible content types to convert to."]
    pub targets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AboutFeatures {
    #[serde(rename = "featureName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the feature."]
    pub feature_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "featureRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request limit rate for this feature, in queries per second."]
    pub feature_rate: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AboutImportFormats {
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The imported file's content type to convert from."]
    pub source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The possible content types to convert to."]
    pub targets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AboutMaxUploadSizes {
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The max upload size for this type."]
    pub size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The file type."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AboutQuotaBytesByService {
    #[serde(rename = "bytesUsed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The storage quota bytes used by the service."]
    pub bytes_used: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The service's name, e.g. DRIVE, GMAIL, or PHOTOS."]
    pub service_name: ::std::option::Option<::std::string::String>,
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
#[doc = "The apps resource provides a list of the apps that a user has installed, with information about each app's supported MIME types, file extensions, and other details."]
pub struct App {
    #[serde(rename = "authorized")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the app is authorized to access data on the user's Drive."]
    pub authorized: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "createInFolderTemplate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The template url to create a new file with this app in a given folder. The template will contain {folderId} to be replaced by the folder to create the new file in."]
    pub create_in_folder_template: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The url to create a new file with this app."]
    pub create_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hasDriveWideScope")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the app has drive-wide scope. An app with drive-wide scope can access all files in the user's drive."]
    pub has_drive_wide_scope: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "icons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The various icons for the app."]
    pub icons: ::std::option::Option<::std::vec::Vec<AppIcons>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the app."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "installed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the app is installed."]
    pub installed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(default = "app_defaults :: kind")]
    #[doc = "This is always drive#app."]
    pub kind: ::std::string::String,
    #[serde(rename = "longDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A long description of the app."]
    pub long_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the app."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of object this app creates (e.g. Chart). If empty, the app name should be used instead."]
    pub object_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "openUrlTemplate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The template url for opening files with this app. The template will contain {ids} and/or {exportIds} to be replaced by the actual file ids. See  Open Files  for the full documentation."]
    pub open_url_template: ::std::option::Option<::std::string::String>,
    #[serde(rename = "primaryFileExtensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of primary file extensions."]
    pub primary_file_extensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "primaryMimeTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of primary mime types."]
    pub primary_mime_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product listing for this app."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the product listing for this app."]
    pub product_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "secondaryFileExtensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of secondary file extensions."]
    pub secondary_file_extensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "secondaryMimeTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of secondary mime types."]
    pub secondary_mime_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "shortDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short description of the app."]
    pub short_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "supportsCreate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this app supports creating new objects."]
    pub supports_create: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "supportsImport")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this app supports importing from Docs Editors."]
    pub supports_import: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "supportsMultiOpen")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this app supports opening more than one file."]
    pub supports_multi_open: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "supportsOfflineCreate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this app supports creating new files when offline."]
    pub supports_offline_create: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "useByDefault")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the app is selected as the default handler for the types it supports."]
    pub use_by_default: ::std::option::Option<::std::primitive::bool>,
}
mod app_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#app")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppIcons {
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Category of the icon. Allowed values are:  \n- application - icon for the application \n- document - icon for a file associated with the app \n- documentShared - icon for a shared file associated with the app"]
    pub category: ::std::option::Option<::std::string::String>,
    #[serde(rename = "iconUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL for the icon."]
    pub icon_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size of the icon. Represented as the maximum of the width and height."]
    pub size: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of third-party applications which the user has installed or given access to Google Drive."]
pub struct AppList {
    #[serde(rename = "defaultAppIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of app IDs that the user has specified to use by default. The list is in reverse-priority order (lowest to highest)."]
    pub default_app_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ETag of the list."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of apps."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<App>>>,
    #[serde(rename = "kind")]
    #[serde(default = "app_list_defaults :: kind")]
    #[doc = "This is always drive#appList."]
    pub kind: ::std::string::String,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link back to this list."]
    pub self_link: ::std::option::Option<::std::string::String>,
}
mod app_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#appList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Representation of a change to a file or shared drive."]
pub struct Change {
    #[serde(rename = "changeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the change. Possible values are file and drive."]
    pub change_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the file or shared drive has been removed from this list of changes, for example by deletion or loss of access."]
    pub deleted: ::std::option::Option<::std::primitive::bool>,
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
    #[doc = "The ID of the file associated with this change."]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the change."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "change_defaults :: kind")]
    #[doc = "This is always drive#change."]
    pub kind: ::std::string::String,
    #[serde(rename = "modificationDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time of this modification."]
    pub modification_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link back to this change."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "teamDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use drive instead."]
    pub team_drive: ::std::option::Option<::std::boxed::Box<TeamDrive>>,
    #[serde(rename = "teamDriveId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use driveId instead."]
    pub team_drive_id: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ETag of the list."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of changes. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Change>>>,
    #[serde(rename = "kind")]
    #[serde(default = "change_list_defaults :: kind")]
    #[doc = "This is always drive#changeList."]
    pub kind: ::std::string::String,
    #[serde(rename = "largestChangeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current largest change ID."]
    pub largest_change_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "newStartPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The starting page token for future changes. This will be present only if the end of the current changes list has been reached."]
    pub new_start_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the next page of changes."]
    pub next_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token for the next page of changes. This will be absent if the end of the changes list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link back to this list."]
    pub self_link: ::std::option::Option<::std::string::String>,
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
#[doc = "A list of children of a file."]
pub struct ChildList {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ETag of the list."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of children. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ChildReference>>>,
    #[serde(rename = "kind")]
    #[serde(default = "child_list_defaults :: kind")]
    #[doc = "This is always drive#childList."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the next page of children."]
    pub next_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token for the next page of children. This will be absent if the end of the children list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link back to this list."]
    pub self_link: ::std::option::Option<::std::string::String>,
}
mod child_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#childList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A reference to a folder's child."]
pub struct ChildReference {
    #[serde(rename = "childLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the child."]
    pub child_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the child."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "child_reference_defaults :: kind")]
    #[doc = "This is always drive#childReference."]
    pub kind: ::std::string::String,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link back to this reference."]
    pub self_link: ::std::option::Option<::std::string::String>,
}
mod child_reference_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#childReference")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A comment on a file in Google Drive."]
pub struct Comment {
    #[serde(rename = "anchor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A region of the document represented as a JSON string. See anchor documentation for details on how to define and interpret anchor properties."]
    pub anchor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The author of the comment. The author's email address and permission ID will not be populated."]
    pub author: ::std::option::Option<::std::boxed::Box<User>>,
    #[serde(rename = "commentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the comment."]
    pub comment_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The plain text content used to create this comment. This is not HTML safe and should only be used as a starting point to make edits to a comment's content."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The context of the file which is being commented on."]
    pub context: ::std::option::Option<CommentContext>,
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date when this comment was first created."]
    pub created_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this comment has been deleted. If a comment has been deleted the content will be cleared and this will only represent a comment that once existed."]
    pub deleted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The file which this comment is addressing."]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the file which this comment is addressing."]
    pub file_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "htmlContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTML formatted content for this comment."]
    pub html_content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "comment_defaults :: kind")]
    #[doc = "This is always drive#comment."]
    pub kind: ::std::string::String,
    #[serde(rename = "modifiedDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date when this comment or any of its replies were last modified."]
    pub modified_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "replies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Replies to this post."]
    pub replies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CommentReply>>>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link back to this comment."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of this comment. Status can be changed by posting a reply to a comment with the desired status.  \n- \"open\" - The comment is still open. \n- \"resolved\" - The comment has been resolved by one of its replies."]
    pub status: ::std::option::Option<::std::string::String>,
}
mod comment_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#comment")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The context of the file which is being commented on."]
pub struct CommentContext {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MIME type of the context snippet."]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Data representation of the segment of the file being commented on. In the case of a text file for example, this would be the actual text that the comment is about."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of comments on a file in Google Drive."]
pub struct CommentList {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of comments. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Comment>>>,
    #[serde(rename = "kind")]
    #[serde(default = "comment_list_defaults :: kind")]
    #[doc = "This is always drive#commentList."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the next page of comments."]
    pub next_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token for the next page of comments. This will be absent if the end of the comments list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link back to this list."]
    pub self_link: ::std::option::Option<::std::string::String>,
}
mod comment_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#commentList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A comment on a file in Google Drive."]
pub struct CommentReply {
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The author of the reply. The author's email address and permission ID will not be populated."]
    pub author: ::std::option::Option<::std::boxed::Box<User>>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The plain text content used to create this reply. This is not HTML safe and should only be used as a starting point to make edits to a reply's content. This field is required on inserts if no verb is specified (resolve/reopen)."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date when this reply was first created."]
    pub created_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this reply has been deleted. If a reply has been deleted the content will be cleared and this will only represent a reply that once existed."]
    pub deleted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "htmlContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTML formatted content for this reply."]
    pub html_content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "comment_reply_defaults :: kind")]
    #[doc = "This is always drive#commentReply."]
    pub kind: ::std::string::String,
    #[serde(rename = "modifiedDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date when this reply was last modified."]
    pub modified_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "replyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the reply."]
    pub reply_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "verb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The action this reply performed to the parent comment. When creating a new reply this is the action to be perform to the parent comment. Possible values are:  \n- \"resolve\" - To resolve a comment. \n- \"reopen\" - To reopen (un-resolve) a comment."]
    pub verb: ::std::option::Option<::std::string::String>,
}
mod comment_reply_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#commentReply")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of replies to a comment on a file in Google Drive."]
pub struct CommentReplyList {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of replies. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CommentReply>>>,
    #[serde(rename = "kind")]
    #[serde(default = "comment_reply_list_defaults :: kind")]
    #[doc = "This is always drive#commentReplyList."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the next page of replies."]
    pub next_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token for the next page of replies. This will be absent if the end of the replies list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link back to this list."]
    pub self_link: ::std::option::Option<::std::string::String>,
}
mod comment_reply_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#commentReplyList")
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
    #[serde(rename = "restrictionDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the content restriction was set (formatted RFC 3339 timestamp). Only populated if readOnly is true."]
    pub restriction_date: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the shared drive was created (RFC 3339 date-time)."]
    pub created_date: ::std::option::Option<::std::string::String>,
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
    #[doc = "This is always drive#drive"]
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
    #[doc = "The ID of the theme from which the background image and color will be set. The set of possible driveThemes can be retrieved from a drive.about.get response. When not specified on a drive.drives.insert request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile."]
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
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of shared drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Drive>>>,
    #[serde(rename = "kind")]
    #[serde(default = "drive_list_defaults :: kind")]
    #[doc = "This is always drive#driveList"]
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
    #[serde(rename = "alternateLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link for opening the file in a relevant Google editor or viewer."]
    pub alternate_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "appDataContents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this file is in the Application Data folder."]
    pub app_data_contents: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canComment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated: use capabilities/canComment."]
    pub can_comment: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "canReadRevisions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated: use capabilities/canReadRevisions."]
    pub can_read_revisions: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take."]
    pub capabilities: ::std::option::Option<FileCapabilities>,
    #[serde(rename = "contentRestrictions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restrictions for accessing the content of the file. Only populated if such a restriction exists."]
    pub content_restrictions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContentRestriction>>>,
    #[serde(rename = "copyRequiresWriterPermission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the options to copy, print, or download this file, should be disabled for readers and commenters."]
    pub copy_requires_writer_permission: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "copyable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated: use capabilities/canCopy."]
    pub copyable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Create time for this file (formatted RFC 3339 timestamp)."]
    pub created_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultOpenWithLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to open this file with the user's default app for this file. Only populated when the drive.apps.readonly scope is used."]
    pub default_open_with_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short description of the file."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "downloadUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Short lived download URL for the file. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files."]
    pub download_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "driveId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the shared drive the file resides in. Only populated for items in shared drives."]
    pub drive_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "editable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated: use capabilities/canEdit."]
    pub editable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "embedLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link for embedding the file."]
    pub embed_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the file."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "explicitlyTrashed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this file has been explicitly trashed, as opposed to recursively trashed."]
    pub explicitly_trashed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "exportLinks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Links for exporting Docs Editors files to specific formats."]
    pub export_links:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "fileExtension")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The final component of fullFileExtension with trailing text that does not appear to be part of the extension removed. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files."]
    pub file_extension: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of the file in bytes. This field is populated for files with content stored in Google Drive and for files in Docs Editors; it is not populated for shortcut files."]
    pub file_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "folderColorRgb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Folder color as an RGB hex string if the file is a folder. The list of supported colors is available in the folderColorPalette field of the About resource. If an unsupported color is specified, it will be changed to the closest color in the palette. Not populated for items in shared drives."]
    pub folder_color_rgb: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fullFileExtension")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full file extension; extracted from the title. May contain multiple concatenated extensions, such as \"tar.gz\". Removing an extension from the title does not clear this field; however, changing the extension on the title does update this field. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files."]
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
    #[doc = "The ID of the file's head revision. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files."]
    pub head_revision_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "iconLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the file's icon."]
    pub icon_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the file."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imageMediaMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about image media. This will only be present for image types, and its contents will depend on what can be parsed from the image content."]
    pub image_media_metadata: ::std::option::Option<FileImageMediaMetadata>,
    #[serde(rename = "indexableText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indexable text attributes for the file (can only be written)"]
    pub indexable_text: ::std::option::Option<FileIndexableText>,
    #[serde(rename = "isAppAuthorized")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the file was created or opened by the requesting app."]
    pub is_app_authorized: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(default = "file_defaults :: kind")]
    #[doc = "The type of file. This is always drive#file."]
    pub kind: ::std::string::String,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A group of labels for the file."]
    pub labels: ::std::option::Option<FileLabels>,
    #[serde(rename = "lastModifyingUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last user to modify this file."]
    pub last_modifying_user: ::std::option::Option<::std::boxed::Box<User>>,
    #[serde(rename = "lastModifyingUserName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the last user to modify this file."]
    pub last_modifying_user_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastViewedByMeDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last time this file was viewed by the user (formatted RFC 3339 timestamp)."]
    pub last_viewed_by_me_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "markedViewedByMeDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub marked_viewed_by_me_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "md5Checksum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An MD5 checksum for the content of this file. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files."]
    pub md5_checksum: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MIME type of the file. This is only mutable on update when uploading new content. This field can be left blank, and the mimetype will be determined from the uploaded content's MIME type."]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "modifiedByMeDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last time this file was modified by the user (formatted RFC 3339 timestamp). Note that setting modifiedDate will also update the modifiedByMe date for the user which set the date."]
    pub modified_by_me_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "modifiedDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last time this file was modified by anyone (formatted RFC 3339 timestamp). This is only mutable on update when the setModifiedDate parameter is set."]
    pub modified_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "openWithLinks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A map of the id of each of the user's apps to a link to open this file with that app. Only populated when the drive.apps.readonly scope is used."]
    pub open_with_links:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "originalFilename")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The original filename of the uploaded content if available, or else the original value of the title field. This is only available for files with binary content in Google Drive."]
    pub original_filename: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ownedByMe")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the file is owned by the current user. Not populated for items in shared drives."]
    pub owned_by_me: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "ownerNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name(s) of the owner(s) of this file. Not populated for items in shared drives."]
    pub owner_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "owners")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The owner(s) of this file. Not populated for items in shared drives."]
    pub owners: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<User>>>,
    #[serde(rename = "parents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Collection of parent folders which contain this file.\nIf not specified as part of an insert request, the file will be placed directly in the user's My Drive folder. If not specified as part of a copy request, the file will inherit any discoverable parents of the source file. Update requests can also use the addParents and removeParents parameters to modify the parents list."]
    pub parents: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ParentReference>>>,
    #[serde(rename = "permissionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of permission IDs for users with access to this file."]
    pub permission_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of permissions for users with access to this file. Not populated for items in shared drives."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Permission>>>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of properties."]
    pub properties: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Property>>>,
    #[serde(rename = "quotaBytesUsed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of quota bytes used by this file."]
    pub quota_bytes_used: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link back to this file."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shareable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated: use capabilities/canShare."]
    pub shareable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "shared")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the file has been shared. Not populated for items in shared drives."]
    pub shared: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "sharedWithMeDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time at which this file was shared with the user (formatted RFC 3339 timestamp)."]
    pub shared_with_me_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sharingUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User that shared the item with the current user, if available."]
    pub sharing_user: ::std::option::Option<::std::boxed::Box<User>>,
    #[serde(rename = "shortcutDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shortcut file details. Only populated for shortcut files, which have the mimeType field set to application/vnd.google-apps.shortcut."]
    pub shortcut_details: ::std::option::Option<FileShortcutDetails>,
    #[serde(rename = "spaces")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of spaces which contain the file. Supported values are 'drive', 'appDataFolder' and 'photos'."]
    pub spaces: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "teamDriveId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use driveId instead."]
    pub team_drive_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thumbnail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A thumbnail for the file. This will only be used if a standard thumbnail cannot be generated."]
    pub thumbnail: ::std::option::Option<FileThumbnail>,
    #[serde(rename = "thumbnailLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short-lived link to the file's thumbnail. Typically lasts on the order of hours. Only populated when the requesting app can access the file's content. If the file isn't shared publicly, the URL returned in Files.thumbnailLink must be fetched using a credentialed request."]
    pub thumbnail_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thumbnailVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The thumbnail version for use in thumbnail cache invalidation."]
    pub thumbnail_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of this file. Note that for immutable items such as the top level folders of shared drives, My Drive root folder, and Application Data folder the title is constant."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trashedDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time that the item was trashed (formatted RFC 3339 timestamp). Only populated for items in shared drives."]
    pub trashed_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trashingUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives."]
    pub trashing_user: ::std::option::Option<::std::boxed::Box<User>>,
    #[serde(rename = "userPermission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The permissions for the authenticated user on this file."]
    pub user_permission: ::std::option::Option<::std::boxed::Box<Permission>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the requesting user."]
    pub version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "videoMediaMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about video media. This will only be present for video types."]
    pub video_media_metadata: ::std::option::Option<FileVideoMediaMetadata>,
    #[serde(rename = "webContentLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link for downloading the content of the file in a browser using cookie based authentication. In cases where the content is shared publicly, the content can be downloaded without any credentials."]
    pub web_content_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "webViewLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link only available on public folders for viewing their static web assets (HTML, CSS, JS, etc) via Google Drive's Website Hosting."]
    pub web_view_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "writersCanShare")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether writers can share the document with other users. Not populated for items in shared drives."]
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
    #[serde(rename = "canChangeRestrictedDownload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated"]
    pub can_change_restricted_download: ::std::option::Option<::std::primitive::bool>,
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
#[doc = "Metadata about image media. This will only be present for image types, and its contents will depend on what can be parsed from the image content."]
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
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date and time the photo was taken (EXIF format timestamp)."]
    pub date: ::std::option::Option<::std::string::String>,
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
#[doc = "Indexable text attributes for the file (can only be written)"]
pub struct FileIndexableText {
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text to be indexed for this file."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A group of labels for the file."]
pub struct FileLabels {
    #[serde(rename = "hidden")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub hidden: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "modified")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the file has been modified by this user."]
    pub modified: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "restricted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use copyRequiresWriterPermission instead."]
    pub restricted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "starred")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this file is starred by the user."]
    pub starred: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "trashed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file. The trashed item is excluded from all files.list responses returned for any user who does not own the file. However, all users with access to the file can see the trashed item metadata in an API response. All users with access can copy, download, export, and share the file."]
    pub trashed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "viewed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this file has been viewed by this user."]
    pub viewed: ::std::option::Option<::std::primitive::bool>,
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
#[doc = "A thumbnail for the file. This will only be used if a standard thumbnail cannot be generated."]
pub struct FileThumbnail {
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL-safe Base64 encoded bytes of the thumbnail image. It should conform to RFC 4648 section 5."]
    pub image: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MIME type of the thumbnail."]
    pub mime_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata about video media. This will only be present for video types."]
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
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ETag of the list."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "incompleteSearch")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the search process was incomplete. If true, then some search results may be missing, since all documents were not searched. This may occur when searching multiple drives with the \"allDrives\" corpora, but all corpora could not be searched. When this happens, it is suggested that clients narrow their query by choosing a different corpus such as \"default\" or \"drive\"."]
    pub incomplete_search: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of files. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<File>>>,
    #[serde(rename = "kind")]
    #[serde(default = "file_list_defaults :: kind")]
    #[doc = "This is always drive#fileList."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the next page of files."]
    pub next_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token for the next page of files. This will be absent if the end of the files list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link back to this list."]
    pub self_link: ::std::option::Option<::std::string::String>,
}
mod file_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#fileList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of generated IDs which can be provided in insert requests"]
pub struct GeneratedIds {
    #[serde(rename = "ids")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IDs generated for the requesting user in the specified space."]
    pub ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "kind")]
    #[serde(default = "generated_ids_defaults :: kind")]
    #[doc = "This is always drive#generatedIds"]
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
#[doc = "A list of a file's parents."]
pub struct ParentList {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ETag of the list."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of parents."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ParentReference>>>,
    #[serde(rename = "kind")]
    #[serde(default = "parent_list_defaults :: kind")]
    #[doc = "This is always drive#parentList."]
    pub kind: ::std::string::String,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link back to this list."]
    pub self_link: ::std::option::Option<::std::string::String>,
}
mod parent_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#parentList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A reference to a file's parent."]
pub struct ParentReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the parent."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isRoot")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not the parent is the root folder."]
    pub is_root: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(default = "parent_reference_defaults :: kind")]
    #[doc = "This is always drive#parentReference."]
    pub kind: ::std::string::String,
    #[serde(rename = "parentLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the parent."]
    pub parent_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link back to this reference."]
    pub self_link: ::std::option::Option<::std::string::String>,
}
mod parent_reference_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#parentReference")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A permission for a file."]
pub struct Permission {
    #[serde(rename = "additionalRoles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional roles for this user. Only commenter is currently allowed, though more may be supported in the future."]
    pub additional_roles: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "authKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub auth_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the account associated with this permission has been deleted. This field only pertains to user and group permissions."]
    pub deleted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The domain name of the entity this permission refers to. This is an output-only field which is present when the permission type is user, group or domain."]
    pub domain: ::std::option::Option<::std::string::String>,
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of the user or group this permission refers to. This is an output-only field which is present when the permission type is user or group."]
    pub email_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ETag of the permission."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which this permission will expire (RFC 3339 date-time). Expiration dates have the following restrictions:  \n- They cannot be set on shared drive items \n- They can only be set on user and group permissions \n- The date must be in the future \n- The date cannot be more than a year in the future \n- The date can only be set on drive.permissions.update or drive.permissions.patch requests"]
    pub expiration_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the user this permission refers to, and identical to the permissionId in the About and Files resources. When making a drive.permissions.insert request, exactly one of the id or value fields must be specified unless the permission type is anyone, in which case both id and value are ignored."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "permission_defaults :: kind")]
    #[doc = "This is always drive#permission."]
    pub kind: ::std::string::String,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name for this permission."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "permissionDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of whether the permissions on this shared drive item are inherited or directly on this item. This is an output-only field which is present only for shared drive items."]
    pub permission_details: ::std::option::Option<::std::vec::Vec<PermissionPermissionDetails>>,
    #[serde(rename = "photoLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the profile photo, if available."]
    pub photo_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The primary role for this user. While new values may be supported in the future, the following are currently allowed:  \n- owner \n- organizer \n- fileOrganizer \n- writer \n- reader"]
    pub role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link back to this permission."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "teamDrivePermissionDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use permissionDetails instead."]
    pub team_drive_permission_details:
        ::std::option::Option<::std::vec::Vec<PermissionTeamDrivePermissionDetails>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The account type. Allowed values are:  \n- user \n- group \n- domain \n- anyone"]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address or domain name for the entity. This is used during inserts and is not populated in responses. When making a drive.permissions.insert request, exactly one of the id or value fields must be specified unless the permission type is anyone, in which case both id and value are ignored."]
    pub value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "view")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the view for this permission. Only populated for permissions that belong to a view. published is the only supported value."]
    pub view: ::std::option::Option<::std::string::String>,
    #[serde(rename = "withLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the link is required for this permission."]
    pub with_link: ::std::option::Option<::std::primitive::bool>,
}
mod permission_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#permission")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PermissionPermissionDetails {
    #[serde(rename = "additionalRoles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional roles for this user. Only commenter is currently possible, though more may be supported in the future."]
    pub additional_roles: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
    #[doc = "The primary role for this user. While new values may be added in the future, the following are currently possible:  \n- organizer \n- fileOrganizer \n- writer \n- reader"]
    pub role: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PermissionTeamDrivePermissionDetails {
    #[serde(rename = "additionalRoles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated - use permissionDetails/additionalRoles instead."]
    pub additional_roles: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
#[doc = "An ID for a user or group as seen in Permission items."]
pub struct PermissionId {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The permission ID."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "permission_id_defaults :: kind")]
    #[doc = "This is always drive#permissionId."]
    pub kind: ::std::string::String,
}
mod permission_id_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#permissionId")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of permissions associated with a file."]
pub struct PermissionList {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ETag of the list."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of permissions."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Permission>>>,
    #[serde(rename = "kind")]
    #[serde(default = "permission_list_defaults :: kind")]
    #[doc = "This is always drive#permissionList."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token for the next page of permissions. This field will be absent if the end of the permissions list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link back to this list."]
    pub self_link: ::std::option::Option<::std::string::String>,
}
mod permission_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#permissionList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A key-value pair attached to a file that is either public or private to an application.\nThe following limits apply to file properties:  \n- Maximum of 100 properties total per file\n- Maximum of 30 private properties per app\n- Maximum of 30 public properties\n- Maximum of 124 bytes size limit on (key + value) string in UTF-8 encoding for a single property."]
pub struct Property {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the property."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The key of this property."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "property_defaults :: kind")]
    #[doc = "This is always drive#property."]
    pub kind: ::std::string::String,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The link back to this property."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of this property."]
    pub value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The visibility of this property. Allowed values are PRIVATE and PUBLIC. (Default: PRIVATE). Private properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties."]
    pub visibility: ::std::option::Option<::std::string::String>,
}
mod property_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#property")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A collection of properties, key-value pairs that are either public or private to an application."]
pub struct PropertyList {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ETag of the list."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of properties."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Property>>>,
    #[serde(rename = "kind")]
    #[serde(default = "property_list_defaults :: kind")]
    #[doc = "This is always drive#propertyList."]
    pub kind: ::std::string::String,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The link back to this list."]
    pub self_link: ::std::option::Option<::std::string::String>,
}
mod property_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#propertyList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A revision of a file."]
pub struct Revision {
    #[serde(rename = "downloadUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub download_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ETag of the revision."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "exportLinks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Links for exporting Docs Editors files to specific formats."]
    pub export_links:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "fileSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of the revision in bytes. This will only be populated on files with content stored in Drive."]
    pub file_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the revision."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "revision_defaults :: kind")]
    #[doc = "This is always drive#revision."]
    pub kind: ::std::string::String,
    #[serde(rename = "lastModifyingUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last user to modify this revision."]
    pub last_modifying_user: ::std::option::Option<::std::boxed::Box<User>>,
    #[serde(rename = "lastModifyingUserName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the last user to modify this revision."]
    pub last_modifying_user_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "md5Checksum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An MD5 checksum for the content of this revision. This will only be populated on files with content stored in Drive."]
    pub md5_checksum: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MIME type of the revision."]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "modifiedDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last time this revision was modified (formatted RFC 3339 timestamp)."]
    pub modified_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "originalFilename")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The original filename when this revision was created. This will only be populated on files with content stored in Drive."]
    pub original_filename: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pinned")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this revision is pinned to prevent automatic purging. This will only be populated and can only be modified on files with content stored in Drive, excluding Docs Editors files. Revisions can also be pinned when they are created through the drive.files.insert/update/copy by using the pinned query parameter. Pinned revisions are stored indefinitely using additional storage quota, up to a maximum of 200 revisions."]
    pub pinned: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "publishAuto")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether subsequent revisions will be automatically republished. This is only populated and can only be modified for Docs Editors files."]
    pub publish_auto: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "published")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this revision is published. This is only populated and can only be modified for Docs Editors files."]
    pub published: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "publishedLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the published revision. This is only populated for Google Sites files."]
    pub published_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publishedOutsideDomain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this revision is published outside the domain. This is only populated and can only be modified for Docs Editors files."]
    pub published_outside_domain: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link back to this revision."]
    pub self_link: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ETag of the list."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of revisions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Revision>>>,
    #[serde(rename = "kind")]
    #[serde(default = "revision_list_defaults :: kind")]
    #[doc = "This is always drive#revisionList."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token for the next page of revisions. This field will be absent if the end of the revisions list has been reached. If the token is rejected for any reason, it should be discarded and pagination should be restarted from the first page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link back to this list."]
    pub self_link: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the Team Drive was created (RFC 3339 date-time)."]
    pub created_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of this Team Drive which is also the ID of the top level folder of this Team Drive."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "team_drive_defaults :: kind")]
    #[doc = "This is always drive#teamDrive"]
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
    #[doc = "The ID of the theme from which the background image and color will be set. The set of possible teamDriveThemes can be retrieved from a drive.about.get response. When not specified on a drive.teamdrives.insert request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile."]
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
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of Team Drives."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TeamDrive>>>,
    #[serde(rename = "kind")]
    #[serde(default = "team_drive_list_defaults :: kind")]
    #[doc = "This is always drive#teamDriveList"]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token for the next page of Team Drives."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
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
    #[doc = "The email address of the user."]
    pub email_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isAuthenticatedUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this user is the same as the authenticated user for whom the request was made."]
    pub is_authenticated_user: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(default = "user_defaults :: kind")]
    #[doc = "This is always drive#user."]
    pub kind: ::std::string::String,
    #[serde(rename = "permissionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's ID as visible in the permissions collection."]
    pub permission_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "picture")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's profile picture."]
    pub picture: ::std::option::Option<UserPicture>,
}
mod user_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("drive#user")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The user's profile picture."]
pub struct UserPicture {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL that points to a profile picture of this user."]
    pub url: ::std::option::Option<::std::string::String>,
}
