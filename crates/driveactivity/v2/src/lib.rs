#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
pub struct QueryParameters {
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "$.xgafv")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "V1 error format."]
    pub xgafv: ::std::option::Option<QueryParametersXgafvEnum>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "access_token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OAuth access token."]
    pub access_token: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ query_parameters_defaults :: alt () }", setter(into))]
    #[serde(rename = "alt")]
    #[serde(default = "query_parameters_defaults :: alt")]
    #[doc = "Data format for response."]
    pub alt: QueryParametersAltEnum,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "callback")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "JSONP"]
    pub callback: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selector specifying which fields to include in a partial response."]
    pub fields: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
    pub key: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "oauth_token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OAuth 2.0 token for the current user."]
    pub oauth_token: ::std::option::Option<::std::string::String>,
    #[builder(
        default = "{ query_parameters_defaults :: pretty_print () }",
        setter(into)
    )]
    #[serde(rename = "prettyPrint")]
    #[serde(default = "query_parameters_defaults :: pretty_print")]
    #[doc = "Returns response with indentations and line breaks."]
    pub pretty_print: ::std::primitive::bool,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "quotaUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
    pub quota_user: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "uploadType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
    pub upload_type: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "upload_protocol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
    pub upload_protocol: ::std::option::Option<::std::string::String>,
}
impl QueryParameters {
    pub fn builder() -> QueryParametersBuilder {
        QueryParametersBuilder::default()
    }
}
mod query_parameters_defaults {
    pub fn alt() -> super::QueryParametersAltEnum {
        serde_json::from_str(&"json").unwrap()
    }
    pub fn pretty_print() -> ::std::primitive::bool {
        serde_json::from_str(&"true").unwrap()
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "V1 error format."]
pub enum QueryParametersXgafvEnum {
    #[serde(rename = "1")]
    #[doc = "v1 error format"]
    _1,
    #[serde(rename = "2")]
    #[doc = "v2 error format"]
    _2,
}
impl ::std::default::Default for QueryParametersXgafvEnum {
    fn default() -> Self {
        Self::_1
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Data format for response."]
pub enum QueryParametersAltEnum {
    #[serde(rename = "json")]
    #[doc = "Responses with Content-Type of application/json"]
    Json,
    #[serde(rename = "media")]
    #[doc = "Media download with context-dependent Content-Type"]
    Media,
    #[serde(rename = "proto")]
    #[doc = "Responses with Content-Type of application/x-protobuf"]
    Proto,
}
impl ::std::default::Default for QueryParametersAltEnum {
    fn default() -> Self {
        Self::Json
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the action."]
    pub struct Action {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actor responsible for this action (or empty if all actors are responsible)."]
        pub actor: ::std::option::Option<::std::boxed::Box<Actor>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type and detailed information about the action."]
        pub detail: ::std::option::Option<::std::boxed::Box<ActionDetail>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target this action affects (or empty if affecting all targets). This represents the state of the target immediately after this action occurred."]
        pub target: ::std::option::Option<::std::boxed::Box<Target>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The action occurred over this time range."]
        pub time_range: ::std::option::Option<::std::boxed::Box<TimeRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The action occurred at this specific time."]
        pub timestamp: ::std::option::Option<::std::string::String>,
    }
    impl Action {
        pub fn builder() -> ActionBuilder {
            ActionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data describing the type and additional information of an action."]
    pub struct ActionDetail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A change about comments was made."]
        pub comment: ::std::option::Option<::std::boxed::Box<Comment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "create")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An object was created."]
        pub create: ::std::option::Option<::std::boxed::Box<Create>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delete")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An object was deleted."]
        pub delete: ::std::option::Option<::std::boxed::Box<Delete>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dlpChange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A change happened in data leak prevention status."]
        pub dlp_change: ::std::option::Option<::std::boxed::Box<DataLeakPreventionChange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "edit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An object was edited."]
        pub edit: ::std::option::Option<::std::boxed::Box<Edit>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "move")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An object was moved."]
        pub _move: ::std::option::Option<::std::boxed::Box<Move>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissionChange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The permission on an object was changed."]
        pub permission_change: ::std::option::Option<::std::boxed::Box<PermissionChange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An object was referenced in an application outside of Drive/Docs."]
        pub reference: ::std::option::Option<::std::boxed::Box<ApplicationReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rename")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An object was renamed."]
        pub rename: ::std::option::Option<::std::boxed::Box<Rename>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A deleted object was restored."]
        pub restore: ::std::option::Option<::std::boxed::Box<Restore>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "settingsChange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Settings were changed."]
        pub settings_change: ::std::option::Option<::std::boxed::Box<SettingsChange>>,
    }
    impl ActionDetail {
        pub fn builder() -> ActionDetailBuilder {
            ActionDetailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The actor of a Drive activity."]
    pub struct Actor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "administrator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An administrator."]
        pub administrator: ::std::option::Option<::std::boxed::Box<Administrator>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "anonymous")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An anonymous user."]
        pub anonymous: ::std::option::Option<::std::boxed::Box<AnonymousUser>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "impersonation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An account acting on behalf of another."]
        pub impersonation: ::std::option::Option<::std::boxed::Box<Impersonation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "system")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A non-user actor (i.e. system triggered)."]
        pub system: ::std::option::Option<::std::boxed::Box<SystemEvent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An end user."]
        pub user: ::std::option::Option<::std::boxed::Box<User>>,
    }
    impl Actor {
        pub fn builder() -> ActorBuilder {
            ActorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Empty message representing an administrator."]
    pub struct Administrator {}
    impl Administrator {
        pub fn builder() -> AdministratorBuilder {
            AdministratorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Empty message representing an anonymous user or indicating the authenticated user should be anonymized."]
    pub struct AnonymousUser {}
    impl AnonymousUser {
        pub fn builder() -> AnonymousUserBuilder {
            AnonymousUserBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents any user (including a logged out user)."]
    pub struct Anyone {}
    impl Anyone {
        pub fn builder() -> AnyoneBuilder {
            AnyoneBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Activity in applications other than Drive."]
    pub struct ApplicationReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reference type corresponding to this event."]
        pub _type: ::std::option::Option<ApplicationReferenceTypeEnum>,
    }
    impl ApplicationReference {
        pub fn builder() -> ApplicationReferenceBuilder {
            ApplicationReferenceBuilder::default()
        }
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
    impl ::std::default::Default for ApplicationReferenceTypeEnum {
        fn default() -> Self {
            Self::UnspecifiedReferenceType
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A comment with an assignment."]
    pub struct Assignment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assignedUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user to whom the comment was assigned."]
        pub assigned_user: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtype")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sub-type of this event."]
        pub subtype: ::std::option::Option<AssignmentSubtypeEnum>,
    }
    impl Assignment {
        pub fn builder() -> AssignmentBuilder {
            AssignmentBuilder::default()
        }
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
    impl ::std::default::Default for AssignmentSubtypeEnum {
        fn default() -> Self {
            Self::SubtypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A change about comments on an object."]
    pub struct Comment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assignment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A change on an assignment."]
        pub assignment: ::std::option::Option<::std::boxed::Box<Assignment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mentionedUsers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Users who are mentioned in this comment."]
        pub mentioned_users: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<User>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "post")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A change on a regular posted comment."]
        pub post: ::std::option::Option<::std::boxed::Box<Post>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suggestion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A change on a suggestion."]
        pub suggestion: ::std::option::Option<::std::boxed::Box<Suggestion>>,
    }
    impl Comment {
        pub fn builder() -> CommentBuilder {
            CommentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "How the individual activities are consolidated. A set of activities may be consolidated into one combined activity if they are related in some way, such as one actor performing the same action on multiple targets, or multiple actors performing the same action on a single target. The strategy defines the rules for which activities are related."]
    pub struct ConsolidationStrategy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "legacy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The individual activities are consolidated using the legacy strategy."]
        pub legacy: ::std::option::Option<::std::boxed::Box<Legacy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "none")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The individual activities are not consolidated."]
        pub none: ::std::option::Option<::std::boxed::Box<NoConsolidation>>,
    }
    impl ConsolidationStrategy {
        pub fn builder() -> ConsolidationStrategyBuilder {
            ConsolidationStrategyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object was created by copying an existing object."]
    pub struct Copy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalObject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The the original object."]
        pub original_object: ::std::option::Option<::std::boxed::Box<TargetReference>>,
    }
    impl Copy {
        pub fn builder() -> CopyBuilder {
            CopyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object was created."]
    pub struct Create {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "copy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, indicates the object was created by copying an existing Drive object."]
        pub copy: ::std::option::Option<::std::boxed::Box<Copy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "new")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, indicates the object was newly created (e.g. as a blank document), not derived from a Drive object or external object."]
        pub new: ::std::option::Option<::std::boxed::Box<New>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, indicates the object originated externally and was uploaded to Drive."]
        pub upload: ::std::option::Option<::std::boxed::Box<Upload>>,
    }
    impl Create {
        pub fn builder() -> CreateBuilder {
            CreateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A change in the object's data leak prevention status."]
    pub struct DataLeakPreventionChange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of Data Leak Prevention (DLP) change."]
        pub _type: ::std::option::Option<DataLeakPreventionChangeTypeEnum>,
    }
    impl DataLeakPreventionChange {
        pub fn builder() -> DataLeakPreventionChangeBuilder {
            DataLeakPreventionChangeBuilder::default()
        }
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
    impl ::std::default::Default for DataLeakPreventionChangeTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object was deleted."]
    pub struct Delete {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of delete action taken."]
        pub _type: ::std::option::Option<DeleteTypeEnum>,
    }
    impl Delete {
        pub fn builder() -> DeleteBuilder {
            DeleteBuilder::default()
        }
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
    impl ::std::default::Default for DeleteTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A user whose account has since been deleted."]
    pub struct DeletedUser {}
    impl DeletedUser {
        pub fn builder() -> DeletedUserBuilder {
            DeletedUserBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a domain."]
    pub struct Domain {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "legacyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An opaque string used to identify this domain."]
        pub legacy_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the domain, e.g. \"google.com\"."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Domain {
        pub fn builder() -> DomainBuilder {
            DomainBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a shared drive."]
    pub struct Drive {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the shared drive. The format is \"COLLECTION_ID/DRIVE_ID\". Clients should not assume a specific collection ID for this resource name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "root")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The root of this shared drive."]
        pub root: ::std::option::Option<::std::boxed::Box<DriveItem>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the shared drive."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl Drive {
        pub fn builder() -> DriveBuilder {
            DriveBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single Drive activity comprising one or more Actions by one or more Actors on one or more Targets. Some Action groupings occur spontaneously, such as moving an item into a shared folder triggering a permission change. Other groupings of related Actions, such as multiple Actors editing one item or moving multiple files into a new folder, are controlled by the selection of a ConsolidationStrategy in the QueryDriveActivityRequest."]
    pub struct DriveActivity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details on all actions in this activity."]
        pub actions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Action>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All actor(s) responsible for the activity."]
        pub actors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Actor>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryActionDetail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key information about the primary action for this activity. This is either representative, or the most important, of all actions in the activity, according to the ConsolidationStrategy in the request."]
        pub primary_action_detail: ::std::option::Option<::std::boxed::Box<ActionDetail>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All Google Drive objects this activity is about (e.g. file, folder, drive). This represents the state of the target immediately after the actions occurred."]
        pub targets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Target>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The activity occurred over this time range."]
        pub time_range: ::std::option::Option<::std::boxed::Box<TimeRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The activity occurred at this specific time."]
        pub timestamp: ::std::option::Option<::std::string::String>,
    }
    impl DriveActivity {
        pub fn builder() -> DriveActivityBuilder {
            DriveActivityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Drive item which is a file."]
    pub struct DriveFile {}
    impl DriveFile {
        pub fn builder() -> DriveFileBuilder {
            DriveFileBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Drive item which is a folder."]
    pub struct DriveFolder {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of Drive folder."]
        pub _type: ::std::option::Option<DriveFolderTypeEnum>,
    }
    impl DriveFolder {
        pub fn builder() -> DriveFolderBuilder {
            DriveFolderBuilder::default()
        }
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
    impl ::std::default::Default for DriveFolderTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Drive item, such as a file or folder."]
    pub struct DriveItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveFile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Drive item is a file."]
        pub drive_file: ::std::option::Option<::std::boxed::Box<DriveFile>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveFolder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Drive item is a folder. Includes information about the type of folder."]
        pub drive_folder: ::std::option::Option<::std::boxed::Box<DriveFolder>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "file")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated; please use the `driveFile` field instead."]
        pub file: ::std::option::Option<::std::boxed::Box<File>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "folder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated; please use the `driveFolder` field instead."]
        pub folder: ::std::option::Option<::std::boxed::Box<Folder>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of the Drive item. See https://developers.google.com/drive/v3/web/mime-types."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target Drive item. The format is \"items/ITEM_ID\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "owner")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the owner of this Drive item."]
        pub owner: ::std::option::Option<::std::boxed::Box<Owner>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the Drive item."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl DriveItem {
        pub fn builder() -> DriveItemBuilder {
            DriveItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A lightweight reference to a Drive item, such as a file or folder."]
    pub struct DriveItemReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveFile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Drive item is a file."]
        pub drive_file: ::std::option::Option<::std::boxed::Box<DriveFile>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveFolder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Drive item is a folder. Includes information about the type of folder."]
        pub drive_folder: ::std::option::Option<::std::boxed::Box<DriveFolder>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "file")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated; please use the `driveFile` field instead."]
        pub file: ::std::option::Option<::std::boxed::Box<File>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "folder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated; please use the `driveFolder` field instead."]
        pub folder: ::std::option::Option<::std::boxed::Box<Folder>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target Drive item. The format is \"items/ITEM_ID\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the Drive item."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl DriveItemReference {
        pub fn builder() -> DriveItemReferenceBuilder {
            DriveItemReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A lightweight reference to a shared drive."]
    pub struct DriveReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the shared drive. The format is \"COLLECTION_ID/DRIVE_ID\". Clients should not assume a specific collection ID for this resource name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the shared drive."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl DriveReference {
        pub fn builder() -> DriveReferenceBuilder {
            DriveReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An empty message indicating an object was edited."]
    pub struct Edit {}
    impl Edit {
        pub fn builder() -> EditBuilder {
            EditBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This item is deprecated; please see `DriveFile` instead."]
    pub struct File {}
    impl File {
        pub fn builder() -> FileBuilder {
            FileBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A comment on a file."]
    pub struct FileComment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "legacyCommentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The comment in the discussion thread. This identifier is an opaque string compatible with the Drive API; see https://developers.google.com/drive/v3/reference/comments/get"]
        pub legacy_comment_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "legacyDiscussionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The discussion thread to which the comment was added. This identifier is an opaque string compatible with the Drive API and references the first comment in a discussion; see https://developers.google.com/drive/v3/reference/comments/get"]
        pub legacy_discussion_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkToDiscussion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The link to the discussion thread containing this comment, for example, \"https://docs.google.com/DOCUMENT_ID/edit?disco=THREAD_ID\"."]
        pub link_to_discussion: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Drive item containing this comment."]
        pub parent: ::std::option::Option<::std::boxed::Box<DriveItem>>,
    }
    impl FileComment {
        pub fn builder() -> FileCommentBuilder {
            FileCommentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This item is deprecated; please see `DriveFolder` instead."]
    pub struct Folder {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated; please see `DriveFolder.type` instead."]
        pub _type: ::std::option::Option<FolderTypeEnum>,
    }
    impl Folder {
        pub fn builder() -> FolderBuilder {
            FolderBuilder::default()
        }
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
    impl ::std::default::Default for FolderTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a group."]
    pub struct Group {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of the group."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the group."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl Group {
        pub fn builder() -> GroupBuilder {
            GroupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about an impersonation, where an admin acts on behalf of an end user. Information about the acting admin is not currently available."]
    pub struct Impersonation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "impersonatedUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The impersonated user."]
        pub impersonated_user: ::std::option::Option<::std::boxed::Box<User>>,
    }
    impl Impersonation {
        pub fn builder() -> ImpersonationBuilder {
            ImpersonationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A known user."]
    pub struct KnownUser {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isCurrentUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if this is the user making the request."]
        pub is_current_user: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "personName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier for this user that can be used with the People API to get more information. The format is \"people/ACCOUNT_ID\". See https://developers.google.com/people/."]
        pub person_name: ::std::option::Option<::std::string::String>,
    }
    impl KnownUser {
        pub fn builder() -> KnownUserBuilder {
            KnownUserBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A strategy which consolidates activities using the grouping rules from the legacy V1 Activity API. Similar actions occurring within a window of time can be grouped across multiple targets (such as moving a set of files at once) or multiple actors (such as several users editing the same item). Grouping rules for this strategy are specific to each type of action."]
    pub struct Legacy {}
    impl Legacy {
        pub fn builder() -> LegacyBuilder {
            LegacyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object was moved."]
    pub struct Move {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addedParents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The added parent object(s)."]
        pub added_parents:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TargetReference>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "removedParents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The removed parent object(s)."]
        pub removed_parents:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TargetReference>>>,
    }
    impl Move {
        pub fn builder() -> MoveBuilder {
            MoveBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object was created from scratch."]
    pub struct New {}
    impl New {
        pub fn builder() -> NewBuilder {
            NewBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A strategy which does no consolidation of individual activities."]
    pub struct NoConsolidation {}
    impl NoConsolidation {
        pub fn builder() -> NoConsolidationBuilder {
            NoConsolidationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the owner of a Drive item."]
    pub struct Owner {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain of the Drive item owner."]
        pub domain: ::std::option::Option<::std::boxed::Box<Domain>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "drive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The drive that owns the item."]
        pub drive: ::std::option::Option<::std::boxed::Box<DriveReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teamDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated; please use the `drive` field instead."]
        pub team_drive: ::std::option::Option<::std::boxed::Box<TeamDriveReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user that owns the Drive item."]
        pub user: ::std::option::Option<::std::boxed::Box<User>>,
    }
    impl Owner {
        pub fn builder() -> OwnerBuilder {
            OwnerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The permission setting of an object."]
    pub struct Permission {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowDiscovery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, the item can be discovered (e.g. in the user's \"Shared with me\" collection) without needing a link to the item."]
        pub allow_discovery: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "anyone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, this permission applies to anyone, even logged out users."]
        pub anyone: ::std::option::Option<::std::boxed::Box<Anyone>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain to whom this permission applies."]
        pub domain: ::std::option::Option<::std::boxed::Box<Domain>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "group")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The group to whom this permission applies."]
        pub group: ::std::option::Option<::std::boxed::Box<Group>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates the Google Drive permissions role. The role determines a user's ability to read, write, and comment on items."]
        pub role: ::std::option::Option<PermissionRoleEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user to whom this permission applies."]
        pub user: ::std::option::Option<::std::boxed::Box<User>>,
    }
    impl Permission {
        pub fn builder() -> PermissionBuilder {
            PermissionBuilder::default()
        }
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
    impl ::std::default::Default for PermissionRoleEnum {
        fn default() -> Self {
            Self::RoleUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A change of the permission setting on an item."]
    pub struct PermissionChange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addedPermissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of permissions added by this change."]
        pub added_permissions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Permission>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "removedPermissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of permissions removed by this change."]
        pub removed_permissions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Permission>>>,
    }
    impl PermissionChange {
        pub fn builder() -> PermissionChangeBuilder {
            PermissionChangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A regular posted comment."]
    pub struct Post {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtype")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sub-type of this event."]
        pub subtype: ::std::option::Option<PostSubtypeEnum>,
    }
    impl Post {
        pub fn builder() -> PostBuilder {
            PostBuilder::default()
        }
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
    impl ::std::default::Default for PostSubtypeEnum {
        fn default() -> Self {
            Self::SubtypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for querying Drive activity."]
    pub struct QueryDriveActivityRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ancestorName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Return activities for this Drive folder and all children and descendants. The format is \"items/ITEM_ID\"."]
        pub ancestor_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consolidationStrategy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details on how to consolidate related actions that make up the activity. If not set, then related actions are not consolidated."]
        pub consolidation_strategy: ::std::option::Option<::std::boxed::Box<ConsolidationStrategy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filtering for items returned from this query request. The format of the filter string is a sequence of expressions, joined by an optional \"AND\", where each expression is of the form \"field operator value\". Supported fields: - time: Uses numerical operators on date values either in terms of milliseconds since Jan 1, 1970 or in RFC 3339 format. Examples: - time > 1452409200000 AND time <= 1492812924310 - time >= \"2016-01-10T01:02:03-05:00\" - detail.action_detail_case: Uses the \"has\" operator (:) and either a singular value or a list of allowed action types enclosed in parentheses. Examples: - detail.action_detail_case: RENAME - detail.action_detail_case:(CREATE EDIT) - -detail.action_detail_case:MOVE"]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Return activities for this Drive item. The format is \"items/ITEM_ID\"."]
        pub item_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The miminum number of activities desired in the response; the server will attempt to return at least this quanitity. The server may also return fewer activities if it has a partial response ready before the request times out. If not set, a default value is used."]
        pub page_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token identifying which page of results to return. Set this to the next_page_token value returned from a previous query to obtain the following page of results. If not set, the first page of results will be returned."]
        pub page_token: ::std::option::Option<::std::string::String>,
    }
    impl QueryDriveActivityRequest {
        pub fn builder() -> QueryDriveActivityRequestBuilder {
            QueryDriveActivityRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for querying Drive activity."]
    pub struct QueryDriveActivityResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of activity requested."]
        pub activities: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DriveActivity>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl QueryDriveActivityResponse {
        pub fn builder() -> QueryDriveActivityResponseBuilder {
            QueryDriveActivityResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object was renamed."]
    pub struct Rename {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new title of the drive object."]
        pub new_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oldTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The previous title of the drive object."]
        pub old_title: ::std::option::Option<::std::string::String>,
    }
    impl Rename {
        pub fn builder() -> RenameBuilder {
            RenameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A deleted object was restored."]
    pub struct Restore {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of restore action taken."]
        pub _type: ::std::option::Option<RestoreTypeEnum>,
    }
    impl Restore {
        pub fn builder() -> RestoreBuilder {
            RestoreBuilder::default()
        }
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
    impl ::std::default::Default for RestoreTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about restriction policy changes to a feature."]
    pub struct RestrictionChange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The feature which had a change in restriction policy."]
        pub feature: ::std::option::Option<RestrictionChangeFeatureEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newRestriction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The restriction in place after the change."]
        pub new_restriction: ::std::option::Option<RestrictionChangeNewRestrictionEnum>,
    }
    impl RestrictionChange {
        pub fn builder() -> RestrictionChangeBuilder {
            RestrictionChangeBuilder::default()
        }
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
    impl ::std::default::Default for RestrictionChangeFeatureEnum {
        fn default() -> Self {
            Self::FeatureUnspecified
        }
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
    impl ::std::default::Default for RestrictionChangeNewRestrictionEnum {
        fn default() -> Self {
            Self::RestrictionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about settings changes."]
    pub struct SettingsChange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restrictionChanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of changes made to restrictions."]
        pub restriction_changes:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RestrictionChange>>>,
    }
    impl SettingsChange {
        pub fn builder() -> SettingsChangeBuilder {
            SettingsChangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A suggestion."]
    pub struct Suggestion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtype")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sub-type of this event."]
        pub subtype: ::std::option::Option<SuggestionSubtypeEnum>,
    }
    impl Suggestion {
        pub fn builder() -> SuggestionBuilder {
            SuggestionBuilder::default()
        }
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
    impl ::std::default::Default for SuggestionSubtypeEnum {
        fn default() -> Self {
            Self::SubtypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Event triggered by system operations instead of end users."]
    pub struct SystemEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the system event that may triggered activity."]
        pub _type: ::std::option::Option<SystemEventTypeEnum>,
    }
    impl SystemEvent {
        pub fn builder() -> SystemEventBuilder {
            SystemEventBuilder::default()
        }
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
    impl ::std::default::Default for SystemEventTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the target of activity."]
    pub struct Target {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "drive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target is a shared drive."]
        pub drive: ::std::option::Option<::std::boxed::Box<Drive>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveItem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target is a Drive item."]
        pub drive_item: ::std::option::Option<::std::boxed::Box<DriveItem>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileComment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target is a comment on a Drive file."]
        pub file_comment: ::std::option::Option<::std::boxed::Box<FileComment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teamDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated; please use the `drive` field instead."]
        pub team_drive: ::std::option::Option<::std::boxed::Box<TeamDrive>>,
    }
    impl Target {
        pub fn builder() -> TargetBuilder {
            TargetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A lightweight reference to the target of activity."]
    pub struct TargetReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "drive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target is a shared drive."]
        pub drive: ::std::option::Option<::std::boxed::Box<DriveReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveItem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target is a Drive item."]
        pub drive_item: ::std::option::Option<::std::boxed::Box<DriveItemReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teamDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated; please use the `drive` field instead."]
        pub team_drive: ::std::option::Option<::std::boxed::Box<TeamDriveReference>>,
    }
    impl TargetReference {
        pub fn builder() -> TargetReferenceBuilder {
            TargetReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This item is deprecated; please see `Drive` instead."]
    pub struct TeamDrive {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated; please see `Drive.name` instead."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "root")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated; please see `Drive.root` instead."]
        pub root: ::std::option::Option<::std::boxed::Box<DriveItem>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated; please see `Drive.title` instead."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl TeamDrive {
        pub fn builder() -> TeamDriveBuilder {
            TeamDriveBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This item is deprecated; please see `DriveReference` instead."]
    pub struct TeamDriveReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated; please see `DriveReference.name` instead."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated; please see `DriveReference.title` instead."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl TeamDriveReference {
        pub fn builder() -> TeamDriveReferenceBuilder {
            TeamDriveReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about time ranges."]
    pub struct TimeRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end of the time range."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start of the time range."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl TimeRange {
        pub fn builder() -> TimeRangeBuilder {
            TimeRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A user about whom nothing is currently known."]
    pub struct UnknownUser {}
    impl UnknownUser {
        pub fn builder() -> UnknownUserBuilder {
            UnknownUserBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object was uploaded into Drive."]
    pub struct Upload {}
    impl Upload {
        pub fn builder() -> UploadBuilder {
            UploadBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about an end user."]
    pub struct User {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deletedUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A user whose account has since been deleted."]
        pub deleted_user: ::std::option::Option<::std::boxed::Box<DeletedUser>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "knownUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A known user."]
        pub known_user: ::std::option::Option<::std::boxed::Box<KnownUser>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unknownUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A user about whom nothing is currently known."]
        pub unknown_user: ::std::option::Option<::std::boxed::Box<UnknownUser>>,
    }
    impl User {
        pub fn builder() -> UserBuilder {
            UserBuilder::default()
        }
    }
}
