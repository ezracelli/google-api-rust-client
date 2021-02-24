#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "All fields are required."]
pub struct AddFirebaseRequest {
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Instead, to set a Project's default GCP resource location, call [`FinalizeDefaultLocation`](../projects.defaultLocation/finalize) after you add Firebase resources to the GCP `Project`. The ID of the Project's default GCP resource location. The location must be one of the available [GCP resource locations](https://firebase.google.com/docs/projects/locations)."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "regionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Instead, to link a Project with a Google Analytics account, call [`AddGoogleAnalytics`](../../v1beta1/projects/addGoogleAnalytics) after you add Firebase resources to the GCP `Project`. The region code (CLDR) that the account will use for Google Analytics data For example: US, GB, or DE In Java, use `com.google.i18n.identifiers.RegionCode`."]
    pub region_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Instead, to link a Project with a Google Analytics account, call [`AddGoogleAnalytics`](../../v1beta1/projects/addGoogleAnalytics) after you add Firebase resources to the GCP `Project`. The time zone that the account will use for Google Analytics data. For example: America/Los_Angeles or Africa/Abidjan"]
    pub time_zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddGoogleAnalyticsRequest {
    #[serde(rename = "analyticsAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID for the existing [Google Analytics account](http://www.google.com/analytics/) that you want to link with the `FirebaseProject`. Specifying this field will provision a new Google Analytics property in your Google Analytics account and associate the new property with the `FirebaseProject`."]
    pub analytics_account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "analyticsPropertyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID for the existing Google Analytics property that you want to associate with the `FirebaseProject`."]
    pub analytics_property_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdminSdkConfig {
    #[serde(rename = "databaseURL")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default Firebase Realtime Database URL."]
    pub database_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the Project's default GCP resource location. The location is one of the available [GCP resource locations](https://firebase.google.com/docs/projects/locations). This field is omitted if the default GCP resource location has not been finalized yet. To set a Project's default GCP resource location, call [`FinalizeDefaultLocation`](../projects.defaultLocation/finalize) after you add Firebase resources to the Project."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. A user-assigned unique identifier for the `FirebaseProject`. This identifier may appear in URLs or names for some Firebase resources associated with the Project, but it should generally be treated as a convenience alias to reference the Project."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storageBucket")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default Cloud Storage for Firebase storage bucket name."]
    pub storage_bucket: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalyticsDetails {
    #[serde(rename = "analyticsProperty")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Analytics Property object associated with the specified `FirebaseProject`. This object contains the details of the Google Analytics property associated with the Project."]
    pub analytics_property: ::std::option::Option<::std::boxed::Box<AnalyticsProperty>>,
    #[serde(rename = "streamMappings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = " - For `AndroidApps` and `IosApps`: a map of `app` to `streamId` for each Firebase App in the specified `FirebaseProject`. Each `app` and `streamId` appears only once. - For `WebApps`: a map of `app` to `streamId` and `measurementId` for each `WebApp` in the specified `FirebaseProject`. Each `app`, `streamId`, and `measurementId` appears only once."]
    pub stream_mappings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StreamMapping>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of a Google Analytics property"]
pub struct AnalyticsProperty {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name of the Google Analytics property associated with the specified `FirebaseProject`."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The globally unique, Google-assigned identifier of the Google Analytics property associated with the specified `FirebaseProject`. If you called [`AddGoogleAnalytics`](../../v1beta1/projects/addGoogleAnalytics) to link the `FirebaseProject` with a Google Analytics account, the value in this `id` field is the same as the ID of the property either specified or provisioned with that call to `AddGoogleAnalytics`."]
    pub id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of a Firebase App for Android."]
pub struct AndroidApp {
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The globally unique, Firebase-assigned identifier for the `AndroidApp`. This identifier should be treated as an opaque token, as the data format is not specified."]
    pub app_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-assigned display name for the `AndroidApp`."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the AndroidApp, in the format: projects/ PROJECT_IDENTIFIER/androidApps/APP_ID * PROJECT_IDENTIFIER: the parent Project's [`ProjectNumber`](../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](../projects.androidApps#AndroidApp.FIELDS.app_id))."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "packageName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The canonical package name of the Android app as would appear in the Google Play Developer Console."]
    pub package_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. A user-assigned unique identifier of the parent FirebaseProject for the `AndroidApp`."]
    pub project_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration metadata of a single Firebase App for Android."]
pub struct AndroidAppConfig {
    #[serde(rename = "configFileContents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The contents of the JSON configuration file."]
    pub config_file_contents: ::std::option::Option<::std::string::String>,
    #[serde(rename = "configFilename")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The filename that the configuration artifact for the `AndroidApp` is typically saved as. For example: `google-services.json`"]
    pub config_filename: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The default resources associated with the Project."]
pub struct DefaultResources {
    #[serde(rename = "hostingSite")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default Firebase Hosting site name, in the format: PROJECT_ID Though rare, your `projectId` might already be used as the name for an existing Hosting site in another project (learn more about creating non-default, [additional sites](https://firebase.google.com/docs/hosting/multisites)). In these cases, your `projectId` is appended with a hyphen then five alphanumeric characters to create your default Hosting site name. For example, if your `projectId` is `myproject123`, your default Hosting site name might be: `myproject123-a5c16`"]
    pub hosting_site: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the Project's default GCP resource location. The location is one of the available [GCP resource locations](https://firebase.google.com/docs/projects/locations). This field is omitted if the default GCP resource location has not been finalized yet. To set a Project's default GCP resource location, call [`FinalizeDefaultLocation`](../projects.defaultLocation/finalize) after you add Firebase resources to the Project."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "realtimeDatabaseInstance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default Firebase Realtime Database instance name, in the format: PROJECT_ID Though rare, your `projectId` might already be used as the name for an existing Realtime Database instance in another project (learn more about [database sharding](https://firebase.google.com/docs/database/usage/sharding)). In these cases, your `projectId` is appended with a hyphen then five alphanumeric characters to create your default Realtime Database instance name. For example, if your `projectId` is `myproject123`, your default database instance name might be: `myproject123-a5c16`"]
    pub realtime_database_instance: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storageBucket")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default Cloud Storage for Firebase storage bucket, in the format: PROJECT_ID.appspot.com"]
    pub storage_bucket: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizeDefaultLocationRequest {
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the Project's default GCP resource location. The location must be one of the available [GCP resource locations](https://firebase.google.com/docs/projects/locations)."]
    pub location_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A high-level summary of an App."]
pub struct FirebaseAppInfo {
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Immutable. The globally unique, Firebase-assigned identifier for the `WebApp`. This identifier should be treated as an opaque token, as the data format is not specified."]
    pub app_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-assigned display name of the Firebase App."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the Firebase App, in the format: projects/PROJECT_ID /iosApps/APP_ID or projects/PROJECT_ID/androidApps/APP_ID or projects/ PROJECT_ID/webApps/APP_ID"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Immutable. The platform-specific identifier of the App. *Note:* For most use cases, use `appId`, which is the canonical, globally unique identifier for referencing an App. This string is derived from a native identifier for each platform: `packageName` for an `AndroidApp`, `bundleId` for an `IosApp`, and `webId` for a `WebApp`. Its contents should be treated as opaque, as the native identifier format may change as platforms evolve. This string is only unique within a `FirebaseProject` and its associated Apps."]
    pub namespace: ::std::option::Option<::std::string::String>,
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The platform of the Firebase App."]
    pub platform: ::std::option::Option<FirebaseAppInfoPlatformEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The platform of the Firebase App."]
pub enum FirebaseAppInfoPlatformEnum {
    #[serde(rename = "PLATFORM_UNSPECIFIED")]
    #[doc = "Unknown state. This is only used for distinguishing unset values."]
    PlatformUnspecified,
    #[serde(rename = "IOS")]
    #[doc = "The Firebase App is associated with iOS."]
    Ios,
    #[serde(rename = "ANDROID")]
    #[doc = "The Firebase App is associated with Android."]
    Android,
    #[serde(rename = "WEB")]
    #[doc = "The Firebase App is associated with web."]
    Web,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A `FirebaseProject` is the top-level Firebase entity. It is the container for Firebase Apps, Firebase Hosting sites, storage systems (Firebase Realtime Database, Cloud Firestore, Cloud Storage buckets), and other Firebase and Google Cloud Platform (GCP) resources. You create a `FirebaseProject` by calling AddFirebase and specifying an *existing* [GCP `Project`](https://cloud.google.com/resource-manager/reference/rest/v1/projects). This adds Firebase resources to the existing GCP `Project`. Since a FirebaseProject is actually also a GCP `Project`, a `FirebaseProject` has the same underlying GCP identifiers (`projectNumber` and `projectId`). This allows for easy interop with Google APIs."]
pub struct FirebaseProject {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-assigned display name of the Project."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the Project, in the format: projects/PROJECT_IDENTIFIER PROJECT_IDENTIFIER: the Project's [`ProjectNumber`](../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. A user-assigned unique identifier for the Project. This identifier may appear in URLs or names for some Firebase resources associated with the Project, but it should generally be treated as a convenience alias to reference the Project."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The globally unique, Google-assigned canonical identifier for the Project. Use this identifier when configuring integrations and/or making API calls to Firebase or third-party services."]
    pub project_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default Firebase resources associated with the Project."]
    pub resources: ::std::option::Option<::std::boxed::Box<DefaultResources>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The lifecycle state of the Project. Updates to the state must be performed via com.google.cloudresourcemanager.v1.Projects.DeleteProject and com.google.cloudresourcemanager.v1.Projects.UndeleteProject"]
    pub state: ::std::option::Option<FirebaseProjectStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The lifecycle state of the Project. Updates to the state must be performed via com.google.cloudresourcemanager.v1.Projects.DeleteProject and com.google.cloudresourcemanager.v1.Projects.UndeleteProject"]
pub enum FirebaseProjectStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Unspecified state."]
    StateUnspecified,
    #[serde(rename = "ACTIVE")]
    #[doc = "The normal and active state."]
    Active,
    #[serde(rename = "DELETED")]
    #[doc = "The Project has been marked for deletion by the user."]
    Deleted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of a Firebase App for iOS."]
pub struct IosApp {
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The globally unique, Firebase-assigned identifier for the `IosApp`. This identifier should be treated as an opaque token, as the data format is not specified."]
    pub app_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "appStoreId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The automatically generated Apple ID assigned to the iOS app by Apple in the iOS App Store."]
    pub app_store_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bundleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The canonical bundle ID of the iOS app as it would appear in the iOS AppStore."]
    pub bundle_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-assigned display name for the `IosApp`."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the IosApp, in the format: projects/PROJECT_IDENTIFIER /iosApps/APP_ID * PROJECT_IDENTIFIER: the parent Project's [`ProjectNumber`](../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](../projects.iosApps#IosApp.FIELDS.app_id))."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. A user-assigned unique identifier of the parent FirebaseProject for the `IosApp`."]
    pub project_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration metadata of a single Firebase App for iOS."]
pub struct IosAppConfig {
    #[serde(rename = "configFileContents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content of the XML configuration file."]
    pub config_file_contents: ::std::option::Option<::std::string::String>,
    #[serde(rename = "configFilename")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The filename that the configuration artifact for the `IosApp` is typically saved as. For example: `GoogleService-Info.plist`"]
    pub config_filename: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListAndroidAppsResponse {
    #[serde(rename = "apps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of each `AndroidApp` associated with the specified `FirebaseProject`."]
    pub apps: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AndroidApp>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the result list is too large to fit in a single response, then a token is returned. If the string is empty, then this response is the last page of results. This token can be used in a subsequent call to `ListAndroidApps` to find the next group of Apps. Page tokens are short-lived and should not be persisted."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListAvailableLocationsResponse {
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "One page of results from a call to `ListAvailableLocations`."]
    pub locations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Location>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the result list is too large to fit in a single response, then a token is returned. If the string is empty, then this response is the last page of results and all available locations have been listed. This token can be used in a subsequent call to `ListAvailableLocations` to find more locations. Page tokens are short-lived and should not be persisted."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListAvailableProjectsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the result list is too large to fit in a single response, then a token is returned. If the string is empty, then this response is the last page of results. This token can be used in a subsequent calls to `ListAvailableProjects` to find the next group of Projects. Page tokens are short-lived and should not be persisted."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of GCP `Projects` which can have Firebase resources added to them."]
    pub project_info: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProjectInfo>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListFirebaseProjectsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the result list is too large to fit in a single response, then a token is returned. If the string is empty, then this response is the last page of results. This token can be used in a subsequent calls to `ListFirebaseProjects` to find the next group of Projects. Page tokens are short-lived and should not be persisted."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "One page of the list of Projects that are accessible to the caller."]
    pub results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FirebaseProject>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListIosAppsResponse {
    #[serde(rename = "apps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of each `IosApp` associated with the specified `FirebaseProject`."]
    pub apps: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IosApp>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the result list is too large to fit in a single response, then a token is returned. If the string is empty, then this response is the last page of results. This token can be used in a subsequent call to `ListIosApps` to find the next group of Apps. Page tokens are short-lived and should not be persisted."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListShaCertificatesResponse {
    #[serde(rename = "certificates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of each `ShaCertificate` associated with the `AndroidApp`."]
    pub certificates: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ShaCertificate>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListWebAppsResponse {
    #[serde(rename = "apps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of each `WebApp` associated with the specified `FirebaseProject`."]
    pub apps: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WebApp>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the result list is too large to fit in a single response, then a token is returned. If the string is empty, then this response is the last page of results. This token can be used in a subsequent call to `ListWebApps` to find the next group of Apps. Page tokens are short-lived and should not be persisted."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A GCP resource location that can be selected for a FirebaseProject."]
pub struct Location {
    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Products and services that are available in the GCP resource location."]
    pub features: ::std::option::Option<::std::vec::Vec<LocationFeaturesEnum>>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the GCP resource location. It will be one of the available [GCP resource locations](https://firebase.google.com/docs/projects/locations#types)."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the GCP resource location is a [regional or multi-regional location](https://firebase.google.com/docs/projects/locations#types) for data replication."]
    pub _type: ::std::option::Option<LocationTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum LocationFeaturesEnum {
    #[serde(rename = "LOCATION_FEATURE_UNSPECIFIED")]
    #[doc = "Used internally for distinguishing unset values and is not intended for external use."]
    LocationFeatureUnspecified,
    #[serde(rename = "FIRESTORE")]
    #[doc = "This location supports Cloud Firestore database instances. App Engine is available in this location, so it can be a Project's [default GCP resource location](//firebase.google.com/docs/projects/locations#default-cloud-location)."]
    Firestore,
    #[serde(rename = "DEFAULT_STORAGE")]
    #[doc = "This location supports default Cloud Storage buckets. App Engine is available in this location, so it can be a Project's [default GCP resource location](//firebase.google.com/docs/projects/locations#default-cloud-location)."]
    DefaultStorage,
    #[serde(rename = "FUNCTIONS")]
    #[doc = "Cloud Functions for Firebase is available in this location."]
    Functions,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Indicates whether the GCP resource location is a [regional or multi-regional location](https://firebase.google.com/docs/projects/locations#types) for data replication."]
pub enum LocationTypeEnum {
    #[serde(rename = "LOCATION_TYPE_UNSPECIFIED")]
    #[doc = "Used internally for distinguishing unset values and is not intended for external use."]
    LocationTypeUnspecified,
    #[serde(rename = "REGIONAL")]
    #[doc = "The location is a regional location. Data in a regional location is replicated in multiple zones within a region."]
    Regional,
    #[serde(rename = "MULTI_REGIONAL")]
    #[doc = "The location is a multi-regional location. Data in a multi-region location is replicated in multiple regions. Within each region, data is replicated in multiple zones."]
    MultiRegional,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This is proto2's version of MessageSet."]
pub struct MessageSet {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This resource represents a long-running operation that is the result of a network API call."]
pub struct Operation {
    #[serde(rename = "done")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
    pub done: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error result of the operation in case of failure or cancellation."]
    pub error: ::std::option::Option<::std::boxed::Box<Status>>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "response")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
    pub response: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A reference to a Google Cloud Platform (GCP) `Project`."]
pub struct ProjectInfo {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-assigned display name of the GCP `Project`, for example: `My App`"]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the Project's default GCP resource location. The location is one of the available [GCP resource locations](https://firebase.google.com/docs/projects/locations). Not all Projects will have this field populated. If it is not populated, it means that the Project does not yet have a default GCP resource location. To set a Project's default GCP resource location, call [`FinalizeDefaultLocation`](../projects.defaultLocation/finalize) after you add Firebase resources to the Project."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the GCP `Project` to which Firebase resources can be added, in the format: projects/PROJECT_IDENTIFIER Refer to the `FirebaseProject` [`name`](../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values."]
    pub project: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemoveAnalyticsRequest {
    #[serde(rename = "analyticsPropertyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The ID of the Google Analytics property associated with the specified `FirebaseProject`. - If not set, then the Google Analytics property that is currently associated with the specified `FirebaseProject` is removed. - If set, and the specified `FirebaseProject` is currently associated with a *different* Google Analytics property, then the response is a `412 Precondition Failed` error. "]
    pub analytics_property_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchFirebaseAppsResponse {
    #[serde(rename = "apps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "One page of results from a call to `SearchFirebaseApps`."]
    pub apps: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FirebaseAppInfo>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the result list is too large to fit in a single response, then a token is returned. This token can be used in a subsequent calls to `SearchFirebaseApps` to find the next group of Apps. Page tokens are short-lived and should not be persisted."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A SHA-1 or SHA-256 certificate associated with the AndroidApp."]
pub struct ShaCertificate {
    #[serde(rename = "certType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of SHA certificate encoded in the hash."]
    pub cert_type: ::std::option::Option<ShaCertificateCertTypeEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the ShaCertificate for the AndroidApp, in the format: projects/PROJECT_IDENTIFIER/androidApps/APP_ID/sha/SHA_HASH * PROJECT_IDENTIFIER: the parent Project's [`ProjectNumber`](../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](../projects.androidApps#AndroidApp.FIELDS.app_id)). * SHA_HASH: the certificate hash for the App (see [`shaHash`](../projects.androidApps.sha#ShaCertificate.FIELDS.sha_hash))."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shaHash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The certificate hash for the `AndroidApp`."]
    pub sha_hash: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of SHA certificate encoded in the hash."]
pub enum ShaCertificateCertTypeEnum {
    #[serde(rename = "SHA_CERTIFICATE_TYPE_UNSPECIFIED")]
    #[doc = "Unknown state. This is only used for distinguishing unset values."]
    ShaCertificateTypeUnspecified,
    #[serde(rename = "SHA_1")]
    #[doc = "Certificate is a SHA-1 type certificate."]
    Sha1,
    #[serde(rename = "SHA_256")]
    #[doc = "Certificate is a SHA-256 type certificate."]
    Sha256,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
pub struct Status {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status code, which should be an enum value of google.rpc.Code."]
    pub code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
    pub details: ::std::option::Option<
        ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    >,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Wire-format for a Status object"]
pub struct StatusProto {
    #[serde(rename = "canonicalCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The canonical error code (see codes.proto) that most closely corresponds to this status. May be missing."]
    pub canonical_code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Numeric code drawn from the space specified below. Often, this is the canonical error space, and code is drawn from google3/util/task/codes.proto"]
    pub code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detail message"]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "messageSet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "message_set associates an arbitrary proto message with the status."]
    pub message_set: ::std::option::Option<::std::boxed::Box<MessageSet>>,
    #[serde(rename = "space")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The following are usually only present when code != 0 Space to which this status belongs"]
    pub space: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mapping of a Firebase App to a Google Analytics data stream"]
pub struct StreamMapping {
    #[serde(rename = "app")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the Firebase App associated with the Google Analytics data stream, in the format: projects/PROJECT_IDENTIFIER/androidApps/APP_ID or projects/PROJECT_IDENTIFIER/iosApps/APP_ID or projects/PROJECT_IDENTIFIER /webApps/APP_ID Refer to the `FirebaseProject` [`name`](../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values."]
    pub app: ::std::option::Option<::std::string::String>,
    #[serde(rename = "measurementId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Applicable for Firebase Web Apps only. The unique Google-assigned identifier of the Google Analytics web stream associated with the Firebase Web App. Firebase SDKs use this ID to interact with Google Analytics APIs. Learn more about this ID and Google Analytics web streams in the [Analytics documentation](https://support.google.com/analytics/topic/9303475)."]
    pub measurement_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "streamId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique Google-assigned identifier of the Google Analytics data stream associated with the Firebase App. Learn more about Google Analytics data streams in the [Analytics documentation](https://support.google.com/analytics/answer/9303323)."]
    pub stream_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of a Firebase App for the web."]
pub struct WebApp {
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The globally unique, Firebase-assigned identifier for the `WebApp`. This identifier should be treated as an opaque token, as the data format is not specified."]
    pub app_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "appUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URLs where the `WebApp` is hosted."]
    pub app_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-assigned display name for the `WebApp`."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the WebApp, in the format: projects/PROJECT_IDENTIFIER /webApps/APP_ID * PROJECT_IDENTIFIER: the parent Project's [`ProjectNumber`](../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](../projects.webApps#WebApp.FIELDS.app_id))."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. A user-assigned unique identifier of the parent FirebaseProject for the `WebApp`."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "webId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Immutable. A unique, Firebase-assigned identifier for the `WebApp`. This identifier is only used to populate the `namespace` value for the `WebApp`. For most use cases, use `appId` to identify or reference the App. The `webId` value is only unique within a `FirebaseProject` and its associated Apps."]
    pub web_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration metadata of a single Firebase App for the web."]
pub struct WebAppConfig {
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API key associated with the `WebApp`."]
    pub api_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The globally unique, Firebase-assigned identifier for the `WebApp`."]
    pub app_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "authDomain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The domain Firebase Auth configures for OAuth redirects, in the format: PROJECT_ID.firebaseapp.com"]
    pub auth_domain: ::std::option::Option<::std::string::String>,
    #[serde(rename = "databaseURL")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default Firebase Realtime Database URL."]
    pub database_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the Project's default GCP resource location. The location is one of the available [GCP resource locations](https://firebase.google.com/docs/projects/locations). This field is omitted if the default GCP resource location has not been finalized yet. To set a Project's default GCP resource location, call [`FinalizeDefaultLocation`](../projects.defaultLocation/finalize) after you add Firebase resources to the Project."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "measurementId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique Google-assigned identifier of the Google Analytics web stream associated with the `WebApp`. Firebase SDKs use this ID to interact with Google Analytics APIs. This field is only present if the `WebApp` is linked to a web stream in a Google Analytics App + Web property. Learn more about this ID and Google Analytics web streams in the [Analytics documentation](https://support.google.com/analytics/topic/9303475). To generate a `measurementId` and link the `WebApp` with a Google Analytics web stream, call [`AddGoogleAnalytics`](../../v1beta1/projects/addGoogleAnalytics). For apps using the Firebase JavaScript SDK v7.20.0 and later, Firebase dynamically fetches the `measurementId` when your app initializes Analytics. Having this ID in your config object is optional, but it does serve as a fallback in the rare case that the dynamic fetch fails."]
    pub measurement_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "messagingSenderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sender ID for use with Firebase Cloud Messaging."]
    pub messaging_sender_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. A user-assigned unique identifier for the `FirebaseProject`."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storageBucket")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default Cloud Storage for Firebase storage bucket name."]
    pub storage_bucket: ::std::option::Option<::std::string::String>,
}
