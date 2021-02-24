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
    pub mod projects {
        pub mod resources {
            pub mod locations {
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "filter")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The standard list filter."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The standard list page size."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The standard list page token."]
                                    pub page_token: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                            pub mod wait {
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
                                    #[serde(rename = "timeout")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The maximum duration to wait before timing out. If left blank, the wait will be at most the time permitted by the underlying HTTP/RPC protocol. If RPC context deadline is also specified, the shorter one will be used."]
                                    pub timeout: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                    }
                    pub mod scans {
                        pub mod resources {
                            pub mod vulnerabilities {
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
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The number of vulnerabilities to retrieve."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The page token, resulting from a previous call to ListVulnerabilities."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
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
            }
        }
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An alias to a repo revision."]
    pub struct AliasContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The alias kind."]
        pub kind: ::std::option::Option<AliasContextKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The alias name."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl AliasContext {
        pub fn builder() -> AliasContextBuilder {
            AliasContextBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The alias kind."]
    pub enum AliasContextKindEnum {
        #[serde(rename = "KIND_UNSPECIFIED")]
        #[doc = "Unknown."]
        KindUnspecified,
        #[serde(rename = "FIXED")]
        #[doc = "Git tag."]
        Fixed,
        #[serde(rename = "MOVABLE")]
        #[doc = "Git branch."]
        Movable,
        #[serde(rename = "OTHER")]
        #[doc = "Used to specify non-standard aliases. For example, if a Git repo has a ref named \"refs/foo/bar\"."]
        Other,
    }
    impl ::std::default::Default for AliasContextKindEnum {
        fn default() -> Self {
            Self::KindUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "AnalyzePackagesMetadata contains metadata for an active scan of a container image."]
    pub struct AnalyzePackagesMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the scan was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource URI of the container image being scanned."]
        pub resource_uri: ::std::option::Option<::std::string::String>,
    }
    impl AnalyzePackagesMetadata {
        pub fn builder() -> AnalyzePackagesMetadataBuilder {
            AnalyzePackagesMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "AnalyzePackagesRequest is the request to analyze a list of packages and create Vulnerability Occurrences for it."]
    pub struct AnalyzePackagesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The packages to analyze."]
        pub packages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PackageData>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The resource URI of the container image being scanned."]
        pub resource_uri: ::std::option::Option<::std::string::String>,
    }
    impl AnalyzePackagesRequest {
        pub fn builder() -> AnalyzePackagesRequestBuilder {
            AnalyzePackagesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "AnalyzePackagesResponse contains the information necessary to find results for the given scan."]
    pub struct AnalyzePackagesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the scan resource created by this successful scan."]
        pub scan: ::std::option::Option<::std::string::String>,
    }
    impl AnalyzePackagesResponse {
        pub fn builder() -> AnalyzePackagesResponseBuilder {
            AnalyzePackagesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Artifact describes a build product."]
    pub struct Artifact {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "checksum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hash or checksum value of a binary, or Docker Registry 2.0 digest of a container."]
        pub checksum: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Artifact ID, if any; for container images, this will be a URL by digest like `gcr.io/projectID/imagename@sha256:123456`."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "names")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Related artifact names. This may be the path to a binary or jar file, or in the case of a container build, the name used to push the container image to Google Container Registry, as presented to `docker push`. Note that a single Artifact ID can have multiple names, for example if two tags are applied to one image."]
        pub names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Artifact {
        pub fn builder() -> ArtifactBuilder {
            ArtifactBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Occurrence that represents a single \"attestation\". The authenticity of an attestation can be verified using the attached signature. If the verifier trusts the public key of the signer, then verifying the signature is sufficient to establish trust. In this circumstance, the authority to which this attestation is attached is primarily useful for lookup (how to find this attestation if you already know the authority and artifact to be verified) and intent (for which authority this attestation was intended to sign."]
    pub struct AttestationOccurrence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jwts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One or more JWTs encoding a self-contained attestation. Each JWT encodes the payload that it verifies within the JWT itself. Verifier implementation SHOULD ignore the `serialized_payload` field when verifying these JWTs. If only JWTs are present on this AttestationOccurrence, then the `serialized_payload` SHOULD be left empty. Each JWT SHOULD encode a claim specific to the `resource_uri` of this Occurrence, but this is not validated by Grafeas metadata API implementations. The JWT itself is opaque to Grafeas."]
        pub jwts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Jwt>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serializedPayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The serialized payload that is verified by one or more `signatures`."]
        pub serialized_payload: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One or more signatures over `serialized_payload`. Verifier implementations should consider this attestation message verified if at least one `signature` verifies `serialized_payload`. See `Signature` in common.proto for more details on signature structure and verification."]
        pub signatures: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Signature>>>,
    }
    impl AttestationOccurrence {
        pub fn builder() -> AttestationOccurrenceBuilder {
            AttestationOccurrenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a build occurrence."]
    pub struct BuildOccurrence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The actual provenance for the build."]
        pub provenance: ::std::option::Option<::std::boxed::Box<BuildProvenance>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenanceBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized JSON representation of the provenance, used in generating the build signature in the corresponding build note. After verifying the signature, `provenance_bytes` can be unmarshalled and compared to the provenance to confirm that it is unchanged. A base64-encoded string representation of the provenance bytes is used for the signature in order to interoperate with openssl which expects this format for signature verification. The serialized form is captured both to avoid ambiguity in how the provenance is marshalled to json as well to prevent incompatibilities with future changes."]
        pub provenance_bytes: ::std::option::Option<::std::string::String>,
    }
    impl BuildOccurrence {
        pub fn builder() -> BuildOccurrenceBuilder {
            BuildOccurrenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provenance of a build. Contains all information needed to verify the full details about the build from source to completion."]
    pub struct BuildProvenance {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Special options applied to this build. This is a catch-all field where build providers can enter any desired additional details."]
        pub build_options:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "builderVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Version string of the builder at the time this build was executed."]
        pub builder_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "builtArtifacts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output of the build."]
        pub built_artifacts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Artifact>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commands")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Commands requested by the build."]
        pub commands: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Command>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time at which the build was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "E-mail address of the user who initiated this build. Note that this was the user's e-mail address at the time the build was initiated; this address may not represent the same end-user for all time."]
        pub creator: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time at which execution of the build was finished."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Unique identifier of the build."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logsUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URI where any logs for this provenance were written."]
        pub logs_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the project."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceProvenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of the Source input to the build."]
        pub source_provenance: ::std::option::Option<::std::boxed::Box<Source>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time at which execution of the build was started."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Trigger identifier if the build was triggered automatically; empty if not."]
        pub trigger_id: ::std::option::Option<::std::string::String>,
    }
    impl BuildProvenance {
        pub fn builder() -> BuildProvenanceBuilder {
            BuildProvenanceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The category to which the update belongs."]
    pub struct Category {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categoryId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of the category."]
        pub category_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localized name of the category."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Category {
        pub fn builder() -> CategoryBuilder {
            CategoryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A CloudRepoSourceContext denotes a particular revision in a Google Cloud Source Repo."]
    pub struct CloudRepoSourceContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aliasContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An alias, which may be a branch or tag."]
        pub alias_context: ::std::option::Option<::std::boxed::Box<AliasContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repoId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the repo."]
        pub repo_id: ::std::option::Option<::std::boxed::Box<RepoId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A revision ID."]
        pub revision_id: ::std::option::Option<::std::string::String>,
    }
    impl CloudRepoSourceContext {
        pub fn builder() -> CloudRepoSourceContextBuilder {
            CloudRepoSourceContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Command describes a step performed as part of the build pipeline."]
    pub struct Command {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "args")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Command-line arguments used when executing this command."]
        pub args: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dir")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Working directory (relative to project source root) used when running this command."]
        pub dir: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "env")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Environment variables set before running this command."]
        pub env: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional unique identifier for this command, used in wait_for to reference this command as a dependency."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Name of the command, as presented on the command line, or if the command is packaged as a Docker container, as presented to `docker pull`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "waitFor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID(s) of the command(s) that this command depends on."]
        pub wait_for: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Command {
        pub fn builder() -> CommandBuilder {
            CommandBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The period during which some deployable was active in a runtime."]
    pub struct DeploymentOccurrence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Address of the runtime element hosting this deployment."]
        pub address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "config")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration used to create this deployment."]
        pub config: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deployTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Beginning of the lifetime of this deployment."]
        pub deploy_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Platform hosting this deployment."]
        pub platform: ::std::option::Option<DeploymentOccurrencePlatformEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource URI for the artifact being deployed taken from the deployable field with the same name."]
        pub resource_uri: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "undeployTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End of the lifetime of this deployment."]
        pub undeploy_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identity of the user that triggered this deployment."]
        pub user_email: ::std::option::Option<::std::string::String>,
    }
    impl DeploymentOccurrence {
        pub fn builder() -> DeploymentOccurrenceBuilder {
            DeploymentOccurrenceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Platform hosting this deployment."]
    pub enum DeploymentOccurrencePlatformEnum {
        #[serde(rename = "PLATFORM_UNSPECIFIED")]
        #[doc = "Unknown."]
        PlatformUnspecified,
        #[serde(rename = "GKE")]
        #[doc = "Google Container Engine."]
        Gke,
        #[serde(rename = "FLEX")]
        #[doc = "Google App Engine: Flexible Environment."]
        Flex,
        #[serde(rename = "CUSTOM")]
        #[doc = "Custom user-defined platform."]
        Custom,
    }
    impl ::std::default::Default for DeploymentOccurrencePlatformEnum {
        fn default() -> Self {
            Self::PlatformUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides information about the analysis status of a discovered resource."]
    pub struct DiscoveryOccurrence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "analysisStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of discovery for the resource."]
        pub analysis_status: ::std::option::Option<DiscoveryOccurrenceAnalysisStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "analysisStatusError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When an error is encountered this will contain a LocalizedMessage under details to show to the user. The LocalizedMessage is output only and populated by the API."]
        pub analysis_status_error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "continuousAnalysis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the resource is continuously analyzed."]
        pub continuous_analysis: ::std::option::Option<DiscoveryOccurrenceContinuousAnalysisEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpe")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The CPE of the resource being scanned."]
        pub cpe: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastScanTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last time this resource was scanned."]
        pub last_scan_time: ::std::option::Option<::std::string::String>,
    }
    impl DiscoveryOccurrence {
        pub fn builder() -> DiscoveryOccurrenceBuilder {
            DiscoveryOccurrenceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The status of discovery for the resource."]
    pub enum DiscoveryOccurrenceAnalysisStatusEnum {
        #[serde(rename = "ANALYSIS_STATUS_UNSPECIFIED")]
        #[doc = "Unknown."]
        AnalysisStatusUnspecified,
        #[serde(rename = "PENDING")]
        #[doc = "Resource is known but no action has been taken yet."]
        Pending,
        #[serde(rename = "SCANNING")]
        #[doc = "Resource is being analyzed."]
        Scanning,
        #[serde(rename = "FINISHED_SUCCESS")]
        #[doc = "Analysis has finished successfully."]
        FinishedSuccess,
        #[serde(rename = "FINISHED_FAILED")]
        #[doc = "Analysis has finished unsuccessfully, the analysis itself is in a bad state."]
        FinishedFailed,
        #[serde(rename = "FINISHED_UNSUPPORTED")]
        #[doc = "The resource is known not to be supported"]
        FinishedUnsupported,
    }
    impl ::std::default::Default for DiscoveryOccurrenceAnalysisStatusEnum {
        fn default() -> Self {
            Self::AnalysisStatusUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Whether the resource is continuously analyzed."]
    pub enum DiscoveryOccurrenceContinuousAnalysisEnum {
        #[serde(rename = "CONTINUOUS_ANALYSIS_UNSPECIFIED")]
        #[doc = "Unknown."]
        ContinuousAnalysisUnspecified,
        #[serde(rename = "ACTIVE")]
        #[doc = "The resource is continuously analyzed."]
        Active,
        #[serde(rename = "INACTIVE")]
        #[doc = "The resource is ignored for continuous analysis."]
        Inactive,
    }
    impl ::std::default::Default for DiscoveryOccurrenceContinuousAnalysisEnum {
        fn default() -> Self {
            Self::ContinuousAnalysisUnspecified
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
    #[doc = "Container message for hashes of byte content of files, used in source messages to verify integrity of source input to the build."]
    pub struct FileHashes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileHash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Collection of file hashes."]
        pub file_hash: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Hash>>>,
    }
    impl FileHashes {
        pub fn builder() -> FileHashesBuilder {
            FileHashesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A set of properties that uniquely identify a given Docker image."]
    pub struct Fingerprint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "v1Name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The layer ID of the final layer in the Docker image's v1 representation."]
        pub v1_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "v2Blob")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The ordered list of v2 blobs that represent a given image."]
        pub v2_blob: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "v2Name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of the image's v2 blobs computed via: [bottom] := v2_blobbottom := sha256(v2_blob[N] + \" \" + v2_name[N+1]) Only the name of the final blob is kept."]
        pub v2_name: ::std::option::Option<::std::string::String>,
    }
    impl Fingerprint {
        pub fn builder() -> FingerprintBuilder {
            FingerprintBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A SourceContext referring to a Gerrit project."]
    pub struct GerritSourceContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aliasContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An alias, which may be a branch or tag."]
        pub alias_context: ::std::option::Option<::std::boxed::Box<AliasContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gerritProject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full project name within the host. Projects may be nested, so \"project/subproject\" is a valid project name. The \"repo name\" is the hostURI/project."]
        pub gerrit_project: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hostUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of a running Gerrit instance."]
        pub host_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A revision (commit) ID."]
        pub revision_id: ::std::option::Option<::std::string::String>,
    }
    impl GerritSourceContext {
        pub fn builder() -> GerritSourceContextBuilder {
            GerritSourceContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A GitSourceContext denotes a particular revision in a third party Git repository (e.g., GitHub)."]
    pub struct GitSourceContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Git commit hash."]
        pub revision_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Git repository URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GitSourceContext {
        pub fn builder() -> GitSourceContextBuilder {
            GitSourceContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Container message for hash values."]
    pub struct Hash {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The type of hash that was performed, e.g. \"SHA-256\"."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The hash value."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl Hash {
        pub fn builder() -> HashBuilder {
            HashBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The unique identifier of the update."]
    pub struct Identity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The revision number of the update."]
        pub revision: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The revision independent identifier of the update."]
        pub update_id: ::std::option::Option<::std::string::String>,
    }
    impl Identity {
        pub fn builder() -> IdentityBuilder {
            IdentityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of the derived image portion of the DockerImage relationship. This image would be produced from a Dockerfile with FROM ."]
    pub struct ImageOccurrence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "baseResourceUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. This contains the base image URL for the derived image occurrence."]
        pub base_resource_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "distance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The number of layers by which this image differs from the associated image basis."]
        pub distance: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The fingerprint of the derived image."]
        pub fingerprint: ::std::option::Option<::std::boxed::Box<Fingerprint>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layerInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This contains layer-specific metadata, if populated it has length \"distance\" and is ordered with [distance] being the layer immediately following the base image and [1] being the final layer."]
        pub layer_info: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Layer>>>,
    }
    impl ImageOccurrence {
        pub fn builder() -> ImageOccurrenceBuilder {
            ImageOccurrenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Jwt {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compactJwt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The compact encoding of a JWS, which is always three base64 encoded strings joined by periods. For details, see: https://tools.ietf.org/html/rfc7515.html#section-3.1"]
        pub compact_jwt: ::std::option::Option<::std::string::String>,
    }
    impl Jwt {
        pub fn builder() -> JwtBuilder {
            JwtBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Layer holds metadata specific to a layer of a Docker image."]
    pub struct Layer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arguments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The recovered arguments to the Dockerfile directive."]
        pub arguments: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "directive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The recovered Dockerfile directive used to construct this layer. See https://docs.docker.com/engine/reference/builder/ for more information."]
        pub directive: ::std::option::Option<::std::string::String>,
    }
    impl Layer {
        pub fn builder() -> LayerBuilder {
            LayerBuilder::default()
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
    #[doc = "ListVulnerabilitiesResponse contains a single page of vulnerabilities resulting from a scan."]
    pub struct ListVulnerabilitiesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A page token that can be used in a subsequent call to ListVulnerabilities to continue retrieving results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "occurrences")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Vulnerability Occurrences resulting from a scan."]
        pub occurrences: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Occurrence>>>,
    }
    impl ListVulnerabilitiesResponse {
        pub fn builder() -> ListVulnerabilitiesResponseBuilder {
            ListVulnerabilitiesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An occurrence of a particular package installation found within a system's filesystem. E.g., glibc was found in `/var/lib/dpkg/status`."]
    pub struct Location {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpeUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The CPE URI in [CPE format](https://cpe.mitre.org/specification/) denoting the package manager version distributing a package."]
        pub cpe_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The path from which we gathered that this package/version is installed."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version installed at this location."]
        pub version: ::std::option::Option<::std::boxed::Box<Version>>,
    }
    impl Location {
        pub fn builder() -> LocationBuilder {
            LocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An instance of an analysis type that has been found on a resource."]
    pub struct Occurrence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attestation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes an attestation of an artifact."]
        pub attestation: ::std::option::Option<::std::boxed::Box<AttestationOccurrence>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "build")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes a verifiable build."]
        pub _build: ::std::option::Option<::std::boxed::Box<BuildOccurrence>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this occurrence was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deployment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the deployment of an artifact on a runtime."]
        pub deployment: ::std::option::Option<::std::boxed::Box<DeploymentOccurrence>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "discovery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes when a resource was discovered."]
        pub discovery: ::std::option::Option<::std::boxed::Box<DiscoveryOccurrence>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes how this resource derives from the basis in the associated note."]
        pub image: ::std::option::Option<::std::boxed::Box<ImageOccurrence>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests."]
        pub kind: ::std::option::Option<OccurrenceKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "noteName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Immutable. The analysis note associated with this occurrence, in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. This field can be used as a filter in list requests."]
        pub note_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "package")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the installation of a package on the linked resource."]
        pub package: ::std::option::Option<::std::boxed::Box<PackageOccurrence>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "remediation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of actions that can be taken to remedy the note."]
        pub remediation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Immutable. A URI that represents the resource for which the occurrence applies. For example, `https://gcr.io/project/image@sha256:123abc` for a Docker image."]
        pub resource_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this occurrence was last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upgrade")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes an available package upgrade on the linked resource."]
        pub upgrade: ::std::option::Option<::std::boxed::Box<UpgradeOccurrence>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vulnerability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes a security vulnerability."]
        pub vulnerability: ::std::option::Option<::std::boxed::Box<VulnerabilityOccurrence>>,
    }
    impl Occurrence {
        pub fn builder() -> OccurrenceBuilder {
            OccurrenceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests."]
    pub enum OccurrenceKindEnum {
        #[serde(rename = "NOTE_KIND_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        NoteKindUnspecified,
        #[serde(rename = "VULNERABILITY")]
        #[doc = "The note and occurrence represent a package vulnerability."]
        Vulnerability,
        #[serde(rename = "BUILD")]
        #[doc = "The note and occurrence assert build provenance."]
        Build,
        #[serde(rename = "IMAGE")]
        #[doc = "This represents an image basis relationship."]
        Image,
        #[serde(rename = "PACKAGE")]
        #[doc = "This represents a package installed via a package manager."]
        Package,
        #[serde(rename = "DEPLOYMENT")]
        #[doc = "The note and occurrence track deployment events."]
        Deployment,
        #[serde(rename = "DISCOVERY")]
        #[doc = "The note and occurrence track the initial discovery status of a resource."]
        Discovery,
        #[serde(rename = "ATTESTATION")]
        #[doc = "This represents a logical \"role\" that can attest to artifacts."]
        Attestation,
        #[serde(rename = "UPGRADE")]
        #[doc = "This represents an available package upgrade."]
        Upgrade,
    }
    impl ::std::default::Default for OccurrenceKindEnum {
        fn default() -> Self {
            Self::NoteKindUnspecified
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
    pub struct PackageData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpeUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cpe_uri in [cpe format] (https://cpe.mitre.org/specification/) in which the vulnerability may manifest. Examples include distro or storage location for vulnerable jar."]
        pub cpe_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "os")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The OS affected by a vulnerability This field is deprecated and the information is in cpe_uri"]
        pub os: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "osVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the OS This field is deprecated and the information is in cpe_uri"]
        pub os_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "package")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The package being analysed for vulnerabilities"]
        pub package: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the package being analysed"]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl PackageData {
        pub fn builder() -> PackageDataBuilder {
            PackageDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A detail for a distro and package this vulnerability occurrence was found in and its associated fix (if one is available)."]
    pub struct PackageIssue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "affectedCpeUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The [CPE URI](https://cpe.mitre.org/specification/) this vulnerability was found in."]
        pub affected_cpe_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "affectedPackage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The package this vulnerability was found in."]
        pub affected_package: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "affectedVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The version of the package that is installed on the resource affected by this vulnerability."]
        pub affected_version: ::std::option::Option<::std::boxed::Box<Version>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fixAvailable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether a fix is available for this package."]
        pub fix_available: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fixedCpeUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [CPE URI](https://cpe.mitre.org/specification/) this vulnerability was fixed in. It is possible for this to be different from the affected_cpe_uri."]
        pub fixed_cpe_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fixedPackage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The package this vulnerability was fixed in. It is possible for this to be different from the affected_package."]
        pub fixed_package: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fixedVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The version of the package this vulnerability was fixed in. Setting this to VersionKind.MAXIMUM means no fix is yet available."]
        pub fixed_version: ::std::option::Option<::std::boxed::Box<Version>>,
    }
    impl PackageIssue {
        pub fn builder() -> PackageIssueBuilder {
            PackageIssueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details on how a particular software package was installed on a system."]
    pub struct PackageOccurrence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. All of the places within the filesystem versions of this package have been found."]
        pub location: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Location>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of the installed package."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl PackageOccurrence {
        pub fn builder() -> PackageOccurrenceBuilder {
            PackageOccurrenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Selects a repo using a Google Cloud Platform project ID (e.g., winged-cargo-31) and a repo name within that project."]
    pub struct ProjectRepoId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the project."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repoName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the repo. Leave empty for the default repo."]
        pub repo_name: ::std::option::Option<::std::string::String>,
    }
    impl ProjectRepoId {
        pub fn builder() -> ProjectRepoIdBuilder {
            ProjectRepoIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for any related URL information."]
    pub struct RelatedUrl {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label to describe usage of the URL."]
        pub label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specific URL associated with the resource."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl RelatedUrl {
        pub fn builder() -> RelatedUrlBuilder {
            RelatedUrlBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A unique identifier for a Cloud Repo."]
    pub struct RepoId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectRepoId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A combination of a project ID and a repo name."]
        pub project_repo_id: ::std::option::Option<::std::boxed::Box<ProjectRepoId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A server-assigned, globally unique identifier."]
        pub uid: ::std::option::Option<::std::string::String>,
    }
    impl RepoId {
        pub fn builder() -> RepoIdBuilder {
            RepoIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Verifiers (e.g. Kritis implementations) MUST verify signatures with respect to the trust anchors defined in policy (e.g. a Kritis policy). Typically this means that the verifier has been configured with a map from `public_key_id` to public key material (and any required parameters, e.g. signing algorithm). In particular, verification implementations MUST NOT treat the signature `public_key_id` as anything more than a key lookup hint. The `public_key_id` DOES NOT validate or authenticate a public key; it only provides a mechanism for quickly selecting a public key ALREADY CONFIGURED on the verifier through a trusted channel. Verification implementations MUST reject signatures in any of the following circumstances: * The `public_key_id` is not recognized by the verifier. * The public key that `public_key_id` refers to does not verify the signature with respect to the payload. The `signature` contents SHOULD NOT be \"attached\" (where the payload is included with the serialized `signature` bytes). Verifiers MUST ignore any \"attached\" payload and only verify signatures with respect to explicitly provided payload (e.g. a `payload` field on the proto message that holds this Signature, or the canonical serialization of the proto message that holds this signature)."]
    pub struct Signature {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicKeyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier for the public key that verifies this signature. * The `public_key_id` is required. * The `public_key_id` SHOULD be an RFC3986 conformant URI. * When possible, the `public_key_id` SHOULD be an immutable reference, such as a cryptographic digest. Examples of valid `public_key_id`s: OpenPGP V4 public key fingerprint: * \"openpgp4fpr:74FAF3B861BDA0870C7B6DEF607E48D2A663AEEA\" See https://www.iana.org/assignments/uri-schemes/prov/openpgp4fpr for more details on this scheme. RFC6920 digest-named SubjectPublicKeyInfo (digest of the DER serialization): * \"ni:///sha-256;cD9o9Cq6LG3jD0iKXqEi_vdjJGecm_iXkbqVoScViaU\" * \"nih:///sha-256;703f68f42aba2c6de30f488a5ea122fef76324679c9bf89791ba95a1271589a5\""]
        pub public_key_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content of the signature, an opaque bytestring. The payload that this signature verifies MUST be unambiguously provided with the Signature during verification. A wrapper message might provide the payload explicitly. Alternatively, a message might have a canonical serialization that can always be unambiguously computed to derive the payload."]
        pub signature: ::std::option::Option<::std::string::String>,
    }
    impl Signature {
        pub fn builder() -> SignatureBuilder {
            SignatureBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Source describes the location of the source used for the build."]
    pub struct Source {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalContexts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If provided, some of the source code used for the build may be found in these locations, in the case where the source repository had multiple remotes or submodules. This list will not include the context specified in the context field."]
        pub additional_contexts:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SourceContext>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "artifactStorageSourceUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If provided, the input binary artifacts for the build came from this location."]
        pub artifact_storage_source_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If provided, the source code used for the build came from this location."]
        pub context: ::std::option::Option<::std::boxed::Box<SourceContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileHashes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hash(es) of the build source, which can be used to verify that the original source integrity was maintained in the build. The keys to this map are file paths used as build source and the values contain the hash values for those files. If the build source came in a single package such as a gzipped tarfile (.tar.gz), the FileHash will be for the single path to that file."]
        pub file_hashes: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<FileHashes>>,
        >,
    }
    impl Source {
        pub fn builder() -> SourceBuilder {
            SourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A SourceContext is a reference to a tree of files. A SourceContext together with a path point to a unique revision of a single file or directory."]
    pub struct SourceContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudRepo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A SourceContext referring to a revision in a Google Cloud Source Repo."]
        pub cloud_repo: ::std::option::Option<::std::boxed::Box<CloudRepoSourceContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gerrit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A SourceContext referring to a Gerrit project."]
        pub gerrit: ::std::option::Option<::std::boxed::Box<GerritSourceContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "git")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A SourceContext referring to any third party Git repo (e.g., GitHub)."]
        pub git: ::std::option::Option<::std::boxed::Box<GitSourceContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Labels with user defined metadata."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl SourceContext {
        pub fn builder() -> SourceContextBuilder {
            SourceContextBuilder::default()
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
    #[doc = "The Upgrade Distribution represents metadata about the Upgrade for each operating system (CPE). Some distributions have additional metadata around updates, classifying them into various categories and severities."]
    pub struct UpgradeDistribution {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "classification")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operating system classification of this Upgrade, as specified by the upstream operating system upgrade feed. For Windows the classification is one of the category_ids listed at https://docs.microsoft.com/en-us/previous-versions/windows/desktop/ff357803(v=vs.85)"]
        pub classification: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpeUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required - The specific operating system this metadata applies to. See https://cpe.mitre.org/specification/."]
        pub cpe_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cve")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cve tied to this Upgrade."]
        pub cve: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The severity as specified by the upstream operating system."]
        pub severity: ::std::option::Option<::std::string::String>,
    }
    impl UpgradeDistribution {
        pub fn builder() -> UpgradeDistributionBuilder {
            UpgradeDistributionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Upgrade Occurrence represents that a specific resource_url could install a specific upgrade. This presence is supplied via local sources (i.e. it is present in the mirror and the running system has noticed its availability). For Windows, both distribution and windows_update contain information for the Windows update."]
    pub struct UpgradeOccurrence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "distribution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata about the upgrade for available for the specific operating system for the resource_url. This allows efficient filtering, as well as making it easier to use the occurrence."]
        pub distribution: ::std::option::Option<::std::boxed::Box<UpgradeDistribution>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "package")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for non-Windows OS. The package this Upgrade is for."]
        pub package: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parsedVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for non-Windows OS. The version of the package in a machine + human readable form."]
        pub parsed_version: ::std::option::Option<::std::boxed::Box<Version>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "windowsUpdate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for Windows OS. Represents the metadata about the Windows update."]
        pub windows_update: ::std::option::Option<::std::boxed::Box<WindowsUpdate>>,
    }
    impl UpgradeOccurrence {
        pub fn builder() -> UpgradeOccurrenceBuilder {
            UpgradeOccurrenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Version contains structured information about the version of a package."]
    pub struct Version {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "epoch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Used to correct mistakes in the version numbering scheme."]
        pub epoch: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human readable version string. This string is of the form :- and is only set when kind is NORMAL."]
        pub full_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inclusive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this version is specifying part of an inclusive range. Grafeas does not have the capability to specify version ranges; instead we have fields that specify start version and end versions. At times this is insufficient - we also need to specify whether the version is included in the range or is excluded from the range. This boolean is expected to be set to true when the version is included in a range."]
        pub inclusive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Distinguishes between sentinel MIN/MAX versions and normal versions."]
        pub kind: ::std::option::Option<VersionKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required only when version kind is NORMAL. The main part of the version name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The iteration of the package build from the above version."]
        pub revision: ::std::option::Option<::std::string::String>,
    }
    impl Version {
        pub fn builder() -> VersionBuilder {
            VersionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Distinguishes between sentinel MIN/MAX versions and normal versions."]
    pub enum VersionKindEnum {
        #[serde(rename = "VERSION_KIND_UNSPECIFIED")]
        #[doc = "Unknown."]
        VersionKindUnspecified,
        #[serde(rename = "NORMAL")]
        #[doc = "A standard package version."]
        Normal,
        #[serde(rename = "MINIMUM")]
        #[doc = "A special version representing negative infinity."]
        Minimum,
        #[serde(rename = "MAXIMUM")]
        #[doc = "A special version representing positive infinity."]
        Maximum,
    }
    impl ::std::default::Default for VersionKindEnum {
        fn default() -> Self {
            Self::VersionKindUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An occurrence of a severity vulnerability on a resource."]
    pub struct VulnerabilityOccurrence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cvssScore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The CVSS score of this vulnerability. CVSS score is on a scale of 0 - 10 where 0 indicates low severity and 10 indicates high severity."]
        pub cvss_score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "effectiveSeverity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The distro assigned severity for this vulnerability when it is available, otherwise this is the note provider assigned severity."]
        pub effective_severity: ::std::option::Option<VulnerabilityOccurrenceEffectiveSeverityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fixAvailable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether at least one of the affected packages has a fix available."]
        pub fix_available: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A detailed description of this vulnerability."]
        pub long_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageIssue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The set of affected locations and their fixes (if available) within the associated resource."]
        pub package_issue: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PackageIssue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relatedUrls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. URLs related to this vulnerability."]
        pub related_urls: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RelatedUrl>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The note provider assigned severity of this vulnerability."]
        pub severity: ::std::option::Option<VulnerabilityOccurrenceSeverityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shortDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A one sentence description of this vulnerability."]
        pub short_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of package; whether native or non native (e.g., ruby gems, node.js packages, etc.)."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl VulnerabilityOccurrence {
        pub fn builder() -> VulnerabilityOccurrenceBuilder {
            VulnerabilityOccurrenceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The distro assigned severity for this vulnerability when it is available, otherwise this is the note provider assigned severity."]
    pub enum VulnerabilityOccurrenceEffectiveSeverityEnum {
        #[serde(rename = "SEVERITY_UNSPECIFIED")]
        #[doc = "Unknown."]
        SeverityUnspecified,
        #[serde(rename = "MINIMAL")]
        #[doc = "Minimal severity."]
        Minimal,
        #[serde(rename = "LOW")]
        #[doc = "Low severity."]
        Low,
        #[serde(rename = "MEDIUM")]
        #[doc = "Medium severity."]
        Medium,
        #[serde(rename = "HIGH")]
        #[doc = "High severity."]
        High,
        #[serde(rename = "CRITICAL")]
        #[doc = "Critical severity."]
        Critical,
    }
    impl ::std::default::Default for VulnerabilityOccurrenceEffectiveSeverityEnum {
        fn default() -> Self {
            Self::SeverityUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The note provider assigned severity of this vulnerability."]
    pub enum VulnerabilityOccurrenceSeverityEnum {
        #[serde(rename = "SEVERITY_UNSPECIFIED")]
        #[doc = "Unknown."]
        SeverityUnspecified,
        #[serde(rename = "MINIMAL")]
        #[doc = "Minimal severity."]
        Minimal,
        #[serde(rename = "LOW")]
        #[doc = "Low severity."]
        Low,
        #[serde(rename = "MEDIUM")]
        #[doc = "Medium severity."]
        Medium,
        #[serde(rename = "HIGH")]
        #[doc = "High severity."]
        High,
        #[serde(rename = "CRITICAL")]
        #[doc = "Critical severity."]
        Critical,
    }
    impl ::std::default::Default for VulnerabilityOccurrenceSeverityEnum {
        fn default() -> Self {
            Self::SeverityUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Windows Update represents the metadata about the update for the Windows operating system. The fields in this message come from the Windows Update API documented at https://docs.microsoft.com/en-us/windows/win32/api/wuapi/nn-wuapi-iupdate."]
    pub struct WindowsUpdate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of categories to which the update belongs."]
        pub categories: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Category>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localized description of the update."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "identity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required - The unique identifier for the update."]
        pub identity: ::std::option::Option<::std::boxed::Box<Identity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kbArticleIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Microsoft Knowledge Base article IDs that are associated with the update."]
        pub kb_article_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastPublishedTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last published timestamp of the update."]
        pub last_published_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hyperlink to the support information for the update."]
        pub support_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localized title of the update."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl WindowsUpdate {
        pub fn builder() -> WindowsUpdateBuilder {
            WindowsUpdateBuilder::default()
        }
    }
}
