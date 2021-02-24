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
pub mod resources {
    pub mod organizations {
        pub mod methods {
            pub mod create {
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
                    #[serde(rename = "parent")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. Name of the GCP project in which to associate the Apigee organization. Pass the information as a query parameter using the following structure in your request: `projects/`"]
                    pub parent: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod get_deployed_ingress_config {
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
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "When set to FULL, additional details about the specific deployments receiving traffic will be included in the IngressConfig response's RoutingRules."]
                    pub view: ::std::option::Option<QueryParametersViewEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "When set to FULL, additional details about the specific deployments receiving traffic will be included in the IngressConfig response's RoutingRules."]
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "INGRESS_CONFIG_VIEW_UNSPECIFIED")]
                    #[doc = "The default/unset value. The API will default to the BASIC view."]
                    IngressConfigViewUnspecified,
                    #[serde(rename = "BASIC")]
                    #[doc = "Include all ingress config data necessary for the runtime to configure ingress, but no more. Routing rules will include only basepath and destination environment. This the default value."]
                    Basic,
                    #[serde(rename = "FULL")]
                    #[doc = "Include all ingress config data, including internal debug info for each routing rule such as the proxy claiming a particular basepath and when the routing rule first appeared in the env group."]
                    Full,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::IngressConfigViewUnspecified
                    }
                }
            }
        }
        pub mod resources {
            pub mod analytics {
                pub mod resources {
                    pub mod datastores {
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
                                    #[serde(rename = "targetType")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. TargetType is used to fetch all Datastores that match the type"]
                                    pub target_type: ::std::option::Option<::std::string::String>,
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
            pub mod apiproducts {
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
                            #[serde(rename = "attributename")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Name of the attribute used to filter the search."]
                            pub attributename: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "attributevalue")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Value of the attribute used to filter the search."]
                            pub attributevalue: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "count")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Enter the number of API products you want returned in the API call. The limit is 1000."]
                            pub count: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "expand")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Flag that specifies whether to expand the results. Set to `true` to get expanded details about each API."]
                            pub expand: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startKey")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Gets a list of API products starting with a specific API product in the list. For example, if you're returning 50 API products at a time (using the `count` query parameter), you can view products 50-99 by entering the name of the 50th API product in the first API (without using `startKey`). Product name is case sensitive."]
                            pub start_key: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod apis {
                pub mod methods {
                    pub mod create {
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
                            #[serde(rename = "action")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Action to perform when importing an API proxy configuration bundle. Set this parameter to one of the following values: * `import` to import the API proxy configuration bundle. * `validate` to validate the API proxy configuration bundle without importing it."]
                            pub action: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "name")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Name of the API proxy. Restrict the characters used to: A-Za-z0-9._-"]
                            pub name: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "validate")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Ignored. All uploads are validated regardless of the value of this field. Maintained for compatibility with Apigee Edge API."]
                            pub validate: ::std::option::Option<::std::primitive::bool>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
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
                            #[serde(rename = "includeMetaData")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Flag that specifies whether to include API proxy metadata in the response."]
                            pub include_meta_data: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "includeRevisions")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Flag that specifies whether to include a list of revisions in the response."]
                            pub include_revisions: ::std::option::Option<::std::primitive::bool>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod revisions {
                        pub mod methods {
                            pub mod get {
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
                                    #[serde(rename = "format")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Format used when downloading the API proxy configuration revision. Set to `bundle` to download the API proxy configuration revision as a zip file."]
                                    pub format: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                            pub mod update_api_proxy_revision {
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
                                    #[serde(rename = "validate")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Ignored. All uploads are validated regardless of the value of this field. Maintained for compatibility with Apigee Edge API."]
                                    pub validate: ::std::option::Option<::std::primitive::bool>,
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
            pub mod apps {
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
                            #[serde(rename = "apiProduct")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "API product."]
                            pub api_product: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "apptype")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Filter by the type of the app. Valid values are `company` or `developer`. Defaults to `developer`."]
                            pub apptype: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "expand")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Flag that specifies whether to return an expanded list of apps for the organization. Defaults to `false`."]
                            pub expand: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "ids")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Comma-separated list of app IDs on which to filter."]
                            pub ids: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "includeCred")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Flag that specifies whether to include credentials in the response."]
                            pub include_cred: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "keyStatus")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Key status of the app. Valid values include `approved` or `revoked`. Defaults to `approved`."]
                            pub key_status: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "rows")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Maximum number of app IDs to return. Defaults to 10000."]
                            pub rows: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startKey")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Returns the list of apps starting from the specified app ID."]
                            pub start_key: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "status")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Filter by the status of the app. Valid values are `approved` or `revoked`. Defaults to `approved`."]
                            pub status: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod datacollectors {
                pub mod methods {
                    pub mod create {
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
                            #[serde(rename = "dataCollectorId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "ID of the data collector. Overrides any ID in the data collector resource. Must begin with `dc_`."]
                            pub data_collector_id: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
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
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of data collectors to return. The page size defaults to 25."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Page token, returned from a previous ListDataCollectors call, that you can use to retrieve the next page."]
                            pub page_token: ::std::option::Option<::std::string::String>,
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
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "List of fields to be updated."]
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
            pub mod deployments {
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
                            #[serde(rename = "sharedFlows")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Flag that specifies whether to return shared flow or API proxy deployments. Set to `true` to return shared flow deployments; set to `false` to return API proxy deployments. Defaults to `false`."]
                            pub shared_flows: ::std::option::Option<::std::primitive::bool>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod developers {
                pub mod methods {
                    pub mod get {
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
                            #[serde(rename = "action")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Status of the developer. Valid values are `active` or `inactive`."]
                            pub action: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
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
                            #[serde(rename = "app")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. List only Developers that are associated with the app. Note that start_key, count are not applicable for this filter criteria."]
                            pub app: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "count")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Number of developers to return in the API call. Use with the `startKey` parameter to provide more targeted filtering. The limit is 1000."]
                            pub count: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "expand")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Specifies whether to expand the results. Set to `true` to expand the results. This query parameter is not valid if you use the `count` or `startKey` query parameters."]
                            pub expand: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "ids")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. List of IDs to include, separated by commas."]
                            pub ids: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "includeCompany")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Flag that specifies whether to include company details in the response."]
                            pub include_company: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startKey")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "**Note**: Must be used in conjunction with the `count` parameter. Email address of the developer from which to start displaying the list of developers. For example, if the an unfiltered list returns: ``` westley@example.com fezzik@example.com buttercup@example.com ``` and your `startKey` is `fezzik@example.com`, the list returned will be ``` fezzik@example.com buttercup@example.com ```"]
                            pub start_key: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod set_developer_status {
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
                            #[serde(rename = "action")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Status of the developer. Valid values are `active` and `inactive`."]
                            pub action: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod apps {
                        pub mod methods {
                            pub mod generate_key_pair_or_update_developer_app_status {
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
                                    #[serde(rename = "action")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Action. Valid values are `approve` or `revoke`."]
                                    pub action: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                            pub mod get {
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
                                    #[serde(rename = "entity")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "**Note**: Must be used in conjunction with the `query` parameter. Set to `apiresources` to return the number of API resources that have been approved for access by a developer app in the specified Apigee organization."]
                                    pub entity: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "query")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "**Note**: Must be used in conjunction with the `entity` parameter. Set to `count` to return the number of API resources that have been approved for access by a developer app in the specified Apigee organization."]
                                    pub query: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
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
                                    #[serde(rename = "count")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Number of developer apps to return in the API call. Use with the `startKey` parameter to provide more targeted filtering. The limit is 1000."]
                                    pub count: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "expand")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. Specifies whether to expand the results. Set to `true` to expand the results. This query parameter is not valid if you use the `count` or `startKey` query parameters."]
                                    pub expand: ::std::option::Option<::std::primitive::bool>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "shallowExpand")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. Specifies whether to expand the results in shallow mode. Set to `true` to expand the results in shallow mode."]
                                    pub shallow_expand:
                                        ::std::option::Option<::std::primitive::bool>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "startKey")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "**Note**: Must be used in conjunction with the `count` parameter. Name of the developer app from which to start displaying the list of developer apps. For example, if you're returning 50 developer apps at a time (using the `count` query parameter), you can view developer apps 50-99 by entering the name of the 50th developer app. The developer app name is case sensitive."]
                                    pub start_key: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                        pub mod resources {
                            pub mod keys {
                                pub mod methods {
                                    pub mod update_developer_app_key {
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
                                            #[serde(rename = "action")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Approve or revoke the consumer key by setting this value to `approve` or `revoke`, respectively."]
                                            pub action:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                }
                                pub mod resources {
                                    pub mod apiproducts {
                                        pub mod methods {
                                            pub mod update_developer_app_key_api_product {
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
                                                    #[serde(rename = "action")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "Approve or revoke the consumer key by setting this value to `approve` or `revoke`, respectively."]
                                                    pub action: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
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
            pub mod envgroups {
                pub mod methods {
                    pub mod create {
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
                            #[serde(rename = "name")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "ID of the environment group. Overrides any ID in the environment_group resource."]
                            pub name: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
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
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of environment groups to return. The page size defaults to 25."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Page token, returned from a previous ListEnvironmentGroups call, that you can use to retrieve the next page."]
                            pub page_token: ::std::option::Option<::std::string::String>,
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
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "List of fields to be updated."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod attachments {
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
                                    #[doc = "Maximum number of environment group attachments to return. The page size defaults to 25."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Page token, returned by a previous ListEnvironmentGroupAttachments call, that you can use to retrieve the next page."]
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
                }
            }
            pub mod environments {
                pub mod methods {
                    pub mod create {
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
                            #[serde(rename = "name")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Name of the environment. Alternatively, the name may be specified in the request body in the name field."]
                            pub name: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod get_iam_policy {
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
                            #[serde(rename = "options.requestedPolicyVersion")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The policy format version to be returned. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional bindings must specify version 3. Policies without any conditional bindings may specify any valid value or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
                            pub options_requested_policy_version:
                                ::std::option::Option<::std::primitive::i64>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod update_debugmask {
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
                            #[serde(rename = "replaceRepeatedFields")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Boolean flag that specifies whether to replace existing values in the debug mask when doing an update. Set to true to replace existing values. The default behavior is to append the values (false)."]
                            pub replace_repeated_fields:
                                ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Field debug mask to support partial updates."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod analytics {
                        pub mod resources {
                            pub mod admin {
                                pub mod methods {
                                    pub mod get_schemav2 {
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
                                            #[serde(rename = "type")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Required. Type refers to the dataset name whose schema needs to be retrieved E.g. type=fact or type=agg_cus1"]
                                            pub _type: ::std::option::Option<::std::string::String>,
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
                    pub mod apis {
                        pub mod resources {
                            pub mod revisions {
                                pub mod methods {
                                    pub mod deploy {
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
                                            #[serde(rename = "override")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Flag that specifies whether the new deployment replaces other deployed revisions of the API proxy in the environment. Set override to true to replace other deployed revisions. By default, override is false and the deployment is rejected if other revisions of the API proxy are deployed in the environment."]
                                            pub _override:
                                                ::std::option::Option<::std::primitive::bool>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "sequencedRollout")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "If true, a best-effort attempt will be made to roll out the routing rules corresponding to this deployment and the environment changes to add this deployment in a safe order. This reduces the risk of downtime that could be caused by changing the environment group's routing before the new destination for the affected traffic is ready to receive it. This should only be necessary if the new deployment will be capturing traffic from another environment under a shared environment group or if traffic will be rerouted to a different environment due to a basepath removal. The GenerateDeployChangeReport API may be used to examine routing changes before issuing the deployment request, and its response will indicate if a sequenced rollout is recommended for the deployment."]
                                            pub sequenced_rollout:
                                                ::std::option::Option<::std::primitive::bool>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                    pub mod undeploy {
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
                                            #[serde(rename = "sequencedRollout")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "If true, a best-effort attempt will be made to remove the environment group routing rules corresponding to this deployment before removing the deployment from the runtime. This is likely to be a rare use case; it is only needed when the intended effect of undeploying this proxy is to cause the traffic it currently handles to be rerouted to some other existing proxy in the environment group. The GenerateUndeployChangeReport API may be used to examine routing changes before issuing the undeployment request, and its response will indicate if a sequenced rollout is recommended for the undeployment."]
                                            pub sequenced_rollout:
                                                ::std::option::Option<::std::primitive::bool>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                }
                                pub mod resources {
                                    pub mod debugsessions {
                                        pub mod methods {
                                            pub mod create {
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
                                                    #[doc = "Optional. The time in seconds after which this DebugSession should end. A timeout specified in DebugSession will overwrite this value."]
                                                    pub timeout: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
                                                }
                                                impl QueryParameters {
                                                    pub fn builder() -> QueryParametersBuilder {
                                                        QueryParametersBuilder::default()
                                                    }
                                                }
                                            }
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
                                                    #[doc = "Maximum number of debug sessions to return. The page size defaults to 25."]
                                                    pub page_size: ::std::option::Option<
                                                        ::std::primitive::i64,
                                                    >,
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "pageToken")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "Page token, returned from a previous ListDebugSessions call, that you can use to retrieve the next page."]
                                                    pub page_token: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
                                                }
                                                impl QueryParameters {
                                                    pub fn builder() -> QueryParametersBuilder {
                                                        QueryParametersBuilder::default()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    pub mod deployments {
                                        pub mod methods {
                                            pub mod generate_deploy_change_report {
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
                                                    #[serde(rename = "override")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "Flag that specifies whether to force the deployment of the new revision over the currently deployed revision by overriding conflict checks."]
                                                    pub _override: ::std::option::Option<
                                                        ::std::primitive::bool,
                                                    >,
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
                    pub mod deployments {
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
                                    #[serde(rename = "sharedFlows")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. Flag that specifies whether to return shared flow or API proxy deployments. Set to `true` to return shared flow deployments; set to `false` to return API proxy deployments. Defaults to `false`."]
                                    pub shared_flows: ::std::option::Option<::std::primitive::bool>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                    }
                    pub mod keystores {
                        pub mod methods {
                            pub mod create {
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
                                    #[serde(rename = "name")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. Name of the keystore. Overrides the value in Keystore."]
                                    pub name: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                        pub mod resources {
                            pub mod aliases {
                                pub mod methods {
                                    pub mod create {
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
                                            #[serde(rename = "_password")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "DEPRECATED: For improved security, specify the password in the request body instead of using the query parameter. To specify the password in the request body, set `Content-type: multipart/form-data` part with name `password`. Password for the private key file, if required."]
                                            pub password:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "alias")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Alias for the key/certificate pair. Values must match the regular expression `[\\w\\s-.]{1,255}`. This must be provided for all formats except `selfsignedcert`; self-signed certs may specify the alias in either this parameter or the JSON body."]
                                            pub alias: ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "format")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Required. Format of the data. Valid values include: `selfsignedcert`, `keycertfile`, or `pkcs12`"]
                                            pub format:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "ignoreExpiryValidation")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Flag that specifies whether to ignore expiry validation. If set to `true`, no expiry validation will be performed."]
                                            pub ignore_expiry_validation:
                                                ::std::option::Option<::std::primitive::bool>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "ignoreNewlineValidation")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Flag that specifies whether to ignore newline validation. If set to `true`, no error is thrown when the file contains a certificate chain with no newline between each certificate. Defaults to `false`."]
                                            pub ignore_newline_validation:
                                                ::std::option::Option<::std::primitive::bool>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                    pub mod update {
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
                                            #[serde(rename = "ignoreExpiryValidation")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Required. Flag that specifies whether to ignore expiry validation. If set to `true`, no expiry validation will be performed."]
                                            pub ignore_expiry_validation:
                                                ::std::option::Option<::std::primitive::bool>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "ignoreNewlineValidation")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Flag that specifies whether to ignore newline validation. If set to `true`, no error is thrown when the file contains a certificate chain with no newline between each certificate. Defaults to `false`."]
                                            pub ignore_newline_validation:
                                                ::std::option::Option<::std::primitive::bool>,
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
                    pub mod optimized_stats {
                        pub mod methods {
                            pub mod get {
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
                                    #[serde(rename = "accuracy")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Legacy field: not used anymore."]
                                    pub accuracy: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "aggTable")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "If customers want to query custom aggregate tables, then this parameter can be used to specify the table name. If this parameter is skipped, then Edge Query will try to retrieve the data from fact tables which will be expensive."]
                                    pub agg_table: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "filter")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Enables drill-down on specific dimension values."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "limit")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "This parameter is used to limit the number of result items. Default and the max value is 14400."]
                                    pub limit: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "offset")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Use offset with limit to enable pagination of results. For example, to display results 11-20, set limit to '10' and offset to '10'."]
                                    pub offset: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "realtime")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Legacy field: not used anymore."]
                                    pub realtime: ::std::option::Option<::std::primitive::bool>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "select")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Required. The select parameter contains a comma separated list of metrics. E.g. sum(message_count),sum(error_count)"]
                                    pub select: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "sonar")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "This parameter routes the query to api monitoring service for last hour."]
                                    pub sonar: ::std::option::Option<::std::primitive::bool>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "sort")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "This parameter specifies if the sort order should be ascending or descending Supported values are DESC and ASC."]
                                    pub sort: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "sortby")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Comma separated list of columns to sort the final result."]
                                    pub sortby: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "timeRange")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Required. Time interval for the interactive query. Time range is specified as start~end E.g. 04/15/2017 00:00~05/15/2017 23:59"]
                                    pub time_range: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "timeUnit")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A value of second, minute, hour, day, week, month. Time Unit specifies the granularity of metrics returned."]
                                    pub time_unit: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "topk")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Take 'top k' results from results, for example, to return the top 5 results 'topk=5'."]
                                    pub topk: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "tsAscending")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Lists timestamps in ascending order if set to true. Recommend setting this value to true if you are using sortby with sort=DESC."]
                                    pub ts_ascending: ::std::option::Option<::std::primitive::bool>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "tzo")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "This parameters contains the timezone offset value."]
                                    pub tzo: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                    }
                    pub mod queries {
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
                                    #[serde(rename = "dataset")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Filter response list by dataset. Example: `api`, `mint`"]
                                    pub dataset: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "from")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Filter response list by returning asynchronous queries that created after this date time. Time must be in ISO date-time format like '2011-12-03T10:15:30Z'."]
                                    pub from: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "inclQueriesWithoutReport")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Flag to include asynchronous queries that don't have a report denifition."]
                                    pub incl_queries_without_report:
                                        ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "status")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Filter response list by asynchronous query status."]
                                    pub status: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "submittedBy")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Filter response list by user who submitted queries."]
                                    pub submitted_by: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "to")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Filter response list by returning asynchronous queries that created before this date time. Time must be in ISO date-time format like '2011-12-03T10:16:30Z'."]
                                    pub to: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                    }
                    pub mod resourcefiles {
                        pub mod methods {
                            pub mod create {
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
                                    #[serde(rename = "name")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Required. Name of the resource file. Must match the regular expression: [a-zA-Z0-9:/\\\\!@#$%^&{}\\[\\]()+\\-=,.~'` ]{1,255}"]
                                    pub name: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "type")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Required. Resource file type. {{ resource_file_type }}"]
                                    pub _type: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
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
                                    #[serde(rename = "type")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. Type of resource files to list. {{ resource_file_type }}"]
                                    pub _type: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                    }
                    pub mod sharedflows {
                        pub mod resources {
                            pub mod revisions {
                                pub mod methods {
                                    pub mod deploy {
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
                                            #[serde(rename = "override")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Flag that specifies whether to force the deployment of the new revision over the currently deployed revision by overriding conflict checks. If an existing shared flow revision is deployed, to ensure seamless deployment with no downtime, set this parameter to `true`. In this case, hybrid deploys the new revision fully before undeploying the existing revision. If set to `false`, you must undeploy the existing revision before deploying the new revision."]
                                            pub _override:
                                                ::std::option::Option<::std::primitive::bool>,
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
                    pub mod stats {
                        pub mod methods {
                            pub mod get {
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
                                    #[serde(rename = "accuracy")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Legacy field: not used anymore. This field is present to support UI calls which still use this parameter."]
                                    pub accuracy: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "aggTable")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "If customers want to query custom aggregate tables, then this parameter can be used to specify the table name. If this parameter is skipped, then Edge Query will try to retrieve the data from fact tables which will be expensive."]
                                    pub agg_table: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "filter")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Enables drill-down on specific dimension values"]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "limit")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "This parameter is used to limit the number of result items. Default and the max value is 14400."]
                                    pub limit: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "offset")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Use offset with limit to enable pagination of results. For example, to display results 11-20, set limit to '10' and offset to '10'."]
                                    pub offset: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "realtime")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Legacy field: not used anymore."]
                                    pub realtime: ::std::option::Option<::std::primitive::bool>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "select")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The select parameter contains a comma separated list of metrics. E.g. sum(message_count),sum(error_count)"]
                                    pub select: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "sonar")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "This parameter routes the query to api monitoring service for last hour."]
                                    pub sonar: ::std::option::Option<::std::primitive::bool>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "sort")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "This parameter specifies if the sort order should be ascending or descending Supported values are DESC and ASC."]
                                    pub sort: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "sortby")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Comma separated list of columns to sort the final result."]
                                    pub sortby: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "timeRange")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Time interval for the interactive query. Time range is specified as start~end E.g. 04/15/2017 00:00~05/15/2017 23:59"]
                                    pub time_range: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "timeUnit")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A value of second, minute, hour, day, week, month. Time Unit specifies the granularity of metrics returned."]
                                    pub time_unit: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "topk")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Take 'top k' results from results, for example, to return the top 5 results 'topk=5'."]
                                    pub topk: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "tsAscending")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Lists timestamps in ascending order if set to true. Recommend setting this value to true if you are using sortby with sort=DESC."]
                                    pub ts_ascending: ::std::option::Option<::std::primitive::bool>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "tzo")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "This parameters contains the timezone offset value."]
                                    pub tzo: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                    }
                    pub mod targetservers {
                        pub mod methods {
                            pub mod create {
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
                                    #[serde(rename = "name")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. The ID to give the TargetServer. This will overwrite the value in TargetServer."]
                                    pub name: ::std::option::Option<::std::string::String>,
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
            pub mod host_queries {
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
                            #[serde(rename = "dataset")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Filter response list by dataset. Example: `api`, `mint`"]
                            pub dataset: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "envgroupHostname")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. Filter response list by hostname."]
                            pub envgroup_hostname: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "from")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Filter response list by returning asynchronous queries that created after this date time. Time must be in ISO date-time format like '2011-12-03T10:15:30Z'."]
                            pub from: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "inclQueriesWithoutReport")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Flag to include asynchronous queries that don't have a report denifition."]
                            pub incl_queries_without_report:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "status")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Filter response list by asynchronous query status."]
                            pub status: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "submittedBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Filter response list by user who submitted queries."]
                            pub submitted_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "to")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Filter response list by returning asynchronous queries that created before this date time. Time must be in ISO date-time format like '2011-12-03T10:16:30Z'."]
                            pub to: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod host_stats {
                pub mod methods {
                    pub mod get {
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
                            #[serde(rename = "accuracy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Legacy field: not used anymore."]
                            pub accuracy: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "envgroupHostname")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. The hostname for which the interactive query will be executed."]
                            pub envgroup_hostname: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Enables drill-down on specific dimension values."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "limit")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "This parameter is used to limit the number of result items. Default and the max value is 14400."]
                            pub limit: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "offset")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Use offset with limit to enable pagination of results. For example, to display results 11-20, set limit to '10' and offset to '10'."]
                            pub offset: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "realtime")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Legacy field: not used anymore."]
                            pub realtime: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "select")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The select parameter contains a comma separated list of metrics. E.g. sum(message_count),sum(error_count)"]
                            pub select: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "sort")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "This parameter specifies if the sort order should be ascending or descending Supported values are DESC and ASC."]
                            pub sort: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "sortby")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Comma separated list of columns to sort the final result."]
                            pub sortby: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "timeRange")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Time interval for the interactive query. Time range is specified as start~end E.g. 04/15/2017 00:00~05/15/2017 23:59"]
                            pub time_range: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "timeUnit")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A value of second, minute, hour, day, week, month. Time Unit specifies the granularity of metrics returned."]
                            pub time_unit: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "topk")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Take 'top k' results from results, for example, to return the top 5 results 'topk=5'."]
                            pub topk: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "tsAscending")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Lists timestamps in ascending order if set to true. Recommend setting this value to true if you are using sortby with sort=DESC."]
                            pub ts_ascending: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "tzo")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "This parameters contains the timezone offset value."]
                            pub tzo: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod instances {
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
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of instances to return. Defaults to 25."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Page token, returned from a previous ListInstances call, that you can use to retrieve the next page of content."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod attachments {
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
                                    #[doc = "Maximum number of instance attachments to return. Defaults to 25."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Page token, returned by a previous ListInstanceAttachments call, that you can use to retrieve the next page of content."]
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
                    pub mod nat_addresses {
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
                                    #[doc = "Maximum number of natAddresses to return. Defaults to 25."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Page token, returned from a previous ListNatAddresses call, that you can use to retrieve the next page of content."]
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list filter."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list page size."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
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
            pub mod optimized_host_stats {
                pub mod methods {
                    pub mod get {
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
                            #[serde(rename = "accuracy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Legacy field: not used anymore."]
                            pub accuracy: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "envgroupHostname")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. The hostname for which the interactive query will be executed."]
                            pub envgroup_hostname: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Enables drill-down on specific dimension values."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "limit")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "This parameter is used to limit the number of result items. Default and the max value is 14400."]
                            pub limit: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "offset")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Use offset with limit to enable pagination of results. For example, to display results 11-20, set limit to '10' and offset to '10'."]
                            pub offset: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "realtime")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Legacy field: not used anymore."]
                            pub realtime: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "select")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. The select parameter contains a comma separated list of metrics. E.g. sum(message_count),sum(error_count)"]
                            pub select: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "sort")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "This parameter specifies if the sort order should be ascending or descending Supported values are DESC and ASC."]
                            pub sort: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "sortby")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Comma separated list of columns to sort the final result."]
                            pub sortby: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "timeRange")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. Time interval for the interactive query. Time range is specified as start~end. E.g 04/15/2017 00:00~05/15/2017 23:59."]
                            pub time_range: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "timeUnit")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A value of second, minute, hour, day, week, month. Time Unit specifies the granularity of metrics returned."]
                            pub time_unit: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "topk")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Take 'top k' results from results, for example, to return the top 5 results 'topk=5'."]
                            pub topk: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "tsAscending")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Lists timestamps in ascending order if set to true. Recommend setting this value to true if you are using sortby with sort=DESC."]
                            pub ts_ascending: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "tzo")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "This parameters contains the timezone offset value."]
                            pub tzo: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod reports {
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
                            #[serde(rename = "expand")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Set to 'true' to get expanded details about each custom report."]
                            pub expand: ::std::option::Option<::std::primitive::bool>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod sharedflows {
                pub mod methods {
                    pub mod create {
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
                            #[serde(rename = "action")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. Must be set to either `import` or `validate`."]
                            pub action: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "name")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. The name to give the shared flow"]
                            pub name: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
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
                            #[serde(rename = "includeMetaData")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Indicates whether to include shared flow metadata in the response."]
                            pub include_meta_data: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "includeRevisions")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Indicates whether to include a list of revisions in the response."]
                            pub include_revisions: ::std::option::Option<::std::primitive::bool>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod revisions {
                        pub mod methods {
                            pub mod get {
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
                                    #[serde(rename = "format")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Specify `bundle` to export the contents of the shared flow bundle. Otherwise, the bundle metadata is returned."]
                                    pub format: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                            pub mod update_shared_flow_revision {
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
                                    #[serde(rename = "validate")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Ignored. All uploads are validated regardless of the value of this field. It is kept for compatibility with existing APIs. Must be `true` or `false` if provided."]
                                    pub validate: ::std::option::Option<::std::primitive::bool>,
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
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message that represents an arbitrary HTTP body. It should only be used for payload formats that can't be represented as JSON, such as raw binary or an HTML page. This message can be used both in streaming and non-streaming API methods in the request as well as the response. It can be used as a top-level request field, which is convenient if one wants to extract parameters from either the URL or HTTP template into the request fields and also want access to the raw HTTP body. Example: message GetResourceRequest { // A unique request id. string request_id = 1; // The raw HTTP body is bound to this field. google.api.HttpBody http_body = 2; } service ResourceService { rpc GetResource(GetResourceRequest) returns (google.api.HttpBody); rpc UpdateResource(google.api.HttpBody) returns (google.protobuf.Empty); } Example with streaming methods: service CaldavService { rpc GetCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); rpc UpdateCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); } Use of this type only changes how the request and response bodies are handled, all other features will continue to work unchanged."]
    pub struct GoogleApiHttpBody {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP Content-Type header value specifying the content type of the body."]
        pub content_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP request/response body as raw binary."]
        pub data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "extensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Application specific response metadata. Must be set in the first response for streaming APIs."]
        pub extensions: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
    }
    impl GoogleApiHttpBody {
        pub fn builder() -> GoogleApiHttpBodyBuilder {
            GoogleApiHttpBodyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1Access {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "Get")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub get: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1AccessGet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "Remove")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub remove: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1AccessRemove>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "Set")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub set: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1AccessSet>>,
    }
    impl GoogleCloudApigeeV1Access {
        pub fn builder() -> GoogleCloudApigeeV1AccessBuilder {
            GoogleCloudApigeeV1AccessBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Get action. For example, \"Get\" : { \"name\" : \"target.name\", \"value\" : \"default\" }"]
    pub struct GoogleCloudApigeeV1AccessGet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1AccessGet {
        pub fn builder() -> GoogleCloudApigeeV1AccessGetBuilder {
            GoogleCloudApigeeV1AccessGetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Remove action. For example, \"Remove\" : { \"name\" : \"target.name\", \"success\" : true }"]
    pub struct GoogleCloudApigeeV1AccessRemove {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "success")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub success: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudApigeeV1AccessRemove {
        pub fn builder() -> GoogleCloudApigeeV1AccessRemoveBuilder {
            GoogleCloudApigeeV1AccessRemoveBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set action. For example, \"Set\" : { \"name\" : \"target.name\", \"success\" : true, \"value\" : \"default\" }"]
    pub struct GoogleCloudApigeeV1AccessSet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "success")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub success: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1AccessSet {
        pub fn builder() -> GoogleCloudApigeeV1AccessSetBuilder {
            GoogleCloudApigeeV1AccessSetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for ActivateNatAddressRequest. Activate the nat address request."]
    pub struct GoogleCloudApigeeV1ActivateNatAddressRequest {}
    impl GoogleCloudApigeeV1ActivateNatAddressRequest {
        pub fn builder() -> GoogleCloudApigeeV1ActivateNatAddressRequestBuilder {
            GoogleCloudApigeeV1ActivateNatAddressRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Reference to a certificate or key/certificate pair."]
    pub struct GoogleCloudApigeeV1Alias {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alias")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource ID for this alias. Values must match the regular expression `[^/]{1,255}`."]
        pub alias: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "certsInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Chain of certificates under this alias."]
        pub certs_info: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1Certificate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of alias."]
        pub _type: ::std::option::Option<GoogleCloudApigeeV1AliasTypeEnum>,
    }
    impl GoogleCloudApigeeV1Alias {
        pub fn builder() -> GoogleCloudApigeeV1AliasBuilder {
            GoogleCloudApigeeV1AliasBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of alias."]
    pub enum GoogleCloudApigeeV1AliasTypeEnum {
        #[serde(rename = "ALIAS_TYPE_UNSPECIFIED")]
        #[doc = "Alias type is not specified."]
        AliasTypeUnspecified,
        #[serde(rename = "CERT")]
        #[doc = "Certificate."]
        Cert,
        #[serde(rename = "KEY_CERT")]
        #[doc = "Key/certificate pair."]
        KeyCert,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1AliasTypeEnum {
        fn default() -> Self {
            Self::AliasTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1AliasRevisionConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location of the alias file. For example, a Google Cloud Storage URI."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the alias revision included in the keystore in the following format: `organizations/{org}/environments/{env}/keystores/{keystore}/aliases/{alias}/revisions/{rev}`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub _type: ::std::option::Option<GoogleCloudApigeeV1AliasRevisionConfigTypeEnum>,
    }
    impl GoogleCloudApigeeV1AliasRevisionConfig {
        pub fn builder() -> GoogleCloudApigeeV1AliasRevisionConfigBuilder {
            GoogleCloudApigeeV1AliasRevisionConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleCloudApigeeV1AliasRevisionConfigTypeEnum {
        #[serde(rename = "ALIAS_TYPE_UNSPECIFIED")]
        #[doc = "Alias type is not specified."]
        AliasTypeUnspecified,
        #[serde(rename = "CERT")]
        #[doc = "Certificate."]
        Cert,
        #[serde(rename = "KEY_CERT")]
        #[doc = "Key/certificate pair."]
        KeyCert,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1AliasRevisionConfigTypeEnum {
        fn default() -> Self {
            Self::AliasTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "the Api category resource wrapped with response status, error_code etc."]
    pub struct GoogleCloudApigeeV1ApiCategory {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of category."]
        pub data: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1ApiCategoryData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID that can be used to find errors in the log files."]
        pub error_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the operation."]
        pub message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID that can be used to find request details in the log files."]
        pub request_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the operation."]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ApiCategory {
        pub fn builder() -> GoogleCloudApigeeV1ApiCategoryBuilder {
            GoogleCloudApigeeV1ApiCategoryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "the Api category resource."]
    pub struct GoogleCloudApigeeV1ApiCategoryData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the category (a UUID)."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the category."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "siteId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the portal."]
        pub site_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the category was last modified in milliseconds since epoch."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ApiCategoryData {
        pub fn builder() -> GoogleCloudApigeeV1ApiCategoryDataBuilder {
            GoogleCloudApigeeV1ApiCategoryDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1ApiProduct {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiResources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub api_resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "approvalType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag that specifies how API keys are approved to access the APIs defined by the API product. If set to `manual`, the consumer key is generated and returned in \"pending\" state. In this case, the API keys won't work until they have been explicitly approved. If set to `auto`, the consumer key is generated and returned in \"approved\" state and can be used immediately. **Note:** Typically, `auto` is used to provide access to free or trial API products that provide limited quota or capabilities."]
        pub approval_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Array of attributes that may be used to extend the default API product profile with customer-specific metadata. You can specify a maximum of 18 attributes. Use this property to specify the access level of the API product as either `public`, `private`, or `internal`. Only products marked `public` are available to developers in the Apigee developer portal. For example, you can set a product to `internal` while it is in development and then change access to `public` when it is ready to release on the portal. API products marked as `private` do not appear on the portal, but can be accessed by external developers."]
        pub attributes:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Response only. Creation time of this environment as milliseconds since epoch."]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the API product. Include key information about the API product that is not captured by other fields. Comma-separated list of API resources to be bundled in the API product. By default, the resource paths are mapped from the `proxy.pathsuffix` variable. The proxy path suffix is defined as the URI fragment following the ProxyEndpoint base path. For example, if the `apiResources` element is defined to be `/forecastrss` and the base path defined for the API proxy is `/weather`, then only requests to `/weather/forecastrss` are permitted by the API product. You can select a specific path, or you can select all subpaths with the following wildcard: - `/**`: Indicates that all sub-URIs are included. - `/*` : Indicates that only URIs one level down are included. By default, / supports the same resources as /** as well as the base path defined by the API proxy. For example, if the base path of the API proxy is `/v1/weatherapikey`, then the API product supports requests to `/v1/weatherapikey` and to any sub-URIs, such as `/v1/weatherapikey/forecastrss`, `/v1/weatherapikey/region/CA`, and so on. For more information, see Managing API products."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name displayed in the UI or developer portal to developers registering for API access."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Comma-separated list of environment names to which the API product is bound. Requests to environments that are not listed are rejected. By specifying one or more environments, you can bind the resources listed in the API product to a specific environment, preventing developers from accessing those resources through API proxies deployed in another environment. This setting is used, for example, to prevent resources associated with API proxies in `prod` from being accessed by API proxies deployed in `test`."]
        pub environments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Response only. Modified time of this environment as milliseconds since epoch."]
        pub last_modified_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Internal name of the API product. Characters you can use in the name are restricted to: `A-Z0-9._\\-$ %`. **Note:** The internal name cannot be edited when updating the API product."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration used to group Apigee proxies or remote services with resources, method types, and quotas. The resource refers to the resource URI (excluding the base path). With this grouping, the API product creator is able to fine-tune and give precise control over which REST methods have access to specific resources and how many calls can be made (using the `quota` setting). **Note:** The `api_resources` setting cannot be specified for both the API product and operation group; otherwise the call will fail."]
        pub operation_group:
            ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1OperationGroup>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "proxies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Comma-separated list of API proxy names to which this API product is bound. By specifying API proxies, you can associate resources in the API product with specific API proxies, preventing developers from accessing those resources through other API proxies. Apigee rejects requests to API proxies that are not listed. **Note:** The API proxy names must already exist in the specified environment as they will be validated upon creation."]
        pub proxies: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quota")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of request messages permitted per app by this API product for the specified `quotaInterval` and `quotaTimeUnit`. For example, a `quota` of 50, for a `quotaInterval` of 12 and a `quotaTimeUnit` of hours means 50 requests are allowed every 12 hours."]
        pub quota: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaInterval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time interval over which the number of request messages is calculated."]
        pub quota_interval: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaTimeUnit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time unit defined for the `quotaInterval`. Valid values include `minute`, `hour`, `day`, or `month`."]
        pub quota_time_unit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scopes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Comma-separated list of OAuth scopes that are validated at runtime. Apigee validates that the scopes in any access token presented match the scopes defined in the OAuth policy associated with the API product."]
        pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudApigeeV1ApiProduct {
        pub fn builder() -> GoogleCloudApigeeV1ApiProductBuilder {
            GoogleCloudApigeeV1ApiProductBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1ApiProductRef {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiproduct")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the API product."]
        pub apiproduct: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the API product."]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ApiProductRef {
        pub fn builder() -> GoogleCloudApigeeV1ApiProductRefBuilder {
            GoogleCloudApigeeV1ApiProductRefBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata describing the API proxy"]
    pub struct GoogleCloudApigeeV1ApiProxy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latestRevisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of the most recently created revision for this api proxy."]
        pub latest_revision_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metaData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata describing the API proxy."]
        pub meta_data: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1EntityMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the API proxy."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of revisons defined for the API proxy."]
        pub revision: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudApigeeV1ApiProxy {
        pub fn builder() -> GoogleCloudApigeeV1ApiProxyBuilder {
            GoogleCloudApigeeV1ApiProxyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "API proxy revision."]
    pub struct GoogleCloudApigeeV1ApiProxyRevision {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basepaths")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Base URL of the API proxy."]
        pub basepaths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "configurationVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Version of the API proxy configuration schema to which the API proxy conforms. Currently, the only supported value is 4.0 (`majorVersion.minorVersion`). This setting may be used in the future to track the evolution of the API proxy format."]
        pub configuration_version:
            ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1ConfigVersion>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contextInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Revision number, app name, and organization for the API proxy."]
        pub context_info: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time that the API proxy revision was created in milliseconds since epoch."]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the API proxy revision."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human-readable name of the API proxy."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityMetaDataAsProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata describing the API proxy revision as a key-value map."]
        pub entity_meta_data_as_properties:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time that the API proxy revision was last modified in milliseconds since epoch."]
        pub last_modified_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the API proxy."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of policy names included in the API proxy revision.."]
        pub policies: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "proxies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of proxy names included in the API proxy revision."]
        pub proxies: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "proxyEndpoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of ProxyEndpoints in the `/proxies` directory of the API proxy. Typically, this element is included only when the API proxy was created using the Edge UI. This is a 'manifest' setting designed to provide visibility into the contents of the API proxy."]
        pub proxy_endpoints: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceFiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of resource files included in the API proxy revision."]
        pub resource_files:
            ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1ResourceFiles>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of the resources included in the API proxy revision formatted as \"{type}://{name}\"."]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "API proxy revision."]
        pub revision: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sharedFlows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of the shared flows included in the API proxy revision."]
        pub shared_flows: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OpenAPI Specification that is associated with the API proxy. The value is set to a URL or to a path in the specification store."]
        pub spec: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetEndpoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of TargetEndpoints in the `/targets` directory of the API proxy. Typically, this element is included only when the API proxy was created using the Edge UI. This is a 'manifest' setting designed to provide visibility into the contents of the API proxy."]
        pub target_endpoints: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetServers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of TargetServers referenced in any TargetEndpoint in the API proxy. Typically, you will see this element only when the API proxy was created using the Edge UI. This is a 'manifest' setting designed to provide visibility into the contents of the API proxy."]
        pub target_servers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of the targets included in the API proxy revision."]
        pub targets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of the teams included in the API proxy revision."]
        pub teams: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type. Set to `Application`. Maintained for compatibility with the Apigee Edge API."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ApiProxyRevision {
        pub fn builder() -> GoogleCloudApigeeV1ApiProxyRevisionBuilder {
            GoogleCloudApigeeV1ApiProxyRevisionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1ApiResponseWrapper {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID that can be used to find errors in the log files."]
        pub error_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the operation."]
        pub message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID that can be used to find request details in the log files."]
        pub request_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the operation."]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ApiResponseWrapper {
        pub fn builder() -> GoogleCloudApigeeV1ApiResponseWrapperBuilder {
            GoogleCloudApigeeV1ApiResponseWrapperBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1App {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiProducts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of API products associated with the app."]
        pub api_products: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ApiProductRef>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the app."]
        pub app_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of attributes."]
        pub attributes:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "callbackUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Callback URL used by OAuth 2.0 authorization servers to communicate authorization codes back to apps."]
        pub callback_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "companyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the company that owns the app."]
        pub company_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Unix time when the app was created."]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "credentials")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Set of credentials for the app. Credentials are API key/secret pairs associated with API products."]
        pub credentials: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Credential>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "developerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the developer."]
        pub developer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyExpiresIn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Duration, in milliseconds, of the consumer key that will be generated for the app. The default value, -1, indicates an infinite validity period. Once set, the expiration can't be updated. json key: keyExpiresIn"]
        pub key_expires_in: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Last modified time as milliseconds since epoch."]
        pub last_modified_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the app."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scopes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scopes to apply to the app. The specified scope names must already exist on the API product that you associate with the app."]
        pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the credential."]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1App {
        pub fn builder() -> GoogleCloudApigeeV1AppBuilder {
            GoogleCloudApigeeV1AppBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1AsyncQuery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creation time of the query."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "envgroupHostname")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hostname is available only when query is executed at host level."]
        pub envgroup_hostname: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error is set when query fails."]
        pub error: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ExecutionTime is available only after the query is completed."]
        pub execution_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Asynchronous Query Name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryParams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains information like metrics, dimenstions etc of the AsyncQuery."]
        pub query_params:
            ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1QueryMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportDefinitionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Asynchronous Report ID."]
        pub report_definition_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "result")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Result is available only after the query is completed."]
        pub result: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1AsyncQueryResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resultFileSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ResultFileSize is available only after the query is completed."]
        pub result_file_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resultRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ResultRows is available only after the query is completed."]
        pub result_rows: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "self")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Self link of the query. Example: `/organizations/myorg/environments/myenv/queries/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd` or following format if query is running at host level: `/organizations/myorg/hostQueries/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd`"]
        pub _self: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Query state could be \"enqueued\", \"running\", \"completed\", \"failed\"."]
        pub state: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last updated timestamp for the query."]
        pub updated: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1AsyncQuery {
        pub fn builder() -> GoogleCloudApigeeV1AsyncQueryBuilder {
            GoogleCloudApigeeV1AsyncQueryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1AsyncQueryResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expires")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Query result will be unaccessable after this time."]
        pub expires: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "self")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Self link of the query results. Example: `/organizations/myorg/environments/myenv/queries/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd/result` or following format if query is running at host level: `/organizations/myorg/hostQueries/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd/result`"]
        pub _self: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1AsyncQueryResult {
        pub fn builder() -> GoogleCloudApigeeV1AsyncQueryResultBuilder {
            GoogleCloudApigeeV1AsyncQueryResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1AsyncQueryResultView {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error code when there is a failure."]
        pub code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error message when there is a failure."]
        pub error: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata contains information like metrics, dimenstions etc of the AsyncQuery."]
        pub metadata: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1QueryMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rows of query result. Each row is a JSON object. Example: {sum(message_count): 1, developer_app: \"(not set)\",…}"]
        pub rows: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State of retrieving ResultView."]
        pub state: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1AsyncQueryResultView {
        pub fn builder() -> GoogleCloudApigeeV1AsyncQueryResultViewBuilder {
            GoogleCloudApigeeV1AsyncQueryResultViewBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Key-value pair to store extra metadata."]
    pub struct GoogleCloudApigeeV1Attribute {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "API key of the attribute."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of the attribute."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1Attribute {
        pub fn builder() -> GoogleCloudApigeeV1AttributeBuilder {
            GoogleCloudApigeeV1AttributeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1Attributes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attribute")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of attributes."]
        pub attribute:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
    }
    impl GoogleCloudApigeeV1Attributes {
        pub fn builder() -> GoogleCloudApigeeV1AttributesBuilder {
            GoogleCloudApigeeV1AttributesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "CanaryEvaluation represents the canary analysis between two versions of the runtime that is serving requests."]
    pub struct GoogleCloudApigeeV1CanaryEvaluation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "control")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The stable version that is serving requests."]
        pub control: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Create time of the canary evaluation."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. End time for the evaluation's analysis."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Labels used to filter the metrics used for a canary evaluation."]
        pub metric_labels: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudApigeeV1CanaryEvaluationMetricLabels>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Name of the canary evalution."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Start time for the canary evaluation's analysis."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The current state of the canary evaluation."]
        pub state: ::std::option::Option<GoogleCloudApigeeV1CanaryEvaluationStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "treatment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The newer version that is serving requests."]
        pub treatment: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verdict")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The resulting verdict of the canary evaluations: NONE, PASS, or FAIL."]
        pub verdict: ::std::option::Option<GoogleCloudApigeeV1CanaryEvaluationVerdictEnum>,
    }
    impl GoogleCloudApigeeV1CanaryEvaluation {
        pub fn builder() -> GoogleCloudApigeeV1CanaryEvaluationBuilder {
            GoogleCloudApigeeV1CanaryEvaluationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The current state of the canary evaluation."]
    pub enum GoogleCloudApigeeV1CanaryEvaluationStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "No state has been specified."]
        StateUnspecified,
        #[serde(rename = "RUNNING")]
        #[doc = "The canary evaluation is still in progress."]
        Running,
        #[serde(rename = "SUCCEEDED")]
        #[doc = "The canary evaluation has finished."]
        Succeeded,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1CanaryEvaluationStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The resulting verdict of the canary evaluations: NONE, PASS, or FAIL."]
    pub enum GoogleCloudApigeeV1CanaryEvaluationVerdictEnum {
        #[serde(rename = "VERDICT_UNSPECIFIED")]
        #[doc = "Verdict is not available yet."]
        VerdictUnspecified,
        #[serde(rename = "NONE")]
        #[doc = "No verdict reached."]
        None,
        #[serde(rename = "FAIL")]
        #[doc = "Evaluation is not good."]
        Fail,
        #[serde(rename = "PASS")]
        #[doc = "Evaluation is good."]
        Pass,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1CanaryEvaluationVerdictEnum {
        fn default() -> Self {
            Self::VerdictUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Labels that can be used to filter Apigee metrics."]
    pub struct GoogleCloudApigeeV1CanaryEvaluationMetricLabels {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "env")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The environment ID associated with the metrics."]
        pub env: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instance_id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The instance ID associated with the metrics. In Apigee Hybrid, the value is configured during installation."]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The location associated with the metrics."]
        pub location: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1CanaryEvaluationMetricLabels {
        pub fn builder() -> GoogleCloudApigeeV1CanaryEvaluationMetricLabelsBuilder {
            GoogleCloudApigeeV1CanaryEvaluationMetricLabelsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "X.509 certificate as defined in RFC 5280."]
    pub struct GoogleCloudApigeeV1CertInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConstraints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "X.509 basic constraints extension."]
        pub basic_constraints: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiryDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "X.509 `notAfter` validity period in milliseconds since epoch."]
        pub expiry_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isValid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag that specifies whether the certificate is valid. Flag is set to `Yes` if the certificate is valid, `No` if expired, or `Not yet` if not yet valid."]
        pub is_valid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "issuer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "X.509 issuer."]
        pub issuer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Public key component of the X.509 subject public key info."]
        pub public_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serialNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "X.509 serial number."]
        pub serial_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sigAlgName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "X.509 signatureAlgorithm."]
        pub sig_alg_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "X.509 subject."]
        pub subject: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subjectAlternativeNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "X.509 subject alternative names (SANs) extension."]
        pub subject_alternative_names:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validFrom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "X.509 `notBefore` validity period in milliseconds since epoch."]
        pub valid_from: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "X.509 version."]
        pub version: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudApigeeV1CertInfo {
        pub fn builder() -> GoogleCloudApigeeV1CertInfoBuilder {
            GoogleCloudApigeeV1CertInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1Certificate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "certInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Chain of certificates under this name."]
        pub cert_info:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1CertInfo>>>,
    }
    impl GoogleCloudApigeeV1Certificate {
        pub fn builder() -> GoogleCloudApigeeV1CertificateBuilder {
            GoogleCloudApigeeV1CertificateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1CommonNameConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchWildCards")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub match_wild_cards: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1CommonNameConfig {
        pub fn builder() -> GoogleCloudApigeeV1CommonNameConfigBuilder {
            GoogleCloudApigeeV1CommonNameConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Version of the API proxy configuration schema. Currently, only 4.0 is supported."]
    pub struct GoogleCloudApigeeV1ConfigVersion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "majorVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Major version of the API proxy configuration schema."]
        pub major_version: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minorVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minor version of the API proxy configuration schema."]
        pub minor_version: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudApigeeV1ConfigVersion {
        pub fn builder() -> GoogleCloudApigeeV1ConfigVersionBuilder {
            GoogleCloudApigeeV1ConfigVersionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1Credential {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiProducts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of API products this credential can be used for."]
        pub api_products: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ApiProductRef>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of attributes associated with this credential."]
        pub attributes:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Consumer key."]
        pub consumer_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerSecret")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Secret key."]
        pub consumer_secret: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiresAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the credential will expire in milliseconds since epoch."]
        pub expires_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "issuedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the credential was issued in milliseconds since epoch."]
        pub issued_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scopes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of scopes to apply to the app. Specified scopes must already exist on the API product that you associate with the app."]
        pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the credential."]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1Credential {
        pub fn builder() -> GoogleCloudApigeeV1CredentialBuilder {
            GoogleCloudApigeeV1CredentialBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1CustomReport {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chartType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains the chart type for the report"]
        pub chart_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Legacy field: not used. This field contains a list of comments associated with custom report"]
        pub comments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Unix time when the app was created json key: createdAt"]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This contains the list of dimensions for the report"]
        pub dimensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is the display name for the report"]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Environment name"]
        pub environment: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains the filter expression"]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fromTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Legacy field: not used. Contains the from time for the report"]
        pub from_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Modified time of this entity as milliseconds since epoch. json key: lastModifiedAt"]
        pub last_modified_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastViewedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Last viewed time of this entity as milliseconds since epoch"]
        pub last_viewed_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Legacy field: not used This field contains the limit for the result retrieved"]
        pub limit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. This contains the list of metrics"]
        pub metrics: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1CustomReportMetric>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Unique identifier for the report T his is a legacy field used to encode custom report unique id"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Legacy field: not used. This field contains the offset for the data"]
        pub offset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "organization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Organization name"]
        pub organization: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains report properties such as ui metadata etc."]
        pub properties: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ReportProperty>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortByCols")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Legacy field: not used much. Contains the list of sort by columns"]
        pub sort_by_cols: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Legacy field: not used much. Contains the sort order for the sort columns"]
        pub sort_order: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Legacy field: not used. This field contains a list of tags associated with custom report"]
        pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeUnit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains the time unit of aggregation for the report"]
        pub time_unit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "toTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Legacy field: not used. Contains the end time for the report"]
        pub to_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topk")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Legacy field: not used. This field contains the top k parameter value for restricting the result"]
        pub topk: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1CustomReport {
        pub fn builder() -> GoogleCloudApigeeV1CustomReportBuilder {
            GoogleCloudApigeeV1CustomReportBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This encapsulates a metric property of the form sum(message_count) where name is message_count and function is sum"]
    pub struct GoogleCloudApigeeV1CustomReportMetric {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "function")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "aggregate function"]
        pub function: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "name of the metric"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1CustomReportMetric {
        pub fn builder() -> GoogleCloudApigeeV1CustomReportMetricBuilder {
            GoogleCloudApigeeV1CustomReportMetricBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data collector configuration."]
    pub struct GoogleCloudApigeeV1DataCollector {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which the data collector was created in milliseconds since the epoch."]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of the data collector."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which the Data Collector was last updated in milliseconds since the epoch."]
        pub last_modified_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the data collector. Must begin with `dc_`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. The type of data this data collector will collect."]
        pub _type: ::std::option::Option<GoogleCloudApigeeV1DataCollectorTypeEnum>,
    }
    impl GoogleCloudApigeeV1DataCollector {
        pub fn builder() -> GoogleCloudApigeeV1DataCollectorBuilder {
            GoogleCloudApigeeV1DataCollectorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Immutable. The type of data this data collector will collect."]
    pub enum GoogleCloudApigeeV1DataCollectorTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "For future compatibility."]
        TypeUnspecified,
        #[serde(rename = "INTEGER")]
        #[doc = "For integer values."]
        Integer,
        #[serde(rename = "FLOAT")]
        #[doc = "For float values."]
        Float,
        #[serde(rename = "STRING")]
        #[doc = "For string values."]
        String,
        #[serde(rename = "BOOLEAN")]
        #[doc = "For boolean values."]
        Boolean,
        #[serde(rename = "DATETIME")]
        #[doc = "For datetime values."]
        Datetime,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1DataCollectorTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data collector and its configuration."]
    pub struct GoogleCloudApigeeV1DataCollectorConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the data collector in the following format: `organizations/{org}/datacollectors/{datacollector}`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data type accepted by the data collector."]
        pub _type: ::std::option::Option<GoogleCloudApigeeV1DataCollectorConfigTypeEnum>,
    }
    impl GoogleCloudApigeeV1DataCollectorConfig {
        pub fn builder() -> GoogleCloudApigeeV1DataCollectorConfigBuilder {
            GoogleCloudApigeeV1DataCollectorConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Data type accepted by the data collector."]
    pub enum GoogleCloudApigeeV1DataCollectorConfigTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "For future compatibility."]
        TypeUnspecified,
        #[serde(rename = "INTEGER")]
        #[doc = "For integer values."]
        Integer,
        #[serde(rename = "FLOAT")]
        #[doc = "For float values."]
        Float,
        #[serde(rename = "STRING")]
        #[doc = "For string values."]
        String,
        #[serde(rename = "BOOLEAN")]
        #[doc = "For boolean values."]
        Boolean,
        #[serde(rename = "DATETIME")]
        #[doc = "For datetime values."]
        Datetime,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1DataCollectorConfigTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The data store defines the connection to export data repository (Cloud Storage, BigQuery), including the credentials used to access the data repository."]
    pub struct GoogleCloudApigeeV1Datastore {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Datastore create time, in milliseconds since the epoch of 1970-01-01T00:00:00Z"]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datastoreConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Datastore Configurations."]
        pub datastore_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1DatastoreConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Display name in UI"]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Datastore last update time, in milliseconds since the epoch of 1970-01-01T00:00:00Z"]
        pub last_update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "org")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Organization that the datastore belongs to"]
        pub org: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "self")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource link of Datastore. Example: `/organizations/{org}/analytics/datastores/{uuid}`"]
        pub _self: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Destination storage type. Supported types `gcs` or `bigquery`."]
        pub target_type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1Datastore {
        pub fn builder() -> GoogleCloudApigeeV1DatastoreBuilder {
            GoogleCloudApigeeV1DatastoreBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration detail for datastore"]
    pub struct GoogleCloudApigeeV1DatastoreConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the Cloud Storage bucket. Required for `gcs` target_type."]
        pub bucket_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datasetName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "BigQuery dataset name Required for `bigquery` target_type."]
        pub dataset_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Path of Cloud Storage bucket Required for `gcs` target_type."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. GCP project in which the datastore exists"]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tablePrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Prefix of BigQuery table Required for `bigquery` target_type."]
        pub table_prefix: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1DatastoreConfig {
        pub fn builder() -> GoogleCloudApigeeV1DatastoreConfigBuilder {
            GoogleCloudApigeeV1DatastoreConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Date range of the data to export."]
    pub struct GoogleCloudApigeeV1DateRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "end")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. End date (exclusive) of the data to export in the format `yyyy-mm-dd`. The date range ends at 00:00:00 UTC on the end date- which will not be in the output."]
        pub end: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Start date of the data to export in the format `yyyy-mm-dd`. The date range begins at 00:00:00 UTC on the start date."]
        pub start: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1DateRange {
        pub fn builder() -> GoogleCloudApigeeV1DateRangeBuilder {
            GoogleCloudApigeeV1DateRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1DebugMask {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faultJSONPaths")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of JSON paths that specify the JSON elements to be filtered from JSON payloads in error flows."]
        pub fault_json_paths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faultXPaths")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of XPaths that specify the XML elements to be filtered from XML payloads in error flows."]
        pub fault_x_paths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the debug mask."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namespaces")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map of namespaces to URIs."]
        pub namespaces:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestJSONPaths")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of JSON paths that specify the JSON elements to be filtered from JSON request message payloads."]
        pub request_json_paths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestXPaths")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of XPaths that specify the XML elements to be filtered from XML request message payloads."]
        pub request_x_paths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseJSONPaths")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of JSON paths that specify the JSON elements to be filtered from JSON response message payloads."]
        pub response_json_paths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseXPaths")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of XPaths that specify the XML elements to be filtered from XML response message payloads."]
        pub response_x_paths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "variables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of variables that should be masked from the debug output."]
        pub variables: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudApigeeV1DebugMask {
        pub fn builder() -> GoogleCloudApigeeV1DebugMaskBuilder {
            GoogleCloudApigeeV1DebugMaskBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1DebugSession {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The number of request to be traced. Min = 1, Max = 15, Default = 10."]
        pub count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A conditional statement which is evaluated against the request message to determine if it should be traced. Syntax matches that of on API Proxy bundle flow Condition."]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique ID for this DebugSession."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The time in seconds after which this DebugSession should end. This value will override the value in query param, if both are provided."]
        pub timeout: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tracesize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The maximum number of bytes captured from the response payload. Min = 0, Max = 5120, Default = 5120."]
        pub tracesize: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The length of time, in seconds, that this debug session is valid, starting from when it's received in the control plane. Min = 1, Max = 15, Default = 10."]
        pub validity: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudApigeeV1DebugSession {
        pub fn builder() -> GoogleCloudApigeeV1DebugSessionBuilder {
            GoogleCloudApigeeV1DebugSessionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A transaction contains all of the debug information of the entire message flow of an API call processed by the runtime plane. The information is collected and recorded at critical points of the message flow in the runtime apiproxy."]
    pub struct GoogleCloudApigeeV1DebugSessionTransaction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "completed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag indicating whether a transaction is completed or not"]
        pub completed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "point")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of debug data collected by runtime plane at various defined points in the flow."]
        pub point:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Point>>>,
    }
    impl GoogleCloudApigeeV1DebugSessionTransaction {
        pub fn builder() -> GoogleCloudApigeeV1DebugSessionTransactionBuilder {
            GoogleCloudApigeeV1DebugSessionTransactionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1DeleteCustomReportResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The response contains only a message field."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1DeleteCustomReportResponse {
        pub fn builder() -> GoogleCloudApigeeV1DeleteCustomReportResponseBuilder {
            GoogleCloudApigeeV1DeleteCustomReportResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1Deployment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiProxy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "API proxy."]
        pub api_proxy: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deployStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the API proxy was marked `deployed` in the control plane in millisconds since epoch."]
        pub deploy_start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Environment."]
        pub environment: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Errors reported for this deployment. Populated only when state == ERROR. This field is not populated in List APIs."]
        pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instances")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status reported by each runtime instance. This field is not populated in List APIs."]
        pub instances: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1InstanceDeploymentStatus>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pods")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status reported by runtime pods. This field is not populated for List APIs."]
        pub pods:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1PodStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "API proxy revision."]
        pub revision: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "routeConflicts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Conflicts in the desired state routing configuration. The presence of conflicts does not cause the state to be ERROR, but it will mean that some of the deployments basepaths are not routed to its environment. If the conflicts change, the state will transition to PROGRESSING until the latest configuration is rolled out to all instances. This field is not populated in List APIs."]
        pub route_conflicts: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudApigeeV1DeploymentChangeReportRoutingConflict>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Current state of the deployment. This field is not populated in List APIs."]
        pub state: ::std::option::Option<GoogleCloudApigeeV1DeploymentStateEnum>,
    }
    impl GoogleCloudApigeeV1Deployment {
        pub fn builder() -> GoogleCloudApigeeV1DeploymentBuilder {
            GoogleCloudApigeeV1DeploymentBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Current state of the deployment. This field is not populated in List APIs."]
    pub enum GoogleCloudApigeeV1DeploymentStateEnum {
        #[serde(rename = "RUNTIME_STATE_UNSPECIFIED")]
        #[doc = "This value should never be returned."]
        RuntimeStateUnspecified,
        #[serde(rename = "READY")]
        #[doc = "The runtime has loaded the deployment."]
        Ready,
        #[serde(rename = "PROGRESSING")]
        #[doc = "The deployment is not fully ready in the runtime."]
        Progressing,
        #[serde(rename = "ERROR")]
        #[doc = "There is an error with the deployment that requires intervention."]
        Error,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1DeploymentStateEnum {
        fn default() -> Self {
            Self::RuntimeStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for GenerateDeployChangeReport and GenerateUndeployChangeReport. This report contains any validation failures that would cause the deployment to be rejected, as well changes and conflicts in routing that may occur due to the new deployment. The existence of a routing warning does not necessarily imply that the deployment request is bad, if the desired state of the deployment request is to effect a routing change. The primary purposes of the routing messages are: 1) To inform users of routing changes that may have an effect on traffic currently being routed to other existing deployments. 2) To warn users if some basepath in the proxy will not receive traffic due to an existing deployment having already claimed that basepath. The presence of routing conflicts/changes will not cause non-dry-run DeployApiProxy/UndeployApiProxy requests to be rejected."]
    pub struct GoogleCloudApigeeV1DeploymentChangeReport {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "routingChanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All routing changes that may result from a deployment request."]
        pub routing_changes: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudApigeeV1DeploymentChangeReportRoutingChange>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "routingConflicts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All basepath conflicts detected for a deployment request."]
        pub routing_conflicts: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudApigeeV1DeploymentChangeReportRoutingConflict>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validationErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Validation errors that would cause the deployment change request to be rejected."]
        pub validation_errors:
            ::std::option::Option<::std::boxed::Box<GoogleRpcPreconditionFailure>>,
    }
    impl GoogleCloudApigeeV1DeploymentChangeReport {
        pub fn builder() -> GoogleCloudApigeeV1DeploymentChangeReportBuilder {
            GoogleCloudApigeeV1DeploymentChangeReportBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a potential routing change that may occur as a result of some deployment operation."]
    pub struct GoogleCloudApigeeV1DeploymentChangeReportRoutingChange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A human-readable description of this routing change."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environmentGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the environment group affected by this routing change."]
        pub environment_group: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fromDeployment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basepath/deployment that may stop receiving some traffic."]
        pub from_deployment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudApigeeV1DeploymentChangeReportRoutingDeployment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shouldSequenceRollout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if using sequenced rollout would make this routing change safer. Note: this does not necessarily imply that automated sequenced rollout mode is supported for the operation."]
        pub should_sequence_rollout: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "toDeployment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basepath/deployment that may start receiving that traffic. May be null if no deployment is able to receive the traffic."]
        pub to_deployment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudApigeeV1DeploymentChangeReportRoutingDeployment>,
        >,
    }
    impl GoogleCloudApigeeV1DeploymentChangeReportRoutingChange {
        pub fn builder() -> GoogleCloudApigeeV1DeploymentChangeReportRoutingChangeBuilder {
            GoogleCloudApigeeV1DeploymentChangeReportRoutingChangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a routing conflict that may cause a deployment not to receive traffic at some basepath."]
    pub struct GoogleCloudApigeeV1DeploymentChangeReportRoutingConflict {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conflictingDeployment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The existing basepath/deployment causing the conflict."]
        pub conflicting_deployment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudApigeeV1DeploymentChangeReportRoutingDeployment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A human-readable description of this conflict."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environmentGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the environment group in which this conflict exists."]
        pub environment_group: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1DeploymentChangeReportRoutingConflict {
        pub fn builder() -> GoogleCloudApigeeV1DeploymentChangeReportRoutingConflictBuilder {
            GoogleCloudApigeeV1DeploymentChangeReportRoutingConflictBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A tuple representing a basepath and the deployment containing it."]
    pub struct GoogleCloudApigeeV1DeploymentChangeReportRoutingDeployment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiProxy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the deployed proxy revision containing the basepath."]
        pub api_proxy: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basepath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basepath receiving traffic."]
        pub basepath: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the environment in which the proxy is deployed."]
        pub environment: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the deployed proxy revision containing the basepath."]
        pub revision: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1DeploymentChangeReportRoutingDeployment {
        pub fn builder() -> GoogleCloudApigeeV1DeploymentChangeReportRoutingDeploymentBuilder {
            GoogleCloudApigeeV1DeploymentChangeReportRoutingDeploymentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1DeploymentConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional key-value metadata for the deployment."]
        pub attributes:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Base path where the application will be hosted. Defaults to \"/\"."]
        pub base_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location of the API proxy bundle as a URI."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the API or shared flow revision to be deployed in the following format: `organizations/{org}/apis/{api}/revisions/{rev}` or `organizations/{org}/sharedflows/{sharedflow}/revisions/{rev}`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "proxyUid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique ID of the API proxy revision."]
        pub proxy_uid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique ID. The ID will only change if the deployment is deleted and recreated."]
        pub uid: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1DeploymentConfig {
        pub fn builder() -> GoogleCloudApigeeV1DeploymentConfigBuilder {
            GoogleCloudApigeeV1DeploymentConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1Developer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Access type."]
        pub access_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appFamily")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Developer app family."]
        pub app_family: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of apps associated with the developer."]
        pub apps: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Developer attributes (name/value pairs). The custom attribute limit is 18."]
        pub attributes:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "companies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of companies associated with the developer."]
        pub companies: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time at which the developer was created in milliseconds since epoch."]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "developerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the developer. **Note**: IDs are generated internally by Apigee and are not guaranteed to stay the same over time."]
        pub developer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Email address of the developer. This value is used to uniquely identify the developer in Apigee hybrid. Note that the email address has to be in lowercase only."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. First name of the developer."]
        pub first_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time at which the developer was last modified in milliseconds since epoch."]
        pub last_modified_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Last name of the developer."]
        pub last_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "organizationName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Name of the Apigee organization in which the developer resides."]
        pub organization_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Status of the developer. Valid values are `active` and `inactive`."]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. User name of the developer. Not used by Apigee hybrid."]
        pub user_name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1Developer {
        pub fn builder() -> GoogleCloudApigeeV1DeveloperBuilder {
            GoogleCloudApigeeV1DeveloperBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1DeveloperApp {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiProducts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of API products associated with the developer app."]
        pub api_products: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appFamily")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Developer app family."]
        pub app_family: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the developer app."]
        pub app_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of attributes for the developer app."]
        pub attributes:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "callbackUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Callback URL used by OAuth 2.0 authorization servers to communicate authorization codes back to developer apps."]
        pub callback_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time the developer app was created in milliseconds since epoch."]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "credentials")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Set of credentials for the developer app consisting of the consumer key/secret pairs associated with the API products."]
        pub credentials: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Credential>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "developerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the developer."]
        pub developer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyExpiresIn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Expiration time, in milliseconds, for the consumer key that is generated for the developer app. If not set or left to the default value of `-1`, the API key never expires. The expiration time can't be updated after it is set."]
        pub key_expires_in: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time the developer app was modified in milliseconds since epoch."]
        pub last_modified_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the developer app."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scopes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scopes to apply to the developer app. The specified scopes must already exist for the API product that you associate with the developer app."]
        pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the credential. Valid values include `approved` or `revoked`."]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1DeveloperApp {
        pub fn builder() -> GoogleCloudApigeeV1DeveloperAppBuilder {
            GoogleCloudApigeeV1DeveloperAppBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1DeveloperAppKey {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiProducts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of API products for which the credential can be used. **Note**: Do not specify the list of API products when creating a consumer key and secret for a developer app. Instead, use the UpdateDeveloperAppKey API to make the association after the consumer key and secret are created."]
        pub api_products: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of attributes associated with the credential."]
        pub attributes:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Consumer key."]
        pub consumer_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerSecret")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Secret key."]
        pub consumer_secret: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiresAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the developer app expires in milliseconds since epoch."]
        pub expires_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiresInSeconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input only. Expiration time, in seconds, for the consumer key. If not set or left to the default value of `-1`, the API key never expires. The expiration time can't be updated after it is set."]
        pub expires_in_seconds: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "issuedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the developer app was created in milliseconds since epoch."]
        pub issued_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scopes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scopes to apply to the app. The specified scope names must already be defined for the API product that you associate with the app."]
        pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the credential. Valid values include `approved` or `revoked`."]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1DeveloperAppKey {
        pub fn builder() -> GoogleCloudApigeeV1DeveloperAppKeyBuilder {
            GoogleCloudApigeeV1DeveloperAppKeyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This message type encapsulates a metric grouped by dimension."]
    pub struct GoogleCloudApigeeV1DimensionMetric {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains a list of metrics."]
        pub metrics:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Metric>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains the name of the dimension."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1DimensionMetric {
        pub fn builder() -> GoogleCloudApigeeV1DimensionMetricBuilder {
            GoogleCloudApigeeV1DimensionMetricBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata common to many entities in this API."]
    pub struct GoogleCloudApigeeV1EntityMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time at which the API proxy was created, in milliseconds since epoch."]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time at which the API proxy was most recently modified, in milliseconds since epoch."]
        pub last_modified_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of entity described"]
        pub sub_type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1EntityMetadata {
        pub fn builder() -> GoogleCloudApigeeV1EntityMetadataBuilder {
            GoogleCloudApigeeV1EntityMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1Environment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Creation time of this environment as milliseconds since epoch."]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Description of the environment."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Display name for this environment."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Last modification time of this environment as milliseconds since epoch."]
        pub last_modified_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Name of the environment. Values must match the regular expression `^[.\\\\p{Alnum}-_]{1,255}$`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Key-value pairs that may be used for customizing the environment."]
        pub properties: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1Properties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. State of the environment. Values other than ACTIVE means the resource is not ready to use."]
        pub state: ::std::option::Option<GoogleCloudApigeeV1EnvironmentStateEnum>,
    }
    impl GoogleCloudApigeeV1Environment {
        pub fn builder() -> GoogleCloudApigeeV1EnvironmentBuilder {
            GoogleCloudApigeeV1EnvironmentBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. State of the environment. Values other than ACTIVE means the resource is not ready to use."]
    pub enum GoogleCloudApigeeV1EnvironmentStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Resource is in an unspecified state."]
        StateUnspecified,
        #[serde(rename = "CREATING")]
        #[doc = "Resource is being created."]
        Creating,
        #[serde(rename = "ACTIVE")]
        #[doc = "Resource is provisioned and ready to use."]
        Active,
        #[serde(rename = "DELETING")]
        #[doc = "The resource is being deleted."]
        Deleting,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1EnvironmentStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1EnvironmentConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time that the environment configuration was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataCollectors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of data collectors used by the deployments in the environment."]
        pub data_collectors: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1DataCollectorConfig>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "debugMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Debug mask that applies to all deployments in the environment."]
        pub debug_mask: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1DebugMask>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deployments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of deployments in the environment."]
        pub deployments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1DeploymentConfig>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "featureFlags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature flags inherited from the organization and environment."]
        pub feature_flags:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "flowhooks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of flow hooks in the environment."]
        pub flowhooks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1FlowHookConfig>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keystores")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of keystores in the environment."]
        pub keystores: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1KeystoreConfig>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the environment configuration in the following format: `organizations/{org}/environments/{env}/configs/{config}`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provider")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Used by the Control plane to add context information to help detect the source of the document during diagnostics and debugging."]
        pub provider: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pubsubTopic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the PubSub topic for the environment."]
        pub pubsub_topic: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceReferences")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of resource references in the environment."]
        pub resource_references: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ReferenceConfig>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of resource versions in the environment."]
        pub resources: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ResourceConfig>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Revision ID of the environment configuration. The higher the value, the more recently the configuration was deployed."]
        pub revision_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sequenceNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED: Use revision_id."]
        pub sequence_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of target servers in the environment. Disabled target servers are not displayed."]
        pub targets: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1TargetServerConfig>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "traceConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Trace configurations. Contains config for the environment and config overrides for specific API proxies."]
        pub trace_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1RuntimeTraceConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique ID for the environment configuration. The ID will only change if the environment is deleted and recreated."]
        pub uid: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1EnvironmentConfig {
        pub fn builder() -> GoogleCloudApigeeV1EnvironmentConfigBuilder {
            GoogleCloudApigeeV1EnvironmentConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "EnvironmentGroup configuration. An environment group is used to group one or more Apigee environments under a single host name."]
    pub struct GoogleCloudApigeeV1EnvironmentGroup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which the environment group was created as milliseconds since epoch."]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hostnames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Host names for this environment group."]
        pub hostnames: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which the environment group was last updated as milliseconds since epoch."]
        pub last_modified_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the environment group."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. State of the environment group. Values other than ACTIVE means the resource is not ready to use."]
        pub state: ::std::option::Option<GoogleCloudApigeeV1EnvironmentGroupStateEnum>,
    }
    impl GoogleCloudApigeeV1EnvironmentGroup {
        pub fn builder() -> GoogleCloudApigeeV1EnvironmentGroupBuilder {
            GoogleCloudApigeeV1EnvironmentGroupBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. State of the environment group. Values other than ACTIVE means the resource is not ready to use."]
    pub enum GoogleCloudApigeeV1EnvironmentGroupStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Resource is in an unspecified state."]
        StateUnspecified,
        #[serde(rename = "CREATING")]
        #[doc = "Resource is being created."]
        Creating,
        #[serde(rename = "ACTIVE")]
        #[doc = "Resource is provisioned and ready to use."]
        Active,
        #[serde(rename = "DELETING")]
        #[doc = "The resource is being deleted."]
        Deleting,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1EnvironmentGroupStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "EnvironmentGroupAttachment is a resource which defines an attachment of an environment to an environment group."]
    pub struct GoogleCloudApigeeV1EnvironmentGroupAttachment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which the environment group attachment was created as milliseconds since epoch."]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. ID of the attached environment."]
        pub environment: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the environment group attachment."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1EnvironmentGroupAttachment {
        pub fn builder() -> GoogleCloudApigeeV1EnvironmentGroupAttachmentBuilder {
            GoogleCloudApigeeV1EnvironmentGroupAttachmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "EnvironmentGroupConfig is a revisioned snapshot of an EnvironmentGroup and its associated routing rules."]
    pub struct GoogleCloudApigeeV1EnvironmentGroupConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hostnames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Host names for the environment group."]
        pub hostnames: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the environment group in the following format: `organizations/{org}/envgroups/{envgroup}`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Revision id that defines the ordering of the EnvironmentGroupConfig resource. The higher the revision, the more recently the configuration was deployed."]
        pub revision_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "routingRules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ordered list of routing rules defining how traffic to this environment group's hostnames should be routed to different environments."]
        pub routing_rules: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1RoutingRule>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique id for the environment group config that will only change if the environment group is deleted and recreated."]
        pub uid: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1EnvironmentGroupConfig {
        pub fn builder() -> GoogleCloudApigeeV1EnvironmentGroupConfigBuilder {
            GoogleCloudApigeeV1EnvironmentGroupConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of an export job."]
    pub struct GoogleCloudApigeeV1Export {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time the export job was created."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datastoreName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the datastore that is the destination of the export job [datastore]"]
        pub datastore_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the export job."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Error is set when export fails"]
        pub error: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Execution time for this export job. If the job is still in progress, it will be set to the amount of time that has elapsed since`created`, in seconds. Else, it will set to (`updated` - `created`), in seconds."]
        pub execution_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display name of the export job."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "self")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Self link of the export job. A URI that can be used to retrieve the status of an export job. Example: `/organizations/myorg/environments/myenv/analytics/exports/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd`"]
        pub _self: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Status of the export job. Valid values include `enqueued`, `running`, `completed`, and `failed`."]
        pub state: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time the export job was last updated."]
        pub updated: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1Export {
        pub fn builder() -> GoogleCloudApigeeV1ExportBuilder {
            GoogleCloudApigeeV1ExportBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request body for [CreateExportRequest]"]
    pub struct GoogleCloudApigeeV1ExportRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "csvDelimiter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Delimiter used in the CSV file, if `outputFormat` is set to `csv`. Defaults to the `,` (comma) character. Supported delimiter characters include comma (`,`), pipe (`|`), and tab (`\\t`)."]
        pub csv_delimiter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datastoreName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Name of the preconfigured datastore."]
        pub datastore_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Date range of the data to export."]
        pub date_range: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1DateRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Description of the export job."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Display name of the export job."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Output format of the export. Valid values include: `csv` or `json`. Defaults to `json`. Note: Configure the delimiter for CSV output using the `csvDelimiter` property."]
        pub output_format: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ExportRequest {
        pub fn builder() -> GoogleCloudApigeeV1ExportRequestBuilder {
            GoogleCloudApigeeV1ExportRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1FlowHook {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "continueOnError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Flag that specifies whether execution should continue if the flow hook throws an exception. Set to `true` to continue execution. Set to `false` to stop execution if the flow hook throws an exception.Defaults to `true`."]
        pub continue_on_error: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the flow hook."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "flowHookPoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Where in the API call flow the flow hook is invoked. Must be one of `PreProxyFlowHook`, `PostProxyFlowHook`, `PreTargetFlowHook`, or `PostTargetFlowHook`."]
        pub flow_hook_point: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sharedFlow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shared flow attached to this flow hook, or empty if there is none attached."]
        pub shared_flow: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1FlowHook {
        pub fn builder() -> GoogleCloudApigeeV1FlowHookBuilder {
            GoogleCloudApigeeV1FlowHookBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1FlowHookConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "continueOnError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag that specifies whether the flow should abort after an error in the flow hook. Defaults to `true` (continue on error)."]
        pub continue_on_error: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the flow hook in the following format: `organizations/{org}/environments/{env}/flowhooks/{point}`. Valid `point` values include: `PreProxyFlowHook`, `PostProxyFlowHook`, `PreTargetFlowHook`, and `PostTargetFlowHook`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sharedFlowName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the shared flow to invoke in the following format: `organizations/{org}/sharedflows/{sharedflow}`"]
        pub shared_flow_name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1FlowHookConfig {
        pub fn builder() -> GoogleCloudApigeeV1FlowHookConfigBuilder {
            GoogleCloudApigeeV1FlowHookConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for GetSyncAuthorization."]
    pub struct GoogleCloudApigeeV1GetSyncAuthorizationRequest {}
    impl GoogleCloudApigeeV1GetSyncAuthorizationRequest {
        pub fn builder() -> GoogleCloudApigeeV1GetSyncAuthorizationRequestBuilder {
            GoogleCloudApigeeV1GetSyncAuthorizationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1IngressConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environmentGroups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of environment groups in the organization."]
        pub environment_groups: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1EnvironmentGroupConfig>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the resource in the following format: `organizations/{org}/deployedIngressConfig`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionCreateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time at which the IngressConfig revision was created."]
        pub revision_create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Revision id that defines the ordering on IngressConfig resources. The higher the revision, the more recently the configuration was deployed."]
        pub revision_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique id for the ingress config that will only change if the organization is deleted and recreated."]
        pub uid: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1IngressConfig {
        pub fn builder() -> GoogleCloudApigeeV1IngressConfigBuilder {
            GoogleCloudApigeeV1IngressConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Apigee runtime instance."]
    pub struct GoogleCloudApigeeV1Instance {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time the instance was created in milliseconds since epoch."]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Description of the instance."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskEncryptionKeyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Customer Managed Encryption Key (CMEK) used for disk and volume encryption. Required for Apigee paid subscriptions only. Use the following format: `projects/([^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/]+)`"]
        pub disk_encryption_key_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Display name for the instance."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "host")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Hostname or IP address of the exposed Apigee endpoint used by clients to connect to the service."]
        pub host: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time the instance was last modified in milliseconds since epoch."]
        pub last_modified_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Compute Engine location where the instance resides."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Resource ID of the instance. Values must match the regular expression `^a-z{0,30}[a-z\\d]$`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "peeringCidrRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The size of the CIDR block range that will be reserved by the instance. If not specified, default to SLASH_16."]
        pub peering_cidr_range:
            ::std::option::Option<GoogleCloudApigeeV1InstancePeeringCidrRangeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "port")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Port number of the exposed Apigee endpoint."]
        pub port: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. State of the instance. Values other than ACTIVE means the resource is not ready to use."]
        pub state: ::std::option::Option<GoogleCloudApigeeV1InstanceStateEnum>,
    }
    impl GoogleCloudApigeeV1Instance {
        pub fn builder() -> GoogleCloudApigeeV1InstanceBuilder {
            GoogleCloudApigeeV1InstanceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The size of the CIDR block range that will be reserved by the instance. If not specified, default to SLASH_16."]
    pub enum GoogleCloudApigeeV1InstancePeeringCidrRangeEnum {
        #[serde(rename = "CIDR_RANGE_UNSPECIFIED")]
        #[doc = "Range not specified."]
        CidrRangeUnspecified,
        #[serde(rename = "SLASH_16")]
        #[doc = "The \"/16\" CIDR range."]
        Slash16,
        #[serde(rename = "SLASH_20")]
        #[doc = "The \"/20\" CIDR range."]
        Slash20,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1InstancePeeringCidrRangeEnum {
        fn default() -> Self {
            Self::CidrRangeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. State of the instance. Values other than ACTIVE means the resource is not ready to use."]
    pub enum GoogleCloudApigeeV1InstanceStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Resource is in an unspecified state."]
        StateUnspecified,
        #[serde(rename = "CREATING")]
        #[doc = "Resource is being created."]
        Creating,
        #[serde(rename = "ACTIVE")]
        #[doc = "Resource is provisioned and ready to use."]
        Active,
        #[serde(rename = "DELETING")]
        #[doc = "The resource is being deleted."]
        Deleting,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1InstanceStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "InstanceAttachment represents the installation of an environment onto an instance."]
    pub struct GoogleCloudApigeeV1InstanceAttachment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time the attachment was created in milliseconds since epoch."]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the attached environment."]
        pub environment: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. ID of the attachment."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1InstanceAttachment {
        pub fn builder() -> GoogleCloudApigeeV1InstanceAttachmentBuilder {
            GoogleCloudApigeeV1InstanceAttachmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The status of a deployment as reported by a single instance."]
    pub struct GoogleCloudApigeeV1InstanceDeploymentStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deployedRevisions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Revisions currently deployed in MPs."]
        pub deployed_revisions: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRevision>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deployedRoutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current routes deployed in the ingress routing table. A route which is missing will appear in missing_routes."]
        pub deployed_routes: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRoute>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the instance reporting the status."]
        pub instance: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1InstanceDeploymentStatus {
        pub fn builder() -> GoogleCloudApigeeV1InstanceDeploymentStatusBuilder {
            GoogleCloudApigeeV1InstanceDeploymentStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Revisions deployed in the MPs."]
    pub struct GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRevision {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "percentage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The percentage of MP replicas reporting this revision"]
        pub percentage: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The proxy revision reported as deployed."]
        pub revision: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRevision {
        pub fn builder() -> GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRevisionBuilder {
            GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRevisionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A route deployed in the ingress routing table."]
    pub struct GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRoute {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basepath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basepath in the routing table."]
        pub basepath: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "envgroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The envgroup where this route is installed."]
        pub envgroup: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The destination environment. This will be empty if the route is not yet reported."]
        pub environment: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "percentage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The percentage of ingress replicas reporting this route."]
        pub percentage: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRoute {
        pub fn builder() -> GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRouteBuilder {
            GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRouteBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1KeyAliasReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aliasId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Alias ID. Must exist in the keystore referred to by the reference."]
        pub alias_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reference name in the following format: `organizations/{org}/environments/{env}/references/{reference}`"]
        pub reference: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1KeyAliasReference {
        pub fn builder() -> GoogleCloudApigeeV1KeyAliasReferenceBuilder {
            GoogleCloudApigeeV1KeyAliasReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A collection of key, value string pairs"]
    pub struct GoogleCloudApigeeV1KeyValueMap {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encrypted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If `true` entry values will be encrypted."]
        pub encrypted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The id of the key value map."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1KeyValueMap {
        pub fn builder() -> GoogleCloudApigeeV1KeyValueMapBuilder {
            GoogleCloudApigeeV1KeyValueMapBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Datastore for Certificates and Aliases."]
    pub struct GoogleCloudApigeeV1Keystore {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aliases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Aliases in this keystore."]
        pub aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Resource ID for this keystore. Values must match the regular expression `[\\w[:space:]-.]{1,255}`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1Keystore {
        pub fn builder() -> GoogleCloudApigeeV1KeystoreBuilder {
            GoogleCloudApigeeV1KeystoreBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1KeystoreConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aliases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aliases in the keystore."]
        pub aliases: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1AliasRevisionConfig>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name in the following format: `organizations/{org}/environments/{env}/keystores/{keystore}`"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1KeystoreConfig {
        pub fn builder() -> GoogleCloudApigeeV1KeystoreConfigBuilder {
            GoogleCloudApigeeV1KeystoreConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "the response for ListApiCategoriesRequest."]
    pub struct GoogleCloudApigeeV1ListApiCategoriesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of categories."]
        pub data: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ApiCategoryData>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID that can be used to find errors in the log files."]
        pub error_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the operation."]
        pub message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID that can be used to find request details in the log files."]
        pub request_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the operation."]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ListApiCategoriesResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListApiCategoriesResponseBuilder {
            GoogleCloudApigeeV1ListApiCategoriesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1ListApiProductsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiProduct")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Lists all API product names defined for an organization."]
        pub api_product: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ApiProduct>>,
        >,
    }
    impl GoogleCloudApigeeV1ListApiProductsResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListApiProductsResponseBuilder {
            GoogleCloudApigeeV1ListApiProductsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1ListApiProxiesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "proxies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub proxies:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ApiProxy>>>,
    }
    impl GoogleCloudApigeeV1ListApiProxiesResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListApiProxiesResponseBuilder {
            GoogleCloudApigeeV1ListApiProxiesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1ListAppsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "app")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub app: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1App>>>,
    }
    impl GoogleCloudApigeeV1ListAppsResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListAppsResponseBuilder {
            GoogleCloudApigeeV1ListAppsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for ListAsyncQueries."]
    pub struct GoogleCloudApigeeV1ListAsyncQueriesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The asynchronous queries belong to requested resource name."]
        pub queries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1AsyncQuery>>,
        >,
    }
    impl GoogleCloudApigeeV1ListAsyncQueriesResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListAsyncQueriesResponseBuilder {
            GoogleCloudApigeeV1ListAsyncQueriesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This message encapsulates a list of custom report definitions"]
    pub struct GoogleCloudApigeeV1ListCustomReportsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "qualifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub qualifier: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1CustomReport>>,
        >,
    }
    impl GoogleCloudApigeeV1ListCustomReportsResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListCustomReportsResponseBuilder {
            GoogleCloudApigeeV1ListCustomReportsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for ListDataCollectors."]
    pub struct GoogleCloudApigeeV1ListDataCollectorsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataCollectors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data collectors in the specified organization."]
        pub data_collectors: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1DataCollector>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page token that you can include in a ListDataCollectors request to retrieve the next page. If omitted, no subsequent pages exist."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ListDataCollectorsResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListDataCollectorsResponseBuilder {
            GoogleCloudApigeeV1ListDataCollectorsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for ListDatastores"]
    pub struct GoogleCloudApigeeV1ListDatastoresResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datastores")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of datastores"]
        pub datastores:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Datastore>>>,
    }
    impl GoogleCloudApigeeV1ListDatastoresResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListDatastoresResponseBuilder {
            GoogleCloudApigeeV1ListDatastoresResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1ListDebugSessionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page token that you can include in a ListDebugSessionsRequest to retrieve the next page. If omitted, no subsequent pages exist."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Session info that includes debug session ID and the first transaction creation timestamp."]
        pub sessions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Session>>>,
    }
    impl GoogleCloudApigeeV1ListDebugSessionsResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListDebugSessionsResponseBuilder {
            GoogleCloudApigeeV1ListDebugSessionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1ListDeploymentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deployments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of deployments."]
        pub deployments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Deployment>>,
        >,
    }
    impl GoogleCloudApigeeV1ListDeploymentsResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListDeploymentsResponseBuilder {
            GoogleCloudApigeeV1ListDeploymentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1ListDeveloperAppsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "app")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of developer apps and their credentials."]
        pub app: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1DeveloperApp>>,
        >,
    }
    impl GoogleCloudApigeeV1ListDeveloperAppsResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListDeveloperAppsResponseBuilder {
            GoogleCloudApigeeV1ListDeveloperAppsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for ListEnvironmentGroupAttachments."]
    pub struct GoogleCloudApigeeV1ListEnvironmentGroupAttachmentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environmentGroupAttachments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "EnvironmentGroupAttachments for the specified environment group."]
        pub environment_group_attachments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1EnvironmentGroupAttachment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page token that you can include in a ListEnvironmentGroupAttachments request to retrieve the next page. If omitted, no subsequent pages exist."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ListEnvironmentGroupAttachmentsResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListEnvironmentGroupAttachmentsResponseBuilder {
            GoogleCloudApigeeV1ListEnvironmentGroupAttachmentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for ListEnvironmentGroups."]
    pub struct GoogleCloudApigeeV1ListEnvironmentGroupsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environmentGroups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "EnvironmentGroups in the specified organization."]
        pub environment_groups: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1EnvironmentGroup>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page token that you can include in a ListEnvironmentGroups request to retrieve the next page. If omitted, no subsequent pages exist."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ListEnvironmentGroupsResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListEnvironmentGroupsResponseBuilder {
            GoogleCloudApigeeV1ListEnvironmentGroupsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for ListEnvironmentResources"]
    pub struct GoogleCloudApigeeV1ListEnvironmentResourcesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceFile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of resources files."]
        pub resource_file: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ResourceFile>>,
        >,
    }
    impl GoogleCloudApigeeV1ListEnvironmentResourcesResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListEnvironmentResourcesResponseBuilder {
            GoogleCloudApigeeV1ListEnvironmentResourcesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for ListExports"]
    pub struct GoogleCloudApigeeV1ListExportsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exports")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of the export jobs."]
        pub exports:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Export>>>,
    }
    impl GoogleCloudApigeeV1ListExportsResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListExportsResponseBuilder {
            GoogleCloudApigeeV1ListExportsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1ListHybridIssuersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "issuers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Lists of hybrid services and its trusted issuer email ids."]
        pub issuers: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ServiceIssuersMapping>>,
        >,
    }
    impl GoogleCloudApigeeV1ListHybridIssuersResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListHybridIssuersResponseBuilder {
            GoogleCloudApigeeV1ListHybridIssuersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for ListInstanceAttachments."]
    pub struct GoogleCloudApigeeV1ListInstanceAttachmentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attachments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Attachments for the instance."]
        pub attachments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1InstanceAttachment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page token that you can include in a ListInstanceAttachments request to retrieve the next page of content. If omitted, no subsequent pages exist."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ListInstanceAttachmentsResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListInstanceAttachmentsResponseBuilder {
            GoogleCloudApigeeV1ListInstanceAttachmentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for ListInstances."]
    pub struct GoogleCloudApigeeV1ListInstancesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instances")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instances in the specified organization."]
        pub instances:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Instance>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page token that you can include in a ListInstance request to retrieve the next page of content. If omitted, no subsequent pages exist."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ListInstancesResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListInstancesResponseBuilder {
            GoogleCloudApigeeV1ListInstancesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for ListNatAddresses."]
    pub struct GoogleCloudApigeeV1ListNatAddressesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "natAddresses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of NAT Addresses for the instance."]
        pub nat_addresses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1NatAddress>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page token that you can include in a ListNatAddresses request to retrieve the next page of content. If omitted, no subsequent pages exist."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ListNatAddressesResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListNatAddressesResponseBuilder {
            GoogleCloudApigeeV1ListNatAddressesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1ListOfDevelopersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "developer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of developers."]
        pub developer:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Developer>>>,
    }
    impl GoogleCloudApigeeV1ListOfDevelopersResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListOfDevelopersResponseBuilder {
            GoogleCloudApigeeV1ListOfDevelopersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1ListOrganizationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "organizations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of Apigee organizations and associated GCP projects."]
        pub organizations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1OrganizationProjectMapping>>,
        >,
    }
    impl GoogleCloudApigeeV1ListOrganizationsResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListOrganizationsResponseBuilder {
            GoogleCloudApigeeV1ListOrganizationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1ListSharedFlowsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sharedFlows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub shared_flows: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1SharedFlow>>,
        >,
    }
    impl GoogleCloudApigeeV1ListSharedFlowsResponse {
        pub fn builder() -> GoogleCloudApigeeV1ListSharedFlowsResponseBuilder {
            GoogleCloudApigeeV1ListSharedFlowsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This message type encapsulates additional information about query execution."]
    pub struct GoogleCloudApigeeV1Metadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of error messages as strings."]
        pub errors: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of additional information such as data source, if result was truncated etc. E.g \"notices\": [ \"Source:Postgres\", \"PG Host:uappg0rw.e2e.apigeeks.net\", \"query served by:4b64601e-40de-4eb1-bfb9-eeee7ac929ed\", \"Table used: edge.api.uapgroup2.agg_api\" ]"]
        pub notices: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudApigeeV1Metadata {
        pub fn builder() -> GoogleCloudApigeeV1MetadataBuilder {
            GoogleCloudApigeeV1MetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This message type encapsulates the metric data point. Example: { \"name\": \"sum(message_count)\", \"values\" : [ { \"timestamp\": 1549004400000, \"value\": \"39.0\" }, { \"timestamp\" : 1548997200000, \"value\" : \"0.0\" } ] } or { \"name\": \"sum(message_count)\", \"values\" : [\"39.0\"] }"]
    pub struct GoogleCloudApigeeV1Metric {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains the metric name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of metric values. Possible value format: \"values\":[\"39.0\"] or \"values\":[ { \"value\": \"39.0\", \"timestamp\": 1232434354} ]"]
        pub values: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
    }
    impl GoogleCloudApigeeV1Metric {
        pub fn builder() -> GoogleCloudApigeeV1MetricBuilder {
            GoogleCloudApigeeV1MetricBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Apigee NAT(network address translation) address. A NAT address is a static external IP address used for Internet egress traffic."]
    pub struct GoogleCloudApigeeV1NatAddress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The static IPV4 address."]
        pub ip_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Resource ID of the NAT address."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. State of the nat address."]
        pub state: ::std::option::Option<GoogleCloudApigeeV1NatAddressStateEnum>,
    }
    impl GoogleCloudApigeeV1NatAddress {
        pub fn builder() -> GoogleCloudApigeeV1NatAddressBuilder {
            GoogleCloudApigeeV1NatAddressBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. State of the nat address."]
    pub enum GoogleCloudApigeeV1NatAddressStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "The resource is in an unspecified state."]
        StateUnspecified,
        #[serde(rename = "CREATING")]
        #[doc = "The NAT address is being created."]
        Creating,
        #[serde(rename = "RESERVED")]
        #[doc = "The NAT address is reserved but not yet used for Internet egress."]
        Reserved,
        #[serde(rename = "ACTIVE")]
        #[doc = "The NAT address is active and used for Internet egress."]
        Active,
        #[serde(rename = "DELETING")]
        #[doc = "The NAT address is being deleted."]
        Deleting,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1NatAddressStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Operation represents the pairing of REST resource path and the actions (verbs) allowed on the resource path."]
    pub struct GoogleCloudApigeeV1Operation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "methods")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "methods refers to the REST verbs as in https://www.w3.org/Protocols/rfc2616/rfc2616-sec9.html. When none specified, all verb types are allowed."]
        pub methods: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. resource represents REST resource path associated with the proxy/remote service."]
        pub resource: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1Operation {
        pub fn builder() -> GoogleCloudApigeeV1OperationBuilder {
            GoogleCloudApigeeV1OperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "OperationConfig binds the resources in a proxy or remote service with the allowed REST methods and its associated quota enforcement."]
    pub struct GoogleCloudApigeeV1OperationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. API proxy or remote service name with which the resources, methods, and quota are associated."]
        pub api_source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom attributes associated with the operation."]
        pub attributes:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of resource/method pairs for the proxy/remote service, upon which quota will applied. **Note**: Currently, you can specify only a single resource/method pair. The call will fail if more than one resource/method pair is provided."]
        pub operations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Operation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quota")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Quota parameters to be enforced for the resources, methods, api_source combination. If none are specified, quota enforcement will not be done."]
        pub quota: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1Quota>>,
    }
    impl GoogleCloudApigeeV1OperationConfig {
        pub fn builder() -> GoogleCloudApigeeV1OperationConfigBuilder {
            GoogleCloudApigeeV1OperationConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "List of operation configuration details associated with Apigee API proxies or remote services. Remote services are non-Apigee proxies, such as Istio-Envoy."]
    pub struct GoogleCloudApigeeV1OperationGroup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationConfigType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag that specifes whether the configuration is for Apigee API proxy or a remote service. Valid values are `proxy` or `remoteservice`. Defaults to `proxy`. Set to `proxy` when Apigee API proxies are associated with the API product. Set to `remoteservice` when non-Apigee proxies like Istio-Envoy are associated with the API product."]
        pub operation_config_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. List of operation configurations for either Apigee API proxies or other remote services that are associated with this API product."]
        pub operation_configs: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1OperationConfig>>,
        >,
    }
    impl GoogleCloudApigeeV1OperationGroup {
        pub fn builder() -> GoogleCloudApigeeV1OperationGroupBuilder {
            GoogleCloudApigeeV1OperationGroupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata describing an Operation."]
    pub struct GoogleCloudApigeeV1OperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub operation_type:
            ::std::option::Option<GoogleCloudApigeeV1OperationMetadataOperationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub state: ::std::option::Option<GoogleCloudApigeeV1OperationMetadataStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetResourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the resource for which the operation is operating on."]
        pub target_resource_name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1OperationMetadata {
        pub fn builder() -> GoogleCloudApigeeV1OperationMetadataBuilder {
            GoogleCloudApigeeV1OperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleCloudApigeeV1OperationMetadataOperationTypeEnum {
        #[serde(rename = "OPERATION_TYPE_UNSPECIFIED")]
        #[doc = ""]
        OperationTypeUnspecified,
        #[serde(rename = "INSERT")]
        #[doc = ""]
        Insert,
        #[serde(rename = "DELETE")]
        #[doc = ""]
        Delete,
        #[serde(rename = "UPDATE")]
        #[doc = ""]
        Update,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1OperationMetadataOperationTypeEnum {
        fn default() -> Self {
            Self::OperationTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleCloudApigeeV1OperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = ""]
        StateUnspecified,
        #[serde(rename = "NOT_STARTED")]
        #[doc = ""]
        NotStarted,
        #[serde(rename = "IN_PROGRESS")]
        #[doc = ""]
        InProgress,
        #[serde(rename = "FINISHED")]
        #[doc = ""]
        Finished,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1OperationMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1OptimizedStats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "Response")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field wraps the stats response for Js Optimized Scenario with a Response key. E.g. { \"Response\": { \"TimeUnit\": [], \"metaData\": { \"errors\": [], \"notices\": [ \"Source:Postgres\", \"Table used: edge.api.aaxgroup001.agg_api\", \"PG Host:ruappg08-ro.production.apigeeks.net\", \"query served by:80c4ebca-6a10-4a2e-8faf-c60c1ee306ca\" ] }, \"resultTruncated\": false, \"stats\": { \"data\": [ { \"identifier\": { \"names\": [ \"apiproxy\" ], \"values\": [ \"sirjee\" ] }, \"metric\": [ { \"env\": \"prod\", \"name\": \"sum(message_count)\", \"values\": [ 36.0 ] }, { \"env\": \"prod\", \"name\": \"sum(is_error)\", \"values\": [ 36.0 ] } ] } ] } } }"]
        pub response:
            ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1OptimizedStatsResponse>>,
    }
    impl GoogleCloudApigeeV1OptimizedStats {
        pub fn builder() -> GoogleCloudApigeeV1OptimizedStatsBuilder {
            GoogleCloudApigeeV1OptimizedStatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This message type encapsulates a data node as represented below: { \"identifier\": { \"names\": [ \"apiproxy\" ], \"values\": [ \"sirjee\" ] }, \"metric\": [ { \"env\": \"prod\", \"name\": \"sum(message_count)\", \"values\": [ 36.0 ] } ] } OR { \"env\": \"prod\", \"name\": \"sum(message_count)\", \"values\": [ 36.0 ] } Depending on whether a dimension is present in the query or not the data node type can be a simple metric value or dimension identifier with list of metrics."]
    pub struct GoogleCloudApigeeV1OptimizedStatsNode {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
    }
    impl GoogleCloudApigeeV1OptimizedStatsNode {
        pub fn builder() -> GoogleCloudApigeeV1OptimizedStatsNodeBuilder {
            GoogleCloudApigeeV1OptimizedStatsNodeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This message type encapsulates a response format for Js Optimized Scenario."]
    pub struct GoogleCloudApigeeV1OptimizedStatsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "TimeUnit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains a list of time unit values. Time unit refers to an epoch timestamp value."]
        pub time_unit: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metaData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains metadata information about the query executed"]
        pub meta_data: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1Metadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resultTruncated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This ia a boolean field to indicate if the results were truncated based on the limit parameter."]
        pub result_truncated: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains a stats results."]
        pub stats: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1OptimizedStatsNode>>,
    }
    impl GoogleCloudApigeeV1OptimizedStatsResponse {
        pub fn builder() -> GoogleCloudApigeeV1OptimizedStatsResponseBuilder {
            GoogleCloudApigeeV1OptimizedStatsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1Organization {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "analyticsRegion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Primary GCP region for analytics data storage. For valid values, see [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org)."]
        pub analytics_region: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Not used by Apigee."]
        pub attributes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorizedNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Compute Engine network used for Service Networking to be peered with Apigee runtime instances. See [Getting started with the Service Networking API](https://cloud.google.com/service-infrastructure/docs/service-networking/getting-started). Valid only when [RuntimeType](#RuntimeType) is set to `CLOUD`. The value can be updated only when there are no runtime instances. For example: `default`. Apigee also supports shared VPC (that is, the host network project is not the same as the one that is peering with Apigee). See [Shared VPC overview](https://cloud.google.com/vpc/docs/shared-vpc). To use a shared VPC network, use the following format: `projects/{host-project-id}/{region}/networks/{network-name}`. For example: `projects/my-sharedvpc-host/global/networks/mynetwork` **Note:** Not supported for Apigee hybrid."]
        pub authorized_network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "billingType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Billing type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing)."]
        pub billing_type: ::std::option::Option<GoogleCloudApigeeV1OrganizationBillingTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caCertificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Base64-encoded public certificate for the root CA of the Apigee organization. Valid only when [RuntimeType](#RuntimeType) is `CLOUD`."]
        pub ca_certificate: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time that the Apigee organization was created in milliseconds since epoch."]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Not used by Apigee."]
        pub customer_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the Apigee organization."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. List of environments in the Apigee organization."]
        pub environments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time that the Apigee organization was last modified in milliseconds since epoch."]
        pub last_modified_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Name of the Apigee organization."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Project ID associated with the Apigee organization."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Properties defined in the Apigee organization profile."]
        pub properties: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1Properties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "runtimeDatabaseEncryptionKeyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cloud KMS key name used for encrypting the data that is stored and replicated across runtime instances. Update is not allowed after the organization is created. If not specified, a Google-Managed encryption key will be used. Valid only when [RuntimeType](#RuntimeType) is `CLOUD`. For example: \"projects/foo/locations/us/keyRings/bar/cryptoKeys/baz\". **Note:** Not supported for Apigee hybrid."]
        pub runtime_database_encryption_key_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "runtimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Runtime type of the Apigee organization based on the Apigee subscription purchased."]
        pub runtime_type: ::std::option::Option<GoogleCloudApigeeV1OrganizationRuntimeTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. State of the organization. Values other than ACTIVE means the resource is not ready to use."]
        pub state: ::std::option::Option<GoogleCloudApigeeV1OrganizationStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscriptionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. DEPRECATED: This will eventually be replaced by BillingType. Subscription type of the Apigee organization. Valid values include trial (free, limited, and for evaluation purposes only) or paid (full subscription has been purchased). See [Apigee pricing](https://cloud.google.com/apigee/pricing/)."]
        pub subscription_type:
            ::std::option::Option<GoogleCloudApigeeV1OrganizationSubscriptionTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Not used by Apigee."]
        pub _type: ::std::option::Option<GoogleCloudApigeeV1OrganizationTypeEnum>,
    }
    impl GoogleCloudApigeeV1Organization {
        pub fn builder() -> GoogleCloudApigeeV1OrganizationBuilder {
            GoogleCloudApigeeV1OrganizationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Billing type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing)."]
    pub enum GoogleCloudApigeeV1OrganizationBillingTypeEnum {
        #[serde(rename = "BILLING_TYPE_UNSPECIFIED")]
        #[doc = "Billing type not specified."]
        BillingTypeUnspecified,
        #[serde(rename = "SUBSCRIPTION")]
        #[doc = "A pre-paid subscription to Apigee."]
        Subscription,
        #[serde(rename = "EVALUATION")]
        #[doc = "Free and limited access to Apigee for evaluation purposes only. only."]
        Evaluation,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1OrganizationBillingTypeEnum {
        fn default() -> Self {
            Self::BillingTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Runtime type of the Apigee organization based on the Apigee subscription purchased."]
    pub enum GoogleCloudApigeeV1OrganizationRuntimeTypeEnum {
        #[serde(rename = "RUNTIME_TYPE_UNSPECIFIED")]
        #[doc = "Runtime type not specified."]
        RuntimeTypeUnspecified,
        #[serde(rename = "CLOUD")]
        #[doc = "Google-managed Apigee runtime."]
        Cloud,
        #[serde(rename = "HYBRID")]
        #[doc = "User-managed Apigee hybrid runtime."]
        Hybrid,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1OrganizationRuntimeTypeEnum {
        fn default() -> Self {
            Self::RuntimeTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. State of the organization. Values other than ACTIVE means the resource is not ready to use."]
    pub enum GoogleCloudApigeeV1OrganizationStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Resource is in an unspecified state."]
        StateUnspecified,
        #[serde(rename = "CREATING")]
        #[doc = "Resource is being created."]
        Creating,
        #[serde(rename = "ACTIVE")]
        #[doc = "Resource is provisioned and ready to use."]
        Active,
        #[serde(rename = "DELETING")]
        #[doc = "The resource is being deleted."]
        Deleting,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1OrganizationStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. DEPRECATED: This will eventually be replaced by BillingType. Subscription type of the Apigee organization. Valid values include trial (free, limited, and for evaluation purposes only) or paid (full subscription has been purchased). See [Apigee pricing](https://cloud.google.com/apigee/pricing/)."]
    pub enum GoogleCloudApigeeV1OrganizationSubscriptionTypeEnum {
        #[serde(rename = "SUBSCRIPTION_TYPE_UNSPECIFIED")]
        #[doc = "Subscription type not specified."]
        SubscriptionTypeUnspecified,
        #[serde(rename = "PAID")]
        #[doc = "Full subscription to Apigee has been purchased."]
        Paid,
        #[serde(rename = "TRIAL")]
        #[doc = "Subscription to Apigee is free, limited, and used for evaluation purposes only."]
        Trial,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1OrganizationSubscriptionTypeEnum {
        fn default() -> Self {
            Self::SubscriptionTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Not used by Apigee."]
    pub enum GoogleCloudApigeeV1OrganizationTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Subscription type not specified."]
        TypeUnspecified,
        #[serde(rename = "TYPE_TRIAL")]
        #[doc = "Subscription to Apigee is free, limited, and used for evaluation purposes only."]
        TypeTrial,
        #[serde(rename = "TYPE_PAID")]
        #[doc = "Full subscription to Apigee has been purchased. See [Apigee pricing](https://cloud.google.com/apigee/pricing/)."]
        TypePaid,
        #[serde(rename = "TYPE_INTERNAL")]
        #[doc = "For internal users only."]
        TypeInternal,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1OrganizationTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1OrganizationProjectMapping {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "organization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the Apigee organization."]
        pub organization: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of GCP projects associated with the Apigee organization."]
        pub project_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudApigeeV1OrganizationProjectMapping {
        pub fn builder() -> GoogleCloudApigeeV1OrganizationProjectMappingBuilder {
            GoogleCloudApigeeV1OrganizationProjectMappingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1PodStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Version of the application running in the pod."]
        pub app_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deploymentStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the deployment. Valid values include: - `deployed`: Successful. - `error` : Failed. - `pending` : Pod has not yet reported on the deployment."]
        pub deployment_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deploymentStatusTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the deployment status was reported in milliseconds since epoch."]
        pub deployment_status_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deploymentTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the proxy was deployed in milliseconds since epoch."]
        pub deployment_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "podName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the pod which is reporting the status."]
        pub pod_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "podStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Overall status of the pod (not this specific deployment). Valid values include: - `active`: Up to date. - `stale` : Recently out of date. Pods that have not reported status in a long time are excluded from the output."]
        pub pod_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "podStatusTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the pod status was reported in milliseconds since epoch."]
        pub pod_status_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statusCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Code associated with the deployment status."]
        pub status_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statusCodeDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human-readable message associated with the status code."]
        pub status_code_details: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1PodStatus {
        pub fn builder() -> GoogleCloudApigeeV1PodStatusBuilder {
            GoogleCloudApigeeV1PodStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Point is a group of information collected by runtime plane at critical points of the message flow of the processed API request. This is a list of supported point IDs, categorized to three major buckets. For each category, debug points that we are currently supporting are listed below: - Flow status debug points: StateChange FlowInfo Condition Execution DebugMask Error - Flow control debug points: FlowCallout Paused Resumed FlowReturn BreakFlow Error - Runtime debug points: ScriptExecutor FlowCalloutStepDefinition CustomTarget StepDefinition Oauth2ServicePoint RaiseFault NodeJS The detail information of the given debug point is stored in a list of results."]
    pub struct GoogleCloudApigeeV1Point {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of a step in the transaction."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results extracted from a given debug point."]
        pub results:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Result>>>,
    }
    impl GoogleCloudApigeeV1Point {
        pub fn builder() -> GoogleCloudApigeeV1PointBuilder {
            GoogleCloudApigeeV1PointBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message for compatibility with legacy Edge specification for Java Properties object in JSON."]
    pub struct GoogleCloudApigeeV1Properties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of all properties in the object"]
        pub property:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Property>>>,
    }
    impl GoogleCloudApigeeV1Properties {
        pub fn builder() -> GoogleCloudApigeeV1PropertiesBuilder {
            GoogleCloudApigeeV1PropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single property entry in the Properties message."]
    pub struct GoogleCloudApigeeV1Property {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The property key"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The property value"]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1Property {
        pub fn builder() -> GoogleCloudApigeeV1PropertyBuilder {
            GoogleCloudApigeeV1PropertyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for ProvisionOrganization."]
    pub struct GoogleCloudApigeeV1ProvisionOrganizationRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "analyticsRegion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Primary Cloud Platform region for analytics data storage. For valid values, see [Create an organization](https://cloud.google.com/apigee/docs/hybrid/latest/precog-provision). Defaults to `us-west1`."]
        pub analytics_region: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorizedNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the customer project's VPC network. If provided, the network needs to be peered through Service Networking. If none is provided, the organization will have access only to the public internet."]
        pub authorized_network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "runtimeLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cloud Platform location for the runtime instance. Defaults to `us-west1-a`."]
        pub runtime_location: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ProvisionOrganizationRequest {
        pub fn builder() -> GoogleCloudApigeeV1ProvisionOrganizationRequestBuilder {
            GoogleCloudApigeeV1ProvisionOrganizationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1Query {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "csvDelimiter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Delimiter used in the CSV file, if `outputFormat` is set to `csv`. Defaults to the `,` (comma) character. Supported delimiter characters include comma (`,`), pipe (`|`), and tab (`\\t`)."]
        pub csv_delimiter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of dimensions. https://docs.apigee.com/api-platform/analytics/analytics-reference#dimensions"]
        pub dimensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "envgroupHostname")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hostname needs to be specified if query intends to run at host level. This field is only allowed when query is submitted by CreateHostAsyncQuery where analytics data will be grouped by organization and hostname."]
        pub envgroup_hostname: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boolean expression that can be used to filter data. Filter expressions can be combined using AND/OR terms and should be fully parenthesized to avoid ambiguity. See Analytics metrics, dimensions, and filters reference https://docs.apigee.com/api-platform/analytics/analytics-reference for more information on the fields available to filter on. For more information on the tokens that you use to build filter expressions, see Filter expression syntax. https://docs.apigee.com/api-platform/analytics/asynch-reports-api#filter-expression-syntax"]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupByTimeUnit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time unit used to group the result set. Valid values include: second, minute, hour, day, week, or month. If a query includes groupByTimeUnit, then the result is an aggregation based on the specified time unit and the resultant timestamp does not include milliseconds precision. If a query omits groupByTimeUnit, then the resultant timestamp includes milliseconds precision."]
        pub group_by_time_unit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum number of rows that can be returned in the result."]
        pub limit: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of Metrics."]
        pub metrics: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1QueryMetric>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Asynchronous Query Name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Valid values include: `csv` or `json`. Defaults to `json`. Note: Configure the delimiter for CSV output using the csvDelimiter property."]
        pub output_format: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportDefinitionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Asynchronous Report ID."]
        pub report_definition_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Time range for the query. Can use the following predefined strings to specify the time range: `last60minutes` `last24hours` `last7days` Or, specify the timeRange as a structure describing start and end timestamps in the ISO format: yyyy-mm-ddThh:mm:ssZ. Example: \"timeRange\": { \"start\": \"2018-07-29T00:13:00Z\", \"end\": \"2018-08-01T00:18:00Z\" }"]
        pub time_range: ::std::option::Option<::serde_json::Value>,
    }
    impl GoogleCloudApigeeV1Query {
        pub fn builder() -> GoogleCloudApigeeV1QueryBuilder {
            GoogleCloudApigeeV1QueryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1QueryMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dimensions of the AsyncQuery."]
        pub dimensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End timestamp of the query range."]
        pub end_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metrics of the AsyncQuery. Example: [\"name:message_count,func:sum,alias:sum_message_count\"]"]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output format."]
        pub output_format: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start timestamp of the query range."]
        pub start_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeUnit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Query GroupBy time unit."]
        pub time_unit: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1QueryMetadata {
        pub fn builder() -> GoogleCloudApigeeV1QueryMetadataBuilder {
            GoogleCloudApigeeV1QueryMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "More info about Metric: https://docs.apigee.com/api-platform/analytics/analytics-reference#metrics"]
    pub struct GoogleCloudApigeeV1QueryMetric {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alias")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Alias for the metric. Alias will be used to replace metric name in query results."]
        pub alias: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "function")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aggregation function: avg, min, max, or sum."]
        pub function: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Metric name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One of `+`, `-`, `/`, `%`, `*`."]
        pub operator: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operand value should be provided when operator is set."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1QueryMetric {
        pub fn builder() -> GoogleCloudApigeeV1QueryMetricBuilder {
            GoogleCloudApigeeV1QueryMetricBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Quota contains the essential parameters needed that can be applied on a proxy/remote service, resources and methods combination associated with this API product. While setting of Quota is optional, setting it prevents requests from exceeding the provisioned parameters."]
    pub struct GoogleCloudApigeeV1Quota {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "interval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Time interval over which the number of request messages is calculated."]
        pub interval: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Upper limit allowed for the time interval and time unit specified. Requests exceeding this limit will be rejected."]
        pub limit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeUnit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time unit defined for the `interval`. Valid values include `minute`, `hour`, `day`, or `month`. If `limit` and `interval` are valid, the default value is `hour`; otherwise, the default is null."]
        pub time_unit: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1Quota {
        pub fn builder() -> GoogleCloudApigeeV1QuotaBuilder {
            GoogleCloudApigeeV1QuotaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Reference configuration. References must refer to a keystore that also exists in the parent environment."]
    pub struct GoogleCloudApigeeV1Reference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A human-readable description of this reference."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The resource id of this reference. Values must match the regular expression [\\w\\s\\-.]+."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "refers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The id of the resource to which this reference refers. Must be the id of a resource that exists in the parent environment and is of the given resource_type."]
        pub refers: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of resource referred to by this reference. Valid values are 'KeyStore' or 'TrustStore'."]
        pub resource_type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1Reference {
        pub fn builder() -> GoogleCloudApigeeV1ReferenceBuilder {
            GoogleCloudApigeeV1ReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1ReferenceConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the reference in the following format: `organizations/{org}/environments/{env}/references/{reference}`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the referenced resource in the following format: `organizations/{org}/environments/{env}/keystores/{keystore}` Only references to keystore resources are supported."]
        pub resource_name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ReferenceConfig {
        pub fn builder() -> GoogleCloudApigeeV1ReferenceConfigBuilder {
            GoogleCloudApigeeV1ReferenceConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for ReportInstanceStatus."]
    pub struct GoogleCloudApigeeV1ReportInstanceStatusRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceUid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique ID for the instance which is guaranteed to be unique in case the user installs multiple hybrid runtimes with the same instance ID."]
        pub instance_uid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time the report was generated in the runtime. Used to prevent an old status from overwriting a newer one. An instance should space out it's status reports so that clock skew does not play a factor."]
        pub report_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status for config resources"]
        pub resources: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ResourceStatus>>,
        >,
    }
    impl GoogleCloudApigeeV1ReportInstanceStatusRequest {
        pub fn builder() -> GoogleCloudApigeeV1ReportInstanceStatusRequestBuilder {
            GoogleCloudApigeeV1ReportInstanceStatusRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Placeholder for future enhancements to status reporting protocol"]
    pub struct GoogleCloudApigeeV1ReportInstanceStatusResponse {}
    impl GoogleCloudApigeeV1ReportInstanceStatusResponse {
        pub fn builder() -> GoogleCloudApigeeV1ReportInstanceStatusResponseBuilder {
            GoogleCloudApigeeV1ReportInstanceStatusResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1ReportProperty {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "name of the property"]
        pub property: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "property values"]
        pub value:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
    }
    impl GoogleCloudApigeeV1ReportProperty {
        pub fn builder() -> GoogleCloudApigeeV1ReportPropertyBuilder {
            GoogleCloudApigeeV1ReportPropertyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1ResourceConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location of the resource as a URI."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name in the following format: `organizations/{org}/environments/{env}/resourcefiles/{type}/{file}/revisions/{rev}` Only environment-scoped resource files are supported."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ResourceConfig {
        pub fn builder() -> GoogleCloudApigeeV1ResourceConfigBuilder {
            GoogleCloudApigeeV1ResourceConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata about a resource file."]
    pub struct GoogleCloudApigeeV1ResourceFile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the resource file."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource file type. {{ resource_file_type }}"]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ResourceFile {
        pub fn builder() -> GoogleCloudApigeeV1ResourceFileBuilder {
            GoogleCloudApigeeV1ResourceFileBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "List of resource files."]
    pub struct GoogleCloudApigeeV1ResourceFiles {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceFile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of resource files."]
        pub resource_file: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ResourceFile>>,
        >,
    }
    impl GoogleCloudApigeeV1ResourceFiles {
        pub fn builder() -> GoogleCloudApigeeV1ResourceFilesBuilder {
            GoogleCloudApigeeV1ResourceFilesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The status of a resource loaded in the runtime."]
    pub struct GoogleCloudApigeeV1ResourceStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name. Currently only two resources are supported: EnvironmentGroup - organizations/{org}/envgroups/{envgroup} EnvironmentConfig - organizations/{org}/environments/{environment}/deployedConfig"]
        pub resource: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Revisions of the resource currently deployed in the instance."]
        pub revisions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1RevisionStatus>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalReplicas")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of replicas that should have this resource."]
        pub total_replicas: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The uid of the resource. In the unexpected case that the instance has multiple uids for the same name, they should be reported under separate ResourceStatuses."]
        pub uid: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ResourceStatus {
        pub fn builder() -> GoogleCloudApigeeV1ResourceStatusBuilder {
            GoogleCloudApigeeV1ResourceStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Result is short for \"action result\", could be different types identified by \"action_result\" field. Supported types: 1. DebugInfo : generic debug info collected by runtime recorded as a list of properties. For example, the contents could be virtual host info, state change result, or execution metadata. Required fields : properties, timestamp 2. RequestMessage: information of a http request. Contains headers, request URI and http methods type.Required fields : headers, uri, verb 3. ResponseMessage: information of a http response. Contains headers, reason phrase and http status code. Required fields : headers, reasonPhrase, statusCode 4. ErrorMessage: information of a http error message. Contains detail error message, reason phrase and status code. Required fields : content, headers, reasonPhrase, statusCode 5. VariableAccess: a list of variable access actions, can be Get, Set and Remove. Required fields : accessList"]
    pub struct GoogleCloudApigeeV1Result {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ActionResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the action result. Can be one of the five: DebugInfo, RequestMessage, ResponseMessage, ErrorMessage, VariableAccess"]
        pub action_result: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of variable access actions agaist the api proxy. Supported values: Get, Set, Remove."]
        pub access_list:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Access>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error message content. for example, \"content\" : \"{\\\"fault\\\":{\\\"faultstring\\\":\\\"API timed out\\\",\\\"detail\\\":{\\\"errorcode\\\":\\\"flow.APITimedOut\\\"}}}\""]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of HTTP headers. for example, '\"headers\" : [ { \"name\" : \"Content-Length\", \"value\" : \"83\" }, { \"name\" : \"Content-Type\", \"value\" : \"application/json\" } ]'"]
        pub headers:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Property>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name value pairs used for DebugInfo ActionResult."]
        pub properties: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1Properties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonPhrase")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP response phrase"]
        pub reason_phrase: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statusCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP response code"]
        pub status_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of when the result is recorded. Its format is dd-mm-yy hh:mm:ss:xxx. For example, `\"timestamp\" : \"12-08-19 00:31:59:960\"`"]
        pub timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uRI")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relative path of the api proxy. for example, `\"uRI\" : \"/iloveapis\"`"]
        pub u_ri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP method verb"]
        pub verb: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1Result {
        pub fn builder() -> GoogleCloudApigeeV1ResultBuilder {
            GoogleCloudApigeeV1ResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The status of a specific resource revision."]
    pub struct GoogleCloudApigeeV1RevisionStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Errors reported when attempting to load this revision."]
        pub errors: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1UpdateError>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jsonSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The json content of the resource revision."]
        pub json_spec: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replicas")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of replicas that have successfully loaded this revision."]
        pub replicas: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The revision of the resource."]
        pub revision_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1RevisionStatus {
        pub fn builder() -> GoogleCloudApigeeV1RevisionStatusBuilder {
            GoogleCloudApigeeV1RevisionStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1RoutingRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basepath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URI path prefix used to route to the specified environment. May contain one or more wildcards. For example, path segments consisting of a single `*` character will match any string."]
        pub basepath: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "envGroupRevision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The env group config revision_id when this rule was added or last updated. This value is set when the rule is created and will only update if the the environment_id changes. It is used to determine if the runtime is up to date with respect to this rule. This field is omitted from the IngressConfig unless the GetDeployedIngressConfig API is called with view=FULL."]
        pub env_group_revision: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of an environment bound to the environment group in the following format: `organizations/{org}/environments/{env}`."]
        pub environment: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "receiver")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the proxy revision that is receiving this basepath in the following format: `organizations/{org}/apis/{api}/revisions/{rev}`. This field is omitted from the IngressConfig unless the GetDeployedIngressConfig API is called with view=FULL."]
        pub receiver: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unix timestamp when this rule was updated. This is updated whenever env_group_revision is updated. This field is omitted from the IngressConfig unless the GetDeployedIngressConfig API is called with view=FULL."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1RoutingRule {
        pub fn builder() -> GoogleCloudApigeeV1RoutingRuleBuilder {
            GoogleCloudApigeeV1RoutingRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "NEXT ID: 8 RuntimeTraceConfig defines the configurations for distributed trace in an environment."]
    pub struct GoogleCloudApigeeV1RuntimeTraceConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endpoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Endpoint of the exporter."]
        pub endpoint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exporter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Exporter that is used to view the distributed trace captured using OpenCensus. An exporter sends traces to any backend that is capable of consuming them. Recorded spans can be exported by registered exporters."]
        pub exporter: ::std::option::Option<GoogleCloudApigeeV1RuntimeTraceConfigExporterEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the trace config in the following format: `organizations/{org}/environment/{env}/traceConfig`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "overrides")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of trace configuration overrides for spicific API proxies."]
        pub overrides: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1RuntimeTraceConfigOverride>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionCreateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp that the revision was created or updated."]
        pub revision_create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Revision number which can be used by the runtime to detect if the trace config has changed between two versions."]
        pub revision_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "samplingConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Trace configuration for all API proxies in an environment."]
        pub sampling_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1RuntimeTraceSamplingConfig>>,
    }
    impl GoogleCloudApigeeV1RuntimeTraceConfig {
        pub fn builder() -> GoogleCloudApigeeV1RuntimeTraceConfigBuilder {
            GoogleCloudApigeeV1RuntimeTraceConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Exporter that is used to view the distributed trace captured using OpenCensus. An exporter sends traces to any backend that is capable of consuming them. Recorded spans can be exported by registered exporters."]
    pub enum GoogleCloudApigeeV1RuntimeTraceConfigExporterEnum {
        #[serde(rename = "EXPORTER_UNSPECIFIED")]
        #[doc = "Exporter unspecified"]
        ExporterUnspecified,
        #[serde(rename = "JAEGER")]
        #[doc = "Jaeger exporter"]
        Jaeger,
        #[serde(rename = "CLOUD_TRACE")]
        #[doc = "Cloudtrace exporter"]
        CloudTrace,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1RuntimeTraceConfigExporterEnum {
        fn default() -> Self {
            Self::ExporterUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "NEXT ID: 7 Trace configuration override for a specific API proxy in an environment."]
    pub struct GoogleCloudApigeeV1RuntimeTraceConfigOverride {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiProxy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the API proxy that will have its trace configuration overridden following format: `organizations/{org}/apis/{api}`"]
        pub api_proxy: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the trace config override in the following format: `organizations/{org}/environment/{env}/traceConfig/overrides/{override}`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionCreateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp that the revision was created or updated."]
        pub revision_create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Revision number which can be used by the runtime to detect if the trace config override has changed between two versions."]
        pub revision_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "samplingConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Trace configuration override for a specific API proxy in an environment."]
        pub sampling_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1RuntimeTraceSamplingConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique ID for the configuration override. The ID will only change if the override is deleted and recreated. Corresponds to name's \"override\" field."]
        pub uid: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1RuntimeTraceConfigOverride {
        pub fn builder() -> GoogleCloudApigeeV1RuntimeTraceConfigOverrideBuilder {
            GoogleCloudApigeeV1RuntimeTraceConfigOverrideBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "NEXT ID: 3 RuntimeTraceSamplingConfig represents the detail settings of distributed tracing. Only the fields that are defined in the distributed trace configuration can be overridden using the distribute trace configuration override APIs."]
    pub struct GoogleCloudApigeeV1RuntimeTraceSamplingConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampler")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sampler of distributed tracing. OFF is the default value."]
        pub sampler:
            ::std::option::Option<GoogleCloudApigeeV1RuntimeTraceSamplingConfigSamplerEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "samplingRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Field sampling rate. This value is only applicable when using the PROBABILITY sampler. The supported values are > 0 and <= 0.5."]
        pub sampling_rate: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudApigeeV1RuntimeTraceSamplingConfig {
        pub fn builder() -> GoogleCloudApigeeV1RuntimeTraceSamplingConfigBuilder {
            GoogleCloudApigeeV1RuntimeTraceSamplingConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Sampler of distributed tracing. OFF is the default value."]
    pub enum GoogleCloudApigeeV1RuntimeTraceSamplingConfigSamplerEnum {
        #[serde(rename = "SAMPLER_UNSPECIFIED")]
        #[doc = "Sampler unspecified."]
        SamplerUnspecified,
        #[serde(rename = "OFF")]
        #[doc = "OFF means distributed trace is disabled, or the sampling probability is 0."]
        Off,
        #[serde(rename = "PROBABILITY")]
        #[doc = "PROBABILITY means traces are captured on a probability that defined by sampling_rate. The sampling rate is limited to 0 to 0.5 when this is set."]
        Probability,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1RuntimeTraceSamplingConfigSamplerEnum {
        fn default() -> Self {
            Self::SamplerUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for Schema call"]
    pub struct GoogleCloudApigeeV1Schema {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of schema fiels grouped as dimensions."]
        pub dimensions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1SchemaSchemaElement>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meta")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional metadata associated with schema. This is a legacy field and usually consists of an empty array of strings."]
        pub meta: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of schema fields grouped as dimensions. These are fields that can be used with an aggregate function such as sum, avg, min, max."]
        pub metrics: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1SchemaSchemaElement>>,
        >,
    }
    impl GoogleCloudApigeeV1Schema {
        pub fn builder() -> GoogleCloudApigeeV1SchemaBuilder {
            GoogleCloudApigeeV1SchemaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message type for the schema element"]
    pub struct GoogleCloudApigeeV1SchemaSchemaElement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the field"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Property of the schema field E.g. { \"createTime\": \"2016-02-26T10:23:09.592Z\", \"custom\": \"false\", \"type\": \"string\" }"]
        pub properties:
            ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1SchemaSchemaProperty>>,
    }
    impl GoogleCloudApigeeV1SchemaSchemaElement {
        pub fn builder() -> GoogleCloudApigeeV1SchemaSchemaElementBuilder {
            GoogleCloudApigeeV1SchemaSchemaElementBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message type for schema property"]
    pub struct GoogleCloudApigeeV1SchemaSchemaProperty {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creation time of the field"]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "custom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom is a flag signifying if the field was provided as part of the standard dataset or a custom field created by the customer"]
        pub custom: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data type of the field."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1SchemaSchemaProperty {
        pub fn builder() -> GoogleCloudApigeeV1SchemaSchemaPropertyBuilder {
            GoogleCloudApigeeV1SchemaSchemaPropertyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1ServiceIssuersMapping {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of trusted issuer email ids."]
        pub email_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "service")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "String indicating the Apigee service name."]
        pub service: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1ServiceIssuersMapping {
        pub fn builder() -> GoogleCloudApigeeV1ServiceIssuersMappingBuilder {
            GoogleCloudApigeeV1ServiceIssuersMappingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Session carries the debug session id and its creation time."]
    pub struct GoogleCloudApigeeV1Session {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The debug session ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestampMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The first transaction creation timestamp in millisecond, recoreded by UAP."]
        pub timestamp_ms: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1Session {
        pub fn builder() -> GoogleCloudApigeeV1SessionBuilder {
            GoogleCloudApigeeV1SessionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The metadata describing a shared flow"]
    pub struct GoogleCloudApigeeV1SharedFlow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latestRevisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of the most recently created revision for this shared flow."]
        pub latest_revision_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metaData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata describing the shared flow."]
        pub meta_data: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1EntityMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the shared flow."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of revisions of this shared flow."]
        pub revision: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudApigeeV1SharedFlow {
        pub fn builder() -> GoogleCloudApigeeV1SharedFlowBuilder {
            GoogleCloudApigeeV1SharedFlowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The metadata describing a shared flow revision."]
    pub struct GoogleCloudApigeeV1SharedFlowRevision {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "configurationVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the configuration schema to which this shared flow conforms. The only supported value currently is majorVersion 4 and minorVersion 0. This setting may be used in the future to enable evolution of the shared flow format."]
        pub configuration_version:
            ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1ConfigVersion>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contextInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A textual description of the shared flow revision."]
        pub context_info: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time at which this shared flow revision was created, in milliseconds since epoch."]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the shared flow revision."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The human readable name of this shared flow."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityMetaDataAsProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Key-Value map of metadata about this shared flow revision."]
        pub entity_meta_data_as_properties:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time at which this shared flow revision was most recently modified, in milliseconds since epoch."]
        pub last_modified_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource ID of the parent shared flow."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of policy names included in this shared flow revision."]
        pub policies: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceFiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource files included in this shared flow revision."]
        pub resource_files:
            ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1ResourceFiles>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of the resources included in this shared flow revision formatted as \"{type}://{name}\"."]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource ID of this revision."]
        pub revision: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sharedFlows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of the shared flow names included in this shared flow revision."]
        pub shared_flows: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The string \"Application\""]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1SharedFlowRevision {
        pub fn builder() -> GoogleCloudApigeeV1SharedFlowRevisionBuilder {
            GoogleCloudApigeeV1SharedFlowRevisionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This message type encapsulates a stats response."]
    pub struct GoogleCloudApigeeV1Stats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains a list of query results on environment level."]
        pub environments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1StatsEnvironmentStats>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hosts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains a list of query results grouped by host."]
        pub hosts: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1StatsHostStats>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metaData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains the metadata information."]
        pub meta_data: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1Metadata>>,
    }
    impl GoogleCloudApigeeV1Stats {
        pub fn builder() -> GoogleCloudApigeeV1StatsBuilder {
            GoogleCloudApigeeV1StatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This message type encapsulates the environment wrapper: \"environments\": [ { \"metrics\": [ { \"name\": \"sum(message_count)\", \"values\": [ \"2.52056245E8\" ] } ], \"name\": \"prod\" } ]"]
    pub struct GoogleCloudApigeeV1StatsEnvironmentStats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains the list of metrics grouped under dimensions."]
        pub dimensions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1DimensionMetric>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "In the final response, only one of the following fields will be present based on the dimensions provided. If no dimensions are provided, then only a top level metrics is provided. If dimensions are included, then there will be a top level dimensions field under environments which will contain metrics values and the dimension name. Example: \"environments\": [ { \"dimensions\": [ { \"metrics\": [ { \"name\": \"sum(message_count)\", \"values\": [ \"2.14049521E8\" ] } ], \"name\": \"nit_proxy\" } ], \"name\": \"prod\" } ] OR \"environments\": [ { \"metrics\": [ { \"name\": \"sum(message_count)\", \"values\": [ \"2.19026331E8\" ] } ], \"name\": \"prod\" } ] This field contains the list of metric values."]
        pub metrics:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Metric>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1StatsEnvironmentStats {
        pub fn builder() -> GoogleCloudApigeeV1StatsEnvironmentStatsBuilder {
            GoogleCloudApigeeV1StatsEnvironmentStatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This message type encapsulates the hostname wrapper: \"hosts\": [ { \"metrics\": [ { \"name\": \"sum(message_count)\", \"values\": [ \"2.52056245E8\" ] } ], \"name\": \"example.com\" } ]"]
    pub struct GoogleCloudApigeeV1StatsHostStats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains the list of metrics grouped under dimensions."]
        pub dimensions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1DimensionMetric>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "In the final response, only one of the following fields will be present based on the dimensions provided. If no dimensions are provided, then only a top level metrics is provided. If dimensions are included, then there will be a top level dimensions field under hostnames which will contain metrics values and the dimension name. Example: \"hosts\": [ { \"dimensions\": [ { \"metrics\": [ { \"name\": \"sum(message_count)\", \"values\": [ \"2.14049521E8\" ] } ], \"name\": \"nit_proxy\" } ], \"name\": \"example.com\" } ] OR \"hosts\": [ { \"metrics\": [ { \"name\": \"sum(message_count)\", \"values\": [ \"2.19026331E8\" ] } ], \"name\": \"example.com\" } ] This field contains the list of metric values."]
        pub metrics:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Metric>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains the hostname used in query."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1StatsHostStats {
        pub fn builder() -> GoogleCloudApigeeV1StatsHostStatsBuilder {
            GoogleCloudApigeeV1StatsHostStatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Pub/Sub subscription of an environment."]
    pub struct GoogleCloudApigeeV1Subscription {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full name of the Pub/Sub subcription. Use the following structure in your request: `subscription \"projects/foo/subscription/bar\"`"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1Subscription {
        pub fn builder() -> GoogleCloudApigeeV1SubscriptionBuilder {
            GoogleCloudApigeeV1SubscriptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1SyncAuthorization {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity tag (ETag) used for optimistic concurrency control as a way to help prevent simultaneous updates from overwriting each other. For example, when you call [getSyncAuthorization](organizations/getSyncAuthorization) an ETag is returned in the response. Pass that ETag when calling the [setSyncAuthorization](organizations/setSyncAuthorization) to ensure that you are updating the correct version. If you don't pass the ETag in the call to `setSyncAuthorization`, then the existing authorization is overwritten indiscriminately. **Note**: We strongly recommend that you use the ETag in the read-modify-write cycle to avoid race conditions."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "identities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Array of service accounts to grant access to control plane resources, each specified using the following format: `serviceAccount:` service-account-name. The service-account-name is formatted like an email address. For example: `my-synchronizer-manager-service_account@my_project_id.iam.gserviceaccount.com` You might specify multiple service accounts, for example, if you have multiple environments and wish to assign a unique service account to each one. The service accounts must have **Apigee Synchronizer Manager** role. See also [Create service accounts](https://cloud.google.com/apigee/docs/hybrid/latest/sa-about#create-the-service-accounts)."]
        pub identities: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudApigeeV1SyncAuthorization {
        pub fn builder() -> GoogleCloudApigeeV1SyncAuthorizationBuilder {
            GoogleCloudApigeeV1SyncAuthorizationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "TargetServer configuration. TargetServers are used to decouple a proxy's TargetEndpoint HTTPTargetConnections from concrete URLs for backend services."]
    pub struct GoogleCloudApigeeV1TargetServer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A human-readable description of this TargetServer."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "host")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The host name this target connects to. Value must be a valid hostname as described by RFC-1123."]
        pub host: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Enabling/disabling a TargetServer is useful when TargetServers are used in load balancing configurations, and one or more TargetServers need to taken out of rotation periodically. Defaults to true."]
        pub is_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The resource id of this target server. Values must match the regular expression "]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "port")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The port number this target connects to on the given host. Value must be between 1 and 65535, inclusive."]
        pub port: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sSLInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Specifies TLS configuration info for this TargetServer. The JSON name is `sSLInfo` for legacy/backwards compatibility reasons -- Edge originally supported SSL, and the name is still used for TLS configuration."]
        pub s_sl_info: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1TlsInfo>>,
    }
    impl GoogleCloudApigeeV1TargetServer {
        pub fn builder() -> GoogleCloudApigeeV1TargetServerBuilder {
            GoogleCloudApigeeV1TargetServerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1TargetServerConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "host")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Host name of the target server."]
        pub host: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target server revision name in the following format: `organizations/{org}/environments/{env}/targetservers/{targetserver}/revisions/{rev}`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "port")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Port number for the target server."]
        pub port: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tlsInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "TLS settings for the target server."]
        pub tls_info: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1TlsInfoConfig>>,
    }
    impl GoogleCloudApigeeV1TargetServerConfig {
        pub fn builder() -> GoogleCloudApigeeV1TargetServerConfigBuilder {
            GoogleCloudApigeeV1TargetServerConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for TestDatastore"]
    pub struct GoogleCloudApigeeV1TestDatastoreResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Error message of test connection failure"]
        pub error: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. It could be `completed` or `failed`"]
        pub state: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1TestDatastoreResponse {
        pub fn builder() -> GoogleCloudApigeeV1TestDatastoreResponseBuilder {
            GoogleCloudApigeeV1TestDatastoreResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "TLS configuration information for VirtualHosts and TargetServers."]
    pub struct GoogleCloudApigeeV1TlsInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ciphers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SSL/TLS cipher suites to be used. Must be one of the cipher suite names listed in: http://docs.oracle.com/javase/8/docs/technotes/guides/security/StandardNames.html#ciphersuites"]
        pub ciphers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientAuthEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Enables two-way TLS."]
        pub client_auth_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The TLS Common Name of the certificate."]
        pub common_name:
            ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1TlsInfoCommonName>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Enables TLS. If false, neither one-way nor two-way TLS will be enabled."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ignoreValidationErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, Edge ignores TLS certificate errors. Valid when configuring TLS for target servers and target endpoints, and when configuring virtual hosts that use 2-way TLS. When used with a target endpoint/target server, if the backend system uses SNI and returns a cert with a subject Distinguished Name (DN) that does not match the hostname, there is no way to ignore the error and the connection fails."]
        pub ignore_validation_errors: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyAlias")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required if `client_auth_enabled` is true. The resource ID for the alias containing the private key and cert."]
        pub key_alias: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyStore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required if `client_auth_enabled` is true. The resource ID of the keystore. References not yet supported."]
        pub key_store: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protocols")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The TLS versioins to be used."]
        pub protocols: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trustStore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource ID of the truststore. References not yet supported."]
        pub trust_store: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1TlsInfo {
        pub fn builder() -> GoogleCloudApigeeV1TlsInfoBuilder {
            GoogleCloudApigeeV1TlsInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1TlsInfoCommonName {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The TLS Common Name string of the certificate."]
        pub value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "wildcardMatch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the cert should be matched against as a wildcard cert."]
        pub wildcard_match: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudApigeeV1TlsInfoCommonName {
        pub fn builder() -> GoogleCloudApigeeV1TlsInfoCommonNameBuilder {
            GoogleCloudApigeeV1TlsInfoCommonNameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudApigeeV1TlsInfoConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ciphers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of ciphers that are granted access."]
        pub ciphers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientAuthEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag that specifies whether client-side authentication is enabled for the target server. Enables two-way TLS."]
        pub client_auth_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Common name to validate the target server against."]
        pub common_name:
            ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1CommonNameConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag that specifies whether one-way TLS is enabled. Set to `true` to enable one-way TLS."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ignoreValidationErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag that specifies whether to ignore TLS certificate validation errors. Set to `true` to ignore errors."]
        pub ignore_validation_errors: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyAlias")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the alias used for client-side authentication in the following format: `organizations/{org}/environments/{env}/keystores/{keystore}/aliases/{alias}`"]
        pub key_alias: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyAliasReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reference name and alias pair to use for client-side authentication."]
        pub key_alias_reference:
            ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1KeyAliasReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protocols")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of TLS protocols that are granted access."]
        pub protocols: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trustStore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the keystore or keystore reference containing trusted certificates for the server in the following format: `organizations/{org}/environments/{env}/keystores/{keystore}` or `organizations/{org}/environments/{env}/references/{reference}`"]
        pub trust_store: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1TlsInfoConfig {
        pub fn builder() -> GoogleCloudApigeeV1TlsInfoConfigBuilder {
            GoogleCloudApigeeV1TlsInfoConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details on why a resource update failed in the runtime."]
    pub struct GoogleCloudApigeeV1UpdateError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status code."]
        pub code: ::std::option::Option<GoogleCloudApigeeV1UpdateErrorCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-friendly error message."]
        pub message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sub resource specific to this error (e.g. a proxy deployed within the EnvironmentConfig). If empty the error refers to the top level resource."]
        pub resource: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A string that uniquely identifies the type of error. This provides a more reliable means to deduplicate errors across revisions and instances."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudApigeeV1UpdateError {
        pub fn builder() -> GoogleCloudApigeeV1UpdateErrorBuilder {
            GoogleCloudApigeeV1UpdateErrorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Status code."]
    pub enum GoogleCloudApigeeV1UpdateErrorCodeEnum {
        #[serde(rename = "OK")]
        #[doc = "Not an error; returned on success HTTP Mapping: 200 OK"]
        Ok,
        #[serde(rename = "CANCELLED")]
        #[doc = "The operation was cancelled, typically by the caller. HTTP Mapping: 499 Client Closed Request"]
        Cancelled,
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown error. For example, this error may be returned when a `Status` value received from another address space belongs to an error space that is not known in this address space. Also errors raised by APIs that do not return enough error information may be converted to this error. HTTP Mapping: 500 Internal Server Error"]
        Unknown,
        #[serde(rename = "INVALID_ARGUMENT")]
        #[doc = "The client specified an invalid argument. Note that this differs from `FAILED_PRECONDITION`. `INVALID_ARGUMENT` indicates arguments that are problematic regardless of the state of the system (e.g., a malformed file name). HTTP Mapping: 400 Bad Request"]
        InvalidArgument,
        #[serde(rename = "DEADLINE_EXCEEDED")]
        #[doc = "The deadline expired before the operation could complete. For operations that change the state of the system, this error may be returned even if the operation has completed successfully. For example, a successful response from a server could have been delayed long enough for the deadline to expire. HTTP Mapping: 504 Gateway Timeout"]
        DeadlineExceeded,
        #[serde(rename = "NOT_FOUND")]
        #[doc = "Some requested entity (e.g., file or directory) was not found. Note to server developers: if a request is denied for an entire class of users, such as gradual feature rollout or undocumented allowlist, `NOT_FOUND` may be used. If a request is denied for some users within a class of users, such as user-based access control, `PERMISSION_DENIED` must be used. HTTP Mapping: 404 Not Found"]
        NotFound,
        #[serde(rename = "ALREADY_EXISTS")]
        #[doc = "The entity that a client attempted to create (e.g., file or directory) already exists. HTTP Mapping: 409 Conflict"]
        AlreadyExists,
        #[serde(rename = "PERMISSION_DENIED")]
        #[doc = "The caller does not have permission to execute the specified operation. `PERMISSION_DENIED` must not be used for rejections caused by exhausting some resource (use `RESOURCE_EXHAUSTED` instead for those errors). `PERMISSION_DENIED` must not be used if the caller can not be identified (use `UNAUTHENTICATED` instead for those errors). This error code does not imply the request is valid or the requested entity exists or satisfies other pre-conditions. HTTP Mapping: 403 Forbidden"]
        PermissionDenied,
        #[serde(rename = "UNAUTHENTICATED")]
        #[doc = "The request does not have valid authentication credentials for the operation. HTTP Mapping: 401 Unauthorized"]
        Unauthenticated,
        #[serde(rename = "RESOURCE_EXHAUSTED")]
        #[doc = "Some resource has been exhausted, perhaps a per-user quota, or perhaps the entire file system is out of space. HTTP Mapping: 429 Too Many Requests"]
        ResourceExhausted,
        #[serde(rename = "FAILED_PRECONDITION")]
        #[doc = "The operation was rejected because the system is not in a state required for the operation's execution. For example, the directory to be deleted is non-empty, an rmdir operation is applied to a non-directory, etc. Service implementors can use the following guidelines to decide between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`: (a) Use `UNAVAILABLE` if the client can retry just the failing call. (b) Use `ABORTED` if the client should retry at a higher level (e.g., when a client-specified test-and-set fails, indicating the client should restart a read-modify-write sequence). (c) Use `FAILED_PRECONDITION` if the client should not retry until the system state has been explicitly fixed. E.g., if an \"rmdir\" fails because the directory is non-empty, `FAILED_PRECONDITION` should be returned since the client should not retry unless the files are deleted from the directory. HTTP Mapping: 400 Bad Request"]
        FailedPrecondition,
        #[serde(rename = "ABORTED")]
        #[doc = "The operation was aborted, typically due to a concurrency issue such as a sequencer check failure or transaction abort. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 409 Conflict"]
        Aborted,
        #[serde(rename = "OUT_OF_RANGE")]
        #[doc = "The operation was attempted past the valid range. E.g., seeking or reading past end-of-file. Unlike `INVALID_ARGUMENT`, this error indicates a problem that may be fixed if the system state changes. For example, a 32-bit file system will generate `INVALID_ARGUMENT` if asked to read at an offset that is not in the range [0,2^32-1], but it will generate `OUT_OF_RANGE` if asked to read from an offset past the current file size. There is a fair bit of overlap between `FAILED_PRECONDITION` and `OUT_OF_RANGE`. We recommend using `OUT_OF_RANGE` (the more specific error) when it applies so that callers who are iterating through a space can easily look for an `OUT_OF_RANGE` error to detect when they are done. HTTP Mapping: 400 Bad Request"]
        OutOfRange,
        #[serde(rename = "UNIMPLEMENTED")]
        #[doc = "The operation is not implemented or is not supported/enabled in this service. HTTP Mapping: 501 Not Implemented"]
        Unimplemented,
        #[serde(rename = "INTERNAL")]
        #[doc = "Internal errors. This means that some invariants expected by the underlying system have been broken. This error code is reserved for serious errors. HTTP Mapping: 500 Internal Server Error"]
        Internal,
        #[serde(rename = "UNAVAILABLE")]
        #[doc = "The service is currently unavailable. This is most likely a transient condition, which can be corrected by retrying with a backoff. Note that it is not always safe to retry non-idempotent operations. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 503 Service Unavailable"]
        Unavailable,
        #[serde(rename = "DATA_LOSS")]
        #[doc = "Unrecoverable data loss or corruption. HTTP Mapping: 500 Internal Server Error"]
        DataLoss,
    }
    impl ::std::default::Default for GoogleCloudApigeeV1UpdateErrorCodeEnum {
        fn default() -> Self {
            Self::Ok
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
    #[doc = "Request message for `SetIamPolicy` method."]
    pub struct GoogleIamV1SetIamPolicyRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them."]
        pub policy: ::std::option::Option<::std::boxed::Box<GoogleIamV1Policy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: \"bindings, etag\"`"]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl GoogleIamV1SetIamPolicyRequest {
        pub fn builder() -> GoogleIamV1SetIamPolicyRequestBuilder {
            GoogleIamV1SetIamPolicyRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for `TestIamPermissions` method."]
    pub struct GoogleIamV1TestIamPermissionsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of permissions to check for the `resource`. Permissions with wildcards (such as '*' or 'storage.*') are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions)."]
        pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleIamV1TestIamPermissionsRequest {
        pub fn builder() -> GoogleIamV1TestIamPermissionsRequestBuilder {
            GoogleIamV1TestIamPermissionsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for `TestIamPermissions` method."]
    pub struct GoogleIamV1TestIamPermissionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is allowed."]
        pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleIamV1TestIamPermissionsResponse {
        pub fn builder() -> GoogleIamV1TestIamPermissionsResponseBuilder {
            GoogleIamV1TestIamPermissionsResponseBuilder::default()
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
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct GoogleProtobufEmpty {}
    impl GoogleProtobufEmpty {
        pub fn builder() -> GoogleProtobufEmptyBuilder {
            GoogleProtobufEmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes what preconditions have failed. For example, if an RPC failed because it required the Terms of Service to be acknowledged, it could list the terms of service violation in the PreconditionFailure message."]
    pub struct GoogleRpcPreconditionFailure {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "violations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes all precondition violations."]
        pub violations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleRpcPreconditionFailureViolation>>,
        >,
    }
    impl GoogleRpcPreconditionFailure {
        pub fn builder() -> GoogleRpcPreconditionFailureBuilder {
            GoogleRpcPreconditionFailureBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A message type used to describe a single precondition failure."]
    pub struct GoogleRpcPreconditionFailureViolation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of how the precondition failed. Developers can use this description to understand how to fix the failure. For example: \"Terms of service not accepted\"."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The subject, relative to the type, that failed. For example, \"google.com/cloud\" relative to the \"TOS\" type would indicate which terms of service is being referenced."]
        pub subject: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of PreconditionFailure. We recommend using a service-specific enum type to define the supported precondition violation subjects. For example, \"TOS\" for \"Terms of Service violation\"."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleRpcPreconditionFailureViolation {
        pub fn builder() -> GoogleRpcPreconditionFailureViolationBuilder {
            GoogleRpcPreconditionFailureViolationBuilder::default()
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
