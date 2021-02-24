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
        serde_json::from_str(&"\"json\"").unwrap()
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
    #[doc = "Addresses specify either a logical or physical address and port, which are used to tell Envoy where to bind/listen, connect to upstream and find management servers."]
    pub struct Address {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pipe")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub pipe: ::std::option::Option<::std::boxed::Box<Pipe>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "socketAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub socket_address: ::std::option::Option<::std::boxed::Box<SocketAddress>>,
    }
    impl Address {
        pub fn builder() -> AddressBuilder {
            AddressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "BuildVersion combines SemVer version of extension with free-form build information (i.e. 'alpha', 'private-build') as a set of strings."]
    pub struct BuildVersion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Free-form build information. Envoy defines several well known keys in the source/common/version/version.h file"]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "SemVer version of extension."]
        pub version: ::std::option::Option<::std::boxed::Box<SemanticVersion>>,
    }
    impl BuildVersion {
        pub fn builder() -> BuildVersionBuilder {
            BuildVersionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "All xds configs for a particular client."]
    pub struct ClientConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "node")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Node for a particular client."]
        pub node: ::std::option::Option<::std::boxed::Box<Node>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "xdsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub xds_config: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PerXdsConfig>>>,
    }
    impl ClientConfig {
        pub fn builder() -> ClientConfigBuilder {
            ClientConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for client status of clients identified by a list of NodeMatchers."]
    pub struct ClientStatusRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeMatchers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Management server can use these match criteria to identify clients. The match follows OR semantics."]
        pub node_matchers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NodeMatcher>>>,
    }
    impl ClientStatusRequest {
        pub fn builder() -> ClientStatusRequestBuilder {
            ClientStatusRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ClientStatusResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "config")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client configs for the clients specified in the ClientStatusRequest."]
        pub config: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ClientConfig>>>,
    }
    impl ClientStatusResponse {
        pub fn builder() -> ClientStatusResponseBuilder {
            ClientStatusResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Envoy's cluster manager fills this message with all currently known clusters. Cluster configuration information can be used to recreate an Envoy configuration by populating all clusters as static clusters or by returning them in a CDS response."]
    pub struct ClustersConfigDump {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dynamicActiveClusters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dynamically loaded active clusters. These are clusters that are available to service data plane traffic."]
        pub dynamic_active_clusters:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DynamicCluster>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dynamicWarmingClusters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dynamically loaded warming clusters. These are clusters that are currently undergoing warming in preparation to service data plane traffic. Note that if attempting to recreate an Envoy configuration from a configuration dump, the warming clusters should generally be discarded."]
        pub dynamic_warming_clusters:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DynamicCluster>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "staticClusters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The statically loaded cluster configs."]
        pub static_clusters:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StaticCluster>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is the :ref:`version_info ` in the last processed CDS discovery response. If there are only static bootstrap clusters, this field will be \"\"."]
        pub version_info: ::std::option::Option<::std::string::String>,
    }
    impl ClustersConfigDump {
        pub fn builder() -> ClustersConfigDumpBuilder {
            ClustersConfigDumpBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies the way to match a double value."]
    pub struct DoubleMatcher {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exact")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, the input double value must be equal to the value specified here."]
        pub exact: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, the input double value must be in the range specified here. Note: The range is using half-open interval semantics [start, end)."]
        pub range: ::std::option::Option<::std::boxed::Box<DoubleRange>>,
    }
    impl DoubleMatcher {
        pub fn builder() -> DoubleMatcherBuilder {
            DoubleMatcherBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies the double start and end of the range using half-open interval semantics [start, end)."]
    pub struct DoubleRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "end")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "end of the range (exclusive)"]
        pub end: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "start of the range (inclusive)"]
        pub start: ::std::option::Option<::std::primitive::f64>,
    }
    impl DoubleRange {
        pub fn builder() -> DoubleRangeBuilder {
            DoubleRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a dynamically loaded cluster via the CDS API."]
    pub struct DynamicCluster {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cluster")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cluster config."]
        pub cluster:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the Cluster was last updated."]
        pub last_updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is the per-resource version information. This version is currently taken from the :ref:`version_info ` field at the time that the cluster was loaded. In the future, discrete per-cluster versions may be supported by the API."]
        pub version_info: ::std::option::Option<::std::string::String>,
    }
    impl DynamicCluster {
        pub fn builder() -> DynamicClusterBuilder {
            DynamicClusterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a dynamically loaded listener via the LDS API. [#next-free-field: 6]"]
    pub struct DynamicListener {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activeState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The listener state for any active listener by this name. These are listeners that are available to service data plane traffic."]
        pub active_state: ::std::option::Option<::std::boxed::Box<DynamicListenerState>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "drainingState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The listener state for any draining listener by this name. These are listeners that are currently undergoing draining in preparation to stop servicing data plane traffic. Note that if attempting to recreate an Envoy configuration from a configuration dump, the draining listeners should generally be discarded."]
        pub draining_state: ::std::option::Option<::std::boxed::Box<DynamicListenerState>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set if the last update failed, cleared after the next successful update."]
        pub error_state: ::std::option::Option<::std::boxed::Box<UpdateFailureState>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name or unique id of this listener, pulled from the DynamicListenerState config."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warmingState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The listener state for any warming listener by this name. These are listeners that are currently undergoing warming in preparation to service data plane traffic. Note that if attempting to recreate an Envoy configuration from a configuration dump, the warming listeners should generally be discarded."]
        pub warming_state: ::std::option::Option<::std::boxed::Box<DynamicListenerState>>,
    }
    impl DynamicListener {
        pub fn builder() -> DynamicListenerBuilder {
            DynamicListenerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DynamicListenerState {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the Listener was last successfully updated."]
        pub last_updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "listener")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The listener config."]
        pub listener:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is the per-resource version information. This version is currently taken from the :ref:`version_info ` field at the time that the listener was loaded. In the future, discrete per-listener versions may be supported by the API."]
        pub version_info: ::std::option::Option<::std::string::String>,
    }
    impl DynamicListenerState {
        pub fn builder() -> DynamicListenerStateBuilder {
            DynamicListenerStateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DynamicRouteConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the Route was last updated."]
        pub last_updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "routeConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The route config."]
        pub route_config:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is the per-resource version information. This version is currently taken from the :ref:`version_info ` field at the time that the route configuration was loaded."]
        pub version_info: ::std::option::Option<::std::string::String>,
    }
    impl DynamicRouteConfig {
        pub fn builder() -> DynamicRouteConfigBuilder {
            DynamicRouteConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DynamicScopedRouteConfigs {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the scoped route config set was last updated."]
        pub last_updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name assigned to the scoped route configurations."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scopedRouteConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The scoped route configurations."]
        pub scoped_route_configs: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is the per-resource version information. This version is currently taken from the :ref:`version_info ` field at the time that the scoped routes configuration was loaded."]
        pub version_info: ::std::option::Option<::std::string::String>,
    }
    impl DynamicScopedRouteConfigs {
        pub fn builder() -> DynamicScopedRouteConfigsBuilder {
            DynamicScopedRouteConfigsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Version and identification for an Envoy extension. [#next-free-field: 6]"]
    pub struct Extension {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "category")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Category of the extension. Extension category names use reverse DNS notation. For instance \"envoy.filters.listener\" for Envoy's built-in listener filters or \"com.acme.filters.http\" for HTTP filters from acme.com vendor. [#comment:"]
        pub category: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates that the extension is present but was disabled via dynamic configuration."]
        pub disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is the name of the Envoy filter as specified in the Envoy configuration, e.g. envoy.filters.http.router, com.acme.widget."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "typeDescriptor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[#not-implemented-hide:] Type descriptor of extension configuration proto. [#comment:"]
        pub type_descriptor: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version is a property of the extension and maintained independently of other extensions and the Envoy API. This field is not set when extension did not provide version information."]
        pub version: ::std::option::Option<::std::boxed::Box<BuildVersion>>,
    }
    impl Extension {
        pub fn builder() -> ExtensionBuilder {
            ExtensionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Google's `RE2 `_ regex engine. The regex string must adhere to the documented `syntax `_. The engine is designed to complete execution in linear time as well as limit the amount of memory used. Envoy supports program size checking via runtime. The runtime keys `re2.max_program_size.error_level` and `re2.max_program_size.warn_level` can be set to integers as the maximum program size or complexity that a compiled regex can have before an exception is thrown or a warning is logged, respectively. `re2.max_program_size.error_level` defaults to 100, and `re2.max_program_size.warn_level` has no default if unset (will not check/log a warning). Envoy emits two stats for tracking the program size of regexes: the histogram `re2.program_size`, which records the program size, and the counter `re2.exceeded_warn_level`, which is incremented each time the program size exceeds the warn level threshold."]
    pub struct GoogleRe2 {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxProgramSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field controls the RE2 \"program size\" which is a rough estimate of how complex a compiled regex is to evaluate. A regex that has a program size greater than the configured value will fail to compile. In this case, the configured max program size can be increased or the regex can be simplified. If not specified, the default is 100. This field is deprecated; regexp validation should be performed on the management server instead of being done by each individual client."]
        pub max_program_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleRe2 {
        pub fn builder() -> GoogleRe2Builder {
            GoogleRe2Builder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct InlineScopedRouteConfigs {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the scoped route config set was last updated."]
        pub last_updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name assigned to the scoped route configurations."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scopedRouteConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The scoped route configurations."]
        pub scoped_route_configs: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
    }
    impl InlineScopedRouteConfigs {
        pub fn builder() -> InlineScopedRouteConfigsBuilder {
            InlineScopedRouteConfigsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies the way to match a list value."]
    pub struct ListMatcher {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oneOf")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, at least one of the values in the list must match the value specified."]
        pub one_of: ::std::option::Option<::std::boxed::Box<ValueMatcher>>,
    }
    impl ListMatcher {
        pub fn builder() -> ListMatcherBuilder {
            ListMatcherBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Envoy's listener manager fills this message with all currently known listeners. Listener configuration information can be used to recreate an Envoy configuration by populating all listeners as static listeners or by returning them in a LDS response."]
    pub struct ListenersConfigDump {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dynamicListeners")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State for any warming, active, or draining listeners."]
        pub dynamic_listeners:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DynamicListener>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "staticListeners")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The statically loaded listener configs."]
        pub static_listeners:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StaticListener>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is the :ref:`version_info ` in the last processed LDS discovery response. If there are only static bootstrap listeners, this field will be \"\"."]
        pub version_info: ::std::option::Option<::std::string::String>,
    }
    impl ListenersConfigDump {
        pub fn builder() -> ListenersConfigDumpBuilder {
            ListenersConfigDumpBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifies location of where either Envoy runs or where upstream hosts run."]
    pub struct Locality {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Region this :ref:`zone ` belongs to."]
        pub region: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When used for locality of upstream hosts, this field further splits zone into smaller chunks of sub-zones so they can be load balanced independently."]
        pub sub_zone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the local service zone where Envoy is running. Though optional, it should be set if discovery service routing is used and the discovery service exposes :ref:`zone data `, either in this message or via :option:`--service-zone`. The meaning of zone is context dependent, e.g. `Availability Zone (AZ) `_ on AWS, `Zone `_ on GCP, etc."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl Locality {
        pub fn builder() -> LocalityBuilder {
            LocalityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifies a specific Envoy instance. The node identifier is presented to the management server, which may use this identifier to distinguish per Envoy configuration for serving. [#next-free-field: 12]"]
    pub struct Node {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is motivated by informing a management server during canary which version of Envoy is being tested in a heterogeneous fleet. This will be set by Envoy in management server RPCs. This field is deprecated in favor of the user_agent_name and user_agent_version values."]
        pub build_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientFeatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client feature support list. These are well known features described in the Envoy API repository for a given major version of an API. Client features use reverse DNS naming scheme, for example `com.acme.feature`. See :ref:`the list of features ` that xDS client may support."]
        pub client_features: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cluster")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the local service cluster name where Envoy is running. Though optional, it should be set if any of the following features are used: :ref:`statsd `, :ref:`health check cluster verification `, :ref:`runtime override directory `, :ref:`user agent addition `, :ref:`HTTP global rate limiting `, :ref:`CDS `, and :ref:`HTTP tracing `, either in this message or via :option:`--service-cluster`."]
        pub cluster: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "extensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of extensions and their versions supported by the node."]
        pub extensions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Extension>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An opaque node identifier for the Envoy node. This also provides the local service node name. It should be set if any of the following features are used: :ref:`statsd `, :ref:`CDS `, and :ref:`HTTP tracing `, either in this message or via :option:`--service-node`."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "listeningAddresses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Known listening ports on the node as a generic hint to the management server for filtering :ref:`listeners ` to be returned. For example, if there is a listener bound to port 80, the list can optionally contain the SocketAddress `(0.0.0.0,80)`. The field is optional and just a hint."]
        pub listening_addresses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Address>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Locality specifying where the Envoy instance is running."]
        pub locality: ::std::option::Option<::std::boxed::Box<Locality>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque metadata extending the node identifier. Envoy will pass this directly to the management server."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userAgentBuildVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Structured version of the entity requesting config."]
        pub user_agent_build_version: ::std::option::Option<::std::boxed::Box<BuildVersion>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userAgentName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Free-form string that identifies the entity requesting config. E.g. \"envoy\" or \"grpc\""]
        pub user_agent_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userAgentVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Free-form string that identifies the version of the entity requesting config. E.g. \"1.12.2\" or \"abcd1234\", or \"SpecialEnvoyBuild\""]
        pub user_agent_version: ::std::option::Option<::std::string::String>,
    }
    impl Node {
        pub fn builder() -> NodeBuilder {
            NodeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies the way to match a Node. The match follows AND semantics."]
    pub struct NodeMatcher {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies match criteria on the node id."]
        pub node_id: ::std::option::Option<::std::boxed::Box<StringMatcher>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeMetadatas")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies match criteria on the node metadata."]
        pub node_metadatas:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StructMatcher>>>,
    }
    impl NodeMatcher {
        pub fn builder() -> NodeMatcherBuilder {
            NodeMatcherBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "NullMatch is an empty message to specify a null value."]
    pub struct NullMatch {}
    impl NullMatch {
        pub fn builder() -> NullMatchBuilder {
            NullMatchBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies the segment in a path to retrieve value from Struct."]
    pub struct PathSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, use the key to retrieve the value in a Struct."]
        pub key: ::std::option::Option<::std::string::String>,
    }
    impl PathSegment {
        pub fn builder() -> PathSegmentBuilder {
            PathSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detailed config (per xDS) with status. [#next-free-field: 6]"]
    pub struct PerXdsConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub cluster_config: ::std::option::Option<::std::boxed::Box<ClustersConfigDump>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "listenerConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub listener_config: ::std::option::Option<::std::boxed::Box<ListenersConfigDump>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "routeConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub route_config: ::std::option::Option<::std::boxed::Box<RoutesConfigDump>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scopedRouteConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub scoped_route_config: ::std::option::Option<::std::boxed::Box<ScopedRoutesConfigDump>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<PerXdsConfigStatusEnum>,
    }
    impl PerXdsConfig {
        pub fn builder() -> PerXdsConfigBuilder {
            PerXdsConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum PerXdsConfigStatusEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Status info is not available/unknown."]
        Unknown,
        #[serde(rename = "SYNCED")]
        #[doc = "Management server has sent the config to client and received ACK."]
        Synced,
        #[serde(rename = "NOT_SENT")]
        #[doc = "Config is not sent."]
        NotSent,
        #[serde(rename = "STALE")]
        #[doc = "Management server has sent the config to client but hasn’t received ACK/NACK."]
        Stale,
        #[serde(rename = "ERROR")]
        #[doc = "Management server has sent the config to client but received NACK."]
        Error,
    }
    impl ::std::default::Default for PerXdsConfigStatusEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Pipe {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The mode for the Pipe. Not applicable for abstract sockets."]
        pub mode: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unix Domain Socket path. On Linux, paths starting with '@' will use the abstract namespace. The starting '@' is replaced by a null byte by Envoy. Paths starting with '@' will result in an error in environments other than Linux."]
        pub path: ::std::option::Option<::std::string::String>,
    }
    impl Pipe {
        pub fn builder() -> PipeBuilder {
            PipeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A regex matcher designed for safety when used with untrusted input."]
    pub struct RegexMatcher {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "googleRe2")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google's RE2 regex engine."]
        pub google_re2: ::std::option::Option<::std::boxed::Box<GoogleRe2>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The regex match string. The string must be supported by the configured engine."]
        pub regex: ::std::option::Option<::std::string::String>,
    }
    impl RegexMatcher {
        pub fn builder() -> RegexMatcherBuilder {
            RegexMatcherBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Envoy's RDS implementation fills this message with all currently loaded routes, as described by their RouteConfiguration objects. Static routes that are either defined in the bootstrap configuration or defined inline while configuring listeners are separated from those configured dynamically via RDS. Route configuration information can be used to recreate an Envoy configuration by populating all routes as static routes or by returning them in RDS responses."]
    pub struct RoutesConfigDump {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dynamicRouteConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dynamically loaded route configs."]
        pub dynamic_route_configs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DynamicRouteConfig>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "staticRouteConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The statically loaded route configs."]
        pub static_route_configs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StaticRouteConfig>>>,
    }
    impl RoutesConfigDump {
        pub fn builder() -> RoutesConfigDumpBuilder {
            RoutesConfigDumpBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Envoy's scoped RDS implementation fills this message with all currently loaded route configuration scopes (defined via ScopedRouteConfigurationsSet protos). This message lists both the scopes defined inline with the higher order object (i.e., the HttpConnectionManager) and the dynamically obtained scopes via the SRDS API."]
    pub struct ScopedRoutesConfigDump {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dynamicScopedRouteConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dynamically loaded scoped route configs."]
        pub dynamic_scoped_route_configs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DynamicScopedRouteConfigs>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inlineScopedRouteConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The statically loaded scoped route configs."]
        pub inline_scoped_route_configs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InlineScopedRouteConfigs>>>,
    }
    impl ScopedRoutesConfigDump {
        pub fn builder() -> ScopedRoutesConfigDumpBuilder {
            ScopedRoutesConfigDumpBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Envoy uses SemVer (https://semver.org/). Major/minor versions indicate expected behaviors and APIs, the patch version field is used only for security fixes and can be generally ignored."]
    pub struct SemanticVersion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "majorNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub major_number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minorNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub minor_number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "patch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub patch: ::std::option::Option<::std::primitive::i64>,
    }
    impl SemanticVersion {
        pub fn builder() -> SemanticVersionBuilder {
            SemanticVersionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "[#next-free-field: 7]"]
    pub struct SocketAddress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The address for this socket. :ref:`Listeners ` will bind to the address. An empty address is not allowed. Specify ``0.0.0.0`` or ``::`` to bind to any address. [#comment:TODO(zuercher) reinstate when implemented: It is possible to distinguish a Listener address via the prefix/suffix matching in :ref:`FilterChainMatch `.] When used within an upstream :ref:`BindConfig `, the address controls the source address of outbound connections. For :ref:`clusters `, the cluster type determines whether the address must be an IP (*STATIC* or *EDS* clusters) or a hostname resolved by DNS (*STRICT_DNS* or *LOGICAL_DNS* clusters). Address resolution can be customized via :ref:`resolver_name `."]
        pub address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipv4Compat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When binding to an IPv6 address above, this enables `IPv4 compatibility `_. Binding to ``::`` will allow both IPv4 and IPv6 connections, with peer IPv4 addresses mapped into IPv6 space as ``::FFFF:``."]
        pub ipv4_compat: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namedPort")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is only valid if :ref:`resolver_name ` is specified below and the named resolver is capable of named port resolution."]
        pub named_port: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "portValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub port_value: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protocol")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub protocol: ::std::option::Option<SocketAddressProtocolEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resolverName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the custom resolver. This must have been registered with Envoy. If this is empty, a context dependent default applies. If the address is a concrete IP address, no resolution will occur. If address is a hostname this should be set for resolution other than DNS. Specifying a custom resolver with *STRICT_DNS* or *LOGICAL_DNS* will generate an error at runtime."]
        pub resolver_name: ::std::option::Option<::std::string::String>,
    }
    impl SocketAddress {
        pub fn builder() -> SocketAddressBuilder {
            SocketAddressBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum SocketAddressProtocolEnum {
        #[serde(rename = "TCP")]
        #[doc = ""]
        Tcp,
        #[serde(rename = "UDP")]
        #[doc = ""]
        Udp,
    }
    impl ::std::default::Default for SocketAddressProtocolEnum {
        fn default() -> Self {
            Self::Tcp
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a statically loaded cluster."]
    pub struct StaticCluster {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cluster")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cluster config."]
        pub cluster:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the Cluster was last updated."]
        pub last_updated: ::std::option::Option<::std::string::String>,
    }
    impl StaticCluster {
        pub fn builder() -> StaticClusterBuilder {
            StaticClusterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a statically loaded listener."]
    pub struct StaticListener {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the Listener was last successfully updated."]
        pub last_updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "listener")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The listener config."]
        pub listener:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl StaticListener {
        pub fn builder() -> StaticListenerBuilder {
            StaticListenerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct StaticRouteConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the Route was last updated."]
        pub last_updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "routeConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The route config."]
        pub route_config:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl StaticRouteConfig {
        pub fn builder() -> StaticRouteConfigBuilder {
            StaticRouteConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies the way to match a string. [#next-free-field: 7]"]
    pub struct StringMatcher {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exact")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The input string must match exactly the string specified here. Examples: * *abc* only matches the value *abc*."]
        pub exact: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ignoreCase")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, indicates the exact/prefix/suffix matching should be case insensitive. This has no effect for the safe_regex match. For example, the matcher *data* will match both input string *Data* and *data* if set to true."]
        pub ignore_case: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead. Examples: * *abc* matches the value *abc.xyz*"]
        pub prefix: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The input string must match the regular expression specified here. The regex grammar is defined `here `_. Examples: * The regex ``\\d{3}`` matches the value *123* * The regex ``\\d{3}`` does not match the value *1234* * The regex ``\\d{3}`` does not match the value *123.456* .. attention:: This field has been deprecated in favor of `safe_regex` as it is not safe for use with untrusted input in all cases."]
        pub regex: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "safeRegex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The input string must match the regular expression specified here."]
        pub safe_regex: ::std::option::Option<::std::boxed::Box<RegexMatcher>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suffix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead. Examples: * *abc* matches the value *xyz.abc*"]
        pub suffix: ::std::option::Option<::std::string::String>,
    }
    impl StringMatcher {
        pub fn builder() -> StringMatcherBuilder {
            StringMatcherBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "StructMatcher provides a general interface to check if a given value is matched in google.protobuf.Struct. It uses `path` to retrieve the value from the struct and then check if it's matched to the specified value. For example, for the following Struct: .. code-block:: yaml fields: a: struct_value: fields: b: struct_value: fields: c: string_value: pro t: list_value: values: - string_value: m - string_value: n The following MetadataMatcher is matched as the path [a, b, c] will retrieve a string value \"pro\" from the Metadata which is matched to the specified prefix match. .. code-block:: yaml path: - key: a - key: b - key: c value: string_match: prefix: pr The following StructMatcher is matched as the code will match one of the string values in the list at the path [a, t]. .. code-block:: yaml path: - key: a - key: t value: list_match: one_of: string_match: exact: m An example use of StructMatcher is to match metadata in envoy.v*.core.Node."]
    pub struct StructMatcher {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The path to retrieve the Value from the Struct."]
        pub path: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PathSegment>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The StructMatcher is matched if the value retrieved by path is matched to this value."]
        pub value: ::std::option::Option<::std::boxed::Box<ValueMatcher>>,
    }
    impl StructMatcher {
        pub fn builder() -> StructMatcherBuilder {
            StructMatcherBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UpdateFailureState {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "details")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details about the last failed update attempt."]
        pub details: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failedConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "What the component configuration would have been if the update had succeeded."]
        pub failed_configuration:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdateAttempt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time of the latest failed update attempt."]
        pub last_update_attempt: ::std::option::Option<::std::string::String>,
    }
    impl UpdateFailureState {
        pub fn builder() -> UpdateFailureStateBuilder {
            UpdateFailureStateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies the way to match a ProtobufWkt::Value. Primitive values and ListValue are supported. StructValue is not supported and is always not matched. [#next-free-field: 7]"]
    pub struct ValueMatcher {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boolMatch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, a match occurs if and only if the target value is a bool value and is equal to this field."]
        pub bool_match: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "doubleMatch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, a match occurs if and only if the target value is a double value and is matched to this field."]
        pub double_match: ::std::option::Option<::std::boxed::Box<DoubleMatcher>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "listMatch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, a match occurs if and only if the target value is a list value and is matched to this field."]
        pub list_match: ::std::option::Option<::std::boxed::Box<ListMatcher>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nullMatch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, a match occurs if and only if the target value is a NullValue."]
        pub null_match: ::std::option::Option<::std::boxed::Box<NullMatch>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "presentMatch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, value match will be performed based on whether the path is referring to a valid primitive value in the metadata. If the path is referring to a non-primitive value, the result is always not matched."]
        pub present_match: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringMatch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, a match occurs if and only if the target value is a string value and is matched to this field."]
        pub string_match: ::std::option::Option<::std::boxed::Box<StringMatcher>>,
    }
    impl ValueMatcher {
        pub fn builder() -> ValueMatcherBuilder {
            ValueMatcherBuilder::default()
        }
    }
}
