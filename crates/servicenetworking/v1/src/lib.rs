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
pub mod resources {
    pub mod operations {
        pub mod methods {
            pub mod list {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The standard list filter."]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The standard list page size."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The standard list page token."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod services {
        pub mod resources {
            pub mod connections {
                pub mod methods {
                    pub mod list {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "network")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The name of service consumer's VPC network that's connected with service producer network through a private connection. The network name must be in the following format: `projects/{project}/global/networks/{network}`. {project} is a project number, such as in `12345` that includes the VPC service consumer's VPC network. {network} is the name of the service consumer's VPC network."]
                            pub network: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod patch {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "force")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If a previously defined allocated range is removed, force flag must be set to true."]
                            pub force: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The update mask. If this is omitted, it defaults to \"*\". You can only update the listed peering ranges."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
        }
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata provided through GetOperation request for the LRO generated by AddDnsRecordSet API"]
    pub struct AddDnsRecordSetMetadata {}
    impl AddDnsRecordSetMetadata {
        pub fn builder() -> AddDnsRecordSetMetadataBuilder {
            AddDnsRecordSetMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to add a record set to a private managed DNS zone in the shared producer host project."]
    pub struct AddDnsRecordSetRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The network that the consumer is using to connect with services. Must be in the form of projects/{project}/global/networks/{network} {project} is the project number, as in '12345' {network} is the network name."]
        pub consumer_network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dnsRecordSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The DNS record set to add."]
        pub dns_record_set: ::std::option::Option<::std::boxed::Box<DnsRecordSet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the private DNS zone in the shared producer host project to which the record set will be added."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl AddDnsRecordSetRequest {
        pub fn builder() -> AddDnsRecordSetRequestBuilder {
            AddDnsRecordSetRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata provided through GetOperation request for the LRO generated by AddDnsZone API"]
    pub struct AddDnsZoneMetadata {}
    impl AddDnsZoneMetadata {
        pub fn builder() -> AddDnsZoneMetadataBuilder {
            AddDnsZoneMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to add a private managed DNS zone in the shared producer host project and a matching DNS peering zone in the consumer project."]
    pub struct AddDnsZoneRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The network that the consumer is using to connect with services. Must be in the form of projects/{project}/global/networks/{network} {project} is the project number, as in '12345' {network} is the network name."]
        pub consumer_network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dnsSuffix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The DNS name suffix for the zones e.g. `example.com`."]
        pub dns_suffix: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name for both the private zone in the shared producer host project and the peering zone in the consumer project. Must be unique within both projects. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl AddDnsZoneRequest {
        pub fn builder() -> AddDnsZoneRequestBuilder {
            AddDnsZoneRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents managed DNS zones created in the shared producer host and consumer projects."]
    pub struct AddDnsZoneResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerPeeringZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The DNS peering zone created in the consumer project."]
        pub consumer_peering_zone: ::std::option::Option<::std::boxed::Box<DnsZone>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "producerPrivateZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The private DNS zone created in the shared producer host project."]
        pub producer_private_zone: ::std::option::Option<::std::boxed::Box<DnsZone>>,
    }
    impl AddDnsZoneResponse {
        pub fn builder() -> AddDnsZoneResponseBuilder {
            AddDnsZoneResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata provided through GetOperation request for the LRO generated by AddRoles API"]
    pub struct AddRolesMetadata {}
    impl AddRolesMetadata {
        pub fn builder() -> AddRolesMetadataBuilder {
            AddRolesMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for AddRoles to allow Service Producers to add roles in the shared VPC host project for them to use."]
    pub struct AddRolesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The network that the consumer is using to connect with services. Must be in the form of projects/{project}/global/networks/{network} {project} is a project number, as in '12345' {network} is a network name."]
        pub consumer_network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policyBinding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. List of policy bindings to add to shared VPC host project."]
        pub policy_binding:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PolicyBinding>>>,
    }
    impl AddRolesRequest {
        pub fn builder() -> AddRolesRequestBuilder {
            AddRolesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents IAM roles added to the shared VPC host project."]
    pub struct AddRolesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policyBinding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. List of policy bindings that were added to the shared VPC host project."]
        pub policy_binding:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PolicyBinding>>>,
    }
    impl AddRolesResponse {
        pub fn builder() -> AddRolesResponseBuilder {
            AddRolesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to create a subnetwork in a previously peered service network."]
    pub struct AddSubnetworkRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A resource that represents the service consumer, such as `projects/123456`. The project number can be different from the value in the consumer network parameter. For example, the network might be part of a Shared VPC network. In those cases, Service Networking validates that this resource belongs to that Shared VPC."]
        pub consumer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the service consumer's VPC network. The network must have an existing private connection that was provisioned through the connections.create method. The name must be in the following format: `projects/{project}/global/networks/{network}`, where {project} is a project number, such as `12345`. {network} is the name of a VPC network in the project."]
        pub consumer_network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Description of the subnet."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipPrefixLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The prefix length of the subnet's IP address range. Use CIDR range notation, such as `30` to provision a subnet with an `x.x.x.x/30` CIDR range. The IP address range is drawn from a pool of available ranges in the service consumer's allocated range."]
        pub ip_prefix_length: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of a [region](/compute/docs/regions-zones) for the subnet, such `europe-west1`."]
        pub region: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The starting address of a range. The address must be a valid IPv4 address in the x.x.x.x format. This value combined with the IP prefix range is the CIDR range for the subnet. The range must be within the allocated range that is assigned to the private connection. If the CIDR range isn't available, the call fails."]
        pub requested_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The name of one or more allocated IP address ranges associated with this private service access connection. If no range names are provided all ranges associated with this connection will be considered. If a CIDR range with the specified IP prefix length is not available within these ranges, the call fails."]
        pub requested_ranges: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secondaryIpRangeSpecs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A list of secondary IP ranges to be created within the new subnetwork."]
        pub secondary_ip_range_specs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SecondaryIpRangeSpec>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subnetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A name for the new subnet. For information about the naming requirements, see [subnetwork](/compute/docs/reference/rest/v1/subnetworks) in the Compute API documentation."]
        pub subnetwork: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subnetworkUsers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of members that are granted the `compute.networkUser` role on the subnet."]
        pub subnetwork_users: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl AddSubnetworkRequest {
        pub fn builder() -> AddSubnetworkRequestBuilder {
            AddSubnetworkRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Api is a light-weight descriptor for an API Interface. Interfaces are also described as \"protocol buffer services\" in some contexts, such as by the \"service\" keyword in a .proto file, but they are different from API Services, which represent a concrete implementation of an interface as opposed to simply a description of methods and bindings. They are also sometimes simply referred to as \"APIs\" in other contexts, such as the name of this message itself. See https://cloud.google.com/apis/design/glossary for detailed terminology."]
    pub struct Api {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "methods")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The methods of this interface, in unspecified order."]
        pub methods: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Method>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mixins")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Included interfaces. See Mixin."]
        pub mixins: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Mixin>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fully qualified name of this interface, including package name followed by the interface's simple name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "options")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Any metadata attached to the interface."]
        pub options: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Option>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Source context for the protocol buffer service represented by this message."]
        pub source_context: ::std::option::Option<::std::boxed::Box<SourceContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "syntax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source syntax of the service."]
        pub syntax: ::std::option::Option<ApiSyntaxEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A version string for this interface. If specified, must have the form `major-version.minor-version`, as in `1.10`. If the minor version is omitted, it defaults to zero. If the entire version field is empty, the major version is derived from the package name, as outlined below. If the field is not empty, the version in the package name will be verified to be consistent with what is provided here. The versioning schema uses [semantic versioning](http://semver.org) where the major version number indicates a breaking change and the minor version an additive, non-breaking change. Both version numbers are signals to users what to expect from different versions, and should be carefully chosen based on the product plan. The major version is also reflected in the package name of the interface, which must end in `v`, as in `google.feature.v1`. For major versions 0 and 1, the suffix can be omitted. Zero major versions must only be used for experimental, non-GA interfaces. "]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl Api {
        pub fn builder() -> ApiBuilder {
            ApiBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The source syntax of the service."]
    pub enum ApiSyntaxEnum {
        #[serde(rename = "SYNTAX_PROTO2")]
        #[doc = "Syntax `proto2`."]
        SyntaxProto2,
        #[serde(rename = "SYNTAX_PROTO3")]
        #[doc = "Syntax `proto3`."]
        SyntaxProto3,
    }
    impl ::std::default::Default for ApiSyntaxEnum {
        fn default() -> Self {
            Self::SyntaxProto2
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for an authentication provider, including support for [JSON Web Token (JWT)](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32)."]
    pub struct AuthProvider {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audiences")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of JWT [audiences](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.3). that are allowed to access. A JWT containing any of these audiences will be accepted. When this setting is absent, JWTs with audiences: - \"https://[service.name]/[google.protobuf.Api.name]\" - \"https://[service.name]/\" will be accepted. For example, if no audiences are in the setting, LibraryService API will accept JWTs with the following audiences: - https://library-example.googleapis.com/google.example.library.v1.LibraryService - https://library-example.googleapis.com/ Example: audiences: bookstore_android.apps.googleusercontent.com, bookstore_web.apps.googleusercontent.com"]
        pub audiences: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorizationUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Redirect URL if JWT token is required but not present or is expired. Implement authorizationUrl of securityDefinitions in OpenAPI spec."]
        pub authorization_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the auth provider. It will be referred to by `AuthRequirement.provider_id`. Example: \"bookstore_auth\"."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "issuer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies the principal that issued the JWT. See https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.1 Usually a URL or an email address. Example: https://securetoken.google.com Example: 1234567-compute@developer.gserviceaccount.com"]
        pub issuer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jwksUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of the provider's public key set to validate signature of the JWT. See [OpenID Discovery](https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata). Optional if the key set document: - can be retrieved from [OpenID Discovery](https://openid.net/specs/openid-connect-discovery-1_0.html) of the issuer. - can be inferred from the email domain of the issuer (e.g. a Google service account). Example: https://www.googleapis.com/oauth2/v1/certs"]
        pub jwks_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jwtLocations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the locations to extract the JWT. JWT locations can be either from HTTP headers or URL query parameters. The rule is that the first match wins. The checking order is: checking all headers first, then URL query parameters. If not specified, default to use following 3 locations: 1) Authorization: Bearer 2) x-goog-iap-jwt-assertion 3) access_token query parameter Default locations can be specified as followings: jwt_locations: - header: Authorization value_prefix: \"Bearer \" - header: x-goog-iap-jwt-assertion - query: access_token"]
        pub jwt_locations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<JwtLocation>>>,
    }
    impl AuthProvider {
        pub fn builder() -> AuthProviderBuilder {
            AuthProviderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "User-defined authentication requirements, including support for [JSON Web Token (JWT)](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32)."]
    pub struct AuthRequirement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audiences")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "NOTE: This will be deprecated soon, once AuthProvider.audiences is implemented and accepted in all the runtime components. The list of JWT [audiences](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.3). that are allowed to access. A JWT containing any of these audiences will be accepted. When this setting is absent, only JWTs with audience \"https://Service_name/API_name\" will be accepted. For example, if no audiences are in the setting, LibraryService API will only accept JWTs with the following audience \"https://library-example.googleapis.com/google.example.library.v1.LibraryService\". Example: audiences: bookstore_android.apps.googleusercontent.com, bookstore_web.apps.googleusercontent.com"]
        pub audiences: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "providerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "id from authentication provider. Example: provider_id: bookstore_auth"]
        pub provider_id: ::std::option::Option<::std::string::String>,
    }
    impl AuthRequirement {
        pub fn builder() -> AuthRequirementBuilder {
            AuthRequirementBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`Authentication` defines the authentication configuration for an API. Example for an API targeted for external use: name: calendar.googleapis.com authentication: providers: - id: google_calendar_auth jwks_uri: https://www.googleapis.com/oauth2/v1/certs issuer: https://securetoken.google.com rules: - selector: \"*\" requirements: provider_id: google_calendar_auth"]
    pub struct Authentication {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "providers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines a set of authentication providers that a service supports."]
        pub providers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuthProvider>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of authentication rules that apply to individual API methods. **NOTE:** All service configuration rules follow \"last one wins\" order."]
        pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuthenticationRule>>>,
    }
    impl Authentication {
        pub fn builder() -> AuthenticationBuilder {
            AuthenticationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Authentication rules for the service. By default, if a method has any authentication requirements, every request must include a valid credential matching one of the requirements. It's an error to include more than one kind of credential in a single request. If a method doesn't have any auth requirements, request credentials will be ignored."]
    pub struct AuthenticationRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowWithoutCredential")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, the service accepts API keys without any other credential. This flag only applies to HTTP and gRPC requests."]
        pub allow_without_credential: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requirements for OAuth credentials."]
        pub oauth: ::std::option::Option<::std::boxed::Box<OAuthRequirements>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requirements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Requirements for additional authentication providers."]
        pub requirements:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuthRequirement>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selector")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Selects the methods to which this rule applies. Refer to selector for syntax details."]
        pub selector: ::std::option::Option<::std::string::String>,
    }
    impl AuthenticationRule {
        pub fn builder() -> AuthenticationRuleBuilder {
            AuthenticationRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`Backend` defines the backend configuration for a service."]
    pub struct Backend {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of API backend rules that apply to individual API methods. **NOTE:** All service configuration rules follow \"last one wins\" order."]
        pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BackendRule>>>,
    }
    impl Backend {
        pub fn builder() -> BackendBuilder {
            BackendBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A backend rule provides configuration for an individual API element."]
    pub struct BackendRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The address of the API backend. The scheme is used to determine the backend protocol and security. The following schemes are accepted: SCHEME PROTOCOL SECURITY http:// HTTP None https:// HTTP TLS grpc:// gRPC None grpcs:// gRPC TLS It is recommended to explicitly include a scheme. Leaving out the scheme may cause constrasting behaviors across platforms. If the port is unspecified, the default is: - 80 for schemes without TLS - 443 for schemes with TLS For HTTP backends, use protocol to specify the protocol version."]
        pub address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deadline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of seconds to wait for a response from a request. The default varies based on the request protocol and deployment environment."]
        pub deadline: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableAuth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When disable_auth is true, a JWT ID token won't be generated and the original \"Authorization\" HTTP header will be preserved. If the header is used to carry the original token and is expected by the backend, this field must be set to true to preserve the header."]
        pub disable_auth: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jwtAudience")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The JWT audience is used when generating a JWT ID token for the backend. This ID token will be added in the HTTP \"authorization\" header, and sent to the backend."]
        pub jwt_audience: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minDeadline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum deadline in seconds needed for this method. Calls having deadline value lower than this will be rejected."]
        pub min_deadline: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationDeadline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of seconds to wait for the completion of a long running operation. The default is no deadline."]
        pub operation_deadline: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pathTranslation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub path_translation: ::std::option::Option<BackendRulePathTranslationEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protocol")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The protocol used for sending a request to the backend. The supported values are \"http/1.1\" and \"h2\". The default value is inferred from the scheme in the address field: SCHEME PROTOCOL http:// http/1.1 https:// http/1.1 grpc:// h2 grpcs:// h2 For secure HTTP backends (https://) that support HTTP/2, set this field to \"h2\" for improved performance. Configuring this field to non-default values is only supported for secure HTTP backends. This field will be ignored for all other backends. See https://www.iana.org/assignments/tls-extensiontype-values/tls-extensiontype-values.xhtml#alpn-protocol-ids for more details on the supported values."]
        pub protocol: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selector")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Selects the methods to which this rule applies. Refer to selector for syntax details."]
        pub selector: ::std::option::Option<::std::string::String>,
    }
    impl BackendRule {
        pub fn builder() -> BackendRuleBuilder {
            BackendRuleBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum BackendRulePathTranslationEnum {
        #[serde(rename = "PATH_TRANSLATION_UNSPECIFIED")]
        #[doc = ""]
        PathTranslationUnspecified,
        #[serde(rename = "CONSTANT_ADDRESS")]
        #[doc = "Use the backend address as-is, with no modification to the path. If the URL pattern contains variables, the variable names and values will be appended to the query string. If a query string parameter and a URL pattern variable have the same name, this may result in duplicate keys in the query string. # Examples Given the following operation config: Method path: /api/company/{cid}/user/{uid} Backend address: https://example.cloudfunctions.net/getUser Requests to the following request paths will call the backend at the translated path: Request path: /api/company/widgetworks/user/johndoe Translated: https://example.cloudfunctions.net/getUser?cid=widgetworks&uid=johndoe Request path: /api/company/widgetworks/user/johndoe?timezone=EST Translated: https://example.cloudfunctions.net/getUser?timezone=EST&cid=widgetworks&uid=johndoe"]
        ConstantAddress,
        #[serde(rename = "APPEND_PATH_TO_ADDRESS")]
        #[doc = "The request path will be appended to the backend address. # Examples Given the following operation config: Method path: /api/company/{cid}/user/{uid} Backend address: https://example.appspot.com Requests to the following request paths will call the backend at the translated path: Request path: /api/company/widgetworks/user/johndoe Translated: https://example.appspot.com/api/company/widgetworks/user/johndoe Request path: /api/company/widgetworks/user/johndoe?timezone=EST Translated: https://example.appspot.com/api/company/widgetworks/user/johndoe?timezone=EST"]
        AppendPathToAddress,
    }
    impl ::std::default::Default for BackendRulePathTranslationEnum {
        fn default() -> Self {
            Self::PathTranslationUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Billing related configuration of the service. The following example shows how to configure monitored resources and metrics for billing, `consumer_destinations` is the only supported destination and the monitored resources need at least one label key `cloud.googleapis.com/location` to indicate the location of the billing usage, using different monitored resources between monitoring and billing is recommended so they can be evolved independently: monitored_resources: - type: library.googleapis.com/billing_branch labels: - key: cloud.googleapis.com/location description: | Predefined label to support billing location restriction. - key: city description: | Custom label to define the city where the library branch is located in. - key: name description: Custom label to define the name of the library branch. metrics: - name: library.googleapis.com/book/borrowed_count metric_kind: DELTA value_type: INT64 unit: \"1\" billing: consumer_destinations: - monitored_resource: library.googleapis.com/billing_branch metrics: - library.googleapis.com/book/borrowed_count"]
    pub struct Billing {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerDestinations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Billing configurations for sending metrics to the consumer project. There can be multiple consumer destinations per service, each one must have a different monitored resource type. A metric can be used in at most one consumer destination."]
        pub consumer_destinations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BillingDestination>>>,
    }
    impl Billing {
        pub fn builder() -> BillingBuilder {
            BillingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration of a specific billing destination (Currently only support bill against consumer project)."]
    pub struct BillingDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Names of the metrics to report to this billing destination. Each name must be defined in Service.metrics section."]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monitoredResource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The monitored resource type. The type must be defined in Service.monitored_resources section."]
        pub monitored_resource: ::std::option::Option<::std::string::String>,
    }
    impl BillingDestination {
        pub fn builder() -> BillingDestinationBuilder {
            BillingDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for Operations.CancelOperation."]
    pub struct CancelOperationRequest {}
    impl CancelOperationRequest {
        pub fn builder() -> CancelOperationRequestBuilder {
            CancelOperationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a private connection resource. A private connection is implemented as a VPC Network Peering connection between a service producer's VPC network and a service consumer's VPC network."]
    pub struct Connection {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of service consumer's VPC network that's connected with service producer network, in the following format: `projects/{project}/global/networks/{network}`. `{project}` is a project number, such as in `12345` that includes the VPC service consumer's VPC network. `{network}` is the name of the service consumer's VPC network."]
        pub network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "peering")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of the VPC Network Peering connection that was created by the service producer."]
        pub peering: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reservedPeeringRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of one or more allocated IP address ranges for this service producer of type `PEERING`. Note that invoking CreateConnection method with a different range when connection is already established will not modify already provisioned service producer subnetworks. If CreateConnection method is invoked repeatedly to reconnect when peering connection had been disconnected on the consumer side, leaving this field empty will restore previously allocated IP ranges."]
        pub reserved_peering_ranges: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "service")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of the peering service that's associated with this connection, in the following format: `services/{service name}`."]
        pub service: ::std::option::Option<::std::string::String>,
    }
    impl Connection {
        pub fn builder() -> ConnectionBuilder {
            ConnectionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration information for a private service access connection."]
    pub struct ConsumerConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerExportCustomRoutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Export custom routes flag value for peering from consumer to producer."]
        pub consumer_export_custom_routes: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerExportSubnetRoutesWithPublicIp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Export subnet routes with public ip flag value for peering from consumer to producer."]
        pub consumer_export_subnet_routes_with_public_ip:
            ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerImportCustomRoutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Import custom routes flag value for peering from consumer to producer."]
        pub consumer_import_custom_routes: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerImportSubnetRoutesWithPublicIp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Import subnet routes with public ip flag value for peering from consumer to producer."]
        pub consumer_import_subnet_routes_with_public_ip:
            ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "producerExportCustomRoutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Export custom routes flag value for peering from producer to consumer."]
        pub producer_export_custom_routes: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "producerExportSubnetRoutesWithPublicIp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Export subnet routes with public ip flag value for peering from producer to consumer."]
        pub producer_export_subnet_routes_with_public_ip:
            ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "producerImportCustomRoutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Import custom routes flag value for peering from producer to consumer."]
        pub producer_import_custom_routes: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "producerImportSubnetRoutesWithPublicIp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Import subnet routes with public ip flag value for peering from producer to consumer."]
        pub producer_import_subnet_routes_with_public_ip:
            ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "producerNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The VPC host network that is used to host managed service instances. In the format, projects/{project}/global/networks/{network} where {project} is the project number e.g. '12345' and {network} is the network name."]
        pub producer_network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reservedRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The reserved ranges associated with this private service access connection."]
        pub reserved_ranges: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudServicenetworkingV1ConsumerConfigReservedRange>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vpcScReferenceArchitectureEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Indicates whether the VPC Service Controls reference architecture is configured for the producer VPC host network."]
        pub vpc_sc_reference_architecture_enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl ConsumerConfig {
        pub fn builder() -> ConsumerConfigBuilder {
            ConsumerConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata provided through GetOperation request for the LRO generated by UpdateConsumerConfig API."]
    pub struct ConsumerConfigMetadata {}
    impl ConsumerConfigMetadata {
        pub fn builder() -> ConsumerConfigMetadataBuilder {
            ConsumerConfigMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a consumer project."]
    pub struct ConsumerProject {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectNum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Project number of the consumer that is launching the service instance. It can own the network that is peered with Google or, be a service project in an XPN where the host project has the network."]
        pub project_num: ::std::option::Option<::std::string::String>,
    }
    impl ConsumerProject {
        pub fn builder() -> ConsumerProjectBuilder {
            ConsumerProjectBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`Context` defines which contexts an API requests. Example: context: rules: - selector: \"*\" requested: - google.rpc.context.ProjectContext - google.rpc.context.OriginContext The above specifies that all methods in the API request `google.rpc.context.ProjectContext` and `google.rpc.context.OriginContext`. Available context types are defined in package `google.rpc.context`. This also provides mechanism to allowlist any protobuf message extension that can be sent in grpc metadata using “x-goog-ext--bin” and “x-goog-ext--jspb” format. For example, list any service specific protobuf types that can appear in grpc metadata as follows in your yaml file: Example: context: rules: - selector: \"google.example.library.v1.LibraryService.CreateBook\" allowed_request_extensions: - google.foo.v1.NewExtension allowed_response_extensions: - google.foo.v1.NewExtension You can also specify extension ID instead of fully qualified extension name here."]
    pub struct Context {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of RPC context rules that apply to individual API methods. **NOTE:** All service configuration rules follow \"last one wins\" order."]
        pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContextRule>>>,
    }
    impl Context {
        pub fn builder() -> ContextBuilder {
            ContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A context rule provides information about the context for an individual API element."]
    pub struct ContextRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedRequestExtensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of full type names or extension IDs of extensions allowed in grpc side channel from client to backend."]
        pub allowed_request_extensions:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedResponseExtensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of full type names or extension IDs of extensions allowed in grpc side channel from backend to client."]
        pub allowed_response_extensions:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provided")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of full type names of provided contexts."]
        pub provided: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requested")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of full type names of requested contexts."]
        pub requested: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selector")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Selects the methods to which this rule applies. Refer to selector for syntax details."]
        pub selector: ::std::option::Option<::std::string::String>,
    }
    impl ContextRule {
        pub fn builder() -> ContextRuleBuilder {
            ContextRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Selects and configures the service controller used by the service. The service controller handles features like abuse, quota, billing, logging, monitoring, etc."]
    pub struct Control {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The service control environment to use. If empty, no control plane feature (like quota and billing) will be enabled."]
        pub environment: ::std::option::Option<::std::string::String>,
    }
    impl Control {
        pub fn builder() -> ControlBuilder {
            ControlBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Customize service error responses. For example, list any service specific protobuf types that can appear in error detail lists of error responses. Example: custom_error: types: - google.foo.v1.CustomError - google.foo.v1.AnotherError"]
    pub struct CustomError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of custom error rules that apply to individual API messages. **NOTE:** All service configuration rules follow \"last one wins\" order."]
        pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomErrorRule>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "types")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of custom error detail types, e.g. 'google.foo.v1.CustomError'."]
        pub types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl CustomError {
        pub fn builder() -> CustomErrorBuilder {
            CustomErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A custom error rule."]
    pub struct CustomErrorRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isErrorType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mark this message as possible payload in error response. Otherwise, objects of this type will be filtered when they appear in error payload."]
        pub is_error_type: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selector")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Selects messages to which this rule applies. Refer to selector for syntax details."]
        pub selector: ::std::option::Option<::std::string::String>,
    }
    impl CustomErrorRule {
        pub fn builder() -> CustomErrorRuleBuilder {
            CustomErrorRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A custom pattern is used for defining custom HTTP verb."]
    pub struct CustomHttpPattern {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this custom HTTP verb."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The path matched by this custom verb."]
        pub path: ::std::option::Option<::std::string::String>,
    }
    impl CustomHttpPattern {
        pub fn builder() -> CustomHttpPatternBuilder {
            CustomHttpPatternBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata provided through GetOperation request for the LRO generated by Delete Connection API"]
    pub struct DeleteConnectionMetadata {}
    impl DeleteConnectionMetadata {
        pub fn builder() -> DeleteConnectionMetadataBuilder {
            DeleteConnectionMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata provided through GetOperation request for the LRO generated by DeletePeeredDnsDomain API."]
    pub struct DeletePeeredDnsDomainMetadata {}
    impl DeletePeeredDnsDomainMetadata {
        pub fn builder() -> DeletePeeredDnsDomainMetadataBuilder {
            DeletePeeredDnsDomainMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to disable VPC service controls."]
    pub struct DisableVpcServiceControlsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The network that the consumer is using to connect with services. Must be in the form of projects/{project}/global/networks/{network} {project} is a project number, as in '12345' {network} is network name."]
        pub consumer_network: ::std::option::Option<::std::string::String>,
    }
    impl DisableVpcServiceControlsRequest {
        pub fn builder() -> DisableVpcServiceControlsRequestBuilder {
            DisableVpcServiceControlsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a DNS record set resource."]
    pub struct DnsRecordSet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. As defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1) for examples see https://cloud.google.com/dns/records/json-record."]
        pub data: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The DNS or domain name of the record set, e.g. `test.example.com`."]
        pub domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ttl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The period of time for which this RecordSet can be cached by resolvers."]
        pub ttl: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The identifier of a supported record type."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl DnsRecordSet {
        pub fn builder() -> DnsRecordSetBuilder {
            DnsRecordSetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a DNS zone resource."]
    pub struct DnsZone {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dnsSuffix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The DNS name suffix of this zone e.g. `example.com.`."]
        pub dns_suffix: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User assigned name for this resource. Must be unique within the project. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl DnsZone {
        pub fn builder() -> DnsZoneBuilder {
            DnsZoneBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`Documentation` provides the information for describing a service. Example: documentation: summary: > The Google Calendar API gives access to most calendar features. pages: - name: Overview content: (== include google/foo/overview.md ==) - name: Tutorial content: (== include google/foo/tutorial.md ==) subpages; - name: Java content: (== include google/foo/tutorial_java.md ==) rules: - selector: google.calendar.Calendar.Get description: > ... - selector: google.calendar.Calendar.Put description: > ... Documentation is provided in markdown syntax. In addition to standard markdown features, definition lists, tables and fenced code blocks are supported. Section headers can be provided and are interpreted relative to the section nesting of the context where a documentation fragment is embedded. Documentation from the IDL is merged with documentation defined via the config at normalization time, where documentation provided by config rules overrides IDL provided. A number of constructs specific to the API platform are supported in documentation text. In order to reference a proto element, the following notation can be used: [fully.qualified.proto.name][] To override the display text used for the link, this can be used: [display text][fully.qualified.proto.name] Text can be excluded from doc using the following notation: (-- internal comment --) A few directives are available in documentation. Note that directives must appear on a single line to be properly identified. The `include` directive includes a markdown file from an external source: (== include path/to/file ==) The `resource_for` directive marks a message to be the resource of a collection in REST view. If it is not specified, tools attempt to infer the resource from the operations in a collection: (== resource_for v1.shelves.books ==) The directive `suppress_warning` does not directly affect documentation and is documented together with service config validation."]
    pub struct Documentation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "documentationRootUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL to the root of documentation."]
        pub documentation_root_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "overview")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Declares a single overview page. For example: documentation: summary: ... overview: (== include overview.md ==) This is a shortcut for the following declaration (using pages style): documentation: summary: ... pages: - name: Overview content: (== include overview.md ==) Note: you cannot specify both `overview` field and `pages` field."]
        pub overview: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The top level pages for the documentation set."]
        pub pages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Page>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of documentation rules that apply to individual API elements. **NOTE:** All service configuration rules follow \"last one wins\" order."]
        pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DocumentationRule>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceRootUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the service root url if the default one (the service name from the yaml file) is not suitable. This can be seen in any fully specified service urls as well as sections that show a base that other urls are relative to."]
        pub service_root_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "summary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short summary of what the service does. Can only be provided by plain text."]
        pub summary: ::std::option::Option<::std::string::String>,
    }
    impl Documentation {
        pub fn builder() -> DocumentationBuilder {
            DocumentationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A documentation rule provides information about individual API elements."]
    pub struct DocumentationRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deprecationDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecation description of the selected element(s). It can be provided if an element is marked as `deprecated`."]
        pub deprecation_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the selected API(s)."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selector")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The selector is a comma-separated list of patterns. Each pattern is a qualified name of the element which may end in \"*\", indicating a wildcard. Wildcards are only allowed at the end and for a whole component of the qualified name, i.e. \"foo.*\" is ok, but not \"foo.b*\" or \"foo.*.bar\". A wildcard will match one or more components. To specify a default for all applicable elements, the whole pattern \"*\" is used."]
        pub selector: ::std::option::Option<::std::string::String>,
    }
    impl DocumentationRule {
        pub fn builder() -> DocumentationRuleBuilder {
            DocumentationRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct Empty {}
    impl Empty {
        pub fn builder() -> EmptyBuilder {
            EmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to enable VPC service controls."]
    pub struct EnableVpcServiceControlsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The network that the consumer is using to connect with services. Must be in the form of projects/{project}/global/networks/{network} {project} is a project number, as in '12345' {network} is network name."]
        pub consumer_network: ::std::option::Option<::std::string::String>,
    }
    impl EnableVpcServiceControlsRequest {
        pub fn builder() -> EnableVpcServiceControlsRequestBuilder {
            EnableVpcServiceControlsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`Endpoint` describes a network endpoint of a service that serves a set of APIs. It is commonly known as a service endpoint. A service may expose any number of service endpoints, and all service endpoints share the same service definition, such as quota limits and monitoring metrics. Example service configuration: name: library-example.googleapis.com endpoints: # Below entry makes 'google.example.library.v1.Library' # API be served from endpoint address library-example.googleapis.com. # It also allows HTTP OPTIONS calls to be passed to the backend, for # it to decide whether the subsequent cross-origin request is # allowed to proceed. - name: library-example.googleapis.com allow_cors: true"]
    pub struct Endpoint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aliases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED: This field is no longer supported. Instead of using aliases, please specify multiple google.api.Endpoint for each of the intended aliases. Additional names that this endpoint will be hosted on."]
        pub aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowCors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allowing [CORS](https://en.wikipedia.org/wiki/Cross-origin_resource_sharing), aka cross-domain traffic, would allow the backends served from this endpoint to receive and respond to HTTP OPTIONS requests. The response will be used by the browser to determine whether the subsequent cross-origin request is allowed to proceed."]
        pub allow_cors: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The canonical name of this endpoint."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The specification of an Internet routable address of API frontend that will handle requests to this [API Endpoint](https://cloud.google.com/apis/design/glossary). It should be either a valid IPv4 address or a fully-qualified domain name. For example, \"8.8.8.8\" or \"myservice.appspot.com\"."]
        pub target: ::std::option::Option<::std::string::String>,
    }
    impl Endpoint {
        pub fn builder() -> EndpointBuilder {
            EndpointBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Enum type definition."]
    pub struct Enum {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enumvalue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enum value definitions."]
        pub enumvalue: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EnumValue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enum type name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "options")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Protocol buffer options."]
        pub options: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Option>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source context."]
        pub source_context: ::std::option::Option<::std::boxed::Box<SourceContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "syntax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source syntax."]
        pub syntax: ::std::option::Option<EnumSyntaxEnum>,
    }
    impl Enum {
        pub fn builder() -> EnumBuilder {
            EnumBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The source syntax."]
    pub enum EnumSyntaxEnum {
        #[serde(rename = "SYNTAX_PROTO2")]
        #[doc = "Syntax `proto2`."]
        SyntaxProto2,
        #[serde(rename = "SYNTAX_PROTO3")]
        #[doc = "Syntax `proto3`."]
        SyntaxProto3,
    }
    impl ::std::default::Default for EnumSyntaxEnum {
        fn default() -> Self {
            Self::SyntaxProto2
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Enum value definition."]
    pub struct EnumValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enum value name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "number")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enum value number."]
        pub number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "options")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Protocol buffer options."]
        pub options: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Option>>>,
    }
    impl EnumValue {
        pub fn builder() -> EnumValueBuilder {
            EnumValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single field of a message type."]
    pub struct Field {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cardinality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The field cardinality."]
        pub cardinality: ::std::option::Option<FieldCardinalityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The string value of the default value of this field. Proto2 syntax only."]
        pub default_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jsonName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The field JSON name."]
        pub json_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The field type."]
        pub kind: ::std::option::Option<FieldKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The field name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "number")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The field number."]
        pub number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oneofIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the field type in `Type.oneofs`, for message or enumeration types. The first type has index 1; zero means the type is not in the list."]
        pub oneof_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "options")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The protocol buffer options."]
        pub options: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Option>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to use alternative packed wire representation."]
        pub packed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "typeUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The field type URL, without the scheme, for message or enumeration types. Example: `\"type.googleapis.com/google.protobuf.Timestamp\"`."]
        pub type_url: ::std::option::Option<::std::string::String>,
    }
    impl Field {
        pub fn builder() -> FieldBuilder {
            FieldBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The field cardinality."]
    pub enum FieldCardinalityEnum {
        #[serde(rename = "CARDINALITY_UNKNOWN")]
        #[doc = "For fields with unknown cardinality."]
        CardinalityUnknown,
        #[serde(rename = "CARDINALITY_OPTIONAL")]
        #[doc = "For optional fields."]
        CardinalityOptional,
        #[serde(rename = "CARDINALITY_REQUIRED")]
        #[doc = "For required fields. Proto2 syntax only."]
        CardinalityRequired,
        #[serde(rename = "CARDINALITY_REPEATED")]
        #[doc = "For repeated fields."]
        CardinalityRepeated,
    }
    impl ::std::default::Default for FieldCardinalityEnum {
        fn default() -> Self {
            Self::CardinalityUnknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The field type."]
    pub enum FieldKindEnum {
        #[serde(rename = "TYPE_UNKNOWN")]
        #[doc = "Field type unknown."]
        TypeUnknown,
        #[serde(rename = "TYPE_DOUBLE")]
        #[doc = "Field type double."]
        TypeDouble,
        #[serde(rename = "TYPE_FLOAT")]
        #[doc = "Field type float."]
        TypeFloat,
        #[serde(rename = "TYPE_INT64")]
        #[doc = "Field type int64."]
        TypeInt64,
        #[serde(rename = "TYPE_UINT64")]
        #[doc = "Field type uint64."]
        TypeUint64,
        #[serde(rename = "TYPE_INT32")]
        #[doc = "Field type int32."]
        TypeInt32,
        #[serde(rename = "TYPE_FIXED64")]
        #[doc = "Field type fixed64."]
        TypeFixed64,
        #[serde(rename = "TYPE_FIXED32")]
        #[doc = "Field type fixed32."]
        TypeFixed32,
        #[serde(rename = "TYPE_BOOL")]
        #[doc = "Field type bool."]
        TypeBool,
        #[serde(rename = "TYPE_STRING")]
        #[doc = "Field type string."]
        TypeString,
        #[serde(rename = "TYPE_GROUP")]
        #[doc = "Field type group. Proto2 syntax only, and deprecated."]
        TypeGroup,
        #[serde(rename = "TYPE_MESSAGE")]
        #[doc = "Field type message."]
        TypeMessage,
        #[serde(rename = "TYPE_BYTES")]
        #[doc = "Field type bytes."]
        TypeBytes,
        #[serde(rename = "TYPE_UINT32")]
        #[doc = "Field type uint32."]
        TypeUint32,
        #[serde(rename = "TYPE_ENUM")]
        #[doc = "Field type enum."]
        TypeEnum,
        #[serde(rename = "TYPE_SFIXED32")]
        #[doc = "Field type sfixed32."]
        TypeSfixed32,
        #[serde(rename = "TYPE_SFIXED64")]
        #[doc = "Field type sfixed64."]
        TypeSfixed64,
        #[serde(rename = "TYPE_SINT32")]
        #[doc = "Field type sint32."]
        TypeSint32,
        #[serde(rename = "TYPE_SINT64")]
        #[doc = "Field type sint64."]
        TypeSint64,
    }
    impl ::std::default::Default for FieldKindEnum {
        fn default() -> Self {
            Self::TypeUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Allocated IP address ranges for this private service access connection."]
    pub struct GoogleCloudServicenetworkingV1ConsumerConfigReservedRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting address of the reserved range. The address must be a valid IPv4 address in the x.x.x.x format. This value combined with the IP prefix length is the CIDR range for the reserved range."]
        pub address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipPrefixLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The prefix length of the reserved range."]
        pub ip_prefix_length: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the reserved range."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudServicenetworkingV1ConsumerConfigReservedRange {
        pub fn builder() -> GoogleCloudServicenetworkingV1ConsumerConfigReservedRangeBuilder {
            GoogleCloudServicenetworkingV1ConsumerConfigReservedRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a private connection resource. A private connection is implemented as a VPC Network Peering connection between a service producer's VPC network and a service consumer's VPC network."]
    pub struct GoogleCloudServicenetworkingV1betaConnection {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of service consumer's VPC network that's connected with service producer network, in the following format: `projects/{project}/global/networks/{network}`. `{project}` is a project number, such as in `12345` that includes the VPC service consumer's VPC network. `{network}` is the name of the service consumer's VPC network."]
        pub network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "peering")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of the VPC Network Peering connection that was created by the service producer."]
        pub peering: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reservedPeeringRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of one or more allocated IP address ranges for this service producer of type `PEERING`. Note that invoking this method with a different range when connection is already established will not modify already provisioned service producer subnetworks."]
        pub reserved_peering_ranges: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "service")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of the peering service that's associated with this connection, in the following format: `services/{service name}`."]
        pub service: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudServicenetworkingV1betaConnection {
        pub fn builder() -> GoogleCloudServicenetworkingV1betaConnectionBuilder {
            GoogleCloudServicenetworkingV1betaConnectionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a subnet that was created or discovered by a private access management service."]
    pub struct GoogleCloudServicenetworkingV1betaSubnetwork {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipCidrRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subnetwork CIDR range in `10.x.x.x/y` format."]
        pub ip_cidr_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subnetwork name. See https://cloud.google.com/compute/docs/vpc/"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "In the Shared VPC host project, the VPC network that's peered with the consumer network. For example: `projects/1234321/global/networks/host-network`"]
        pub network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outsideAllocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is a discovered subnet that is not within the current consumer allocated ranges."]
        pub outside_allocation: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudServicenetworkingV1betaSubnetwork {
        pub fn builder() -> GoogleCloudServicenetworkingV1betaSubnetworkBuilder {
            GoogleCloudServicenetworkingV1betaSubnetworkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines the HTTP configuration for an API service. It contains a list of HttpRule, each specifying the mapping of an RPC method to one or more HTTP REST API methods."]
    pub struct Http {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullyDecodeReservedExpansion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When set to true, URL path parameters will be fully URI-decoded except in cases of single segment matches in reserved expansion, where \"%2F\" will be left encoded. The default behavior is to not decode RFC 6570 reserved characters in multi segment matches."]
        pub fully_decode_reserved_expansion: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of HTTP configuration rules that apply to individual API methods. **NOTE:** All service configuration rules follow \"last one wins\" order."]
        pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HttpRule>>>,
    }
    impl Http {
        pub fn builder() -> HttpBuilder {
            HttpBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "# gRPC Transcoding gRPC Transcoding is a feature for mapping between a gRPC method and one or more HTTP REST endpoints. It allows developers to build a single API service that supports both gRPC APIs and REST APIs. Many systems, including [Google APIs](https://github.com/googleapis/googleapis), [Cloud Endpoints](https://cloud.google.com/endpoints), [gRPC Gateway](https://github.com/grpc-ecosystem/grpc-gateway), and [Envoy](https://github.com/envoyproxy/envoy) proxy support this feature and use it for large scale production services. `HttpRule` defines the schema of the gRPC/REST mapping. The mapping specifies how different portions of the gRPC request message are mapped to the URL path, URL query parameters, and HTTP request body. It also controls how the gRPC response message is mapped to the HTTP response body. `HttpRule` is typically specified as an `google.api.http` annotation on the gRPC method. Each mapping specifies a URL path template and an HTTP method. The path template may refer to one or more fields in the gRPC request message, as long as each field is a non-repeated field with a primitive (non-message) type. The path template controls how fields of the request message are mapped to the URL path. Example: service Messaging { rpc GetMessage(GetMessageRequest) returns (Message) { option (google.api.http) = { get: \"/v1/{name=messages/*}\" }; } } message GetMessageRequest { string name = 1; // Mapped to URL path. } message Message { string text = 1; // The resource content. } This enables an HTTP REST to gRPC mapping as below: HTTP | gRPC -----|----- `GET /v1/messages/123456` | `GetMessage(name: \"messages/123456\")` Any fields in the request message which are not bound by the path template automatically become HTTP query parameters if there is no HTTP request body. For example: service Messaging { rpc GetMessage(GetMessageRequest) returns (Message) { option (google.api.http) = { get:\"/v1/messages/{message_id}\" }; } } message GetMessageRequest { message SubMessage { string subfield = 1; } string message_id = 1; // Mapped to URL path. int64 revision = 2; // Mapped to URL query parameter `revision`. SubMessage sub = 3; // Mapped to URL query parameter `sub.subfield`. } This enables a HTTP JSON to RPC mapping as below: HTTP | gRPC -----|----- `GET /v1/messages/123456?revision=2&sub.subfield=foo` | `GetMessage(message_id: \"123456\" revision: 2 sub: SubMessage(subfield: \"foo\"))` Note that fields which are mapped to URL query parameters must have a primitive type or a repeated primitive type or a non-repeated message type. In the case of a repeated type, the parameter can be repeated in the URL as `...?param=A&param=B`. In the case of a message type, each field of the message is mapped to a separate parameter, such as `...?foo.a=A&foo.b=B&foo.c=C`. For HTTP methods that allow a request body, the `body` field specifies the mapping. Consider a REST update method on the message resource collection: service Messaging { rpc UpdateMessage(UpdateMessageRequest) returns (Message) { option (google.api.http) = { patch: \"/v1/messages/{message_id}\" body: \"message\" }; } } message UpdateMessageRequest { string message_id = 1; // mapped to the URL Message message = 2; // mapped to the body } The following HTTP JSON to RPC mapping is enabled, where the representation of the JSON in the request body is determined by protos JSON encoding: HTTP | gRPC -----|----- `PATCH /v1/messages/123456 { \"text\": \"Hi!\" }` | `UpdateMessage(message_id: \"123456\" message { text: \"Hi!\" })` The special name `*` can be used in the body mapping to define that every field not bound by the path template should be mapped to the request body. This enables the following alternative definition of the update method: service Messaging { rpc UpdateMessage(Message) returns (Message) { option (google.api.http) = { patch: \"/v1/messages/{message_id}\" body: \"*\" }; } } message Message { string message_id = 1; string text = 2; } The following HTTP JSON to RPC mapping is enabled: HTTP | gRPC -----|----- `PATCH /v1/messages/123456 { \"text\": \"Hi!\" }` | `UpdateMessage(message_id: \"123456\" text: \"Hi!\")` Note that when using `*` in the body mapping, it is not possible to have HTTP parameters, as all fields not bound by the path end in the body. This makes this option more rarely used in practice when defining REST APIs. The common usage of `*` is in custom methods which don't use the URL at all for transferring data. It is possible to define multiple HTTP methods for one RPC by using the `additional_bindings` option. Example: service Messaging { rpc GetMessage(GetMessageRequest) returns (Message) { option (google.api.http) = { get: \"/v1/messages/{message_id}\" additional_bindings { get: \"/v1/users/{user_id}/messages/{message_id}\" } }; } } message GetMessageRequest { string message_id = 1; string user_id = 2; } This enables the following two alternative HTTP JSON to RPC mappings: HTTP | gRPC -----|----- `GET /v1/messages/123456` | `GetMessage(message_id: \"123456\")` `GET /v1/users/me/messages/123456` | `GetMessage(user_id: \"me\" message_id: \"123456\")` ## Rules for HTTP mapping 1. Leaf request fields (recursive expansion nested messages in the request message) are classified into three categories: - Fields referred by the path template. They are passed via the URL path. - Fields referred by the HttpRule.body. They are passed via the HTTP request body. - All other fields are passed via the URL query parameters, and the parameter name is the field path in the request message. A repeated field can be represented as multiple query parameters under the same name. 2. If HttpRule.body is \"*\", there is no URL query parameter, all fields are passed via URL path and HTTP request body. 3. If HttpRule.body is omitted, there is no HTTP request body, all fields are passed via URL path and URL query parameters. ### Path template syntax Template = \"/\" Segments [ Verb ] ; Segments = Segment { \"/\" Segment } ; Segment = \"*\" | \"**\" | LITERAL | Variable ; Variable = \"{\" FieldPath [ \"=\" Segments ] \"}\" ; FieldPath = IDENT { \".\" IDENT } ; Verb = \":\" LITERAL ; The syntax `*` matches a single URL path segment. The syntax `**` matches zero or more URL path segments, which must be the last part of the URL path except the `Verb`. The syntax `Variable` matches part of the URL path as specified by its template. A variable template must not contain other variables. If a variable matches a single path segment, its template may be omitted, e.g. `{var}` is equivalent to `{var=*}`. The syntax `LITERAL` matches literal text in the URL path. If the `LITERAL` contains any reserved character, such characters should be percent-encoded before the matching. If a variable contains exactly one path segment, such as `\"{var}\"` or `\"{var=*}\"`, when such a variable is expanded into a URL path on the client side, all characters except `[-_.~0-9a-zA-Z]` are percent-encoded. The server side does the reverse decoding. Such variables show up in the [Discovery Document](https://developers.google.com/discovery/v1/reference/apis) as `{var}`. If a variable contains multiple path segments, such as `\"{var=foo/*}\"` or `\"{var=**}\"`, when such a variable is expanded into a URL path on the client side, all characters except `[-_.~/0-9a-zA-Z]` are percent-encoded. The server side does the reverse decoding, except \"%2F\" and \"%2f\" are left unchanged. Such variables show up in the [Discovery Document](https://developers.google.com/discovery/v1/reference/apis) as `{+var}`. ## Using gRPC API Service Configuration gRPC API Service Configuration (service config) is a configuration language for configuring a gRPC service to become a user-facing product. The service config is simply the YAML representation of the `google.api.Service` proto message. As an alternative to annotating your proto file, you can configure gRPC transcoding in your service config YAML files. You do this by specifying a `HttpRule` that maps the gRPC method to a REST endpoint, achieving the same effect as the proto annotation. This can be particularly useful if you have a proto that is reused in multiple services. Note that any transcoding specified in the service config will override any matching transcoding configuration in the proto. Example: http: rules: # Selects a gRPC method and applies HttpRule to it. - selector: example.v1.Messaging.GetMessage get: /v1/messages/{message_id}/{sub.subfield} ## Special notes When gRPC Transcoding is used to map a gRPC to JSON REST endpoints, the proto to JSON conversion must follow the [proto3 specification](https://developers.google.com/protocol-buffers/docs/proto3#json). While the single segment variable follows the semantics of [RFC 6570](https://tools.ietf.org/html/rfc6570) Section 3.2.2 Simple String Expansion, the multi segment variable **does not** follow RFC 6570 Section 3.2.3 Reserved Expansion. The reason is that the Reserved Expansion does not expand special characters like `?` and `#`, which would lead to invalid URLs. As the result, gRPC Transcoding uses a custom encoding for multi segment variables. The path variables **must not** refer to any repeated or mapped field, because client libraries are not capable of handling such variable expansion. The path variables **must not** capture the leading \"/\" character. The reason is that the most common use case \"{var}\" does not capture the leading \"/\" character. For consistency, all path variables must share the same behavior. Repeated message fields must not be mapped to URL query parameters, because no client library can support such complicated mapping. If an API needs to use a JSON array for request or response body, it can map the request or response body to a repeated field. However, some gRPC Transcoding implementations may not support this feature."]
    pub struct HttpRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalBindings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional HTTP bindings for the selector. Nested bindings must not contain an `additional_bindings` field themselves (that is, the nesting may only be one level deep)."]
        pub additional_bindings:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HttpRule>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "body")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the request field whose value is mapped to the HTTP request body, or `*` for mapping all request fields not captured by the path pattern to the HTTP body, or omitted for not having any HTTP request body. NOTE: the referred field must be present at the top-level of the request message type."]
        pub body: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "custom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The custom pattern is used for specifying an HTTP method that is not included in the `pattern` field, such as HEAD, or \"*\" to leave the HTTP method unspecified for this rule. The wild-card rule is useful for services that provide content to Web (HTML) clients."]
        pub custom: ::std::option::Option<::std::boxed::Box<CustomHttpPattern>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delete")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maps to HTTP DELETE. Used for deleting a resource."]
        pub delete: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "get")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maps to HTTP GET. Used for listing and getting information about resources."]
        pub get: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "patch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maps to HTTP PATCH. Used for updating a resource."]
        pub patch: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "post")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maps to HTTP POST. Used for creating a resource or performing an action."]
        pub post: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "put")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maps to HTTP PUT. Used for replacing a resource."]
        pub put: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseBody")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The name of the response field whose value is mapped to the HTTP response body. When omitted, the entire response message will be used as the HTTP response body. NOTE: The referred field must be present at the top-level of the response message type."]
        pub response_body: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selector")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Selects a method to which this rule applies. Refer to selector for syntax details."]
        pub selector: ::std::option::Option<::std::string::String>,
    }
    impl HttpRule {
        pub fn builder() -> HttpRuleBuilder {
            HttpRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies a location to extract JWT from an API request."]
    pub struct JwtLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies HTTP header name to extract JWT token."]
        pub header: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies URL query parameter name to extract JWT token."]
        pub query: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valuePrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value prefix. The value format is \"value_prefix{token}\" Only applies to \"in\" header type. Must be empty for \"in\" query type. If not empty, the header value has to match (case sensitive) this prefix. If not matched, JWT will not be extracted. If matched, JWT will be extracted after the prefix is removed. For example, for \"Authorization: Bearer {JWT}\", value_prefix=\"Bearer \" with a space at the end."]
        pub value_prefix: ::std::option::Option<::std::string::String>,
    }
    impl JwtLocation {
        pub fn builder() -> JwtLocationBuilder {
            JwtLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A description of a label."]
    pub struct LabelDescriptor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A human-readable description for the label."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The label key."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of data that can be assigned to the label."]
        pub value_type: ::std::option::Option<LabelDescriptorValueTypeEnum>,
    }
    impl LabelDescriptor {
        pub fn builder() -> LabelDescriptorBuilder {
            LabelDescriptorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of data that can be assigned to the label."]
    pub enum LabelDescriptorValueTypeEnum {
        #[serde(rename = "STRING")]
        #[doc = "A variable-length string. This is the default."]
        String,
        #[serde(rename = "BOOL")]
        #[doc = "Boolean; true or false."]
        Bool,
        #[serde(rename = "INT64")]
        #[doc = "A 64-bit signed integer."]
        Int64,
    }
    impl ::std::default::Default for LabelDescriptorValueTypeEnum {
        fn default() -> Self {
            Self::String
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "ListConnectionsResponse is the response to list peering states for the given service and consumer project."]
    pub struct ListConnectionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "connections")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Connections."]
        pub connections: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Connection>>>,
    }
    impl ListConnectionsResponse {
        pub fn builder() -> ListConnectionsResponseBuilder {
            ListConnectionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Operations.ListOperations."]
    pub struct ListOperationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of operations that matches the specified filter in the request."]
        pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
    }
    impl ListOperationsResponse {
        pub fn builder() -> ListOperationsResponseBuilder {
            ListOperationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to list peered DNS domains for a given connection."]
    pub struct ListPeeredDnsDomainsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "peeredDnsDomains")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of peered DNS domains."]
        pub peered_dns_domains:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PeeredDnsDomain>>>,
    }
    impl ListPeeredDnsDomainsResponse {
        pub fn builder() -> ListPeeredDnsDomainsResponseBuilder {
            ListPeeredDnsDomainsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A description of a log type. Example in YAML format: - name: library.googleapis.com/activity_history description: The history of borrowing and returning library items. display_name: Activity labels: - key: /customer_id description: Identifier of a library customer"]
    pub struct LogDescriptor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A human-readable description of this log. This information appears in the documentation and can contain details."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The human-readable name for this log. This information appears on the user interface and should be concise."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of labels that are available to describe a specific log entry. Runtime requests that contain labels not specified here are considered invalid."]
        pub labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LabelDescriptor>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the log. It must be less than 512 characters long and can include the following characters: upper- and lower-case alphanumeric characters [A-Za-z0-9], and punctuation characters including slash, underscore, hyphen, period [/_-.]."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl LogDescriptor {
        pub fn builder() -> LogDescriptorBuilder {
            LogDescriptorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Logging configuration of the service. The following example shows how to configure logs to be sent to the producer and consumer projects. In the example, the `activity_history` log is sent to both the producer and consumer projects, whereas the `purchase_history` log is only sent to the producer project. monitored_resources: - type: library.googleapis.com/branch labels: - key: /city description: The city where the library branch is located in. - key: /name description: The name of the branch. logs: - name: activity_history labels: - key: /customer_id - name: purchase_history logging: producer_destinations: - monitored_resource: library.googleapis.com/branch logs: - activity_history - purchase_history consumer_destinations: - monitored_resource: library.googleapis.com/branch logs: - activity_history"]
    pub struct Logging {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerDestinations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Logging configurations for sending logs to the consumer project. There can be multiple consumer destinations, each one must have a different monitored resource type. A log can be used in at most one consumer destination."]
        pub consumer_destinations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LoggingDestination>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "producerDestinations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Logging configurations for sending logs to the producer project. There can be multiple producer destinations, each one must have a different monitored resource type. A log can be used in at most one producer destination."]
        pub producer_destinations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LoggingDestination>>>,
    }
    impl Logging {
        pub fn builder() -> LoggingBuilder {
            LoggingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration of a specific logging destination (the producer project or the consumer project)."]
    pub struct LoggingDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Names of the logs to be sent to this destination. Each name must be defined in the Service.logs section. If the log name is not a domain scoped name, it will be automatically prefixed with the service name followed by \"/\"."]
        pub logs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monitoredResource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The monitored resource type. The type must be defined in the Service.monitored_resources section."]
        pub monitored_resource: ::std::option::Option<::std::string::String>,
    }
    impl LoggingDestination {
        pub fn builder() -> LoggingDestinationBuilder {
            LoggingDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Method represents a method of an API interface."]
    pub struct Method {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The simple name of this method."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "options")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Any metadata attached to the method."]
        pub options: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Option>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestStreaming")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, the request is streamed."]
        pub request_streaming: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestTypeUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL of the input message type."]
        pub request_type_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseStreaming")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, the response is streamed."]
        pub response_streaming: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseTypeUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the output message type."]
        pub response_type_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "syntax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source syntax of this method."]
        pub syntax: ::std::option::Option<MethodSyntaxEnum>,
    }
    impl Method {
        pub fn builder() -> MethodBuilder {
            MethodBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The source syntax of this method."]
    pub enum MethodSyntaxEnum {
        #[serde(rename = "SYNTAX_PROTO2")]
        #[doc = "Syntax `proto2`."]
        SyntaxProto2,
        #[serde(rename = "SYNTAX_PROTO3")]
        #[doc = "Syntax `proto3`."]
        SyntaxProto3,
    }
    impl ::std::default::Default for MethodSyntaxEnum {
        fn default() -> Self {
            Self::SyntaxProto2
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines a metric type and its schema. Once a metric descriptor is created, deleting or altering it stops data collection and makes the metric type's existing data unusable. "]
    pub struct MetricDescriptor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A detailed description of the metric, which can be used in documentation."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A concise name for the metric, which can be displayed in user interfaces. Use sentence case without an ending period, for example \"Request count\". This field is optional but it is recommended to be set for any metrics associated with user-visible concepts, such as Quota."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of labels that can be used to describe a specific instance of this metric type. For example, the `appengine.googleapis.com/http/server/response_latencies` metric type has a label for the HTTP response code, `response_code`, so you can look at latencies for successful responses or just for responses that failed."]
        pub labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LabelDescriptor>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "launchStage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The launch stage of the metric definition."]
        pub launch_stage: ::std::option::Option<MetricDescriptorLaunchStageEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Metadata which can be used to guide usage of the metric."]
        pub metadata: ::std::option::Option<::std::boxed::Box<MetricDescriptorMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricKind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the metric records instantaneous values, changes to a value, etc. Some combinations of `metric_kind` and `value_type` might not be supported."]
        pub metric_kind: ::std::option::Option<MetricDescriptorMetricKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monitoredResourceTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Read-only. If present, then a time series, which is identified partially by a metric type and a MonitoredResourceDescriptor, that is associated with this metric type can only be associated with one of the monitored resource types listed here."]
        pub monitored_resource_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the metric descriptor."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metric type, including its DNS name prefix. The type is not URL-encoded. All user-defined metric types have the DNS name `custom.googleapis.com` or `external.googleapis.com`. Metric types should use a natural hierarchical grouping. For example: \"custom.googleapis.com/invoice/paid/amount\" \"external.googleapis.com/prometheus/up\" \"appengine.googleapis.com/http/server/response_latencies\""]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The units in which the metric value is reported. It is only applicable if the `value_type` is `INT64`, `DOUBLE`, or `DISTRIBUTION`. The `unit` defines the representation of the stored metric values. Different systems might scale the values to be more easily displayed (so a value of `0.02kBy` _might_ be displayed as `20By`, and a value of `3523kBy` _might_ be displayed as `3.5MBy`). However, if the `unit` is `kBy`, then the value of the metric is always in thousands of bytes, no matter how it might be displayed. If you want a custom metric to record the exact number of CPU-seconds used by a job, you can create an `INT64 CUMULATIVE` metric whose `unit` is `s{CPU}` (or equivalently `1s{CPU}` or just `s`). If the job uses 12,005 CPU-seconds, then the value is written as `12005`. Alternatively, if you want a custom metric to record data in a more granular way, you can create a `DOUBLE CUMULATIVE` metric whose `unit` is `ks{CPU}`, and then write the value `12.005` (which is `12005/1000`), or use `Kis{CPU}` and write `11.723` (which is `12005/1024`). The supported units are a subset of [The Unified Code for Units of Measure](https://unitsofmeasure.org/ucum.html) standard: **Basic units (UNIT)** * `bit` bit * `By` byte * `s` second * `min` minute * `h` hour * `d` day * `1` dimensionless **Prefixes (PREFIX)** * `k` kilo (10^3) * `M` mega (10^6) * `G` giga (10^9) * `T` tera (10^12) * `P` peta (10^15) * `E` exa (10^18) * `Z` zetta (10^21) * `Y` yotta (10^24) * `m` milli (10^-3) * `u` micro (10^-6) * `n` nano (10^-9) * `p` pico (10^-12) * `f` femto (10^-15) * `a` atto (10^-18) * `z` zepto (10^-21) * `y` yocto (10^-24) * `Ki` kibi (2^10) * `Mi` mebi (2^20) * `Gi` gibi (2^30) * `Ti` tebi (2^40) * `Pi` pebi (2^50) **Grammar** The grammar also includes these connectors: * `/` division or ratio (as an infix operator). For examples, `kBy/{email}` or `MiBy/10ms` (although you should almost never have `/s` in a metric `unit`; rates should always be computed at query time from the underlying cumulative or delta value). * `.` multiplication or composition (as an infix operator). For examples, `GBy.d` or `k{watt}.h`. The grammar for a unit is as follows: Expression = Component { \".\" Component } { \"/\" Component } ; Component = ( [ PREFIX ] UNIT | \"%\" ) [ Annotation ] | Annotation | \"1\" ; Annotation = \"{\" NAME \"}\" ; Notes: * `Annotation` is just a comment if it follows a `UNIT`. If the annotation is used alone, then the unit is equivalent to `1`. For examples, `{request}/s == 1/s`, `By{transmitted}/s == By/s`. * `NAME` is a sequence of non-blank printable ASCII characters not containing `{` or `}`. * `1` represents a unitary [dimensionless unit](https://en.wikipedia.org/wiki/Dimensionless_quantity) of 1, such as in `1/s`. It is typically used when none of the basic units are appropriate. For example, \"new users per day\" can be represented as `1/d` or `{new-users}/d` (and a metric value `5` would mean \"5 new users). Alternatively, \"thousands of page views per day\" would be represented as `1000/d` or `k1/d` or `k{page_views}/d` (and a metric value of `5.3` would mean \"5300 page views per day\"). * `%` represents dimensionless value of 1/100, and annotates values giving a percentage (so the metric values are typically in the range of 0..100, and a metric value `3` means \"3 percent\"). * `10^2.%` indicates a metric contains a ratio, typically in the range 0..1, that will be multiplied by 100 and displayed as a percentage (so a metric value `0.03` means \"3 percent\")."]
        pub unit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the measurement is an integer, a floating-point number, etc. Some combinations of `metric_kind` and `value_type` might not be supported."]
        pub value_type: ::std::option::Option<MetricDescriptorValueTypeEnum>,
    }
    impl MetricDescriptor {
        pub fn builder() -> MetricDescriptorBuilder {
            MetricDescriptorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The launch stage of the metric definition."]
    pub enum MetricDescriptorLaunchStageEnum {
        #[serde(rename = "LAUNCH_STAGE_UNSPECIFIED")]
        #[doc = "Do not use this default value."]
        LaunchStageUnspecified,
        #[serde(rename = "UNIMPLEMENTED")]
        #[doc = "The feature is not yet implemented. Users can not use it."]
        Unimplemented,
        #[serde(rename = "PRELAUNCH")]
        #[doc = "Prelaunch features are hidden from users and are only visible internally."]
        Prelaunch,
        #[serde(rename = "EARLY_ACCESS")]
        #[doc = "Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released."]
        EarlyAccess,
        #[serde(rename = "ALPHA")]
        #[doc = "Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don’t have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases."]
        Alpha,
        #[serde(rename = "BETA")]
        #[doc = "Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases."]
        Beta,
        #[serde(rename = "GA")]
        #[doc = "GA features are open to all developers and are considered stable and fully qualified for production use."]
        Ga,
        #[serde(rename = "DEPRECATED")]
        #[doc = "Deprecated features are scheduled to be shut down and removed. For more information, see the “Deprecation Policy” section of our [Terms of Service](https://cloud.google.com/terms/) and the [Google Cloud Platform Subject to the Deprecation Policy](https://cloud.google.com/terms/deprecation) documentation."]
        Deprecated,
    }
    impl ::std::default::Default for MetricDescriptorLaunchStageEnum {
        fn default() -> Self {
            Self::LaunchStageUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Whether the metric records instantaneous values, changes to a value, etc. Some combinations of `metric_kind` and `value_type` might not be supported."]
    pub enum MetricDescriptorMetricKindEnum {
        #[serde(rename = "METRIC_KIND_UNSPECIFIED")]
        #[doc = "Do not use this default value."]
        MetricKindUnspecified,
        #[serde(rename = "GAUGE")]
        #[doc = "An instantaneous measurement of a value."]
        Gauge,
        #[serde(rename = "DELTA")]
        #[doc = "The change in a value during a time interval."]
        Delta,
        #[serde(rename = "CUMULATIVE")]
        #[doc = "A value accumulated over a time interval. Cumulative measurements in a time series should have the same start time and increasing end times, until an event resets the cumulative value to zero and sets a new start time for the following points."]
        Cumulative,
    }
    impl ::std::default::Default for MetricDescriptorMetricKindEnum {
        fn default() -> Self {
            Self::MetricKindUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Whether the measurement is an integer, a floating-point number, etc. Some combinations of `metric_kind` and `value_type` might not be supported."]
    pub enum MetricDescriptorValueTypeEnum {
        #[serde(rename = "VALUE_TYPE_UNSPECIFIED")]
        #[doc = "Do not use this default value."]
        ValueTypeUnspecified,
        #[serde(rename = "BOOL")]
        #[doc = "The value is a boolean. This value type can be used only if the metric kind is `GAUGE`."]
        Bool,
        #[serde(rename = "INT64")]
        #[doc = "The value is a signed 64-bit integer."]
        Int64,
        #[serde(rename = "DOUBLE")]
        #[doc = "The value is a double precision floating point number."]
        Double,
        #[serde(rename = "STRING")]
        #[doc = "The value is a text string. This value type can be used only if the metric kind is `GAUGE`."]
        String,
        #[serde(rename = "DISTRIBUTION")]
        #[doc = "The value is a `Distribution`."]
        Distribution,
        #[serde(rename = "MONEY")]
        #[doc = "The value is money."]
        Money,
    }
    impl ::std::default::Default for MetricDescriptorValueTypeEnum {
        fn default() -> Self {
            Self::ValueTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional annotations that can be used to guide the usage of a metric."]
    pub struct MetricDescriptorMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ingestDelay")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The delay of data points caused by ingestion. Data points older than this age are guaranteed to be ingested and available to be read, excluding data loss due to errors."]
        pub ingest_delay: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "launchStage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Must use the MetricDescriptor.launch_stage instead."]
        pub launch_stage: ::std::option::Option<MetricDescriptorMetadataLaunchStageEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "samplePeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sampling period of metric data points. For metrics which are written periodically, consecutive data points are stored at this time interval, excluding data loss due to errors. Metrics with a higher granularity have a smaller sampling period."]
        pub sample_period: ::std::option::Option<::std::string::String>,
    }
    impl MetricDescriptorMetadata {
        pub fn builder() -> MetricDescriptorMetadataBuilder {
            MetricDescriptorMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Deprecated. Must use the MetricDescriptor.launch_stage instead."]
    pub enum MetricDescriptorMetadataLaunchStageEnum {
        #[serde(rename = "LAUNCH_STAGE_UNSPECIFIED")]
        #[doc = "Do not use this default value."]
        LaunchStageUnspecified,
        #[serde(rename = "UNIMPLEMENTED")]
        #[doc = "The feature is not yet implemented. Users can not use it."]
        Unimplemented,
        #[serde(rename = "PRELAUNCH")]
        #[doc = "Prelaunch features are hidden from users and are only visible internally."]
        Prelaunch,
        #[serde(rename = "EARLY_ACCESS")]
        #[doc = "Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released."]
        EarlyAccess,
        #[serde(rename = "ALPHA")]
        #[doc = "Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don’t have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases."]
        Alpha,
        #[serde(rename = "BETA")]
        #[doc = "Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases."]
        Beta,
        #[serde(rename = "GA")]
        #[doc = "GA features are open to all developers and are considered stable and fully qualified for production use."]
        Ga,
        #[serde(rename = "DEPRECATED")]
        #[doc = "Deprecated features are scheduled to be shut down and removed. For more information, see the “Deprecation Policy” section of our [Terms of Service](https://cloud.google.com/terms/) and the [Google Cloud Platform Subject to the Deprecation Policy](https://cloud.google.com/terms/deprecation) documentation."]
        Deprecated,
    }
    impl ::std::default::Default for MetricDescriptorMetadataLaunchStageEnum {
        fn default() -> Self {
            Self::LaunchStageUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Bind API methods to metrics. Binding a method to a metric causes that metric's configured quota behaviors to apply to the method call."]
    pub struct MetricRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricCosts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metrics to update when the selected methods are called, and the associated cost applied to each metric. The key of the map is the metric name, and the values are the amount increased for the metric against which the quota limits are defined. The value must not be negative."]
        pub metric_costs:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selector")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Selects the methods to which this rule applies. Refer to selector for syntax details."]
        pub selector: ::std::option::Option<::std::string::String>,
    }
    impl MetricRule {
        pub fn builder() -> MetricRuleBuilder {
            MetricRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Declares an API Interface to be included in this interface. The including interface must redeclare all the methods from the included interface, but documentation and options are inherited as follows: - If after comment and whitespace stripping, the documentation string of the redeclared method is empty, it will be inherited from the original method. - Each annotation belonging to the service config (http, visibility) which is not set in the redeclared method will be inherited. - If an http annotation is inherited, the path pattern will be modified as follows. Any version prefix will be replaced by the version of the including interface plus the root path if specified. Example of a simple mixin: package google.acl.v1; service AccessControl { // Get the underlying ACL object. rpc GetAcl(GetAclRequest) returns (Acl) { option (google.api.http).get = \"/v1/{resource=**}:getAcl\"; } } package google.storage.v2; service Storage { // rpc GetAcl(GetAclRequest) returns (Acl); // Get a data record. rpc GetData(GetDataRequest) returns (Data) { option (google.api.http).get = \"/v2/{resource=**}\"; } } Example of a mixin configuration: apis: - name: google.storage.v2.Storage mixins: - name: google.acl.v1.AccessControl The mixin construct implies that all methods in `AccessControl` are also declared with same name and request/response types in `Storage`. A documentation generator or annotation processor will see the effective `Storage.GetAcl` method after inheriting documentation and annotations as follows: service Storage { // Get the underlying ACL object. rpc GetAcl(GetAclRequest) returns (Acl) { option (google.api.http).get = \"/v2/{resource=**}:getAcl\"; } ... } Note how the version in the path pattern changed from `v1` to `v2`. If the `root` field in the mixin is specified, it should be a relative path under which inherited HTTP paths are placed. Example: apis: - name: google.storage.v2.Storage mixins: - name: google.acl.v1.AccessControl root: acls This implies the following inherited HTTP annotation: service Storage { // Get the underlying ACL object. rpc GetAcl(GetAclRequest) returns (Acl) { option (google.api.http).get = \"/v2/acls/{resource=**}:getAcl\"; } ... }"]
    pub struct Mixin {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fully qualified name of the interface which is included."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "root")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If non-empty specifies a path under which inherited HTTP paths are rooted."]
        pub root: ::std::option::Option<::std::string::String>,
    }
    impl Mixin {
        pub fn builder() -> MixinBuilder {
            MixinBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object that describes the schema of a MonitoredResource object using a type name and a set of labels. For example, the monitored resource descriptor for Google Compute Engine VM instances has a type of `\"gce_instance\"` and specifies the use of the labels `\"instance_id\"` and `\"zone\"` to identify particular VM instances. Different APIs can support different monitored resource types. APIs generally provide a `list` method that returns the monitored resource descriptors used by the API. "]
    pub struct MonitoredResourceDescriptor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A detailed description of the monitored resource type that might be used in documentation."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A concise name for the monitored resource type that might be displayed in user interfaces. It should be a Title Cased Noun Phrase, without any article or other determiners. For example, `\"Google Cloud SQL Database\"`."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A set of labels used to describe instances of this monitored resource type. For example, an individual Google Cloud SQL database is identified by values for the labels `\"database_id\"` and `\"zone\"`."]
        pub labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LabelDescriptor>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "launchStage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The launch stage of the monitored resource definition."]
        pub launch_stage: ::std::option::Option<MonitoredResourceDescriptorLaunchStageEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The resource name of the monitored resource descriptor: `\"projects/{project_id}/monitoredResourceDescriptors/{type}\"` where {type} is the value of the `type` field in this object and {project_id} is a project ID that provides API-specific context for accessing the type. APIs that do not use project information can use the resource name format `\"monitoredResourceDescriptors/{type}\"`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The monitored resource type. For example, the type `\"cloudsql_database\"` represents databases in Google Cloud SQL."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl MonitoredResourceDescriptor {
        pub fn builder() -> MonitoredResourceDescriptorBuilder {
            MonitoredResourceDescriptorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The launch stage of the monitored resource definition."]
    pub enum MonitoredResourceDescriptorLaunchStageEnum {
        #[serde(rename = "LAUNCH_STAGE_UNSPECIFIED")]
        #[doc = "Do not use this default value."]
        LaunchStageUnspecified,
        #[serde(rename = "UNIMPLEMENTED")]
        #[doc = "The feature is not yet implemented. Users can not use it."]
        Unimplemented,
        #[serde(rename = "PRELAUNCH")]
        #[doc = "Prelaunch features are hidden from users and are only visible internally."]
        Prelaunch,
        #[serde(rename = "EARLY_ACCESS")]
        #[doc = "Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released."]
        EarlyAccess,
        #[serde(rename = "ALPHA")]
        #[doc = "Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don’t have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases."]
        Alpha,
        #[serde(rename = "BETA")]
        #[doc = "Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases."]
        Beta,
        #[serde(rename = "GA")]
        #[doc = "GA features are open to all developers and are considered stable and fully qualified for production use."]
        Ga,
        #[serde(rename = "DEPRECATED")]
        #[doc = "Deprecated features are scheduled to be shut down and removed. For more information, see the “Deprecation Policy” section of our [Terms of Service](https://cloud.google.com/terms/) and the [Google Cloud Platform Subject to the Deprecation Policy](https://cloud.google.com/terms/deprecation) documentation."]
        Deprecated,
    }
    impl ::std::default::Default for MonitoredResourceDescriptorLaunchStageEnum {
        fn default() -> Self {
            Self::LaunchStageUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Monitoring configuration of the service. The example below shows how to configure monitored resources and metrics for monitoring. In the example, a monitored resource and two metrics are defined. The `library.googleapis.com/book/returned_count` metric is sent to both producer and consumer projects, whereas the `library.googleapis.com/book/num_overdue` metric is only sent to the consumer project. monitored_resources: - type: library.googleapis.com/Branch display_name: \"Library Branch\" description: \"A branch of a library.\" launch_stage: GA labels: - key: resource_container description: \"The Cloud container (ie. project id) for the Branch.\" - key: location description: \"The location of the library branch.\" - key: branch_id description: \"The id of the branch.\" metrics: - name: library.googleapis.com/book/returned_count display_name: \"Books Returned\" description: \"The count of books that have been returned.\" launch_stage: GA metric_kind: DELTA value_type: INT64 unit: \"1\" labels: - key: customer_id description: \"The id of the customer.\" - name: library.googleapis.com/book/num_overdue display_name: \"Books Overdue\" description: \"The current number of overdue books.\" launch_stage: GA metric_kind: GAUGE value_type: INT64 unit: \"1\" labels: - key: customer_id description: \"The id of the customer.\" monitoring: producer_destinations: - monitored_resource: library.googleapis.com/Branch metrics: - library.googleapis.com/book/returned_count consumer_destinations: - monitored_resource: library.googleapis.com/Branch metrics: - library.googleapis.com/book/returned_count - library.googleapis.com/book/num_overdue"]
    pub struct Monitoring {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerDestinations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Monitoring configurations for sending metrics to the consumer project. There can be multiple consumer destinations. A monitored resource type may appear in multiple monitoring destinations if different aggregations are needed for different sets of metrics associated with that monitored resource type. A monitored resource and metric pair may only be used once in the Monitoring configuration."]
        pub consumer_destinations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MonitoringDestination>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "producerDestinations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Monitoring configurations for sending metrics to the producer project. There can be multiple producer destinations. A monitored resource type may appear in multiple monitoring destinations if different aggregations are needed for different sets of metrics associated with that monitored resource type. A monitored resource and metric pair may only be used once in the Monitoring configuration."]
        pub producer_destinations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MonitoringDestination>>>,
    }
    impl Monitoring {
        pub fn builder() -> MonitoringBuilder {
            MonitoringBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration of a specific monitoring destination (the producer project or the consumer project)."]
    pub struct MonitoringDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Types of the metrics to report to this monitoring destination. Each type must be defined in Service.metrics section."]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monitoredResource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The monitored resource type. The type must be defined in Service.monitored_resources section."]
        pub monitored_resource: ::std::option::Option<::std::string::String>,
    }
    impl MonitoringDestination {
        pub fn builder() -> MonitoringDestinationBuilder {
            MonitoringDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "OAuth scopes are a way to define data and permissions on data. For example, there are scopes defined for \"Read-only access to Google Calendar\" and \"Access to Cloud Platform\". Users can consent to a scope for an application, giving it permission to access that data on their behalf. OAuth scope specifications should be fairly coarse grained; a user will need to see and understand the text description of what your scope means. In most cases: use one or at most two OAuth scopes for an entire family of products. If your product has multiple APIs, you should probably be sharing the OAuth scope across all of those APIs. When you need finer grained OAuth consent screens: talk with your product management about how developers will use them in practice. Please note that even though each of the canonical scopes is enough for a request to be accepted and passed to the backend, a request can still fail due to the backend requiring additional scopes or permissions."]
    pub struct OAuthRequirements {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canonicalScopes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of publicly documented OAuth scopes that are allowed access. An OAuth token containing any of these scopes will be accepted. Example: canonical_scopes: https://www.googleapis.com/auth/calendar, https://www.googleapis.com/auth/calendar.read"]
        pub canonical_scopes: ::std::option::Option<::std::string::String>,
    }
    impl OAuthRequirements {
        pub fn builder() -> OAuthRequirementsBuilder {
            OAuthRequirementsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This resource represents a long-running operation that is the result of a network API call."]
    pub struct Operation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "done")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
        pub done: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error result of the operation in case of failure or cancellation."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "response")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl Operation {
        pub fn builder() -> OperationBuilder {
            OperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A protocol buffer option, which can be attached to a message, field, enumeration, etc."]
    pub struct Option {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The option's name. For protobuf built-in options (options defined in descriptor.proto), this is the short name. For example, `\"map_entry\"`. For custom options, it should be the fully-qualified name. For example, `\"google.api.http\"`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The option's value packed in an Any message. If the value is a primitive, the corresponding wrapper type defined in google/protobuf/wrappers.proto should be used. If the value is an enum, it should be stored as an int32 value using the google.protobuf.Int32Value type."]
        pub value: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl Option {
        pub fn builder() -> OptionBuilder {
            OptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a documentation page. A page can contain subpages to represent nested documentation set structure."]
    pub struct Page {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Markdown content of the page. You can use (== include {path} ==) to include content from a Markdown file."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the page. It will be used as an identity of the page to generate URI of the page, text of the link to this page in navigation, etc. The full page name (start from the root page name to this page concatenated with `.`) can be used as reference to the page in your documentation. For example: pages: - name: Tutorial content: (== include tutorial.md ==) subpages: - name: Java content: (== include tutorial_java.md ==) You can reference `Java` page using Markdown reference link syntax: `Java`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subpages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subpages of this page. The order of subpages specified here will be honored in the generated docset."]
        pub subpages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Page>>>,
    }
    impl Page {
        pub fn builder() -> PageBuilder {
            PageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "DNS domain suffix for which requests originating in the producer VPC network are resolved in the associated consumer VPC network."]
    pub struct PeeredDnsDomain {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dnsSuffix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The DNS domain name suffix e.g. `example.com.`."]
        pub dns_suffix: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User assigned name for this resource. Must be unique within the consumer network. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl PeeredDnsDomain {
        pub fn builder() -> PeeredDnsDomainBuilder {
            PeeredDnsDomainBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata provided through GetOperation request for the LRO generated by CreatePeeredDnsDomain API."]
    pub struct PeeredDnsDomainMetadata {}
    impl PeeredDnsDomainMetadata {
        pub fn builder() -> PeeredDnsDomainMetadataBuilder {
            PeeredDnsDomainMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Grouping of IAM role and IAM member."]
    pub struct PolicyBinding {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "member")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Member to bind the role with. See /iam/docs/reference/rest/v1/Policy#Binding for how to format each member. Eg. - user:myuser@mydomain.com - serviceAccount:my-service-account@app.gserviceaccount.com"]
        pub member: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Role to apply. Only allowlisted roles can be used at the specified granularity. The role must be one of the following: - 'roles/container.hostServiceAgentUser' applied on the shared VPC host project - 'roles/compute.securityAdmin' applied on the shared VPC host project"]
        pub role: ::std::option::Option<::std::string::String>,
    }
    impl PolicyBinding {
        pub fn builder() -> PolicyBindingBuilder {
            PolicyBindingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Quota configuration helps to achieve fairness and budgeting in service usage. The metric based quota configuration works this way: - The service configuration defines a set of metrics. - For API calls, the quota.metric_rules maps methods to metrics with corresponding costs. - The quota.limits defines limits on the metrics, which will be used for quota checks at runtime. An example quota configuration in yaml format: quota: limits: - name: apiWriteQpsPerProject metric: library.googleapis.com/write_calls unit: \"1/min/{project}\" # rate limit for consumer projects values: STANDARD: 10000 # The metric rules bind all methods to the read_calls metric, # except for the UpdateBook and DeleteBook methods. These two methods # are mapped to the write_calls metric, with the UpdateBook method # consuming at twice rate as the DeleteBook method. metric_rules: - selector: \"*\" metric_costs: library.googleapis.com/read_calls: 1 - selector: google.example.library.v1.LibraryService.UpdateBook metric_costs: library.googleapis.com/write_calls: 2 - selector: google.example.library.v1.LibraryService.DeleteBook metric_costs: library.googleapis.com/write_calls: 1 Corresponding Metric definition: metrics: - name: library.googleapis.com/read_calls display_name: Read requests metric_kind: DELTA value_type: INT64 - name: library.googleapis.com/write_calls display_name: Write requests metric_kind: DELTA value_type: INT64 "]
    pub struct Quota {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "limits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of `QuotaLimit` definitions for the service."]
        pub limits: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<QuotaLimit>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricRules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of `MetricRule` definitions, each one mapping a selected method to one or more metrics."]
        pub metric_rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricRule>>>,
    }
    impl Quota {
        pub fn builder() -> QuotaBuilder {
            QuotaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`QuotaLimit` defines a specific limit that applies over a specified duration for a limit type. There can be at most one limit for a duration and limit type combination defined within a `QuotaGroup`."]
    pub struct QuotaLimit {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultLimit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Default number of tokens that can be consumed during the specified duration. This is the number of tokens assigned when a client application developer activates the service for his/her project. Specifying a value of 0 will block all requests. This can be used if you are provisioning quota to selected consumers and blocking others. Similarly, a value of -1 will indicate an unlimited quota. No other negative values are allowed. Used by group-based quotas only."]
        pub default_limit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. User-visible, extended description for this quota limit. Should be used only when more context is needed to understand this limit than provided by the limit's display name (see: `display_name`)."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-visible display name for this limit. Optional. If not set, the UI will provide a default display name based on the quota configuration. This field can be used to override the default display name generated from the configuration."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Duration of this limit in textual notation. Must be \"100s\" or \"1d\". Used by group-based quotas only."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "freeTier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Free tier value displayed in the Developers Console for this limit. The free tier is the number of tokens that will be subtracted from the billed amount when billing is enabled. This field can only be set on a limit with duration \"1d\", in a billable group; it is invalid on any other limit. If this field is not set, it defaults to 0, indicating that there is no free tier for this service. Used by group-based quotas only."]
        pub free_tier: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxLimit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum number of tokens that can be consumed during the specified duration. Client application developers can override the default limit up to this maximum. If specified, this value cannot be set to a value less than the default limit. If not specified, it is set to the default limit. To allow clients to apply overrides with no upper bound, set this to -1, indicating unlimited maximum quota. Used by group-based quotas only."]
        pub max_limit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metric")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the metric this quota limit applies to. The quota limits with the same metric will be checked together during runtime. The metric must be defined within the service config."]
        pub metric: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the quota limit. The name must be provided, and it must be unique within the service. The name can only include alphanumeric characters as well as '-'. The maximum length of the limit name is 64 characters."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specify the unit of the quota limit. It uses the same syntax as Metric.unit. The supported unit kinds are determined by the quota backend system. Here are some examples: * \"1/min/{project}\" for quota per minute per project. Note: the order of unit components is insignificant. The \"1\" at the beginning is required to follow the metric unit syntax."]
        pub unit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tiered limit values. You must specify this as a key:value pair, with an integer value that is the maximum number of requests allowed for the specified unit. Currently only STANDARD is supported."]
        pub values:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl QuotaLimit {
        pub fn builder() -> QuotaLimitBuilder {
            QuotaLimitBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a found unused range."]
    pub struct Range {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipCidrRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CIDR range in \"10.x.x.x/y\" format that is within the allocated ranges and currently unused."]
        pub ip_cidr_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "In the Shared VPC host project, the VPC network that's peered with the consumer network. For example: `projects/1234321/global/networks/host-network`"]
        pub network: ::std::option::Option<::std::string::String>,
    }
    impl Range {
        pub fn builder() -> RangeBuilder {
            RangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a range reservation."]
    pub struct RangeReservation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipPrefixLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The size of the desired subnet. Use usual CIDR range notation. For example, '30' to find unused x.x.x.x/30 CIDR range. The goal is to determine if one of the allocated ranges has enough free space for a subnet of the requested size."]
        pub ip_prefix_length: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The name of one or more allocated IP address ranges associated with this private service access connection. If no range names are provided all ranges associated with this connection will be considered. If a CIDR range with the specified IP prefix length is not available within these ranges the validation fails."]
        pub requested_ranges: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secondaryRangeIpPrefixLengths")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The size of the desired secondary ranges for the subnet. Use usual CIDR range notation. For example, '30' to find unused x.x.x.x/30 CIDR range. The goal is to determine that the allocated ranges have enough free space for all the requested secondary ranges."]
        pub secondary_range_ip_prefix_lengths:
            ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subnetworkCandidates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. List of subnetwork candidates to validate. The required input fields are `name`, `network`, and `region`. Subnetworks from this list which exist will be returned in the response with the `ip_cidr_range`, `secondary_ip_cider_ranges`, and `outside_allocation` fields set."]
        pub subnetwork_candidates:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Subnetwork>>>,
    }
    impl RangeReservation {
        pub fn builder() -> RangeReservationBuilder {
            RangeReservationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata provided through GetOperation request for the LRO generated by RemoveDnsRecordSet API"]
    pub struct RemoveDnsRecordSetMetadata {}
    impl RemoveDnsRecordSetMetadata {
        pub fn builder() -> RemoveDnsRecordSetMetadataBuilder {
            RemoveDnsRecordSetMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to remove a record set from a private managed DNS zone in the shared producer host project. The name, type, ttl, and data values must all exactly match an existing record set in the specified zone."]
    pub struct RemoveDnsRecordSetRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The network that the consumer is using to connect with services. Must be in the form of projects/{project}/global/networks/{network} {project} is the project number, as in '12345' {network} is the network name."]
        pub consumer_network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dnsRecordSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The DNS record set to remove."]
        pub dns_record_set: ::std::option::Option<::std::boxed::Box<DnsRecordSet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the private DNS zone in the shared producer host project from which the record set will be removed."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl RemoveDnsRecordSetRequest {
        pub fn builder() -> RemoveDnsRecordSetRequestBuilder {
            RemoveDnsRecordSetRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Blank message response type for RemoveDnsRecordSet API"]
    pub struct RemoveDnsRecordSetResponse {}
    impl RemoveDnsRecordSetResponse {
        pub fn builder() -> RemoveDnsRecordSetResponseBuilder {
            RemoveDnsRecordSetResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata provided through GetOperation request for the LRO generated by RemoveDnsZone API"]
    pub struct RemoveDnsZoneMetadata {}
    impl RemoveDnsZoneMetadata {
        pub fn builder() -> RemoveDnsZoneMetadataBuilder {
            RemoveDnsZoneMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to remove a private managed DNS zone in the shared producer host project and a matching DNS peering zone in the consumer project."]
    pub struct RemoveDnsZoneRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The network that the consumer is using to connect with services. Must be in the form of projects/{project}/global/networks/{network} {project} is the project number, as in '12345' {network} is the network name."]
        pub consumer_network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name for both the private zone in the shared producer host project and the peering zone in the consumer project."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl RemoveDnsZoneRequest {
        pub fn builder() -> RemoveDnsZoneRequestBuilder {
            RemoveDnsZoneRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Blank message response type for RemoveDnsZone API"]
    pub struct RemoveDnsZoneResponse {}
    impl RemoveDnsZoneResponse {
        pub fn builder() -> RemoveDnsZoneResponseBuilder {
            RemoveDnsZoneResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a route that was created or discovered by a private access management service."]
    pub struct Route {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Destination CIDR range that this route applies to."]
        pub dest_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Route name. See https://cloud.google.com/vpc/docs/routes"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully-qualified URL of the VPC network in the producer host tenant project that this route applies to. For example: `projects/123456/global/networks/host-network`"]
        pub network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextHopGateway")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully-qualified URL of the gateway that should handle matching packets that this route applies to. For example: `projects/123456/global/gateways/default-internet-gateway`"]
        pub next_hop_gateway: ::std::option::Option<::std::string::String>,
    }
    impl Route {
        pub fn builder() -> RouteBuilder {
            RouteBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to search for an unused range within allocated ranges."]
    pub struct SearchRangeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipPrefixLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The prefix length of the IP range. Use usual CIDR range notation. For example, '30' to find unused x.x.x.x/30 CIDR range. Actual range will be determined using allocated range for the consumer peered network and returned in the result."]
        pub ip_prefix_length: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Network name in the consumer project. This network must have been already peered with a shared VPC network using CreateConnection method. Must be in a form 'projects/{project}/global/networks/{network}'. {project} is a project number, as in '12345' {network} is network name."]
        pub network: ::std::option::Option<::std::string::String>,
    }
    impl SearchRangeRequest {
        pub fn builder() -> SearchRangeRequestBuilder {
            SearchRangeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SecondaryIpRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipCidrRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Secondary IP CIDR range in `x.x.x.x/y` format."]
        pub ip_cidr_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rangeName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the secondary IP range."]
        pub range_name: ::std::option::Option<::std::string::String>,
    }
    impl SecondaryIpRange {
        pub fn builder() -> SecondaryIpRangeBuilder {
            SecondaryIpRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SecondaryIpRangeSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipPrefixLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The prefix length of the secondary IP range. Use CIDR range notation, such as `30` to provision a secondary IP range with an `x.x.x.x/30` CIDR range. The IP address range is drawn from a pool of available ranges in the service consumer's allocated range."]
        pub ip_prefix_length: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rangeName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A name for the secondary IP range. The name must be 1-63 characters long, and comply with RFC1035. The name must be unique within the subnetwork."]
        pub range_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The starting address of a range. The address must be a valid IPv4 address in the x.x.x.x format. This value combined with the IP prefix range is the CIDR range for the secondary IP range. The range must be within the allocated range that is assigned to the private connection. If the CIDR range isn't available, the call fails."]
        pub requested_address: ::std::option::Option<::std::string::String>,
    }
    impl SecondaryIpRangeSpec {
        pub fn builder() -> SecondaryIpRangeSpecBuilder {
            SecondaryIpRangeSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`Service` is the root object of Google service configuration schema. It describes basic information about a service, such as the name and the title, and delegates other aspects to sub-sections. Each sub-section is either a proto message or a repeated proto message that configures a specific aspect, such as auth. See each proto message definition for details. Example: type: google.api.Service name: calendar.googleapis.com title: Google Calendar API apis: - name: google.calendar.v3.Calendar authentication: providers: - id: google_calendar_auth jwks_uri: https://www.googleapis.com/oauth2/v1/certs issuer: https://securetoken.google.com rules: - selector: \"*\" requirements: provider_id: google_calendar_auth"]
    pub struct Service {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of API interfaces exported by this service. Only the `name` field of the google.protobuf.Api needs to be provided by the configuration author, as the remaining fields will be derived from the IDL during the normalization process. It is an error to specify an API interface here which cannot be resolved against the associated IDL files."]
        pub apis: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Api>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authentication")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Auth configuration."]
        pub authentication: ::std::option::Option<::std::boxed::Box<Authentication>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backend")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "API backend configuration."]
        pub backend: ::std::option::Option<::std::boxed::Box<Backend>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "billing")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Billing configuration."]
        pub billing: ::std::option::Option<::std::boxed::Box<Billing>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "configVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The service config compiler always sets this field to `3`."]
        pub config_version: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Context configuration."]
        pub context: ::std::option::Option<::std::boxed::Box<Context>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "control")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for the service control plane."]
        pub control: ::std::option::Option<::std::boxed::Box<Control>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom error configuration."]
        pub custom_error: ::std::option::Option<::std::boxed::Box<CustomError>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "documentation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional API documentation."]
        pub documentation: ::std::option::Option<::std::boxed::Box<Documentation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endpoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for network endpoints. If this is empty, then an endpoint with the same name as the service is automatically generated to service all defined APIs."]
        pub endpoints: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Endpoint>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enums")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of all enum types included in this API service. Enums referenced directly or indirectly by the `apis` are automatically included. Enums which are not referenced but shall be included should be listed here by name. Example: enums: - name: google.someapi.v1.SomeEnum"]
        pub enums: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Enum>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "http")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP configuration."]
        pub http: ::std::option::Option<::std::boxed::Box<Http>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique ID for a specific instance of this message, typically assigned by the client for tracking purpose. Must be no longer than 63 characters and only lower case letters, digits, '.', '_' and '-' are allowed. If empty, the server may choose to generate one instead."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logging")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Logging configuration."]
        pub logging: ::std::option::Option<::std::boxed::Box<Logging>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the logs used by this service."]
        pub logs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LogDescriptor>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the metrics used by this service."]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricDescriptor>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monitoredResources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the monitored resources used by this service. This is required by the Service.monitoring and Service.logging configurations."]
        pub monitored_resources:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MonitoredResourceDescriptor>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monitoring")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Monitoring configuration."]
        pub monitoring: ::std::option::Option<::std::boxed::Box<Monitoring>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The service name, which is a DNS-like logical identifier for the service, such as `calendar.googleapis.com`. The service name typically goes through DNS verification to make sure the owner of the service also owns the DNS name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "producerProjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google project that owns this service."]
        pub producer_project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quota")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Quota configuration."]
        pub quota: ::std::option::Option<::std::boxed::Box<Quota>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The source information for this configuration if available."]
        pub source_info: ::std::option::Option<::std::boxed::Box<SourceInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System parameter configuration."]
        pub system_parameters: ::std::option::Option<::std::boxed::Box<SystemParameters>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of all proto message types included in this API service. It serves similar purpose as [google.api.Service.types], except that these types are not needed by user-defined APIs. Therefore, they will not show up in the generated discovery doc. This field should only be used to define system APIs in ESF."]
        pub system_types: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Type>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The product title for this service."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "types")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of all proto message types included in this API service. Types referenced directly or indirectly by the `apis` are automatically included. Messages which are not referenced but shall be included, such as types used by the `google.protobuf.Any` type, should be listed here by name. Example: types: - name: google.protobuf.Int32"]
        pub types: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Type>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "usage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration controlling usage of this service."]
        pub usage: ::std::option::Option<::std::boxed::Box<Usage>>,
    }
    impl Service {
        pub fn builder() -> ServiceBuilder {
            ServiceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`SourceContext` represents information about the source of a protobuf element, like the file in which it is defined."]
    pub struct SourceContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The path-qualified name of the .proto file that contained the associated protobuf element. For example: `\"google/protobuf/source_context.proto\"`."]
        pub file_name: ::std::option::Option<::std::string::String>,
    }
    impl SourceContext {
        pub fn builder() -> SourceContextBuilder {
            SourceContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Source information used to create a Service Config"]
    pub struct SourceInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceFiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All files used during config generation."]
        pub source_files: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
    }
    impl SourceInfo {
        pub fn builder() -> SourceInfoBuilder {
            SourceInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub struct Status {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        pub code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "details")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
        pub details: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl Status {
        pub fn builder() -> StatusBuilder {
            StatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a subnet that was created or discovered by a private access management service."]
    pub struct Subnetwork {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipCidrRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subnetwork CIDR range in `10.x.x.x/y` format."]
        pub ip_cidr_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subnetwork name. See https://cloud.google.com/compute/docs/vpc/"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "In the Shared VPC host project, the VPC network that's peered with the consumer network. For example: `projects/1234321/global/networks/host-network`"]
        pub network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outsideAllocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is a discovered subnet that is not within the current consumer allocated ranges."]
        pub outside_allocation: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GCP region where the subnetwork is located."]
        pub region: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secondaryIpRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of secondary IP ranges in this subnetwork."]
        pub secondary_ip_ranges:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SecondaryIpRange>>>,
    }
    impl Subnetwork {
        pub fn builder() -> SubnetworkBuilder {
            SubnetworkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Define a parameter's name and location. The parameter may be passed as either an HTTP header or a URL query parameter, and if both are passed the behavior is implementation-dependent."]
    pub struct SystemParameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpHeader")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Define the HTTP header name to use for the parameter. It is case insensitive."]
        pub http_header: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Define the name of the parameter, such as \"api_key\" . It is case sensitive."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "urlQueryParameter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Define the URL query parameter name to use for the parameter. It is case sensitive."]
        pub url_query_parameter: ::std::option::Option<::std::string::String>,
    }
    impl SystemParameter {
        pub fn builder() -> SystemParameterBuilder {
            SystemParameterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Define a system parameter rule mapping system parameter definitions to methods."]
    pub struct SystemParameterRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Define parameters. Multiple names may be defined for a parameter. For a given method call, only one of them should be used. If multiple names are used the behavior is implementation-dependent. If none of the specified names are present the behavior is parameter-dependent."]
        pub parameters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SystemParameter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selector")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Selects the methods to which this rule applies. Use '*' to indicate all methods in all APIs. Refer to selector for syntax details."]
        pub selector: ::std::option::Option<::std::string::String>,
    }
    impl SystemParameterRule {
        pub fn builder() -> SystemParameterRuleBuilder {
            SystemParameterRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "### System parameter configuration A system parameter is a special kind of parameter defined by the API system, not by an individual API. It is typically mapped to an HTTP header and/or a URL query parameter. This configuration specifies which methods change the names of the system parameters."]
    pub struct SystemParameters {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Define system parameters. The parameters defined here will override the default parameters implemented by the system. If this field is missing from the service config, default system parameters will be used. Default system parameters and names is implementation-dependent. Example: define api key for all methods system_parameters rules: - selector: \"*\" parameters: - name: api_key url_query_parameter: api_key Example: define 2 api key names for a specific method. system_parameters rules: - selector: \"/ListShelves\" parameters: - name: api_key http_header: Api-Key1 - name: api_key http_header: Api-Key2 **NOTE:** All service configuration rules follow \"last one wins\" order."]
        pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SystemParameterRule>>>,
    }
    impl SystemParameters {
        pub fn builder() -> SystemParametersBuilder {
            SystemParametersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A protocol buffer message type."]
    pub struct Type {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of fields."]
        pub fields: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Field>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fully qualified message name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oneofs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of types appearing in `oneof` definitions in this type."]
        pub oneofs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "options")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The protocol buffer options."]
        pub options: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Option>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source context."]
        pub source_context: ::std::option::Option<::std::boxed::Box<SourceContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "syntax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source syntax."]
        pub syntax: ::std::option::Option<TypeSyntaxEnum>,
    }
    impl Type {
        pub fn builder() -> TypeBuilder {
            TypeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The source syntax."]
    pub enum TypeSyntaxEnum {
        #[serde(rename = "SYNTAX_PROTO2")]
        #[doc = "Syntax `proto2`."]
        SyntaxProto2,
        #[serde(rename = "SYNTAX_PROTO3")]
        #[doc = "Syntax `proto3`."]
        SyntaxProto3,
    }
    impl ::std::default::Default for TypeSyntaxEnum {
        fn default() -> Self {
            Self::SyntaxProto2
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to update the configuration of a service networking connection including the import/export of custom routes and subnetwork routes with public IP."]
    pub struct UpdateConsumerConfigRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The updated peering config."]
        pub consumer_config: ::std::option::Option<::std::boxed::Box<ConsumerConfig>>,
    }
    impl UpdateConsumerConfigRequest {
        pub fn builder() -> UpdateConsumerConfigRequestBuilder {
            UpdateConsumerConfigRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata provided through GetOperation request for the LRO generated by UpdateDnsRecordSet API"]
    pub struct UpdateDnsRecordSetMetadata {}
    impl UpdateDnsRecordSetMetadata {
        pub fn builder() -> UpdateDnsRecordSetMetadataBuilder {
            UpdateDnsRecordSetMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to update a record set from a private managed DNS zone in the shared producer host project. The name, type, ttl, and data values of the existing record set must all exactly match an existing record set in the specified zone."]
    pub struct UpdateDnsRecordSetRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The network that the consumer is using to connect with services. Must be in the form of projects/{project}/global/networks/{network} {project} is the project number, as in '12345' {network} is the network name."]
        pub consumer_network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "existingDnsRecordSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The existing DNS record set to update."]
        pub existing_dns_record_set: ::std::option::Option<::std::boxed::Box<DnsRecordSet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newDnsRecordSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The new values that the DNS record set should be updated to hold."]
        pub new_dns_record_set: ::std::option::Option<::std::boxed::Box<DnsRecordSet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the private DNS zone in the shared producer host project from which the record set will be removed."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl UpdateDnsRecordSetRequest {
        pub fn builder() -> UpdateDnsRecordSetRequestBuilder {
            UpdateDnsRecordSetRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration controlling usage of a service."]
    pub struct Usage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "producerNotificationChannel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full resource name of a channel used for sending notifications to the service producer. Google Service Management currently only supports [Google Cloud Pub/Sub](https://cloud.google.com/pubsub) as a notification channel. To use Google Cloud Pub/Sub as the channel, this must be the name of a Cloud Pub/Sub topic that uses the Cloud Pub/Sub topic name format documented in https://cloud.google.com/pubsub/docs/overview."]
        pub producer_notification_channel: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requirements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Requirements that must be satisfied before a consumer project can use the service. Each requirement is of the form /; for example 'serviceusage.googleapis.com/billing-enabled'. For Google APIs, a Terms of Service requirement must be included here. Google Cloud APIs must include \"serviceusage.googleapis.com/tos/cloud\". Other Google APIs should include \"serviceusage.googleapis.com/tos/universal\". Additional ToS can be included based on the business needs."]
        pub requirements: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of usage rules that apply to individual API methods. **NOTE:** All service configuration rules follow \"last one wins\" order."]
        pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UsageRule>>>,
    }
    impl Usage {
        pub fn builder() -> UsageBuilder {
            UsageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Usage configuration rules for the service. NOTE: Under development. Use this rule to configure unregistered calls for the service. Unregistered calls are calls that do not contain consumer project identity. (Example: calls that do not contain an API key). By default, API methods do not allow unregistered calls, and each method call must be identified by a consumer project identity. Use this rule to allow/disallow unregistered calls. Example of an API that wants to allow unregistered calls for entire service. usage: rules: - selector: \"*\" allow_unregistered_calls: true Example of a method that wants to allow unregistered calls. usage: rules: - selector: \"google.example.library.v1.LibraryService.CreateBook\" allow_unregistered_calls: true"]
    pub struct UsageRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowUnregisteredCalls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, the selected method allows unregistered calls, e.g. calls that don't identify any user or application."]
        pub allow_unregistered_calls: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selector")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Selects the methods to which this rule applies. Use '*' to indicate all methods in all APIs. Refer to selector for syntax details."]
        pub selector: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skipServiceControl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, the selected method should skip service control and the control plane features, such as quota and billing, will not be available. This flag is used by Google Cloud Endpoints to bypass checks for internal methods, such as service health check methods."]
        pub skip_service_control: ::std::option::Option<::std::primitive::bool>,
    }
    impl UsageRule {
        pub fn builder() -> UsageRuleBuilder {
            UsageRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ValidateConsumerConfigRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The network that the consumer is using to connect with services. Must be in the form of projects/{project}/global/networks/{network} {project} is a project number, as in '12345' {network} is network name."]
        pub consumer_network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerProject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "NETWORK_NOT_IN_CONSUMERS_PROJECT, NETWORK_NOT_IN_CONSUMERS_HOST_PROJECT, and HOST_PROJECT_NOT_FOUND are done when consumer_project is provided."]
        pub consumer_project: ::std::option::Option<::std::boxed::Box<ConsumerProject>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rangeReservation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RANGES_EXHAUSTED, RANGES_EXHAUSTED, and RANGES_DELETED_LATER are done when range_reservation is provided."]
        pub range_reservation: ::std::option::Option<::std::boxed::Box<RangeReservation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validateNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The validations will be performed in the order listed in the ValidationError enum. The first failure will return. If a validation is not requested, then the next one will be performed. SERVICE_NETWORKING_NOT_ENABLED and NETWORK_NOT_PEERED checks are performed for all requests where validation is requested. NETWORK_NOT_FOUND and NETWORK_DISCONNECTED checks are done for requests that have validate_network set to true."]
        pub validate_network: ::std::option::Option<::std::primitive::bool>,
    }
    impl ValidateConsumerConfigRequest {
        pub fn builder() -> ValidateConsumerConfigRequestBuilder {
            ValidateConsumerConfigRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ValidateConsumerConfigResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "existingSubnetworkCandidates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of subnetwork candidates from the request which exist with the `ip_cidr_range`, `secondary_ip_cider_ranges`, and `outside_allocation` fields set."]
        pub existing_subnetwork_candidates:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Subnetwork>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isValid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether all the requested validations passed."]
        pub is_valid: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validationError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The first validation which failed."]
        pub validation_error:
            ::std::option::Option<ValidateConsumerConfigResponseValidationErrorEnum>,
    }
    impl ValidateConsumerConfigResponse {
        pub fn builder() -> ValidateConsumerConfigResponseBuilder {
            ValidateConsumerConfigResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The first validation which failed."]
    pub enum ValidateConsumerConfigResponseValidationErrorEnum {
        #[serde(rename = "VALIDATION_ERROR_UNSPECIFIED")]
        #[doc = ""]
        ValidationErrorUnspecified,
        #[serde(rename = "VALIDATION_NOT_REQUESTED")]
        #[doc = "In case none of the validations are requested."]
        ValidationNotRequested,
        #[serde(rename = "SERVICE_NETWORKING_NOT_ENABLED")]
        #[doc = ""]
        ServiceNetworkingNotEnabled,
        #[serde(rename = "NETWORK_NOT_FOUND")]
        #[doc = "The network provided by the consumer does not exist."]
        NetworkNotFound,
        #[serde(rename = "NETWORK_NOT_PEERED")]
        #[doc = "The network has not been peered with the producer org."]
        NetworkNotPeered,
        #[serde(rename = "NETWORK_PEERING_DELETED")]
        #[doc = "The peering was created and later deleted."]
        NetworkPeeringDeleted,
        #[serde(rename = "NETWORK_NOT_IN_CONSUMERS_PROJECT")]
        #[doc = "The network is a regular VPC but the network is not in the consumer's project."]
        NetworkNotInConsumersProject,
        #[serde(rename = "NETWORK_NOT_IN_CONSUMERS_HOST_PROJECT")]
        #[doc = "The consumer project is a service project, and network is a shared VPC, but the network is not in the host project of this consumer project."]
        NetworkNotInConsumersHostProject,
        #[serde(rename = "HOST_PROJECT_NOT_FOUND")]
        #[doc = "The host project associated with the consumer project was not found."]
        HostProjectNotFound,
        #[serde(rename = "CONSUMER_PROJECT_NOT_SERVICE_PROJECT")]
        #[doc = "The consumer project is not a service project for the specified host project."]
        ConsumerProjectNotServiceProject,
        #[serde(rename = "RANGES_EXHAUSTED")]
        #[doc = "The reserved IP ranges do not have enough space to create a subnet of desired size."]
        RangesExhausted,
        #[serde(rename = "RANGES_NOT_RESERVED")]
        #[doc = "The IP ranges were not reserved."]
        RangesNotReserved,
        #[serde(rename = "RANGES_DELETED_LATER")]
        #[doc = "The IP ranges were reserved but deleted later."]
        RangesDeletedLater,
        #[serde(rename = "COMPUTE_API_NOT_ENABLED")]
        #[doc = "The consumer project does not have the compute api enabled."]
        ComputeApiNotEnabled,
    }
    impl ::std::default::Default for ValidateConsumerConfigResponseValidationErrorEnum {
        fn default() -> Self {
            Self::ValidationErrorUnspecified
        }
    }
}
