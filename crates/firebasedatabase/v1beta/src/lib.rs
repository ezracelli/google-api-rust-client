#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Representation of a Realtime Database instance. Details on interacting with contents of a DatabaseInstance can be found at: https://firebase.google.com/docs/database/rest/start."]
pub struct DatabaseInstance {
    #[serde(rename = "databaseUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The globally unique hostname of the database."]
    pub database_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fully qualified resource name of the database instance, in the form: `projects/{project-number}/locations/{location-id}/instances/{database-id}`. Currently the only supported location is 'us-central1'."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the project this instance belongs to. For example: `projects/{project-number}`."]
    pub project: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The database's lifecycle state. Read-only."]
    pub state: ::std::option::Option<DatabaseInstanceStateEnum>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The database instance type. On creation only USER_DATABASE is allowed, which is also the default when omitted."]
    pub _type: ::std::option::Option<DatabaseInstanceTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The database's lifecycle state. Read-only."]
pub enum DatabaseInstanceStateEnum {
    #[serde(rename = "LIFECYCLE_STATE_UNSPECIFIED")]
    #[doc = "Unspecified state, likely the result of an error on the backend. This is only used for distinguishing unset values."]
    LifecycleStateUnspecified,
    #[serde(rename = "ACTIVE")]
    #[doc = "The normal and active state."]
    Active,
    #[serde(rename = "DISABLED")]
    #[doc = "The database is in a disabled state. It can be re-enabled later."]
    Disabled,
    #[serde(rename = "DELETED")]
    #[doc = "The database is in a deleted state."]
    Deleted,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The database instance type. On creation only USER_DATABASE is allowed, which is also the default when omitted."]
pub enum DatabaseInstanceTypeEnum {
    #[serde(rename = "DATABASE_INSTANCE_TYPE_UNSPECIFIED")]
    #[doc = "Unknown state, likely the result of an error on the backend. This is only used for distinguishing unset values."]
    DatabaseInstanceTypeUnspecified,
    #[serde(rename = "DEFAULT_DATABASE")]
    #[doc = "The default database that is provisioned when a project is created."]
    DefaultDatabase,
    #[serde(rename = "USER_DATABASE")]
    #[doc = "A database that the user created."]
    UserDatabase,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request sent to the DisableDatabaseInstance method."]
pub struct DisableDatabaseInstanceRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response from the ListDatabaseInstances method."]
pub struct ListDatabaseInstancesResponse {
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of each DatabaseInstance that is in the parent Firebase project."]
    pub instances: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DatabaseInstance>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the result list is too large to fit in a single response, then a token is returned. If the string is empty, then this response is the last page of results. This token can be used in a subsequent call to `ListDatabaseInstances` to find the next group of database instances. Page tokens are short-lived and should not be persisted."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request sent to the ReenableDatabaseInstance method."]
pub struct ReenableDatabaseInstanceRequest {}
