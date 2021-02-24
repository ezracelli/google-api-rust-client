#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the action."]
pub struct Action {
    #[serde(rename = "actor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The actor responsible for this action (or empty if all actors are responsible)."]
    pub actor: ::std::option::Option<::std::boxed::Box<Actor>>,
    #[serde(rename = "detail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type and detailed information about the action."]
    pub detail: ::std::option::Option<::std::boxed::Box<ActionDetail>>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target this action affects (or empty if affecting all targets). This represents the state of the target immediately after this action occurred."]
    pub target: ::std::option::Option<::std::boxed::Box<Target>>,
    #[serde(rename = "timeRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The action occurred over this time range."]
    pub time_range: ::std::option::Option<::std::boxed::Box<TimeRange>>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The action occurred at this specific time."]
    pub timestamp: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Data describing the type and additional information of an action."]
pub struct ActionDetail {
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A change about comments was made."]
    pub comment: ::std::option::Option<::std::boxed::Box<Comment>>,
    #[serde(rename = "create")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An object was created."]
    pub create: ::std::option::Option<::std::boxed::Box<Create>>,
    #[serde(rename = "delete")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An object was deleted."]
    pub delete: ::std::option::Option<::std::boxed::Box<Delete>>,
    #[serde(rename = "dlpChange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A change happened in data leak prevention status."]
    pub dlp_change: ::std::option::Option<::std::boxed::Box<DataLeakPreventionChange>>,
    #[serde(rename = "edit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An object was edited."]
    pub edit: ::std::option::Option<::std::boxed::Box<Edit>>,
    #[serde(rename = "move")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An object was moved."]
    pub _move: ::std::option::Option<::std::boxed::Box<Move>>,
    #[serde(rename = "permissionChange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The permission on an object was changed."]
    pub permission_change: ::std::option::Option<::std::boxed::Box<PermissionChange>>,
    #[serde(rename = "reference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An object was referenced in an application outside of Drive/Docs."]
    pub reference: ::std::option::Option<::std::boxed::Box<ApplicationReference>>,
    #[serde(rename = "rename")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An object was renamed."]
    pub rename: ::std::option::Option<::std::boxed::Box<Rename>>,
    #[serde(rename = "restore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A deleted object was restored."]
    pub restore: ::std::option::Option<::std::boxed::Box<Restore>>,
    #[serde(rename = "settingsChange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings were changed."]
    pub settings_change: ::std::option::Option<::std::boxed::Box<SettingsChange>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The actor of a Drive activity."]
pub struct Actor {
    #[serde(rename = "administrator")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An administrator."]
    pub administrator: ::std::option::Option<::std::boxed::Box<Administrator>>,
    #[serde(rename = "anonymous")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An anonymous user."]
    pub anonymous: ::std::option::Option<::std::boxed::Box<AnonymousUser>>,
    #[serde(rename = "impersonation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An account acting on behalf of another."]
    pub impersonation: ::std::option::Option<::std::boxed::Box<Impersonation>>,
    #[serde(rename = "system")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A non-user actor (i.e. system triggered)."]
    pub system: ::std::option::Option<::std::boxed::Box<SystemEvent>>,
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An end user."]
    pub user: ::std::option::Option<::std::boxed::Box<User>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Empty message representing an administrator."]
pub struct Administrator {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Empty message representing an anonymous user or indicating the authenticated user should be anonymized."]
pub struct AnonymousUser {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents any user (including a logged out user)."]
pub struct Anyone {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Activity in applications other than Drive."]
pub struct ApplicationReference {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reference type corresponding to this event."]
    pub _type: ::std::option::Option<ApplicationReferenceTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The reference type corresponding to this event."]
pub enum ApplicationReferenceTypeEnum {
    #[serde(rename = "UNSPECIFIED_REFERENCE_TYPE")]
    #[doc = "The type is not available."]
    UnspecifiedReferenceType,
    #[serde(rename = "LINK")]
    #[doc = "The links of one or more Drive items were posted."]
    Link,
    #[serde(rename = "DISCUSS")]
    #[doc = "Comments were made regarding a Drive item."]
    Discuss,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A comment with an assignment."]
pub struct Assignment {
    #[serde(rename = "assignedUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user to whom the comment was assigned."]
    pub assigned_user: ::std::option::Option<::std::boxed::Box<User>>,
    #[serde(rename = "subtype")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sub-type of this event."]
    pub subtype: ::std::option::Option<AssignmentSubtypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The sub-type of this event."]
pub enum AssignmentSubtypeEnum {
    #[serde(rename = "SUBTYPE_UNSPECIFIED")]
    #[doc = "Subtype not available."]
    SubtypeUnspecified,
    #[serde(rename = "ADDED")]
    #[doc = "An assignment was added."]
    Added,
    #[serde(rename = "DELETED")]
    #[doc = "An assignment was deleted."]
    Deleted,
    #[serde(rename = "REPLY_ADDED")]
    #[doc = "An assignment reply was added."]
    ReplyAdded,
    #[serde(rename = "REPLY_DELETED")]
    #[doc = "An assignment reply was deleted."]
    ReplyDeleted,
    #[serde(rename = "RESOLVED")]
    #[doc = "An assignment was resolved."]
    Resolved,
    #[serde(rename = "REOPENED")]
    #[doc = "A resolved assignment was reopened."]
    Reopened,
    #[serde(rename = "REASSIGNED")]
    #[doc = "An assignment was reassigned."]
    Reassigned,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A change about comments on an object."]
pub struct Comment {
    #[serde(rename = "assignment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A change on an assignment."]
    pub assignment: ::std::option::Option<::std::boxed::Box<Assignment>>,
    #[serde(rename = "mentionedUsers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Users who are mentioned in this comment."]
    pub mentioned_users: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<User>>>,
    #[serde(rename = "post")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A change on a regular posted comment."]
    pub post: ::std::option::Option<::std::boxed::Box<Post>>,
    #[serde(rename = "suggestion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A change on a suggestion."]
    pub suggestion: ::std::option::Option<::std::boxed::Box<Suggestion>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "How the individual activities are consolidated. A set of activities may be consolidated into one combined activity if they are related in some way, such as one actor performing the same action on multiple targets, or multiple actors performing the same action on a single target. The strategy defines the rules for which activities are related."]
pub struct ConsolidationStrategy {
    #[serde(rename = "legacy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The individual activities are consolidated using the legacy strategy."]
    pub legacy: ::std::option::Option<::std::boxed::Box<Legacy>>,
    #[serde(rename = "none")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The individual activities are not consolidated."]
    pub none: ::std::option::Option<::std::boxed::Box<NoConsolidation>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An object was created by copying an existing object."]
pub struct Copy {
    #[serde(rename = "originalObject")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The the original object."]
    pub original_object: ::std::option::Option<::std::boxed::Box<TargetReference>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An object was created."]
pub struct Create {
    #[serde(rename = "copy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If present, indicates the object was created by copying an existing Drive object."]
    pub copy: ::std::option::Option<::std::boxed::Box<Copy>>,
    #[serde(rename = "new")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If present, indicates the object was newly created (e.g. as a blank document), not derived from a Drive object or external object."]
    pub new: ::std::option::Option<::std::boxed::Box<New>>,
    #[serde(rename = "upload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If present, indicates the object originated externally and was uploaded to Drive."]
    pub upload: ::std::option::Option<::std::boxed::Box<Upload>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A change in the object's data leak prevention status."]
pub struct DataLeakPreventionChange {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of Data Leak Prevention (DLP) change."]
    pub _type: ::std::option::Option<DataLeakPreventionChangeTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of Data Leak Prevention (DLP) change."]
pub enum DataLeakPreventionChangeTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "An update to the DLP state that is neither FLAGGED or CLEARED."]
    TypeUnspecified,
    #[serde(rename = "FLAGGED")]
    #[doc = "Document has been flagged as containing sensitive content."]
    Flagged,
    #[serde(rename = "CLEARED")]
    #[doc = "Document is no longer flagged as containing sensitive content."]
    Cleared,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An object was deleted."]
pub struct Delete {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of delete action taken."]
    pub _type: ::std::option::Option<DeleteTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of delete action taken."]
pub enum DeleteTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Deletion type is not available."]
    TypeUnspecified,
    #[serde(rename = "TRASH")]
    #[doc = "An object was put into the trash."]
    Trash,
    #[serde(rename = "PERMANENT_DELETE")]
    #[doc = "An object was deleted permanently."]
    PermanentDelete,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A user whose account has since been deleted."]
pub struct DeletedUser {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about a domain."]
pub struct Domain {
    #[serde(rename = "legacyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An opaque string used to identify this domain."]
    pub legacy_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the domain, e.g. \"google.com\"."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about a shared drive."]
pub struct Drive {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the shared drive. The format is \"COLLECTION_ID/DRIVE_ID\". Clients should not assume a specific collection ID for this resource name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "root")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The root of this shared drive."]
    pub root: ::std::option::Option<::std::boxed::Box<DriveItem>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the shared drive."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single Drive activity comprising one or more Actions by one or more Actors on one or more Targets. Some Action groupings occur spontaneously, such as moving an item into a shared folder triggering a permission change. Other groupings of related Actions, such as multiple Actors editing one item or moving multiple files into a new folder, are controlled by the selection of a ConsolidationStrategy in the QueryDriveActivityRequest."]
pub struct DriveActivity {
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details on all actions in this activity."]
    pub actions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Action>>>,
    #[serde(rename = "actors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All actor(s) responsible for the activity."]
    pub actors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Actor>>>,
    #[serde(rename = "primaryActionDetail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key information about the primary action for this activity. This is either representative, or the most important, of all actions in the activity, according to the ConsolidationStrategy in the request."]
    pub primary_action_detail: ::std::option::Option<::std::boxed::Box<ActionDetail>>,
    #[serde(rename = "targets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All Google Drive objects this activity is about (e.g. file, folder, drive). This represents the state of the target immediately after the actions occurred."]
    pub targets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Target>>>,
    #[serde(rename = "timeRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The activity occurred over this time range."]
    pub time_range: ::std::option::Option<::std::boxed::Box<TimeRange>>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The activity occurred at this specific time."]
    pub timestamp: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Drive item which is a file."]
pub struct DriveFile {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Drive item which is a folder."]
pub struct DriveFolder {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of Drive folder."]
    pub _type: ::std::option::Option<DriveFolderTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of Drive folder."]
pub enum DriveFolderTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "The folder type is unknown."]
    TypeUnspecified,
    #[serde(rename = "MY_DRIVE_ROOT")]
    #[doc = "The folder is the root of a user's MyDrive."]
    MyDriveRoot,
    #[serde(rename = "SHARED_DRIVE_ROOT")]
    #[doc = "The folder is the root of a shared drive."]
    SharedDriveRoot,
    #[serde(rename = "STANDARD_FOLDER")]
    #[doc = "The folder is a standard, non-root, folder."]
    StandardFolder,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Drive item, such as a file or folder."]
pub struct DriveItem {
    #[serde(rename = "driveFile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Drive item is a file."]
    pub drive_file: ::std::option::Option<::std::boxed::Box<DriveFile>>,
    #[serde(rename = "driveFolder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Drive item is a folder. Includes information about the type of folder."]
    pub drive_folder: ::std::option::Option<::std::boxed::Box<DriveFolder>>,
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated; please use the `driveFile` field instead."]
    pub file: ::std::option::Option<::std::boxed::Box<File>>,
    #[serde(rename = "folder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated; please use the `driveFolder` field instead."]
    pub folder: ::std::option::Option<::std::boxed::Box<Folder>>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MIME type of the Drive item. See https://developers.google.com/drive/v3/web/mime-types."]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target Drive item. The format is \"items/ITEM_ID\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the owner of this Drive item."]
    pub owner: ::std::option::Option<::std::boxed::Box<Owner>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the Drive item."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A lightweight reference to a Drive item, such as a file or folder."]
pub struct DriveItemReference {
    #[serde(rename = "driveFile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Drive item is a file."]
    pub drive_file: ::std::option::Option<::std::boxed::Box<DriveFile>>,
    #[serde(rename = "driveFolder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Drive item is a folder. Includes information about the type of folder."]
    pub drive_folder: ::std::option::Option<::std::boxed::Box<DriveFolder>>,
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated; please use the `driveFile` field instead."]
    pub file: ::std::option::Option<::std::boxed::Box<File>>,
    #[serde(rename = "folder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated; please use the `driveFolder` field instead."]
    pub folder: ::std::option::Option<::std::boxed::Box<Folder>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target Drive item. The format is \"items/ITEM_ID\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the Drive item."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A lightweight reference to a shared drive."]
pub struct DriveReference {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the shared drive. The format is \"COLLECTION_ID/DRIVE_ID\". Clients should not assume a specific collection ID for this resource name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the shared drive."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An empty message indicating an object was edited."]
pub struct Edit {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This item is deprecated; please see `DriveFile` instead."]
pub struct File {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A comment on a file."]
pub struct FileComment {
    #[serde(rename = "legacyCommentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The comment in the discussion thread. This identifier is an opaque string compatible with the Drive API; see https://developers.google.com/drive/v3/reference/comments/get"]
    pub legacy_comment_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "legacyDiscussionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The discussion thread to which the comment was added. This identifier is an opaque string compatible with the Drive API and references the first comment in a discussion; see https://developers.google.com/drive/v3/reference/comments/get"]
    pub legacy_discussion_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "linkToDiscussion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The link to the discussion thread containing this comment, for example, \"https://docs.google.com/DOCUMENT_ID/edit?disco=THREAD_ID\"."]
    pub link_to_discussion: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Drive item containing this comment."]
    pub parent: ::std::option::Option<::std::boxed::Box<DriveItem>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This item is deprecated; please see `DriveFolder` instead."]
pub struct Folder {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated; please see `DriveFolder.type` instead."]
    pub _type: ::std::option::Option<FolderTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "This field is deprecated; please see `DriveFolder.type` instead."]
pub enum FolderTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "This item is deprecated; please see `DriveFolder.Type` instead."]
    TypeUnspecified,
    #[serde(rename = "MY_DRIVE_ROOT")]
    #[doc = "This item is deprecated; please see `DriveFolder.Type` instead."]
    MyDriveRoot,
    #[serde(rename = "TEAM_DRIVE_ROOT")]
    #[doc = "This item is deprecated; please see `DriveFolder.Type` instead."]
    TeamDriveRoot,
    #[serde(rename = "STANDARD_FOLDER")]
    #[doc = "This item is deprecated; please see `DriveFolder.Type` instead."]
    StandardFolder,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about a group."]
pub struct Group {
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of the group."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the group."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about an impersonation, where an admin acts on behalf of an end user. Information about the acting admin is not currently available."]
pub struct Impersonation {
    #[serde(rename = "impersonatedUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The impersonated user."]
    pub impersonated_user: ::std::option::Option<::std::boxed::Box<User>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A known user."]
pub struct KnownUser {
    #[serde(rename = "isCurrentUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if this is the user making the request."]
    pub is_current_user: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "personName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier for this user that can be used with the People API to get more information. The format is \"people/ACCOUNT_ID\". See https://developers.google.com/people/."]
    pub person_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A strategy which consolidates activities using the grouping rules from the legacy V1 Activity API. Similar actions occurring within a window of time can be grouped across multiple targets (such as moving a set of files at once) or multiple actors (such as several users editing the same item). Grouping rules for this strategy are specific to each type of action."]
pub struct Legacy {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An object was moved."]
pub struct Move {
    #[serde(rename = "addedParents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The added parent object(s)."]
    pub added_parents: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TargetReference>>>,
    #[serde(rename = "removedParents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The removed parent object(s)."]
    pub removed_parents: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TargetReference>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An object was created from scratch."]
pub struct New {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A strategy which does no consolidation of individual activities."]
pub struct NoConsolidation {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the owner of a Drive item."]
pub struct Owner {
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The domain of the Drive item owner."]
    pub domain: ::std::option::Option<::std::boxed::Box<Domain>>,
    #[serde(rename = "drive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The drive that owns the item."]
    pub drive: ::std::option::Option<::std::boxed::Box<DriveReference>>,
    #[serde(rename = "teamDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated; please use the `drive` field instead."]
    pub team_drive: ::std::option::Option<::std::boxed::Box<TeamDriveReference>>,
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user that owns the Drive item."]
    pub user: ::std::option::Option<::std::boxed::Box<User>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The permission setting of an object."]
pub struct Permission {
    #[serde(rename = "allowDiscovery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, the item can be discovered (e.g. in the user's \"Shared with me\" collection) without needing a link to the item."]
    pub allow_discovery: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "anyone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, this permission applies to anyone, even logged out users."]
    pub anyone: ::std::option::Option<::std::boxed::Box<Anyone>>,
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The domain to whom this permission applies."]
    pub domain: ::std::option::Option<::std::boxed::Box<Domain>>,
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The group to whom this permission applies."]
    pub group: ::std::option::Option<::std::boxed::Box<Group>>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the Google Drive permissions role. The role determines a user's ability to read, write, and comment on items."]
    pub role: ::std::option::Option<PermissionRoleEnum>,
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user to whom this permission applies."]
    pub user: ::std::option::Option<::std::boxed::Box<User>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Indicates the Google Drive permissions role. The role determines a user's ability to read, write, and comment on items."]
pub enum PermissionRoleEnum {
    #[serde(rename = "ROLE_UNSPECIFIED")]
    #[doc = "The role is not available."]
    RoleUnspecified,
    #[serde(rename = "OWNER")]
    #[doc = "A role granting full access."]
    Owner,
    #[serde(rename = "ORGANIZER")]
    #[doc = "A role granting the ability to manage people and settings."]
    Organizer,
    #[serde(rename = "FILE_ORGANIZER")]
    #[doc = "A role granting the ability to contribute and manage content."]
    FileOrganizer,
    #[serde(rename = "EDITOR")]
    #[doc = "A role granting the ability to contribute content. This role is sometimes also known as \"writer\"."]
    Editor,
    #[serde(rename = "COMMENTER")]
    #[doc = "A role granting the ability to view and comment on content."]
    Commenter,
    #[serde(rename = "VIEWER")]
    #[doc = "A role granting the ability to view content. This role is sometimes also known as \"reader\"."]
    Viewer,
    #[serde(rename = "PUBLISHED_VIEWER")]
    #[doc = "A role granting the ability to view content only after it has been published to the web. This role is sometimes also known as \"published reader\". See https://support.google.com/sites/answer/6372880 for more information."]
    PublishedViewer,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A change of the permission setting on an item."]
pub struct PermissionChange {
    #[serde(rename = "addedPermissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of permissions added by this change."]
    pub added_permissions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Permission>>>,
    #[serde(rename = "removedPermissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of permissions removed by this change."]
    pub removed_permissions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Permission>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A regular posted comment."]
pub struct Post {
    #[serde(rename = "subtype")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sub-type of this event."]
    pub subtype: ::std::option::Option<PostSubtypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The sub-type of this event."]
pub enum PostSubtypeEnum {
    #[serde(rename = "SUBTYPE_UNSPECIFIED")]
    #[doc = "Subtype not available."]
    SubtypeUnspecified,
    #[serde(rename = "ADDED")]
    #[doc = "A post was added."]
    Added,
    #[serde(rename = "DELETED")]
    #[doc = "A post was deleted."]
    Deleted,
    #[serde(rename = "REPLY_ADDED")]
    #[doc = "A reply was added."]
    ReplyAdded,
    #[serde(rename = "REPLY_DELETED")]
    #[doc = "A reply was deleted."]
    ReplyDeleted,
    #[serde(rename = "RESOLVED")]
    #[doc = "A posted comment was resolved."]
    Resolved,
    #[serde(rename = "REOPENED")]
    #[doc = "A posted comment was reopened."]
    Reopened,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for querying Drive activity."]
pub struct QueryDriveActivityRequest {
    #[serde(rename = "ancestorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Return activities for this Drive folder and all children and descendants. The format is \"items/ITEM_ID\"."]
    pub ancestor_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "consolidationStrategy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details on how to consolidate related actions that make up the activity. If not set, then related actions are not consolidated."]
    pub consolidation_strategy: ::std::option::Option<::std::boxed::Box<ConsolidationStrategy>>,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The filtering for items returned from this query request. The format of the filter string is a sequence of expressions, joined by an optional \"AND\", where each expression is of the form \"field operator value\". Supported fields: - time: Uses numerical operators on date values either in terms of milliseconds since Jan 1, 1970 or in RFC 3339 format. Examples: - time > 1452409200000 AND time <= 1492812924310 - time >= \"2016-01-10T01:02:03-05:00\" - detail.action_detail_case: Uses the \"has\" operator (:) and either a singular value or a list of allowed action types enclosed in parentheses. Examples: - detail.action_detail_case: RENAME - detail.action_detail_case:(CREATE EDIT) - -detail.action_detail_case:MOVE"]
    pub filter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "itemName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Return activities for this Drive item. The format is \"items/ITEM_ID\"."]
    pub item_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The miminum number of activities desired in the response; the server will attempt to return at least this quanitity. The server may also return fewer activities if it has a partial response ready before the request times out. If not set, a default value is used."]
    pub page_size: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token identifying which page of results to return. Set this to the next_page_token value returned from a previous query to obtain the following page of results. If not set, the first page of results will be returned."]
    pub page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for querying Drive activity."]
pub struct QueryDriveActivityResponse {
    #[serde(rename = "activities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of activity requested."]
    pub activities: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DriveActivity>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An object was renamed."]
pub struct Rename {
    #[serde(rename = "newTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The new title of the drive object."]
    pub new_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "oldTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The previous title of the drive object."]
    pub old_title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A deleted object was restored."]
pub struct Restore {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of restore action taken."]
    pub _type: ::std::option::Option<RestoreTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of restore action taken."]
pub enum RestoreTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "The type is not available."]
    TypeUnspecified,
    #[serde(rename = "UNTRASH")]
    #[doc = "An object was restored from the trash."]
    Untrash,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about restriction policy changes to a feature."]
pub struct RestrictionChange {
    #[serde(rename = "feature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The feature which had a change in restriction policy."]
    pub feature: ::std::option::Option<RestrictionChangeFeatureEnum>,
    #[serde(rename = "newRestriction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The restriction in place after the change."]
    pub new_restriction: ::std::option::Option<RestrictionChangeNewRestrictionEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The feature which had a change in restriction policy."]
pub enum RestrictionChangeFeatureEnum {
    #[serde(rename = "FEATURE_UNSPECIFIED")]
    #[doc = "The feature which changed restriction settings was not available."]
    FeatureUnspecified,
    #[serde(rename = "SHARING_OUTSIDE_DOMAIN")]
    #[doc = "When restricted, this prevents items from being shared outside the domain."]
    SharingOutsideDomain,
    #[serde(rename = "DIRECT_SHARING")]
    #[doc = "When restricted, this prevents direct sharing of individual items."]
    DirectSharing,
    #[serde(rename = "ITEM_DUPLICATION")]
    #[doc = "When restricted, this prevents actions like copy, download, and print that might result in uncontrolled duplicates of items."]
    ItemDuplication,
    #[serde(rename = "DRIVE_FILE_STREAM")]
    #[doc = "When restricted, this prevents use of Drive File Stream."]
    DriveFileStream,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The restriction in place after the change."]
pub enum RestrictionChangeNewRestrictionEnum {
    #[serde(rename = "RESTRICTION_UNSPECIFIED")]
    #[doc = "The type of restriction is not available."]
    RestrictionUnspecified,
    #[serde(rename = "UNRESTRICTED")]
    #[doc = "The feature is available without restriction."]
    Unrestricted,
    #[serde(rename = "FULLY_RESTRICTED")]
    #[doc = "The use of this feature is fully restricted."]
    FullyRestricted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about settings changes."]
pub struct SettingsChange {
    #[serde(rename = "restrictionChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of changes made to restrictions."]
    pub restriction_changes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RestrictionChange>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A suggestion."]
pub struct Suggestion {
    #[serde(rename = "subtype")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sub-type of this event."]
    pub subtype: ::std::option::Option<SuggestionSubtypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The sub-type of this event."]
pub enum SuggestionSubtypeEnum {
    #[serde(rename = "SUBTYPE_UNSPECIFIED")]
    #[doc = "Subtype not available."]
    SubtypeUnspecified,
    #[serde(rename = "ADDED")]
    #[doc = "A suggestion was added."]
    Added,
    #[serde(rename = "DELETED")]
    #[doc = "A suggestion was deleted."]
    Deleted,
    #[serde(rename = "REPLY_ADDED")]
    #[doc = "A suggestion reply was added."]
    ReplyAdded,
    #[serde(rename = "REPLY_DELETED")]
    #[doc = "A suggestion reply was deleted."]
    ReplyDeleted,
    #[serde(rename = "ACCEPTED")]
    #[doc = "A suggestion was accepted."]
    Accepted,
    #[serde(rename = "REJECTED")]
    #[doc = "A suggestion was rejected."]
    Rejected,
    #[serde(rename = "ACCEPT_DELETED")]
    #[doc = "An accepted suggestion was deleted."]
    AcceptDeleted,
    #[serde(rename = "REJECT_DELETED")]
    #[doc = "A rejected suggestion was deleted."]
    RejectDeleted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Event triggered by system operations instead of end users."]
pub struct SystemEvent {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the system event that may triggered activity."]
    pub _type: ::std::option::Option<SystemEventTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the system event that may triggered activity."]
pub enum SystemEventTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "The event type is unspecified."]
    TypeUnspecified,
    #[serde(rename = "USER_DELETION")]
    #[doc = "The event is a consequence of a user account being deleted."]
    UserDeletion,
    #[serde(rename = "TRASH_AUTO_PURGE")]
    #[doc = "The event is due to the system automatically purging trash."]
    TrashAutoPurge,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the target of activity."]
pub struct Target {
    #[serde(rename = "drive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target is a shared drive."]
    pub drive: ::std::option::Option<::std::boxed::Box<Drive>>,
    #[serde(rename = "driveItem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target is a Drive item."]
    pub drive_item: ::std::option::Option<::std::boxed::Box<DriveItem>>,
    #[serde(rename = "fileComment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target is a comment on a Drive file."]
    pub file_comment: ::std::option::Option<::std::boxed::Box<FileComment>>,
    #[serde(rename = "teamDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated; please use the `drive` field instead."]
    pub team_drive: ::std::option::Option<::std::boxed::Box<TeamDrive>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A lightweight reference to the target of activity."]
pub struct TargetReference {
    #[serde(rename = "drive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target is a shared drive."]
    pub drive: ::std::option::Option<::std::boxed::Box<DriveReference>>,
    #[serde(rename = "driveItem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target is a Drive item."]
    pub drive_item: ::std::option::Option<::std::boxed::Box<DriveItemReference>>,
    #[serde(rename = "teamDrive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated; please use the `drive` field instead."]
    pub team_drive: ::std::option::Option<::std::boxed::Box<TeamDriveReference>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This item is deprecated; please see `Drive` instead."]
pub struct TeamDrive {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated; please see `Drive.name` instead."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "root")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated; please see `Drive.root` instead."]
    pub root: ::std::option::Option<::std::boxed::Box<DriveItem>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated; please see `Drive.title` instead."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This item is deprecated; please see `DriveReference` instead."]
pub struct TeamDriveReference {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated; please see `DriveReference.name` instead."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated; please see `DriveReference.title` instead."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about time ranges."]
pub struct TimeRange {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end of the time range."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The start of the time range."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A user about whom nothing is currently known."]
pub struct UnknownUser {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An object was uploaded into Drive."]
pub struct Upload {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about an end user."]
pub struct User {
    #[serde(rename = "deletedUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A user whose account has since been deleted."]
    pub deleted_user: ::std::option::Option<::std::boxed::Box<DeletedUser>>,
    #[serde(rename = "knownUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A known user."]
    pub known_user: ::std::option::Option<::std::boxed::Box<KnownUser>>,
    #[serde(rename = "unknownUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A user about whom nothing is currently known."]
    pub unknown_user: ::std::option::Option<::std::boxed::Box<UnknownUser>>,
}
