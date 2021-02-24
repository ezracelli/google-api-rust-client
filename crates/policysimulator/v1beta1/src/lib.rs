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
    pub mod folders {
        pub mod resources {
            pub mod locations {
                pub mod resources {
                    pub mod replays {
                        pub mod resources {
                            pub mod results {
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
                                            #[doc = "The maximum number of ReplayResult objects to return. Defaults to 5000. The maximum value is 5000; values above 5000 are rounded down to 5000."]
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
                                            #[doc = "A page token, received from a previous Simulator.ListReplayResults call. Provide this token to retrieve the next page of results. When paginating, all other parameters provided to [Simulator.ListReplayResults[] must match the call that provided the page token."]
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
                    #[serde(rename = "name")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The name of the operation's parent resource."]
                    pub name: ::std::option::Option<::std::string::String>,
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
    pub mod organizations {
        pub mod resources {
            pub mod locations {
                pub mod resources {
                    pub mod replays {
                        pub mod resources {
                            pub mod results {
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
                                            #[doc = "The maximum number of ReplayResult objects to return. Defaults to 5000. The maximum value is 5000; values above 5000 are rounded down to 5000."]
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
                                            #[doc = "A page token, received from a previous Simulator.ListReplayResults call. Provide this token to retrieve the next page of results. When paginating, all other parameters provided to [Simulator.ListReplayResults[] must match the call that provided the page token."]
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
    pub mod projects {
        pub mod resources {
            pub mod locations {
                pub mod resources {
                    pub mod replays {
                        pub mod resources {
                            pub mod results {
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
                                            #[doc = "The maximum number of ReplayResult objects to return. Defaults to 5000. The maximum value is 5000; values above 5000 are rounded down to 5000."]
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
                                            #[doc = "A page token, received from a previous Simulator.ListReplayResults call. Provide this token to retrieve the next page of results. When paginating, all other parameters provided to [Simulator.ListReplayResults[] must match the call that provided the page token."]
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
    #[doc = "A summary and comparison of the member's access under the current (baseline) policies and the proposed (simulated) policies for a single access tuple."]
    pub struct GoogleCloudPolicysimulatorV1beta1AccessStateDiff {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessChange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How the member's access, specified in the AccessState field, changed between the current (baseline) policies and proposed (simulated) policies."]
        pub access_change:
            ::std::option::Option<GoogleCloudPolicysimulatorV1beta1AccessStateDiffAccessChangeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "baseline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The results of evaluating the access tuple under the current (baseline) policies. If the AccessState couldn't be fully evaluated, this field explains why."]
        pub baseline: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudPolicysimulatorV1beta1ExplainedAccess>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "simulated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The results of evaluating the access tuple under the proposed (simulated) policies. If the AccessState couldn't be fully evaluated, this field explains why."]
        pub simulated: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudPolicysimulatorV1beta1ExplainedAccess>,
        >,
    }
    impl GoogleCloudPolicysimulatorV1beta1AccessStateDiff {
        pub fn builder() -> GoogleCloudPolicysimulatorV1beta1AccessStateDiffBuilder {
            GoogleCloudPolicysimulatorV1beta1AccessStateDiffBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How the member's access, specified in the AccessState field, changed between the current (baseline) policies and proposed (simulated) policies."]
    pub enum GoogleCloudPolicysimulatorV1beta1AccessStateDiffAccessChangeEnum {
        #[serde(rename = "ACCESS_CHANGE_TYPE_UNSPECIFIED")]
        #[doc = "The access change is unspecified."]
        AccessChangeTypeUnspecified,
        #[serde(rename = "NO_CHANGE")]
        #[doc = "The member's access did not change. This includes the case where both baseline and simulated are UNKNOWN, but the unknown information is equivalent."]
        NoChange,
        #[serde(rename = "UNKNOWN_CHANGE")]
        #[doc = "The member's access under both the current policies and the proposed policies is `UNKNOWN`, but the unknown information differs between them."]
        UnknownChange,
        #[serde(rename = "ACCESS_REVOKED")]
        #[doc = "The member had access under the current policies (`GRANTED`), but will no longer have access after the proposed changes (`NOT_GRANTED`)."]
        AccessRevoked,
        #[serde(rename = "ACCESS_GAINED")]
        #[doc = "The member did not have access under the current policies (`NOT_GRANTED`), but will have access after the proposed changes (`GRANTED`)."]
        AccessGained,
        #[serde(rename = "ACCESS_MAYBE_REVOKED")]
        #[doc = "This result can occur for the following reasons: * The member had access under the current policies (`GRANTED`), but their access after the proposed changes is `UNKNOWN`. * The member's access under the current policies is `UNKNOWN`, but they will not have access after the proposed changes (`NOT_GRANTED`)."]
        AccessMaybeRevoked,
        #[serde(rename = "ACCESS_MAYBE_GAINED")]
        #[doc = "This result can occur for the following reasons: * The member did not have access under the current policies (`NOT_GRANTED`), but their access after the proposed changes is `UNKNOWN`. * The member's access under the current policies is `UNKNOWN`, but they will have access after the proposed changes (`GRANTED`)."]
        AccessMaybeGained,
    }
    impl ::std::default::Default for GoogleCloudPolicysimulatorV1beta1AccessStateDiffAccessChangeEnum {
        fn default() -> Self {
            Self::AccessChangeTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the member, resource, and permission to check."]
    pub struct GoogleCloudPolicysimulatorV1beta1AccessTuple {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullResourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The full resource name that identifies the resource. For example, `//compute.googleapis.com/projects/my-project/zones/us-central1-a/instances/my-instance`. For examples of full resource names for Google Cloud services, see https://cloud.google.com/iam/help/troubleshooter/full-resource-names."]
        pub full_resource_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The IAM permission to check for the specified member and resource. For a complete list of IAM permissions, see https://cloud.google.com/iam/help/permissions/reference. For a complete list of predefined IAM roles and the permissions in each role, see https://cloud.google.com/iam/help/roles/reference."]
        pub permission: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "principal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The member, or principal, whose access you want to check, in the form of the email address that represents that member. For example, `alice@example.com` or `my-service-account@my-project.iam.gserviceaccount.com`. The member must be a Google Account or a service account. Other types of members are not supported."]
        pub principal: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudPolicysimulatorV1beta1AccessTuple {
        pub fn builder() -> GoogleCloudPolicysimulatorV1beta1AccessTupleBuilder {
            GoogleCloudPolicysimulatorV1beta1AccessTupleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details about how a binding in a policy affects a member's ability to use a permission."]
    pub struct GoogleCloudPolicysimulatorV1beta1BindingExplanation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "access")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Indicates whether _this binding_ provides the specified permission to the specified member for the specified resource. This field does _not_ indicate whether the member actually has the permission for the resource. There might be another binding that overrides this binding. To determine whether the member actually has the permission, use the `access` field in the TroubleshootIamPolicyResponse."]
        pub access:
            ::std::option::Option<GoogleCloudPolicysimulatorV1beta1BindingExplanationAccessEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A condition expression that prevents this binding from granting access unless the expression evaluates to `true`. To learn about IAM Conditions, see https://cloud.google.com/iam/docs/conditions-overview."]
        pub condition: ::std::option::Option<::std::boxed::Box<GoogleTypeExpr>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memberships")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether each member in the binding includes the member specified in the request, either directly or indirectly. Each key identifies a member in the binding, and each value indicates whether the member in the binding includes the member in the request. For example, suppose that a binding includes the following members: * `user:alice@example.com` * `group:product-eng@example.com` The member in the replayed access tuple is `user:bob@example.com`. This user is a member of the group `group:product-eng@example.com`. For the first member in the binding, the key is `user:alice@example.com`, and the `membership` field in the value is set to `MEMBERSHIP_NOT_INCLUDED`. For the second member in the binding, the key is `group:product-eng@example.com`, and the `membership` field in the value is set to `MEMBERSHIP_INCLUDED`."]
        pub memberships: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                ::std::boxed::Box<
                    GoogleCloudPolicysimulatorV1beta1BindingExplanationAnnotatedMembership,
                >,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relevance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relevance of this binding to the overall determination for the entire policy."]
        pub relevance:
            ::std::option::Option<GoogleCloudPolicysimulatorV1beta1BindingExplanationRelevanceEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The role that this binding grants. For example, `roles/compute.serviceAgent`. For a complete list of predefined IAM roles, as well as the permissions in each role, see https://cloud.google.com/iam/help/roles/reference."]
        pub role: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rolePermission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the role granted by this binding contains the specified permission."]
        pub role_permission: ::std::option::Option<
            GoogleCloudPolicysimulatorV1beta1BindingExplanationRolePermissionEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rolePermissionRelevance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relevance of the permission's existence, or nonexistence, in the role to the overall determination for the entire policy."]
        pub role_permission_relevance: ::std::option::Option<
            GoogleCloudPolicysimulatorV1beta1BindingExplanationRolePermissionRelevanceEnum,
        >,
    }
    impl GoogleCloudPolicysimulatorV1beta1BindingExplanation {
        pub fn builder() -> GoogleCloudPolicysimulatorV1beta1BindingExplanationBuilder {
            GoogleCloudPolicysimulatorV1beta1BindingExplanationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Indicates whether _this binding_ provides the specified permission to the specified member for the specified resource. This field does _not_ indicate whether the member actually has the permission for the resource. There might be another binding that overrides this binding. To determine whether the member actually has the permission, use the `access` field in the TroubleshootIamPolicyResponse."]
    pub enum GoogleCloudPolicysimulatorV1beta1BindingExplanationAccessEnum {
        #[serde(rename = "ACCESS_STATE_UNSPECIFIED")]
        #[doc = "The access state is not specified."]
        AccessStateUnspecified,
        #[serde(rename = "GRANTED")]
        #[doc = "The member has the permission."]
        Granted,
        #[serde(rename = "NOT_GRANTED")]
        #[doc = "The member does not have the permission."]
        NotGranted,
        #[serde(rename = "UNKNOWN_CONDITIONAL")]
        #[doc = "The member has the permission only if a condition expression evaluates to `true`."]
        UnknownConditional,
        #[serde(rename = "UNKNOWN_INFO_DENIED")]
        #[doc = "The user who created the Replay does not have access to all of the policies that Policy Simulator needs to evaluate."]
        UnknownInfoDenied,
    }
    impl ::std::default::Default for GoogleCloudPolicysimulatorV1beta1BindingExplanationAccessEnum {
        fn default() -> Self {
            Self::AccessStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The relevance of this binding to the overall determination for the entire policy."]
    pub enum GoogleCloudPolicysimulatorV1beta1BindingExplanationRelevanceEnum {
        #[serde(rename = "HEURISTIC_RELEVANCE_UNSPECIFIED")]
        #[doc = "Reserved for future use."]
        HeuristicRelevanceUnspecified,
        #[serde(rename = "NORMAL")]
        #[doc = "The data point has a limited effect on the result. Changing the data point is unlikely to affect the overall determination."]
        Normal,
        #[serde(rename = "HIGH")]
        #[doc = "The data point has a strong effect on the result. Changing the data point is likely to affect the overall determination."]
        High,
    }
    impl ::std::default::Default for GoogleCloudPolicysimulatorV1beta1BindingExplanationRelevanceEnum {
        fn default() -> Self {
            Self::HeuristicRelevanceUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates whether the role granted by this binding contains the specified permission."]
    pub enum GoogleCloudPolicysimulatorV1beta1BindingExplanationRolePermissionEnum {
        #[serde(rename = "ROLE_PERMISSION_UNSPECIFIED")]
        #[doc = "The inclusion of the permission is not specified."]
        RolePermissionUnspecified,
        #[serde(rename = "ROLE_PERMISSION_INCLUDED")]
        #[doc = "The permission is included in the role."]
        RolePermissionIncluded,
        #[serde(rename = "ROLE_PERMISSION_NOT_INCLUDED")]
        #[doc = "The permission is not included in the role."]
        RolePermissionNotIncluded,
        #[serde(rename = "ROLE_PERMISSION_UNKNOWN_INFO_DENIED")]
        #[doc = "The user who created the Replay is not allowed to access the binding."]
        RolePermissionUnknownInfoDenied,
    }
    impl ::std::default::Default
        for GoogleCloudPolicysimulatorV1beta1BindingExplanationRolePermissionEnum
    {
        fn default() -> Self {
            Self::RolePermissionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The relevance of the permission's existence, or nonexistence, in the role to the overall determination for the entire policy."]
    pub enum GoogleCloudPolicysimulatorV1beta1BindingExplanationRolePermissionRelevanceEnum {
        #[serde(rename = "HEURISTIC_RELEVANCE_UNSPECIFIED")]
        #[doc = "Reserved for future use."]
        HeuristicRelevanceUnspecified,
        #[serde(rename = "NORMAL")]
        #[doc = "The data point has a limited effect on the result. Changing the data point is unlikely to affect the overall determination."]
        Normal,
        #[serde(rename = "HIGH")]
        #[doc = "The data point has a strong effect on the result. Changing the data point is likely to affect the overall determination."]
        High,
    }
    impl ::std::default::Default
        for GoogleCloudPolicysimulatorV1beta1BindingExplanationRolePermissionRelevanceEnum
    {
        fn default() -> Self {
            Self::HeuristicRelevanceUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details about whether the binding includes the member."]
    pub struct GoogleCloudPolicysimulatorV1beta1BindingExplanationAnnotatedMembership {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "membership")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the binding includes the member."]
        pub membership: ::std::option::Option<
            GoogleCloudPolicysimulatorV1beta1BindingExplanationAnnotatedMembershipMembershipEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relevance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relevance of the member's status to the overall determination for the binding."]
        pub relevance: ::std::option::Option<
            GoogleCloudPolicysimulatorV1beta1BindingExplanationAnnotatedMembershipRelevanceEnum,
        >,
    }
    impl GoogleCloudPolicysimulatorV1beta1BindingExplanationAnnotatedMembership {
        pub fn builder(
        ) -> GoogleCloudPolicysimulatorV1beta1BindingExplanationAnnotatedMembershipBuilder {
            GoogleCloudPolicysimulatorV1beta1BindingExplanationAnnotatedMembershipBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates whether the binding includes the member."]
    pub enum GoogleCloudPolicysimulatorV1beta1BindingExplanationAnnotatedMembershipMembershipEnum {
        #[serde(rename = "MEMBERSHIP_UNSPECIFIED")]
        #[doc = "The membership is not specified."]
        MembershipUnspecified,
        #[serde(rename = "MEMBERSHIP_INCLUDED")]
        #[doc = "The binding includes the member. The member can be included directly or indirectly. For example: * A member is included directly if that member is listed in the binding. * A member is included indirectly if that member is in a Google group or Google Workspace domain that is listed in the binding."]
        MembershipIncluded,
        #[serde(rename = "MEMBERSHIP_NOT_INCLUDED")]
        #[doc = "The binding does not include the member."]
        MembershipNotIncluded,
        #[serde(rename = "MEMBERSHIP_UNKNOWN_INFO_DENIED")]
        #[doc = "The user who created the Replay is not allowed to access the binding."]
        MembershipUnknownInfoDenied,
        #[serde(rename = "MEMBERSHIP_UNKNOWN_UNSUPPORTED")]
        #[doc = "The member is an unsupported type. Only Google Accounts and service accounts are supported."]
        MembershipUnknownUnsupported,
    }
    impl ::std::default::Default
        for GoogleCloudPolicysimulatorV1beta1BindingExplanationAnnotatedMembershipMembershipEnum
    {
        fn default() -> Self {
            Self::MembershipUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The relevance of the member's status to the overall determination for the binding."]
    pub enum GoogleCloudPolicysimulatorV1beta1BindingExplanationAnnotatedMembershipRelevanceEnum {
        #[serde(rename = "HEURISTIC_RELEVANCE_UNSPECIFIED")]
        #[doc = "Reserved for future use."]
        HeuristicRelevanceUnspecified,
        #[serde(rename = "NORMAL")]
        #[doc = "The data point has a limited effect on the result. Changing the data point is unlikely to affect the overall determination."]
        Normal,
        #[serde(rename = "HIGH")]
        #[doc = "The data point has a strong effect on the result. Changing the data point is likely to affect the overall determination."]
        High,
    }
    impl ::std::default::Default
        for GoogleCloudPolicysimulatorV1beta1BindingExplanationAnnotatedMembershipRelevanceEnum
    {
        fn default() -> Self {
            Self::HeuristicRelevanceUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details about how a set of policies, listed in ExplainedPolicy, resulted in a certain AccessState when replaying an access tuple."]
    pub struct GoogleCloudPolicysimulatorV1beta1ExplainedAccess {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the member in the access tuple has permission to access the resource in the access tuple under the given policies."]
        pub access_state:
            ::std::option::Option<GoogleCloudPolicysimulatorV1beta1ExplainedAccessAccessStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the AccessState is `UNKNOWN`, this field contains a list of errors explaining why the result is `UNKNOWN`. If the `AccessState` is `GRANTED` or `NOT_GRANTED`, this field is omitted."]
        pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the AccessState is `UNKNOWN`, this field contains the policies that led to that result. If the `AccessState` is `GRANTED` or `NOT_GRANTED`, this field is omitted."]
        pub policies: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudPolicysimulatorV1beta1ExplainedPolicy>>,
        >,
    }
    impl GoogleCloudPolicysimulatorV1beta1ExplainedAccess {
        pub fn builder() -> GoogleCloudPolicysimulatorV1beta1ExplainedAccessBuilder {
            GoogleCloudPolicysimulatorV1beta1ExplainedAccessBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Whether the member in the access tuple has permission to access the resource in the access tuple under the given policies."]
    pub enum GoogleCloudPolicysimulatorV1beta1ExplainedAccessAccessStateEnum {
        #[serde(rename = "ACCESS_STATE_UNSPECIFIED")]
        #[doc = "The access state is not specified."]
        AccessStateUnspecified,
        #[serde(rename = "GRANTED")]
        #[doc = "The member has the permission."]
        Granted,
        #[serde(rename = "NOT_GRANTED")]
        #[doc = "The member does not have the permission."]
        NotGranted,
        #[serde(rename = "UNKNOWN_CONDITIONAL")]
        #[doc = "The member has the permission only if a condition expression evaluates to `true`."]
        UnknownConditional,
        #[serde(rename = "UNKNOWN_INFO_DENIED")]
        #[doc = "The user who created the Replay does not have access to all of the policies that Policy Simulator needs to evaluate."]
        UnknownInfoDenied,
    }
    impl ::std::default::Default for GoogleCloudPolicysimulatorV1beta1ExplainedAccessAccessStateEnum {
        fn default() -> Self {
            Self::AccessStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details about how a specific IAM Policy contributed to the access check."]
    pub struct GoogleCloudPolicysimulatorV1beta1ExplainedPolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "access")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether _this policy_ provides the specified permission to the specified member for the specified resource. This field does _not_ indicate whether the member actually has the permission for the resource. There might be another policy that overrides this policy. To determine whether the member actually has the permission, use the `access` field in the TroubleshootIamPolicyResponse."]
        pub access:
            ::std::option::Option<GoogleCloudPolicysimulatorV1beta1ExplainedPolicyAccessEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bindingExplanations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details about how each binding in the policy affects the member's ability, or inability, to use the permission for the resource. If the user who created the Replay does not have access to the policy, this field is omitted."]
        pub binding_explanations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudPolicysimulatorV1beta1BindingExplanation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullResourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full resource name that identifies the resource. For example, `//compute.googleapis.com/projects/my-project/zones/us-central1-a/instances/my-instance`. If the user who created the Replay does not have access to the policy, this field is omitted. For examples of full resource names for Google Cloud services, see https://cloud.google.com/iam/help/troubleshooter/full-resource-names."]
        pub full_resource_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IAM policy attached to the resource. If the user who created the Replay does not have access to the policy, this field is empty."]
        pub policy: ::std::option::Option<::std::boxed::Box<GoogleIamV1Policy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relevance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relevance of this policy to the overall determination in the TroubleshootIamPolicyResponse. If the user who created the Replay does not have access to the policy, this field is omitted."]
        pub relevance:
            ::std::option::Option<GoogleCloudPolicysimulatorV1beta1ExplainedPolicyRelevanceEnum>,
    }
    impl GoogleCloudPolicysimulatorV1beta1ExplainedPolicy {
        pub fn builder() -> GoogleCloudPolicysimulatorV1beta1ExplainedPolicyBuilder {
            GoogleCloudPolicysimulatorV1beta1ExplainedPolicyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates whether _this policy_ provides the specified permission to the specified member for the specified resource. This field does _not_ indicate whether the member actually has the permission for the resource. There might be another policy that overrides this policy. To determine whether the member actually has the permission, use the `access` field in the TroubleshootIamPolicyResponse."]
    pub enum GoogleCloudPolicysimulatorV1beta1ExplainedPolicyAccessEnum {
        #[serde(rename = "ACCESS_STATE_UNSPECIFIED")]
        #[doc = "The access state is not specified."]
        AccessStateUnspecified,
        #[serde(rename = "GRANTED")]
        #[doc = "The member has the permission."]
        Granted,
        #[serde(rename = "NOT_GRANTED")]
        #[doc = "The member does not have the permission."]
        NotGranted,
        #[serde(rename = "UNKNOWN_CONDITIONAL")]
        #[doc = "The member has the permission only if a condition expression evaluates to `true`."]
        UnknownConditional,
        #[serde(rename = "UNKNOWN_INFO_DENIED")]
        #[doc = "The user who created the Replay does not have access to all of the policies that Policy Simulator needs to evaluate."]
        UnknownInfoDenied,
    }
    impl ::std::default::Default for GoogleCloudPolicysimulatorV1beta1ExplainedPolicyAccessEnum {
        fn default() -> Self {
            Self::AccessStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The relevance of this policy to the overall determination in the TroubleshootIamPolicyResponse. If the user who created the Replay does not have access to the policy, this field is omitted."]
    pub enum GoogleCloudPolicysimulatorV1beta1ExplainedPolicyRelevanceEnum {
        #[serde(rename = "HEURISTIC_RELEVANCE_UNSPECIFIED")]
        #[doc = "Reserved for future use."]
        HeuristicRelevanceUnspecified,
        #[serde(rename = "NORMAL")]
        #[doc = "The data point has a limited effect on the result. Changing the data point is unlikely to affect the overall determination."]
        Normal,
        #[serde(rename = "HIGH")]
        #[doc = "The data point has a strong effect on the result. Changing the data point is likely to affect the overall determination."]
        High,
    }
    impl ::std::default::Default for GoogleCloudPolicysimulatorV1beta1ExplainedPolicyRelevanceEnum {
        fn default() -> Self {
            Self::HeuristicRelevanceUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for Simulator.ListReplayResults."]
    pub struct GoogleCloudPolicysimulatorV1beta1ListReplayResultsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token that you can use to retrieve the next page of ReplayResult objects. If this field is omitted, there are no subsequent pages."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replayResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The results of running a Replay."]
        pub replay_results: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudPolicysimulatorV1beta1ReplayResult>>,
        >,
    }
    impl GoogleCloudPolicysimulatorV1beta1ListReplayResultsResponse {
        pub fn builder() -> GoogleCloudPolicysimulatorV1beta1ListReplayResultsResponseBuilder {
            GoogleCloudPolicysimulatorV1beta1ListReplayResultsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A resource describing a `Replay`, or simulation."]
    pub struct GoogleCloudPolicysimulatorV1beta1Replay {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "config")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The configuration used for the `Replay`."]
        pub config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudPolicysimulatorV1beta1ReplayConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The resource name of the `Replay`, which has the following format: `{projects|folders|organizations}/{resource-id}/locations/global/replays/{replay-id}`, where `{resource-id}` is the ID of the project, folder, or organization that owns the Replay. Example: `projects/my-example-project/locations/global/replays/506a5f7f-38ce-4d7d-8e03-479ce1833c36`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resultsSummary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Summary statistics about the replayed log entries."]
        pub results_summary: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudPolicysimulatorV1beta1ReplayResultsSummary>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The current state of the `Replay`."]
        pub state: ::std::option::Option<GoogleCloudPolicysimulatorV1beta1ReplayStateEnum>,
    }
    impl GoogleCloudPolicysimulatorV1beta1Replay {
        pub fn builder() -> GoogleCloudPolicysimulatorV1beta1ReplayBuilder {
            GoogleCloudPolicysimulatorV1beta1ReplayBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The current state of the `Replay`."]
    pub enum GoogleCloudPolicysimulatorV1beta1ReplayStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "The state is unspecified."]
        StateUnspecified,
        #[serde(rename = "PENDING")]
        #[doc = "The `Replay` has not started yet."]
        Pending,
        #[serde(rename = "RUNNING")]
        #[doc = "The `Replay` is currently running."]
        Running,
        #[serde(rename = "SUCCEEDED")]
        #[doc = "The `Replay` has successfully completed."]
        Succeeded,
        #[serde(rename = "FAILED")]
        #[doc = "The `Replay` has finished with an error."]
        Failed,
    }
    impl ::std::default::Default for GoogleCloudPolicysimulatorV1beta1ReplayStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The configuration used for a Replay."]
    pub struct GoogleCloudPolicysimulatorV1beta1ReplayConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The logs to use as input for the Replay."]
        pub log_source:
            ::std::option::Option<GoogleCloudPolicysimulatorV1beta1ReplayConfigLogSourceEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policyOverlay")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A mapping of the resources that you want to simulate policies for and the policies that you want to simulate. Keys are the full resource names for the resources. For example, `//cloudresourcemanager.googleapis.com/projects/my-project`. For examples of full resource names for Google Cloud services, see https://cloud.google.com/iam/help/troubleshooter/full-resource-names. Values are Policy objects representing the policies that you want to simulate. Replays automatically take into account any IAM policies inherited through the resource hierarchy, and any policies set on descendant resources. You do not need to include these policies in the policy overlay."]
        pub policy_overlay: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<GoogleIamV1Policy>>,
        >,
    }
    impl GoogleCloudPolicysimulatorV1beta1ReplayConfig {
        pub fn builder() -> GoogleCloudPolicysimulatorV1beta1ReplayConfigBuilder {
            GoogleCloudPolicysimulatorV1beta1ReplayConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The logs to use as input for the Replay."]
    pub enum GoogleCloudPolicysimulatorV1beta1ReplayConfigLogSourceEnum {
        #[serde(rename = "LOG_SOURCE_UNSPECIFIED")]
        #[doc = "An unspecified log source. If the log source is unspecified, the Replay defaults to using `RECENT_ACCESSES`."]
        LogSourceUnspecified,
        #[serde(rename = "RECENT_ACCESSES")]
        #[doc = "All access logs from the last 90 days. These logs may not include logs from the most recent 7 days."]
        RecentAccesses,
    }
    impl ::std::default::Default for GoogleCloudPolicysimulatorV1beta1ReplayConfigLogSourceEnum {
        fn default() -> Self {
            Self::LogSourceUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The difference between the results of evaluating an access tuple under the current (baseline) policies and under the proposed (simulated) policies. This difference explains how a member's access could change if the proposed policies were applied."]
    pub struct GoogleCloudPolicysimulatorV1beta1ReplayDiff {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessDiff")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A summary and comparison of the member's access under the current (baseline) policies and the proposed (simulated) policies for a single access tuple. The evaluation of the member's access is reported in the AccessState field."]
        pub access_diff: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudPolicysimulatorV1beta1AccessStateDiff>,
        >,
    }
    impl GoogleCloudPolicysimulatorV1beta1ReplayDiff {
        pub fn builder() -> GoogleCloudPolicysimulatorV1beta1ReplayDiffBuilder {
            GoogleCloudPolicysimulatorV1beta1ReplayDiffBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata about a Replay operation."]
    pub struct GoogleCloudPolicysimulatorV1beta1ReplayOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time when the request was received."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudPolicysimulatorV1beta1ReplayOperationMetadata {
        pub fn builder() -> GoogleCloudPolicysimulatorV1beta1ReplayOperationMetadataBuilder {
            GoogleCloudPolicysimulatorV1beta1ReplayOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of replaying a single access tuple against a simulated state."]
    pub struct GoogleCloudPolicysimulatorV1beta1ReplayResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessTuple")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The access tuple that was replayed. This field includes information about the member, resource, and permission that were involved in the access attempt."]
        pub access_tuple:
            ::std::option::Option<::std::boxed::Box<GoogleCloudPolicysimulatorV1beta1AccessTuple>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diff")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The difference between the member's access under the current (baseline) policies and the member's access under the proposed (simulated) policies. This field is only included for access tuples that were successfully replayed and had different results under the current policies and the proposed policies."]
        pub diff:
            ::std::option::Option<::std::boxed::Box<GoogleCloudPolicysimulatorV1beta1ReplayDiff>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error that caused the access tuple replay to fail. This field is only included for access tuples that were not replayed successfully."]
        pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastSeenDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The latest date this access tuple was seen in the logs."]
        pub last_seen_date: ::std::option::Option<::std::boxed::Box<GoogleTypeDate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the `ReplayResult`, in the following format: `{projects|folders|organizations}/{resource-id}/locations/global/replays/{replay-id}/results/{replay-result-id}`, where `{resource-id}` is the ID of the project, folder, or organization that owns the Replay. Example: `projects/my-example-project/locations/global/replays/506a5f7f-38ce-4d7d-8e03-479ce1833c36/results/1234`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Replay that the access tuple was included in."]
        pub parent: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudPolicysimulatorV1beta1ReplayResult {
        pub fn builder() -> GoogleCloudPolicysimulatorV1beta1ReplayResultBuilder {
            GoogleCloudPolicysimulatorV1beta1ReplayResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Summary statistics about the replayed log entries."]
    pub struct GoogleCloudPolicysimulatorV1beta1ReplayResultsSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "differenceCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of replayed log entries with a difference between baseline and simulated policies."]
        pub difference_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of log entries that could not be replayed."]
        pub error_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of log entries replayed."]
        pub log_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newestDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date of the newest log entry replayed."]
        pub newest_date: ::std::option::Option<::std::boxed::Box<GoogleTypeDate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oldestDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date of the oldest log entry replayed."]
        pub oldest_date: ::std::option::Option<::std::boxed::Box<GoogleTypeDate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unchangedCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of replayed log entries with no difference between baseline and simulated policies."]
        pub unchanged_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudPolicysimulatorV1beta1ReplayResultsSummary {
        pub fn builder() -> GoogleCloudPolicysimulatorV1beta1ReplayResultsSummaryBuilder {
            GoogleCloudPolicysimulatorV1beta1ReplayResultsSummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { \"audit_configs\": [ { \"service\": \"allServices\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" }, { \"log_type\": \"ADMIN_READ\" } ] }, { \"service\": \"sampleservice.googleapis.com\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\" }, { \"log_type\": \"DATA_WRITE\", \"exempted_members\": [ \"user:aliya@example.com\" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging."]
    pub struct GoogleIamV1AuditConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "auditLogConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The configuration for logging of each type of permission."]
        pub audit_log_configs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleIamV1AuditLogConfig>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "service")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services."]
        pub service: ::std::option::Option<::std::string::String>,
    }
    impl GoogleIamV1AuditConfig {
        pub fn builder() -> GoogleIamV1AuditConfigBuilder {
            GoogleIamV1AuditConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides the configuration for logging a type of permissions. Example: { \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging."]
    pub struct GoogleIamV1AuditLogConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exemptedMembers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members."]
        pub exempted_members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The log type that this config enables."]
        pub log_type: ::std::option::Option<GoogleIamV1AuditLogConfigLogTypeEnum>,
    }
    impl GoogleIamV1AuditLogConfig {
        pub fn builder() -> GoogleIamV1AuditLogConfigBuilder {
            GoogleIamV1AuditLogConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The log type that this config enables."]
    pub enum GoogleIamV1AuditLogConfigLogTypeEnum {
        #[serde(rename = "LOG_TYPE_UNSPECIFIED")]
        #[doc = "Default case. Should never be this."]
        LogTypeUnspecified,
        #[serde(rename = "ADMIN_READ")]
        #[doc = "Admin reads. Example: CloudIAM getIamPolicy"]
        AdminRead,
        #[serde(rename = "DATA_WRITE")]
        #[doc = "Data writes. Example: CloudSQL Users create"]
        DataWrite,
        #[serde(rename = "DATA_READ")]
        #[doc = "Data reads. Example: CloudSQL Users list"]
        DataRead,
    }
    impl ::std::default::Default for GoogleIamV1AuditLogConfigLogTypeEnum {
        fn default() -> Self {
            Self::LogTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Associates `members` with a `role`."]
    pub struct GoogleIamV1Binding {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the members in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        pub condition: ::std::option::Option<::std::boxed::Box<GoogleTypeExpr>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "members")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the identities requesting access for a Cloud Platform resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. "]
        pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Role that is assigned to `members`. For example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
        pub role: ::std::option::Option<::std::string::String>,
    }
    impl GoogleIamV1Binding {
        pub fn builder() -> GoogleIamV1BindingBuilder {
            GoogleIamV1BindingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/)."]
    pub struct GoogleIamV1Policy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "auditConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies cloud audit logging configuration for this policy."]
        pub audit_configs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleIamV1AuditConfig>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bindings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member."]
        pub bindings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleIamV1Binding>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        pub version: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleIamV1Policy {
        pub fn builder() -> GoogleIamV1PolicyBuilder {
            GoogleIamV1PolicyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Operations.ListOperations."]
    pub struct GoogleLongrunningListOperationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of operations that matches the specified filter in the request."]
        pub operations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleLongrunningOperation>>>,
    }
    impl GoogleLongrunningListOperationsResponse {
        pub fn builder() -> GoogleLongrunningListOperationsResponseBuilder {
            GoogleLongrunningListOperationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This resource represents a long-running operation that is the result of a network API call."]
    pub struct GoogleLongrunningOperation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "done")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
        pub done: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error result of the operation in case of failure or cancellation."]
        pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
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
    impl GoogleLongrunningOperation {
        pub fn builder() -> GoogleLongrunningOperationBuilder {
            GoogleLongrunningOperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub struct GoogleRpcStatus {
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
    impl GoogleRpcStatus {
        pub fn builder() -> GoogleRpcStatusBuilder {
            GoogleRpcStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."]
    pub struct GoogleTypeDate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "day")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
        pub day: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "month")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
        pub month: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "year")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
        pub year: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleTypeDate {
        pub fn builder() -> GoogleTypeDateBuilder {
            GoogleTypeDateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() < 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\" expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description: \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information."]
    pub struct GoogleTypeExpr {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Textual representation of an expression in Common Expression Language syntax."]
        pub expression: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleTypeExpr {
        pub fn builder() -> GoogleTypeExprBuilder {
            GoogleTypeExprBuilder::default()
        }
    }
}
