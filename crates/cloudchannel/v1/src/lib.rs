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
    pub mod accounts {
        pub mod methods {
            pub mod list_subscribers {
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
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. The maximum number of service accounts to return. The service may return fewer than this value. If unspecified, at most 100 service accounts will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. A page token, received from a previous `ListSubscribers` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListSubscribers` must match the call that provided the page token."]
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
            pub mod channel_partner_links {
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
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The level of granularity the ChannelPartnerLink will display."]
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
                        #[doc = "Optional. The level of granularity the ChannelPartnerLink will display."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "UNSPECIFIED")]
                            #[doc = "The default / unset value. The API will default to the BASIC view."]
                            Unspecified,
                            #[serde(rename = "BASIC")]
                            #[doc = "Includes all fields except the ChannelPartnerLink.channel_partner_cloud_identity_info."]
                            Basic,
                            #[serde(rename = "FULL")]
                            #[doc = "Includes all fields."]
                            Full,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::Unspecified
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
                            #[doc = "Optional. Requested page size. Server might return fewer results than requested. If unspecified, server will pick a default size (25). The maximum value is 200, values above 200 will be coerced to 200."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. A token identifying a page of results, if other than the first one. Typically obtained via ListChannelPartnerLinksResponse.next_page_token of the previous CloudChannelService.ListChannelPartnerLinks call."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The level of granularity the ChannelPartnerLink will display."]
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
                        #[doc = "Optional. The level of granularity the ChannelPartnerLink will display."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "UNSPECIFIED")]
                            #[doc = "The default / unset value. The API will default to the BASIC view."]
                            Unspecified,
                            #[serde(rename = "BASIC")]
                            #[doc = "Includes all fields except the ChannelPartnerLink.channel_partner_cloud_identity_info."]
                            Basic,
                            #[serde(rename = "FULL")]
                            #[doc = "Includes all fields."]
                            Full,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::Unspecified
                            }
                        }
                    }
                }
            }
            pub mod customers {
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
                            #[doc = "Optional. The maximum number of customers to return. The service may return fewer than this value. If unspecified, at most 10 customers will be returned. The maximum value is 50; values about 50 will be coerced to 50."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. A token identifying a page of results, if other than the first one. Typically obtained via ListCustomersResponse.next_page_token of the previous CloudChannelService.ListCustomers call."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod list_purchasable_offers {
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
                            #[serde(rename = "changeOfferPurchase.entitlement")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. Resource name of the entitlement. Format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}"]
                            pub change_offer_purchase_entitlement:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "changeOfferPurchase.newSku")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Resource name of the SKU that is being changed to. Should be provided if upgrading or downgrading an entitlement. Format: products/{product_id}/skus/{sku_id}"]
                            pub change_offer_purchase_new_sku:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "createEntitlementPurchase.sku")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. SKU that the result should be restricted to. Format: products/{product_id}/skus/{sku_id}."]
                            pub create_entitlement_purchase_sku:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "languageCode")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The BCP-47 language code, such as \"en-US\". If specified, the response will be localized to the corresponding language code. Default is \"en-US\"."]
                            pub language_code: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Requested page size. Server might return fewer results than requested. If unspecified, at most 100 Offers will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. A token identifying a page of results, if other than the first one."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod list_purchasable_skus {
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
                            #[serde(rename = "changeOfferPurchase.changeType")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. Change Type for the entitlement."]
                            pub change_offer_purchase_change_type: ::std::option::Option<
                                QueryParametersChangeOfferPurchaseChangeTypeEnum,
                            >,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "changeOfferPurchase.entitlement")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. Resource name of the entitlement. Format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}"]
                            pub change_offer_purchase_entitlement:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "createEntitlementPurchase.product")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. List SKUs belonging to this Product. Format: products/{product_id}. Supports products/- to retrieve SKUs for all products."]
                            pub create_entitlement_purchase_product:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "languageCode")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The BCP-47 language code, such as \"en-US\". If specified, the response will be localized to the corresponding language code. Default is \"en-US\"."]
                            pub language_code: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Requested page size. Server might return fewer results than requested. If unspecified, at most 100 SKUs will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. A token identifying a page of results, if other than the first one."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Required. Change Type for the entitlement."]
                        pub enum QueryParametersChangeOfferPurchaseChangeTypeEnum {
                            #[serde(rename = "CHANGE_TYPE_UNSPECIFIED")]
                            #[doc = "Not used."]
                            ChangeTypeUnspecified,
                            #[serde(rename = "UPGRADE")]
                            #[doc = "SKU is an upgrade on the current entitlement."]
                            Upgrade,
                            #[serde(rename = "DOWNGRADE")]
                            #[doc = "SKU is a downgrade on the current entitlement."]
                            Downgrade,
                        }
                        impl ::std::default::Default for QueryParametersChangeOfferPurchaseChangeTypeEnum {
                            fn default() -> Self {
                                Self::ChangeTypeUnspecified
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
                            #[doc = "The update mask that applies to the resource. Optional."]
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
                    pub mod entitlements {
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
                                    #[doc = "Optional. Requested page size. Server might return fewer results than requested. If unspecified, at most 50 entitlements will be returned. The maximum value is 100; values above 100 will be coerced to 100."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. A token identifying a page of results, if other than the first one. Typically obtained via ListEntitlementsResponse.next_page_token of the previous CloudChannelService.ListEntitlements call."]
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
            pub mod offers {
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
                            #[doc = "Optional. The expression to filter results by name (name of the Offer), sku.name (name of the SKU) or sku.product.name (name of the Product). Example 1: sku.product.name=products/p1 AND sku.name!=products/p1/skus/s1 Example 2: name=accounts/a1/offers/o1"]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "languageCode")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The BCP-47 language code, such as \"en-US\". If specified, the response will be localized to the corresponding language code. Default is \"en-US\"."]
                            pub language_code: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Requested page size. Server might return fewer results than requested. If unspecified, at most 500 Offers will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. A token identifying a page of results, if other than the first one."]
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
    pub mod products {
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
                    #[serde(rename = "account")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. The resource name of the reseller account. Format: accounts/{account_id}."]
                    pub account: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "languageCode")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. The BCP-47 language code, such as \"en-US\". If specified, the response will be localized to the corresponding language code. Default is \"en-US\"."]
                    pub language_code: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. Requested page size. Server might return fewer results than requested. If unspecified, at most 100 Products will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. A token identifying a page of results, if other than the first one."]
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
            pub mod skus {
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
                            #[serde(rename = "account")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. Resource name of the reseller. Format: accounts/{account_id}."]
                            pub account: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "languageCode")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The BCP-47 language code, such as \"en-US\". If specified, the response will be localized to the corresponding language code. Default is \"en-US\"."]
                            pub language_code: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Requested page size. Server might return fewer results than requested. If unspecified, at most 100 SKUs will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. A token identifying a page of results, if other than the first one. Optional."]
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
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CloudChannelService.ActivateEntitlement."]
    pub struct GoogleCloudChannelV1ActivateEntitlementRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if the original operation with the same request ID was received, and if so, will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`)."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1ActivateEntitlementRequest {
        pub fn builder() -> GoogleCloudChannelV1ActivateEntitlementRequestBuilder {
            GoogleCloudChannelV1ActivateEntitlementRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information needed to create an Admin User for Google Workspace."]
    pub struct GoogleCloudChannelV1AdminUser {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Primary email of the admin user."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "familyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Family name of the admin user."]
        pub family_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "givenName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Given name of the admin user."]
        pub given_name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1AdminUser {
        pub fn builder() -> GoogleCloudChannelV1AdminUserBuilder {
            GoogleCloudChannelV1AdminUserBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Association links that an entitlement has to other entitlements."]
    pub struct GoogleCloudChannelV1AssociationInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "baseEntitlement")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the base entitlement, for which this entitlement is an add-on."]
        pub base_entitlement: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1AssociationInfo {
        pub fn builder() -> GoogleCloudChannelV1AssociationInfoBuilder {
            GoogleCloudChannelV1AssociationInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CloudChannelService.CancelEntitlement."]
    pub struct GoogleCloudChannelV1CancelEntitlementRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if the original operation with the same request ID was received, and if so, will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`)."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1CancelEntitlementRequest {
        pub fn builder() -> GoogleCloudChannelV1CancelEntitlementRequestBuilder {
            GoogleCloudChannelV1CancelEntitlementRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CloudChannelService.ChangeOffer."]
    pub struct GoogleCloudChannelV1ChangeOfferRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. New Offer. Format: accounts/{account_id}/offers/{offer_id}."]
        pub offer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Parameters needed to purchase the Offer."]
        pub parameters: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1Parameter>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "purchaseOrderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Purchase order id provided by the reseller."]
        pub purchase_order_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if the original operation with the same request ID was received, and if so, will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`)."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1ChangeOfferRequest {
        pub fn builder() -> GoogleCloudChannelV1ChangeOfferRequestBuilder {
            GoogleCloudChannelV1ChangeOfferRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CloudChannelService.ChangeParametersRequest."]
    pub struct GoogleCloudChannelV1ChangeParametersRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Entitlement parameters to update. Only editable parameters are allowed to be changed."]
        pub parameters: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1Parameter>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "purchaseOrderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Purchase order ID provided by the reseller."]
        pub purchase_order_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if the original operation with the same request ID was received, and if so, will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`)."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1ChangeParametersRequest {
        pub fn builder() -> GoogleCloudChannelV1ChangeParametersRequestBuilder {
            GoogleCloudChannelV1ChangeParametersRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CloudChannelService.ChangeRenewalSettings."]
    pub struct GoogleCloudChannelV1ChangeRenewalSettingsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "renewalSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. New renewal settings."]
        pub renewal_settings:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1RenewalSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if the original operation with the same request ID was received, and if so, will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`)."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1ChangeRenewalSettingsRequest {
        pub fn builder() -> GoogleCloudChannelV1ChangeRenewalSettingsRequestBuilder {
            GoogleCloudChannelV1ChangeRenewalSettingsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Entity representing a link between distributors and their indirect resellers in an n-tier resale channel."]
    pub struct GoogleCloudChannelV1ChannelPartnerLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelPartnerCloudIdentityInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Cloud Identity info of the channel partner (IR)."]
        pub channel_partner_cloud_identity_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1CloudIdentityInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp of when the channel partner link is created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inviteLinkUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. URI of the web page where partner accepts the link invitation."]
        pub invite_link_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. State of the channel partner link."]
        pub link_state: ::std::option::Option<GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource name for the channel partner link, in the format accounts/{account_id}/channelPartnerLinks/{id}."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Public identifier that a customer must use to generate a transfer token to move to this distributor-reseller combination."]
        pub public_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resellerCloudIdentityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Cloud Identity ID of the linked reseller."]
        pub reseller_cloud_identity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp of when the channel partner link is updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1ChannelPartnerLink {
        pub fn builder() -> GoogleCloudChannelV1ChannelPartnerLinkBuilder {
            GoogleCloudChannelV1ChannelPartnerLinkBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. State of the channel partner link."]
    pub enum GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum {
        #[serde(rename = "CHANNEL_PARTNER_LINK_STATE_UNSPECIFIED")]
        #[doc = "The state is not specified."]
        ChannelPartnerLinkStateUnspecified,
        #[serde(rename = "INVITED")]
        #[doc = "An invitation has been sent to the reseller to create a channel partner link."]
        Invited,
        #[serde(rename = "ACTIVE")]
        #[doc = "Status when the reseller is active."]
        Active,
        #[serde(rename = "REVOKED")]
        #[doc = "Status when the reseller has been revoked by the distributor."]
        Revoked,
        #[serde(rename = "SUSPENDED")]
        #[doc = "Status when the reseller is suspended by Google or distributor."]
        Suspended,
    }
    impl ::std::default::Default for GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum {
        fn default() -> Self {
            Self::ChannelPartnerLinkStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CloudChannelService.CheckCloudIdentityAccountsExist."]
    pub struct GoogleCloudChannelV1CheckCloudIdentityAccountsExistRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Domain for which the Cloud Identity account customer is fetched."]
        pub domain: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1CheckCloudIdentityAccountsExistRequest {
        pub fn builder() -> GoogleCloudChannelV1CheckCloudIdentityAccountsExistRequestBuilder {
            GoogleCloudChannelV1CheckCloudIdentityAccountsExistRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for CloudChannelService.CheckCloudIdentityAccountsExist."]
    pub struct GoogleCloudChannelV1CheckCloudIdentityAccountsExistResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudIdentityAccounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud Identity accounts associated with the domain."]
        pub cloud_identity_accounts: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1CloudIdentityCustomerAccount>>,
        >,
    }
    impl GoogleCloudChannelV1CheckCloudIdentityAccountsExistResponse {
        pub fn builder() -> GoogleCloudChannelV1CheckCloudIdentityAccountsExistResponseBuilder {
            GoogleCloudChannelV1CheckCloudIdentityAccountsExistResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Entity representing a Cloud Identity account which may or may not be associated with a Channel Services API partner."]
    pub struct GoogleCloudChannelV1CloudIdentityCustomerAccount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerCloudIdentityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cloud Identity ID of the customer. This field is populated ONLY if existing = true."]
        pub customer_cloud_identity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the customer that owns the Cloud Identity account. This field is populated ONLY if owned = true. The customer_name takes the format: accounts/{account_id}/customers/{customer_id}"]
        pub customer_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "existing")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if a Cloud Identity account exists for a specific domain."]
        pub existing: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "owned")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the Cloud Identity account is associated with a customer belonging to the Channel Services partner making the API call."]
        pub owned: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudChannelV1CloudIdentityCustomerAccount {
        pub fn builder() -> GoogleCloudChannelV1CloudIdentityCustomerAccountBuilder {
            GoogleCloudChannelV1CloudIdentityCustomerAccountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Cloud Identity information for the Cloud Channel Customer."]
    pub struct GoogleCloudChannelV1CloudIdentityInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adminConsoleUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. URI of Customer's Admin console dashboard."]
        pub admin_console_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternateEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The alternate email."]
        pub alternate_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CustomerType indicates verification type needed for using services."]
        pub customer_type:
            ::std::option::Option<GoogleCloudChannelV1CloudIdentityInfoCustomerTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eduData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Edu information about the customer."]
        pub edu_data: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1EduData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isDomainVerified")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the domain is verified."]
        pub is_domain_verified: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Language code."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Phone number associated with the Cloud Identity."]
        pub phone_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryDomain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The primary domain name."]
        pub primary_domain: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1CloudIdentityInfo {
        pub fn builder() -> GoogleCloudChannelV1CloudIdentityInfoBuilder {
            GoogleCloudChannelV1CloudIdentityInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "CustomerType indicates verification type needed for using services."]
    pub enum GoogleCloudChannelV1CloudIdentityInfoCustomerTypeEnum {
        #[serde(rename = "CUSTOMER_TYPE_UNSPECIFIED")]
        #[doc = "Default value. This state doesn't show unless an error occurs."]
        CustomerTypeUnspecified,
        #[serde(rename = "DOMAIN")]
        #[doc = "Domain-owning customer which needs domain verification to use services."]
        Domain,
        #[serde(rename = "TEAM")]
        #[doc = "Team customer which needs email verification to use services."]
        Team,
    }
    impl ::std::default::Default for GoogleCloudChannelV1CloudIdentityInfoCustomerTypeEnum {
        fn default() -> Self {
            Self::CustomerTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Commitment settings for commitment-based offers."]
    pub struct GoogleCloudChannelV1CommitmentSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Commitment end timestamp."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "renewalSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Renewal settings applicable for a commitment-based Offer."]
        pub renewal_settings:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1RenewalSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Commitment start timestamp."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1CommitmentSettings {
        pub fn builder() -> GoogleCloudChannelV1CommitmentSettingsBuilder {
            GoogleCloudChannelV1CommitmentSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the constraints for buying the Offer."]
    pub struct GoogleCloudChannelV1Constraints {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerConstraints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents constraints required to purchase the Offer for a customer."]
        pub customer_constraints:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1CustomerConstraints>>,
    }
    impl GoogleCloudChannelV1Constraints {
        pub fn builder() -> GoogleCloudChannelV1ConstraintsBuilder {
            GoogleCloudChannelV1ConstraintsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contact information for a customer account."]
    pub struct GoogleCloudChannelV1ContactInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Display name of the contact in the customer account. Populated by combining customer first name and last name."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email of the contact in the customer account. Email is required for entitlements that need creation of admin.google.com accounts. The email will be the username used in credentials to access the admin.google.com account."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "First name of the contact in the customer account."]
        pub first_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last name of the contact in the customer account."]
        pub last_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Phone number of the contact in the customer account."]
        pub phone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Job title of the contact in the customer account."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1ContactInfo {
        pub fn builder() -> GoogleCloudChannelV1ContactInfoBuilder {
            GoogleCloudChannelV1ContactInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CloudChannelService.CreateEntitlement"]
    pub struct GoogleCloudChannelV1CreateEntitlementRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entitlement")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The entitlement to create."]
        pub entitlement: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Entitlement>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if the original operation with the same request ID was received, and if so, will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`)."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1CreateEntitlementRequest {
        pub fn builder() -> GoogleCloudChannelV1CreateEntitlementRequestBuilder {
            GoogleCloudChannelV1CreateEntitlementRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Entity representing a customer of a reseller or distributor."]
    pub struct GoogleCloudChannelV1Customer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternateEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Secondary contact email. Alternate email and primary contact email are required to have different domains if primary contact email is present. When creating admin.google.com accounts, users get notified credentials at this email. This email address is also used as a recovery email."]
        pub alternate_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelPartnerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cloud Identity ID of the customer's channel partner. Populated only if a channel partner exists for this customer."]
        pub channel_partner_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudIdentityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Customer's cloud_identity_id. Populated only if a Cloud Identity resource exists for this customer."]
        pub cloud_identity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudIdentityInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Cloud Identity information for the customer. Populated only if a Cloud Identity account exists for this customer."]
        pub cloud_identity_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1CloudIdentityInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which the customer is created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Primary domain used by the customer. Domain of primary contact email is required to be same as the provided domain."]
        pub domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see https://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource name of the customer. Format: accounts/{account_id}/customers/{customer_id}"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orgDisplayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Name of the organization that the customer entity represents."]
        pub org_display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orgPostalAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Address of the organization of the customer entity. Region and zip codes are required to enforce US laws and embargoes. Valid address lines are required for all customers. Language code is discarded. Use the Customer-level language code to set the customer's language."]
        pub org_postal_address: ::std::option::Option<::std::boxed::Box<GoogleTypePostalAddress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryContactInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Primary contact info."]
        pub primary_contact_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1ContactInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which the customer is updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1Customer {
        pub fn builder() -> GoogleCloudChannelV1CustomerBuilder {
            GoogleCloudChannelV1CustomerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents constraints required to purchase the Offer for a customer."]
    pub struct GoogleCloudChannelV1CustomerConstraints {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedCustomerTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allowed Customer Type."]
        pub allowed_customer_types: ::std::option::Option<
            ::std::vec::Vec<GoogleCloudChannelV1CustomerConstraintsAllowedCustomerTypesEnum>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedRegions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allowed geographical regions of the customer."]
        pub allowed_regions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "promotionalOrderTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allowed Promotional Order Type. Present for Promotional offers."]
        pub promotional_order_types: ::std::option::Option<
            ::std::vec::Vec<GoogleCloudChannelV1CustomerConstraintsPromotionalOrderTypesEnum>,
        >,
    }
    impl GoogleCloudChannelV1CustomerConstraints {
        pub fn builder() -> GoogleCloudChannelV1CustomerConstraintsBuilder {
            GoogleCloudChannelV1CustomerConstraintsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleCloudChannelV1CustomerConstraintsAllowedCustomerTypesEnum {
        #[serde(rename = "CUSTOMER_TYPE_UNSPECIFIED")]
        #[doc = "Default value. This state doesn't show unless an error occurs."]
        CustomerTypeUnspecified,
        #[serde(rename = "DOMAIN")]
        #[doc = "Domain-owning customer which needs domain verification to use services."]
        Domain,
        #[serde(rename = "TEAM")]
        #[doc = "Team customer which needs email verification to use services."]
        Team,
    }
    impl ::std::default::Default for GoogleCloudChannelV1CustomerConstraintsAllowedCustomerTypesEnum {
        fn default() -> Self {
            Self::CustomerTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleCloudChannelV1CustomerConstraintsPromotionalOrderTypesEnum {
        #[serde(rename = "PROMOTIONAL_TYPE_UNSPECIFIED")]
        #[doc = "Not used."]
        PromotionalTypeUnspecified,
        #[serde(rename = "NEW_UPGRADE")]
        #[doc = "Order used for new customers, trial conversions and upgrades."]
        NewUpgrade,
        #[serde(rename = "TRANSFER")]
        #[doc = "All orders for transferring an existing customer."]
        Transfer,
        #[serde(rename = "PROMOTION_SWITCH")]
        #[doc = "Orders for modifying an existing customer's promotion on the same SKU."]
        PromotionSwitch,
    }
    impl ::std::default::Default for GoogleCloudChannelV1CustomerConstraintsPromotionalOrderTypesEnum {
        fn default() -> Self {
            Self::PromotionalTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents Pub/Sub message content describing customer update."]
    pub struct GoogleCloudChannelV1CustomerEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of the customer. Format: accounts/{account_id}/customers/{customer_id}"]
        pub customer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of event which happened on the customer."]
        pub event_type: ::std::option::Option<GoogleCloudChannelV1CustomerEventEventTypeEnum>,
    }
    impl GoogleCloudChannelV1CustomerEvent {
        pub fn builder() -> GoogleCloudChannelV1CustomerEventBuilder {
            GoogleCloudChannelV1CustomerEventBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of event which happened on the customer."]
    pub enum GoogleCloudChannelV1CustomerEventEventTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Default value. This state doesn't show unless an error occurs."]
        TypeUnspecified,
    }
    impl ::std::default::Default for GoogleCloudChannelV1CustomerEventEventTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Required Edu Attributes"]
    pub struct GoogleCloudChannelV1EduData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instituteSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Size of the institute."]
        pub institute_size: ::std::option::Option<GoogleCloudChannelV1EduDataInstituteSizeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instituteType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Designated institute type of customer."]
        pub institute_type: ::std::option::Option<GoogleCloudChannelV1EduDataInstituteTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "website")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web address for the edu customer's institution."]
        pub website: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1EduData {
        pub fn builder() -> GoogleCloudChannelV1EduDataBuilder {
            GoogleCloudChannelV1EduDataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Size of the institute."]
    pub enum GoogleCloudChannelV1EduDataInstituteSizeEnum {
        #[serde(rename = "INSTITUTE_SIZE_UNSPECIFIED")]
        #[doc = "Default value. This state doesn't show unless an error occurs."]
        InstituteSizeUnspecified,
        #[serde(rename = "SIZE_1_100")]
        #[doc = "1 - 100"]
        Size1100,
        #[serde(rename = "SIZE_101_500")]
        #[doc = "101 - 500"]
        Size101500,
        #[serde(rename = "SIZE_501_1000")]
        #[doc = "501 - 1,000"]
        Size5011000,
        #[serde(rename = "SIZE_1001_2000")]
        #[doc = "1,001 - 2,000"]
        Size10012000,
        #[serde(rename = "SIZE_2001_5000")]
        #[doc = "2,001 - 5,000"]
        Size20015000,
        #[serde(rename = "SIZE_5001_10000")]
        #[doc = "5,001 - 10,000"]
        Size500110000,
        #[serde(rename = "SIZE_10001_OR_MORE")]
        #[doc = "10,001 +"]
        Size10001OrMore,
    }
    impl ::std::default::Default for GoogleCloudChannelV1EduDataInstituteSizeEnum {
        fn default() -> Self {
            Self::InstituteSizeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Designated institute type of customer."]
    pub enum GoogleCloudChannelV1EduDataInstituteTypeEnum {
        #[serde(rename = "INSTITUTE_TYPE_UNSPECIFIED")]
        #[doc = "Default value. This state doesn't show unless an error occurs."]
        InstituteTypeUnspecified,
        #[serde(rename = "K12")]
        #[doc = "Elementary/Secondary Schools & Districts"]
        K12,
        #[serde(rename = "UNIVERSITY")]
        #[doc = "Higher Education Universities & Colleges"]
        University,
    }
    impl ::std::default::Default for GoogleCloudChannelV1EduDataInstituteTypeEnum {
        fn default() -> Self {
            Self::InstituteTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An entitlement is a representation of a customer's ability to use a service."]
    pub struct GoogleCloudChannelV1Entitlement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "associationInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Association information to other entitlements."]
        pub association_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1AssociationInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commitmentSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Commitment settings for a commitment-based Offer. Required for commitment based offers."]
        pub commitment_settings:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1CommitmentSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which the entitlement is created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource name of an entitlement in the form: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The offer resource name for which the entitlement is to be created. Takes the form: accounts/{account_id}/offers/{offer_id}."]
        pub offer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Extended entitlement parameters. When creating an entitlement, valid parameters' names and values are defined in the offer's parameter definitions."]
        pub parameters: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1Parameter>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provisionedService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Service provisioning details for the entitlement."]
        pub provisioned_service:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1ProvisionedService>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provisioningState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Current provisioning state of the entitlement."]
        pub provisioning_state:
            ::std::option::Option<GoogleCloudChannelV1EntitlementProvisioningStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "purchaseOrderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. This purchase order (PO) information is for resellers to use for their company tracking usage. If a purchaseOrderId value is given, it appears in the API responses and shows up in the invoice. The property accepts up to 80 plain text characters."]
        pub purchase_order_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suspensionReasons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Enumerable of all current suspension reasons for an entitlement."]
        pub suspension_reasons: ::std::option::Option<
            ::std::vec::Vec<GoogleCloudChannelV1EntitlementSuspensionReasonsEnum>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trialSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Settings for trial offers."]
        pub trial_settings:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1TrialSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which the entitlement is updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1Entitlement {
        pub fn builder() -> GoogleCloudChannelV1EntitlementBuilder {
            GoogleCloudChannelV1EntitlementBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Current provisioning state of the entitlement."]
    pub enum GoogleCloudChannelV1EntitlementProvisioningStateEnum {
        #[serde(rename = "PROVISIONING_STATE_UNSPECIFIED")]
        #[doc = "Default value. This state doesn't show unless an error occurs."]
        ProvisioningStateUnspecified,
        #[serde(rename = "ACTIVE")]
        #[doc = "The entitlement is currently active."]
        Active,
        #[serde(rename = "SUSPENDED")]
        #[doc = "The entitlement is currently suspended."]
        Suspended,
    }
    impl ::std::default::Default for GoogleCloudChannelV1EntitlementProvisioningStateEnum {
        fn default() -> Self {
            Self::ProvisioningStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleCloudChannelV1EntitlementSuspensionReasonsEnum {
        #[serde(rename = "SUSPENSION_REASON_UNSPECIFIED")]
        #[doc = "Default value. This state doesn't show unless an error occurs."]
        SuspensionReasonUnspecified,
        #[serde(rename = "RESELLER_INITIATED")]
        #[doc = "Entitlement was manually suspended by the Reseller."]
        ResellerInitiated,
        #[serde(rename = "TRIAL_ENDED")]
        #[doc = "Trial ended."]
        TrialEnded,
        #[serde(rename = "RENEWAL_WITH_TYPE_CANCEL")]
        #[doc = "Entitlement renewal was canceled."]
        RenewalWithTypeCancel,
        #[serde(rename = "PENDING_TOS_ACCEPTANCE")]
        #[doc = "Entitlement was automatically suspended on creation for pending ToS acceptance on customer."]
        PendingTosAcceptance,
        #[serde(rename = "OTHER")]
        #[doc = "Other reasons (internal reasons, abuse, etc.)."]
        Other,
    }
    impl ::std::default::Default for GoogleCloudChannelV1EntitlementSuspensionReasonsEnum {
        fn default() -> Self {
            Self::SuspensionReasonUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents Pub/Sub message content describing entitlement update."]
    pub struct GoogleCloudChannelV1EntitlementEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entitlement")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of an entitlement of the form: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}"]
        pub entitlement: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of event which happened on the entitlement."]
        pub event_type: ::std::option::Option<GoogleCloudChannelV1EntitlementEventEventTypeEnum>,
    }
    impl GoogleCloudChannelV1EntitlementEvent {
        pub fn builder() -> GoogleCloudChannelV1EntitlementEventBuilder {
            GoogleCloudChannelV1EntitlementEventBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of event which happened on the entitlement."]
    pub enum GoogleCloudChannelV1EntitlementEventEventTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Default value. This state doesn't show unless an error occurs."]
        TypeUnspecified,
        #[serde(rename = "CREATED")]
        #[doc = "A new entitlement was created."]
        Created,
        #[serde(rename = "PRICE_PLAN_SWITCHED")]
        #[doc = "The offer type associated with an entitlement was changed. This is not triggered if an entitlement converts from a commit offer to a flexible offer as part of a renewal."]
        PricePlanSwitched,
        #[serde(rename = "COMMITMENT_CHANGED")]
        #[doc = "Annual commitment for a commit plan was changed."]
        CommitmentChanged,
        #[serde(rename = "RENEWED")]
        #[doc = "An annual entitlement was renewed."]
        Renewed,
        #[serde(rename = "SUSPENDED")]
        #[doc = "Entitlement was suspended."]
        Suspended,
        #[serde(rename = "ACTIVATED")]
        #[doc = "Entitlement was unsuspended."]
        Activated,
        #[serde(rename = "CANCELLED")]
        #[doc = "Entitlement was cancelled."]
        Cancelled,
        #[serde(rename = "SKU_CHANGED")]
        #[doc = "Entitlement was upgraded or downgraded (e.g. from Google Workspace Business Standard to Google Workspace Business Plus)."]
        SkuChanged,
        #[serde(rename = "RENEWAL_SETTING_CHANGED")]
        #[doc = "The renewal settings of an entitlement has changed."]
        RenewalSettingChanged,
        #[serde(rename = "PAID_SERVICE_STARTED")]
        #[doc = "Paid service has started on trial entitlement."]
        PaidServiceStarted,
        #[serde(rename = "LICENSE_ASSIGNMENT_CHANGED")]
        #[doc = "License was assigned to or revoked from a user."]
        LicenseAssignmentChanged,
    }
    impl ::std::default::Default for GoogleCloudChannelV1EntitlementEventEventTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for CloudChannelService.ListChannelPartnerLinks."]
    pub struct GoogleCloudChannelV1ListChannelPartnerLinksResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelPartnerLinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Channel partner links for a reseller."]
        pub channel_partner_links: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1ChannelPartnerLink>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass to ListChannelPartnerLinksRequest.page_token to obtain that page."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1ListChannelPartnerLinksResponse {
        pub fn builder() -> GoogleCloudChannelV1ListChannelPartnerLinksResponseBuilder {
            GoogleCloudChannelV1ListChannelPartnerLinksResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for CloudChannelService.ListCustomers."]
    pub struct GoogleCloudChannelV1ListCustomersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The customers belonging to the reseller or distributor."]
        pub customers:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1Customer>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass to ListCustomersRequest.page_token to obtain that page."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1ListCustomersResponse {
        pub fn builder() -> GoogleCloudChannelV1ListCustomersResponseBuilder {
            GoogleCloudChannelV1ListCustomersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for CloudChannelService.ListEntitlements."]
    pub struct GoogleCloudChannelV1ListEntitlementsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entitlements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entitlements belonging to the reseller's customer."]
        pub entitlements: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1Entitlement>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to List next page of results. Pass to ListEntitlementsRequest.page_token to obtain that page."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1ListEntitlementsResponse {
        pub fn builder() -> GoogleCloudChannelV1ListEntitlementsResponseBuilder {
            GoogleCloudChannelV1ListEntitlementsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListOffers."]
    pub struct GoogleCloudChannelV1ListOffersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Offers requested."]
        pub offers:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1Offer>>>,
    }
    impl GoogleCloudChannelV1ListOffersResponse {
        pub fn builder() -> GoogleCloudChannelV1ListOffersResponseBuilder {
            GoogleCloudChannelV1ListOffersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListProducts."]
    pub struct GoogleCloudChannelV1ListProductsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "products")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of Products requested."]
        pub products:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1Product>>>,
    }
    impl GoogleCloudChannelV1ListProductsResponse {
        pub fn builder() -> GoogleCloudChannelV1ListProductsResponseBuilder {
            GoogleCloudChannelV1ListProductsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListPurchasableOffers."]
    pub struct GoogleCloudChannelV1ListPurchasableOffersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "purchasableOffers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Offers requested."]
        pub purchasable_offers: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1PurchasableOffer>>,
        >,
    }
    impl GoogleCloudChannelV1ListPurchasableOffersResponse {
        pub fn builder() -> GoogleCloudChannelV1ListPurchasableOffersResponseBuilder {
            GoogleCloudChannelV1ListPurchasableOffersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListPurchasableSkus."]
    pub struct GoogleCloudChannelV1ListPurchasableSkusResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "purchasableSkus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of SKUs requested."]
        pub purchasable_skus: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1PurchasableSku>>,
        >,
    }
    impl GoogleCloudChannelV1ListPurchasableSkusResponse {
        pub fn builder() -> GoogleCloudChannelV1ListPurchasableSkusResponseBuilder {
            GoogleCloudChannelV1ListPurchasableSkusResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListSkus."]
    pub struct GoogleCloudChannelV1ListSkusResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of SKUs requested."]
        pub skus:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1Sku>>>,
    }
    impl GoogleCloudChannelV1ListSkusResponse {
        pub fn builder() -> GoogleCloudChannelV1ListSkusResponseBuilder {
            GoogleCloudChannelV1ListSkusResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response Message for ListSubscribers."]
    pub struct GoogleCloudChannelV1ListSubscribersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceAccounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of service accounts which have subscriber access to the topic."]
        pub service_accounts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the topic registered with the reseller."]
        pub topic: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1ListSubscribersResponse {
        pub fn builder() -> GoogleCloudChannelV1ListSubscribersResponseBuilder {
            GoogleCloudChannelV1ListSubscribersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CloudChannelService.ListTransferableOffers"]
    pub struct GoogleCloudChannelV1ListTransferableOffersRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudIdentityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Customer's Cloud Identity ID"]
        pub cloud_identity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reseller should create a customer and use the resource name of the created customer here."]
        pub customer_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code, such as \"en-US\". If specified, the response will be localized to the corresponding language code. Default is \"en-US\"."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Requested page size. Server might return fewer results than requested. If unspecified, at most 100 Offers will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000."]
        pub page_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token identifying a page of results, if other than the first one. Typically obtained via ListTransferableOffersResponse.next_page_token of the previous CloudChannelService.ListTransferableOffers call."]
        pub page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sku")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. SKU for which the Offers are being looked up."]
        pub sku: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1ListTransferableOffersRequest {
        pub fn builder() -> GoogleCloudChannelV1ListTransferableOffersRequestBuilder {
            GoogleCloudChannelV1ListTransferableOffersRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for CloudChannelService.ListTransferableOffers."]
    pub struct GoogleCloudChannelV1ListTransferableOffersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass to ListTransferableOffersRequest.page_token to obtain that page."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transferableOffers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about Offers for a customer that can be used for transfer."]
        pub transferable_offers: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1TransferableOffer>>,
        >,
    }
    impl GoogleCloudChannelV1ListTransferableOffersResponse {
        pub fn builder() -> GoogleCloudChannelV1ListTransferableOffersResponseBuilder {
            GoogleCloudChannelV1ListTransferableOffersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CloudChannelService.ListTransferableSkus"]
    pub struct GoogleCloudChannelV1ListTransferableSkusRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This token is generated by the Super Admin of the resold customer to authorize a reseller to access their Cloud Identity and purchase entitlements on their behalf. This token can be omitted once the authorization is generated. See https://support.google.com/a/answer/7643790 for more details."]
        pub auth_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudIdentityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Customer's Cloud Identity ID"]
        pub cloud_identity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reseller is required to create a customer and use the resource name of the created customer here. The customer_name takes the format: accounts/{account_id}/customers/{customer_id}"]
        pub customer_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code, such as \"en-US\". If specified, the response will be localized to the corresponding language code. Default is \"en-US\". Optional."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Requested page size. Server might return fewer results than requested. If unspecified, at most 100 SKUs will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000. Optional."]
        pub page_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token identifying a page of results, if other than the first one. Typically obtained via ListTransferableSkusResponse.next_page_token of the previous CloudChannelService.ListTransferableSkus call. Optional."]
        pub page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1ListTransferableSkusRequest {
        pub fn builder() -> GoogleCloudChannelV1ListTransferableSkusRequestBuilder {
            GoogleCloudChannelV1ListTransferableSkusRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for CloudChannelService.ListTransferableSkus."]
    pub struct GoogleCloudChannelV1ListTransferableSkusResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass to ListTransferableSkusRequest.page_token to obtain that page."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transferableSkus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about existing SKUs for a customer that would need to be transferred."]
        pub transferable_skus: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1TransferableSku>>,
        >,
    }
    impl GoogleCloudChannelV1ListTransferableSkusResponse {
        pub fn builder() -> GoogleCloudChannelV1ListTransferableSkusResponseBuilder {
            GoogleCloudChannelV1ListTransferableSkusResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the marketing information for a Product, SKU or Offer."]
    pub struct GoogleCloudChannelV1MarketingInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultLogo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Default logo."]
        pub default_logo: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Media>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human readable description. Description can contain HTML."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human readable name."]
        pub display_name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1MarketingInfo {
        pub fn builder() -> GoogleCloudChannelV1MarketingInfoBuilder {
            GoogleCloudChannelV1MarketingInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents media information."]
    pub struct GoogleCloudChannelV1Media {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of the media."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of the media."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the media."]
        pub _type: ::std::option::Option<GoogleCloudChannelV1MediaTypeEnum>,
    }
    impl GoogleCloudChannelV1Media {
        pub fn builder() -> GoogleCloudChannelV1MediaBuilder {
            GoogleCloudChannelV1MediaBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of the media."]
    pub enum GoogleCloudChannelV1MediaTypeEnum {
        #[serde(rename = "MEDIA_TYPE_UNSPECIFIED")]
        #[doc = "Not used."]
        MediaTypeUnspecified,
        #[serde(rename = "MEDIA_TYPE_IMAGE")]
        #[doc = "Type of image."]
        MediaTypeImage,
    }
    impl ::std::default::Default for GoogleCloudChannelV1MediaTypeEnum {
        fn default() -> Self {
            Self::MediaTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an offer made to resellers for purchase. An offer is associated with a Sku, has a plan for payment, a price, and defines the constraints for buying."]
    pub struct GoogleCloudChannelV1Offer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "constraints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Constraints on transacting the Offer."]
        pub constraints: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Constraints>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. End of the Offer validity time."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "marketingInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Marketing information for the Offer."]
        pub marketing_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1MarketingInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource Name of the Offer. Format: accounts/{account_id}/offers/{offer_id}"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameterDefinitions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters required to use current Offer to purchase."]
        pub parameter_definitions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1ParameterDefinition>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "plan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the payment plan for the Offer."]
        pub plan: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Plan>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "priceByResources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Price for each monetizable resource type."]
        pub price_by_resources: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1PriceByResource>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sku")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "SKU the offer is associated with."]
        pub sku: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Sku>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start of the Offer validity time."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1Offer {
        pub fn builder() -> GoogleCloudChannelV1OfferBuilder {
            GoogleCloudChannelV1OfferBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides contextual information about a google.longrunning.Operation."]
    pub struct GoogleCloudChannelV1OperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The RPC that initiated this Long Running Operation."]
        pub operation_type:
            ::std::option::Option<GoogleCloudChannelV1OperationMetadataOperationTypeEnum>,
    }
    impl GoogleCloudChannelV1OperationMetadata {
        pub fn builder() -> GoogleCloudChannelV1OperationMetadataBuilder {
            GoogleCloudChannelV1OperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The RPC that initiated this Long Running Operation."]
    pub enum GoogleCloudChannelV1OperationMetadataOperationTypeEnum {
        #[serde(rename = "OPERATION_TYPE_UNSPECIFIED")]
        #[doc = "Default value. This state doesn't show unless an error occurs."]
        OperationTypeUnspecified,
        #[serde(rename = "CREATE_ENTITLEMENT")]
        #[doc = "Long Running Operation was triggered by CreateEntitlement."]
        CreateEntitlement,
        #[serde(rename = "CHANGE_RENEWAL_SETTINGS")]
        #[doc = "Long Running Operation was triggered by ChangeRenewalSettings."]
        ChangeRenewalSettings,
        #[serde(rename = "START_PAID_SERVICE")]
        #[doc = "Long Running Operation was triggered by StartPaidService."]
        StartPaidService,
        #[serde(rename = "ACTIVATE_ENTITLEMENT")]
        #[doc = "Long Running Operation was triggered by ActivateEntitlement."]
        ActivateEntitlement,
        #[serde(rename = "SUSPEND_ENTITLEMENT")]
        #[doc = "Long Running Operation was triggered by SuspendEntitlement."]
        SuspendEntitlement,
        #[serde(rename = "CANCEL_ENTITLEMENT")]
        #[doc = "Long Running Operation was triggered by CancelEntitlement."]
        CancelEntitlement,
        #[serde(rename = "TRANSFER_ENTITLEMENTS")]
        #[doc = "Long Running Operation was triggered by TransferEntitlements."]
        TransferEntitlements,
        #[serde(rename = "TRANSFER_ENTITLEMENTS_TO_GOOGLE")]
        #[doc = "Long Running Operation was triggered by TransferEntitlementsToGoogle."]
        TransferEntitlementsToGoogle,
        #[serde(rename = "CHANGE_OFFER")]
        #[doc = "Long Running Operation was triggered by ChangeOffer."]
        ChangeOffer,
        #[serde(rename = "CHANGE_PARAMETERS")]
        #[doc = "Long Running Operation was triggered by ChangeParameters."]
        ChangeParameters,
        #[serde(rename = "PROVISION_CLOUD_IDENTITY")]
        #[doc = "Long Running Operation was triggered by ProvisionCloudIdentity."]
        ProvisionCloudIdentity,
    }
    impl ::std::default::Default for GoogleCloudChannelV1OperationMetadataOperationTypeEnum {
        fn default() -> Self {
            Self::OperationTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Definition for extended entitlement parameters."]
    pub struct GoogleCloudChannelV1Parameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "editable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Specifies whether this parameter is allowed to be changed. For example, for a Google Workspace Business Starter entitlement in commitment plan, num_units is editable when entitlement is active."]
        pub editable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the parameter."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of the parameter."]
        pub value: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Value>>,
    }
    impl GoogleCloudChannelV1Parameter {
        pub fn builder() -> GoogleCloudChannelV1ParameterBuilder {
            GoogleCloudChannelV1ParameterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameter's definition. Specifies what parameter is required to use the current Offer to purchase."]
    pub struct GoogleCloudChannelV1ParameterDefinition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If not empty, parameter values must be drawn from this list. For example, [us-west1, us-west2, ...] Applicable to STRING parameter type."]
        pub allowed_values:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1Value>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum value of the parameter, if applicable. Inclusive. For example, maximum seats when purchasing Google Workspace Business Standard. Applicable to INT64 and DOUBLE parameter types."]
        pub max_value: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimal value of the parameter, if applicable. Inclusive. For example, minimal commitment when purchasing Anthos is 0.01. Applicable to INT64 and DOUBLE parameter types."]
        pub min_value: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the parameter."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "optional")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set to true, parameter is optional to purchase this Offer."]
        pub optional: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameterType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data type of the parameter. Minimal value, Maximum value and allowed values will use specified data type here."]
        pub parameter_type:
            ::std::option::Option<GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum>,
    }
    impl GoogleCloudChannelV1ParameterDefinition {
        pub fn builder() -> GoogleCloudChannelV1ParameterDefinitionBuilder {
            GoogleCloudChannelV1ParameterDefinitionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Data type of the parameter. Minimal value, Maximum value and allowed values will use specified data type here."]
    pub enum GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum {
        #[serde(rename = "PARAMETER_TYPE_UNSPECIFIED")]
        #[doc = "Not used."]
        ParameterTypeUnspecified,
        #[serde(rename = "INT64")]
        #[doc = "Int64 type."]
        Int64,
        #[serde(rename = "STRING")]
        #[doc = "String type."]
        String,
        #[serde(rename = "DOUBLE")]
        #[doc = "Double type."]
        Double,
    }
    impl ::std::default::Default for GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum {
        fn default() -> Self {
            Self::ParameterTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents period in days/months/years."]
    pub struct GoogleCloudChannelV1Period {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total duration of Period Type defined."]
        pub duration: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "periodType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Period Type."]
        pub period_type: ::std::option::Option<GoogleCloudChannelV1PeriodPeriodTypeEnum>,
    }
    impl GoogleCloudChannelV1Period {
        pub fn builder() -> GoogleCloudChannelV1PeriodBuilder {
            GoogleCloudChannelV1PeriodBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Period Type."]
    pub enum GoogleCloudChannelV1PeriodPeriodTypeEnum {
        #[serde(rename = "PERIOD_TYPE_UNSPECIFIED")]
        #[doc = "Not used."]
        PeriodTypeUnspecified,
        #[serde(rename = "DAY")]
        #[doc = "Day."]
        Day,
        #[serde(rename = "MONTH")]
        #[doc = "Month."]
        Month,
        #[serde(rename = "YEAR")]
        #[doc = "Year."]
        Year,
    }
    impl ::std::default::Default for GoogleCloudChannelV1PeriodPeriodTypeEnum {
        fn default() -> Self {
            Self::PeriodTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The payment plan for the Offer. Describes how to make a payment."]
    pub struct GoogleCloudChannelV1Plan {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "billingAccount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reseller Billing account that will be charged when this offer is transacted. Only present for GCP offers."]
        pub billing_account: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paymentCycle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes how frequently the reseller will be billed, such as once per month."]
        pub payment_cycle: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Period>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paymentPlan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes how a reseller will be billed."]
        pub payment_plan: ::std::option::Option<GoogleCloudChannelV1PlanPaymentPlanEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paymentType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies when the payment needs to happen."]
        pub payment_type: ::std::option::Option<GoogleCloudChannelV1PlanPaymentTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trialPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Present for Offers with a trial period. For trial-only Offers, a paid service needs to start before the trial period ends for continued service. For Regular Offers with a trial period, the regular pricing goes into effect when trial period ends, or if paid service is started before the end of the trial period."]
        pub trial_period: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Period>>,
    }
    impl GoogleCloudChannelV1Plan {
        pub fn builder() -> GoogleCloudChannelV1PlanBuilder {
            GoogleCloudChannelV1PlanBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Describes how a reseller will be billed."]
    pub enum GoogleCloudChannelV1PlanPaymentPlanEnum {
        #[serde(rename = "PAYMENT_PLAN_UNSPECIFIED")]
        #[doc = "Not used."]
        PaymentPlanUnspecified,
        #[serde(rename = "COMMITMENT")]
        #[doc = "Commitment."]
        Commitment,
        #[serde(rename = "FLEXIBLE")]
        #[doc = "No commitment."]
        Flexible,
        #[serde(rename = "FREE")]
        #[doc = "Free."]
        Free,
        #[serde(rename = "TRIAL")]
        #[doc = "Trial."]
        Trial,
        #[serde(rename = "OFFLINE")]
        #[doc = "Price and ordering not available through API."]
        Offline,
    }
    impl ::std::default::Default for GoogleCloudChannelV1PlanPaymentPlanEnum {
        fn default() -> Self {
            Self::PaymentPlanUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies when the payment needs to happen."]
    pub enum GoogleCloudChannelV1PlanPaymentTypeEnum {
        #[serde(rename = "PAYMENT_TYPE_UNSPECIFIED")]
        #[doc = "Not used."]
        PaymentTypeUnspecified,
        #[serde(rename = "PREPAY")]
        #[doc = "Prepay. Amount has to be paid before service is rendered."]
        Prepay,
        #[serde(rename = "POSTPAY")]
        #[doc = "Postpay. Reseller is charged at the end of the Payment cycle."]
        Postpay,
    }
    impl ::std::default::Default for GoogleCloudChannelV1PlanPaymentTypeEnum {
        fn default() -> Self {
            Self::PaymentTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the price of the Offer."]
    pub struct GoogleCloudChannelV1Price {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basePrice")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Base price."]
        pub base_price: ::std::option::Option<::std::boxed::Box<GoogleTypeMoney>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "discount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Discount percentage, represented as decimal. For example, a 20% discount will be represent as 0.2."]
        pub discount: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "effectivePrice")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Effective Price after applying the discounts."]
        pub effective_price: ::std::option::Option<::std::boxed::Box<GoogleTypeMoney>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "externalPriceUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to external price list, such as link to Google Voice rate card."]
        pub external_price_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1Price {
        pub fn builder() -> GoogleCloudChannelV1PriceBuilder {
            GoogleCloudChannelV1PriceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents price by resource type."]
    pub struct GoogleCloudChannelV1PriceByResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Price of the Offer. Present if there are no price phases."]
        pub price: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pricePhases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the price by time range."]
        pub price_phases: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1PricePhase>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource Type. Example: SEAT"]
        pub resource_type:
            ::std::option::Option<GoogleCloudChannelV1PriceByResourceResourceTypeEnum>,
    }
    impl GoogleCloudChannelV1PriceByResource {
        pub fn builder() -> GoogleCloudChannelV1PriceByResourceBuilder {
            GoogleCloudChannelV1PriceByResourceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Resource Type. Example: SEAT"]
    pub enum GoogleCloudChannelV1PriceByResourceResourceTypeEnum {
        #[serde(rename = "RESOURCE_TYPE_UNSPECIFIED")]
        #[doc = "Not used."]
        ResourceTypeUnspecified,
        #[serde(rename = "SEAT")]
        #[doc = "Seat."]
        Seat,
        #[serde(rename = "MAU")]
        #[doc = "Monthly active user."]
        Mau,
        #[serde(rename = "GB")]
        #[doc = "GB (used for storage SKUs)."]
        Gb,
        #[serde(rename = "LICENSED_USER")]
        #[doc = "Active licensed users(for Voice SKUs)."]
        LicensedUser,
        #[serde(rename = "MINUTES")]
        #[doc = "Voice usage."]
        Minutes,
        #[serde(rename = "IAAS_USAGE")]
        #[doc = "For IaaS SKUs like Google Cloud Platform, monetization is based on usage accrued on your billing account irrespective of the type of monetizable resource. This enum represents an aggregated resource/container for all usage SKUs on a billing account. Currently, only applicable to Google Cloud Platform."]
        IaasUsage,
        #[serde(rename = "SUBSCRIPTION")]
        #[doc = "For Google Cloud Platform subscriptions like Anthos or SAP."]
        Subscription,
    }
    impl ::std::default::Default for GoogleCloudChannelV1PriceByResourceResourceTypeEnum {
        fn default() -> Self {
            Self::ResourceTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies the price by the duration of months. For example, a 20% discount for the first six months, then a 10% discount starting on the seventh month."]
    pub struct GoogleCloudChannelV1PricePhase {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines first period for the phase."]
        pub first_period: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines first period for the phase."]
        pub last_period: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "periodType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the phase period type."]
        pub period_type: ::std::option::Option<GoogleCloudChannelV1PricePhasePeriodTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Price of the phase. Present if there are no price tiers."]
        pub price: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "priceTiers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Price by the resource tiers."]
        pub price_tiers: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1PriceTier>>,
        >,
    }
    impl GoogleCloudChannelV1PricePhase {
        pub fn builder() -> GoogleCloudChannelV1PricePhaseBuilder {
            GoogleCloudChannelV1PricePhaseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Defines the phase period type."]
    pub enum GoogleCloudChannelV1PricePhasePeriodTypeEnum {
        #[serde(rename = "PERIOD_TYPE_UNSPECIFIED")]
        #[doc = "Not used."]
        PeriodTypeUnspecified,
        #[serde(rename = "DAY")]
        #[doc = "Day."]
        Day,
        #[serde(rename = "MONTH")]
        #[doc = "Month."]
        Month,
        #[serde(rename = "YEAR")]
        #[doc = "Year."]
        Year,
    }
    impl ::std::default::Default for GoogleCloudChannelV1PricePhasePeriodTypeEnum {
        fn default() -> Self {
            Self::PeriodTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines price at resource tier level. For example, an offer with following definition : * Tier 1: Provide 25% discount for all seats between 1 and 25. * Tier 2: Provide 10% discount for all seats between 26 and 100. * Tier 3: Provide flat 15% discount for all seats above 100. Each of these tiers is represented as a PriceTier."]
    pub struct GoogleCloudChannelV1PriceTier {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstResource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "First resource for which the tier price applies."]
        pub first_resource: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastResource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last resource for which the tier price applies."]
        pub last_resource: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Price of the tier."]
        pub price: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Price>>,
    }
    impl GoogleCloudChannelV1PriceTier {
        pub fn builder() -> GoogleCloudChannelV1PriceTierBuilder {
            GoogleCloudChannelV1PriceTierBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Product is the entity a customer uses when placing an order. For example, Google Workspace, Google Voice, etc."]
    pub struct GoogleCloudChannelV1Product {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "marketingInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Marketing information for the product."]
        pub marketing_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1MarketingInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource Name of the Product. Format: products/{product_id}"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1Product {
        pub fn builder() -> GoogleCloudChannelV1ProductBuilder {
            GoogleCloudChannelV1ProductBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CloudChannelService.ProvisionCloudIdentity"]
    pub struct GoogleCloudChannelV1ProvisionCloudIdentityRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudIdentityInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CloudIdentity-specific customer information."]
        pub cloud_identity_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1CloudIdentityInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Admin user information."]
        pub user: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1AdminUser>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validateOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, validate the request and preview the review, but do not actually post it."]
        pub validate_only: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudChannelV1ProvisionCloudIdentityRequest {
        pub fn builder() -> GoogleCloudChannelV1ProvisionCloudIdentityRequestBuilder {
            GoogleCloudChannelV1ProvisionCloudIdentityRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Service provisioned for an entitlement."]
    pub struct GoogleCloudChannelV1ProvisionedService {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The product pertaining to the provisioning resource as specified in the Offer."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provisioningId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Provisioning ID of the entitlement. For Google Workspace, this would be the underlying Subscription ID."]
        pub provisioning_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skuId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The SKU pertaining to the provisioning resource as specified in the Offer."]
        pub sku_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1ProvisionedService {
        pub fn builder() -> GoogleCloudChannelV1ProvisionedServiceBuilder {
            GoogleCloudChannelV1ProvisionedServiceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Offer that can be puchased for a customer. This is used in ListPurchasableOffer API response."]
    pub struct GoogleCloudChannelV1PurchasableOffer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Offer."]
        pub offer: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Offer>>,
    }
    impl GoogleCloudChannelV1PurchasableOffer {
        pub fn builder() -> GoogleCloudChannelV1PurchasableOfferBuilder {
            GoogleCloudChannelV1PurchasableOfferBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SKU that can be used for a puchase. This is used in ListPurchasableSku API response."]
    pub struct GoogleCloudChannelV1PurchasableSku {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sku")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "SKU"]
        pub sku: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Sku>>,
    }
    impl GoogleCloudChannelV1PurchasableSku {
        pub fn builder() -> GoogleCloudChannelV1PurchasableSkuBuilder {
            GoogleCloudChannelV1PurchasableSkuBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request Message for RegisterSubscriber."]
    pub struct GoogleCloudChannelV1RegisterSubscriberRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceAccount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Service account which will provide subscriber access to the registered topic."]
        pub service_account: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1RegisterSubscriberRequest {
        pub fn builder() -> GoogleCloudChannelV1RegisterSubscriberRequestBuilder {
            GoogleCloudChannelV1RegisterSubscriberRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response Message for RegisterSubscriber."]
    pub struct GoogleCloudChannelV1RegisterSubscriberResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the topic to which the subscriber will listen to."]
        pub topic: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1RegisterSubscriberResponse {
        pub fn builder() -> GoogleCloudChannelV1RegisterSubscriberResponseBuilder {
            GoogleCloudChannelV1RegisterSubscriberResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Renewal settings for renewable Offers."]
    pub struct GoogleCloudChannelV1RenewalSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableRenewal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If false, the plan will be completed at the end date."]
        pub enable_renewal: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paymentCycle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes how frequently the reseller will be billed, such as once per month."]
        pub payment_cycle: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Period>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paymentPlan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes how a reseller will be billed."]
        pub payment_plan: ::std::option::Option<GoogleCloudChannelV1RenewalSettingsPaymentPlanEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resizeUnitCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true and enable_renewal = true, the unit (for example seats or licenses) will be set to the number of active units at renewal time."]
        pub resize_unit_count: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudChannelV1RenewalSettings {
        pub fn builder() -> GoogleCloudChannelV1RenewalSettingsBuilder {
            GoogleCloudChannelV1RenewalSettingsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Describes how a reseller will be billed."]
    pub enum GoogleCloudChannelV1RenewalSettingsPaymentPlanEnum {
        #[serde(rename = "PAYMENT_PLAN_UNSPECIFIED")]
        #[doc = "Not used."]
        PaymentPlanUnspecified,
        #[serde(rename = "COMMITMENT")]
        #[doc = "Commitment."]
        Commitment,
        #[serde(rename = "FLEXIBLE")]
        #[doc = "No commitment."]
        Flexible,
        #[serde(rename = "FREE")]
        #[doc = "Free."]
        Free,
        #[serde(rename = "TRIAL")]
        #[doc = "Trial."]
        Trial,
        #[serde(rename = "OFFLINE")]
        #[doc = "Price and ordering not available through API."]
        Offline,
    }
    impl ::std::default::Default for GoogleCloudChannelV1RenewalSettingsPaymentPlanEnum {
        fn default() -> Self {
            Self::PaymentPlanUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a product's purchasable Stock Keeping Unit (SKU). SKUs represent the different variations of the product. For example, Google Workspace Business Standard and Google Workspace Business Plus are Google Workspace product SKUs."]
    pub struct GoogleCloudChannelV1Sku {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "marketingInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Marketing information for the SKU."]
        pub marketing_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1MarketingInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource Name of the SKU. Format: products/{product_id}/skus/{sku_id}"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "product")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Product the SKU is associated with."]
        pub product: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Product>>,
    }
    impl GoogleCloudChannelV1Sku {
        pub fn builder() -> GoogleCloudChannelV1SkuBuilder {
            GoogleCloudChannelV1SkuBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CloudChannelService.StartPaidService."]
    pub struct GoogleCloudChannelV1StartPaidServiceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if the original operation with the same request ID was received, and if so, will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`)."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1StartPaidServiceRequest {
        pub fn builder() -> GoogleCloudChannelV1StartPaidServiceRequestBuilder {
            GoogleCloudChannelV1StartPaidServiceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents information which resellers will get as part of notification from Cloud Pub/Sub."]
    pub struct GoogleCloudChannelV1SubscriberEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerEvent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Customer event send as part of Pub/Sub event to partners."]
        pub customer_event:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1CustomerEvent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entitlementEvent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entitlement event send as part of Pub/Sub event to partners."]
        pub entitlement_event:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1EntitlementEvent>>,
    }
    impl GoogleCloudChannelV1SubscriberEvent {
        pub fn builder() -> GoogleCloudChannelV1SubscriberEventBuilder {
            GoogleCloudChannelV1SubscriberEventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CloudChannelService.SuspendEntitlement."]
    pub struct GoogleCloudChannelV1SuspendEntitlementRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if the original operation with the same request ID was received, and if so, will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`)."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1SuspendEntitlementRequest {
        pub fn builder() -> GoogleCloudChannelV1SuspendEntitlementRequestBuilder {
            GoogleCloudChannelV1SuspendEntitlementRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies transfer eligibility of a SKU."]
    pub struct GoogleCloudChannelV1TransferEligibility {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Localized description if reseller is not eligible to transfer the SKU."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ineligibilityReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specified the reason for ineligibility."]
        pub ineligibility_reason:
            ::std::option::Option<GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isEligible")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether reseller is eligible to transfer the SKU."]
        pub is_eligible: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudChannelV1TransferEligibility {
        pub fn builder() -> GoogleCloudChannelV1TransferEligibilityBuilder {
            GoogleCloudChannelV1TransferEligibilityBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specified the reason for ineligibility."]
    pub enum GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum {
        #[serde(rename = "REASON_UNSPECIFIED")]
        #[doc = "Reason is not available."]
        ReasonUnspecified,
        #[serde(rename = "PENDING_TOS_ACCEPTANCE")]
        #[doc = "Reseller needs to accept TOS before transferring the SKU."]
        PendingTosAcceptance,
        #[serde(rename = "SKU_NOT_ELIGIBLE")]
        #[doc = "Reseller not eligible to sell the SKU."]
        SkuNotEligible,
        #[serde(rename = "SKU_SUSPENDED")]
        #[doc = "SKU subscription is suspended"]
        SkuSuspended,
    }
    impl ::std::default::Default for GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum {
        fn default() -> Self {
            Self::ReasonUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CloudChannelService.TransferEntitlements."]
    pub struct GoogleCloudChannelV1TransferEntitlementsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This token is generated by the Super Admin of the resold customer to authorize a reseller to access their Cloud Identity and purchase entitlements on their behalf. This token can be omitted once the authorization is generated. See https://support.google.com/a/answer/7643790 for more details."]
        pub auth_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entitlements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The new entitlements to be created or transferred."]
        pub entitlements: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1Entitlement>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if the original operation with the same request ID was received, and if so, will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`)."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1TransferEntitlementsRequest {
        pub fn builder() -> GoogleCloudChannelV1TransferEntitlementsRequestBuilder {
            GoogleCloudChannelV1TransferEntitlementsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for CloudChannelService.TransferEntitlements. This will be put into the response field of google.longrunning.Operation."]
    pub struct GoogleCloudChannelV1TransferEntitlementsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entitlements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entitlements that have been transferred."]
        pub entitlements: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1Entitlement>>,
        >,
    }
    impl GoogleCloudChannelV1TransferEntitlementsResponse {
        pub fn builder() -> GoogleCloudChannelV1TransferEntitlementsResponseBuilder {
            GoogleCloudChannelV1TransferEntitlementsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CloudChannelService.TransferEntitlementsToGoogle."]
    pub struct GoogleCloudChannelV1TransferEntitlementsToGoogleRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entitlements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The entitlements to be transferred to Google."]
        pub entitlements: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1Entitlement>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if the original operation with the same request ID was received, and if so, will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`)."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1TransferEntitlementsToGoogleRequest {
        pub fn builder() -> GoogleCloudChannelV1TransferEntitlementsToGoogleRequestBuilder {
            GoogleCloudChannelV1TransferEntitlementsToGoogleRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "TransferableOffer represents an Offer that can be used in Transfer. Read-only."]
    pub struct GoogleCloudChannelV1TransferableOffer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Offer with parameter constraints updated to allow the Transfer."]
        pub offer: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Offer>>,
    }
    impl GoogleCloudChannelV1TransferableOffer {
        pub fn builder() -> GoogleCloudChannelV1TransferableOfferBuilder {
            GoogleCloudChannelV1TransferableOfferBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "TransferableSku represents information a reseller needs to view existing provisioned services for a customer that they do not own. Read-only."]
    pub struct GoogleCloudChannelV1TransferableSku {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sku")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SKU pertaining to the provisioning resource as specified in the Offer."]
        pub sku: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1Sku>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transferEligibility")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the transfer eligibility of a SKU."]
        pub transfer_eligibility:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1TransferEligibility>>,
    }
    impl GoogleCloudChannelV1TransferableSku {
        pub fn builder() -> GoogleCloudChannelV1TransferableSkuBuilder {
            GoogleCloudChannelV1TransferableSkuBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings for trial offers."]
    pub struct GoogleCloudChannelV1TrialSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date when the trial ends. The value is in milliseconds using the UNIX Epoch format. See an example [Epoch converter](https://www.epochconverter.com)."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trial")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines if the entitlement is in a trial or not: * `true` - The entitlement is in trial. * `false` - The entitlement is not in trial."]
        pub trial: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudChannelV1TrialSettings {
        pub fn builder() -> GoogleCloudChannelV1TrialSettingsBuilder {
            GoogleCloudChannelV1TrialSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request Message for UnregisterSubscriber."]
    pub struct GoogleCloudChannelV1UnregisterSubscriberRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceAccount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Service account which will be unregistered from getting subscriber access to the topic."]
        pub service_account: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1UnregisterSubscriberRequest {
        pub fn builder() -> GoogleCloudChannelV1UnregisterSubscriberRequestBuilder {
            GoogleCloudChannelV1UnregisterSubscriberRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response Message for UnregisterSubscriber."]
    pub struct GoogleCloudChannelV1UnregisterSubscriberResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the topic from which the service account subscriber access has been removed."]
        pub topic: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1UnregisterSubscriberResponse {
        pub fn builder() -> GoogleCloudChannelV1UnregisterSubscriberResponseBuilder {
            GoogleCloudChannelV1UnregisterSubscriberResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CloudChannelService.UpdateChannelPartnerLink"]
    pub struct GoogleCloudChannelV1UpdateChannelPartnerLinkRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelPartnerLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The channel partner link to update. Only field channel_partner_link.link_state is allowed to be updated."]
        pub channel_partner_link:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1ChannelPartnerLink>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The update mask that applies to the resource. The only allowable value for update mask is channel_partner_link.link_state."]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1UpdateChannelPartnerLinkRequest {
        pub fn builder() -> GoogleCloudChannelV1UpdateChannelPartnerLinkRequestBuilder {
            GoogleCloudChannelV1UpdateChannelPartnerLinkRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data type and value of a parameter."]
    pub struct GoogleCloudChannelV1Value {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "doubleValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a double value."]
        pub double_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "int64Value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents an int64 value."]
        pub int64_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protoValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents an 'Any' proto value."]
        pub proto_value:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a string value."]
        pub string_value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1Value {
        pub fn builder() -> GoogleCloudChannelV1ValueBuilder {
            GoogleCloudChannelV1ValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Association links that an entitlement has to other entitlements."]
    pub struct GoogleCloudChannelV1alpha1AssociationInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "baseEntitlement")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the base entitlement, for which this entitlement is an add-on."]
        pub base_entitlement: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1alpha1AssociationInfo {
        pub fn builder() -> GoogleCloudChannelV1alpha1AssociationInfoBuilder {
            GoogleCloudChannelV1alpha1AssociationInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Commitment settings for commitment-based offers."]
    pub struct GoogleCloudChannelV1alpha1CommitmentSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Commitment end timestamp."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "renewalSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Renewal settings applicable for a commitment-based Offer."]
        pub renewal_settings:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1alpha1RenewalSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Commitment start timestamp."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1alpha1CommitmentSettings {
        pub fn builder() -> GoogleCloudChannelV1alpha1CommitmentSettingsBuilder {
            GoogleCloudChannelV1alpha1CommitmentSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents Pub/Sub message content describing customer update."]
    pub struct GoogleCloudChannelV1alpha1CustomerEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of the customer. Format: accounts/{account_id}/customers/{customer_id}"]
        pub customer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of event which happened on the customer."]
        pub event_type: ::std::option::Option<GoogleCloudChannelV1alpha1CustomerEventEventTypeEnum>,
    }
    impl GoogleCloudChannelV1alpha1CustomerEvent {
        pub fn builder() -> GoogleCloudChannelV1alpha1CustomerEventBuilder {
            GoogleCloudChannelV1alpha1CustomerEventBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of event which happened on the customer."]
    pub enum GoogleCloudChannelV1alpha1CustomerEventEventTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Default value. This state doesn't show unless an error occurs."]
        TypeUnspecified,
    }
    impl ::std::default::Default for GoogleCloudChannelV1alpha1CustomerEventEventTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An entitlement is a representation of a customer's ability to use a service."]
    pub struct GoogleCloudChannelV1alpha1Entitlement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assignedUnits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current number of users that are assigned a license for the product defined in provisioned_service.skuId. Read-only. Deprecated: Use `parameters` instead."]
        pub assigned_units: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "associationInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Association information to other entitlements."]
        pub association_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1alpha1AssociationInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelPartnerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cloud Identity ID of a channel partner who will be the direct reseller for the customer's order. This field is generally used in 2-tier ordering, where the order is placed by a top-level distributor on behalf of their channel partner or reseller. Required for distributors. Deprecated: `channel_partner_id` has been moved to the Customer."]
        pub channel_partner_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commitmentSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Commitment settings for a commitment-based Offer. Required for commitment based offers."]
        pub commitment_settings:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1alpha1CommitmentSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which the entitlement is created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxUnits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum number of units for a non commitment-based Offer, such as Flexible, Trial or Free entitlements. For commitment-based entitlements, this is a read-only field, which only the internal support team can update. Deprecated: Use `parameters` instead."]
        pub max_units: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource name of an entitlement in the form: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numUnits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of units for a commitment-based Offer. For example, for seat-based Offers, this would be the number of seats; for license-based Offers, this would be the number of licenses. Required for creating commitment-based Offers. Deprecated: Use `parameters` instead."]
        pub num_units: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The offer resource name for which the entitlement is to be created. Takes the form: accounts/{account_id}/offers/{offer_id}."]
        pub offer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Extended entitlement parameters. When creating an entitlement, valid parameters' names and values are defined in the offer's parameter definitions."]
        pub parameters: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1alpha1Parameter>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provisionedService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Service provisioning details for the entitlement."]
        pub provisioned_service:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1alpha1ProvisionedService>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provisioningState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Current provisioning state of the entitlement."]
        pub provisioning_state:
            ::std::option::Option<GoogleCloudChannelV1alpha1EntitlementProvisioningStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "purchaseOrderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. This purchase order (PO) information is for resellers to use for their company tracking usage. If a purchaseOrderId value is given, it appears in the API responses and shows up in the invoice. The property accepts up to 80 plain text characters."]
        pub purchase_order_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suspensionReasons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Enumerable of all current suspension reasons for an entitlement."]
        pub suspension_reasons: ::std::option::Option<
            ::std::vec::Vec<GoogleCloudChannelV1alpha1EntitlementSuspensionReasonsEnum>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trialSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Settings for trial offers."]
        pub trial_settings:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1alpha1TrialSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which the entitlement is updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1alpha1Entitlement {
        pub fn builder() -> GoogleCloudChannelV1alpha1EntitlementBuilder {
            GoogleCloudChannelV1alpha1EntitlementBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Current provisioning state of the entitlement."]
    pub enum GoogleCloudChannelV1alpha1EntitlementProvisioningStateEnum {
        #[serde(rename = "PROVISIONING_STATE_UNSPECIFIED")]
        #[doc = "Default value. This state doesn't show unless an error occurs."]
        ProvisioningStateUnspecified,
        #[serde(rename = "ACTIVE")]
        #[doc = "The entitlement is currently active."]
        Active,
        #[serde(rename = "CANCELED")]
        #[doc = "The entitlement was canceled. After an entitlement is `CANCELED`, its status will not change. Deprecated: Canceled entitlements will no longer be visible."]
        Canceled,
        #[serde(rename = "COMPLETE")]
        #[doc = "The entitlement reached end of term and was not renewed. After an entitlement is `COMPLETE`, its status will not change. Deprecated: This is represented as ProvisioningState=SUSPENDED and suspensionReason in (TRIAL_ENDED, RENEWAL_WITH_TYPE_CANCEL)"]
        Complete,
        #[serde(rename = "PENDING")]
        #[doc = "The entitlement is pending. Deprecated: This is represented as ProvisioningState=SUSPENDED and suspensionReason=PENDING_TOS_ACCEPTANCE"]
        Pending,
        #[serde(rename = "SUSPENDED")]
        #[doc = "The entitlement is currently suspended."]
        Suspended,
    }
    impl ::std::default::Default for GoogleCloudChannelV1alpha1EntitlementProvisioningStateEnum {
        fn default() -> Self {
            Self::ProvisioningStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleCloudChannelV1alpha1EntitlementSuspensionReasonsEnum {
        #[serde(rename = "SUSPENSION_REASON_UNSPECIFIED")]
        #[doc = "Default value. This state doesn't show unless an error occurs."]
        SuspensionReasonUnspecified,
        #[serde(rename = "RESELLER_INITIATED")]
        #[doc = "Entitlement was manually suspended by the Reseller."]
        ResellerInitiated,
        #[serde(rename = "TRIAL_ENDED")]
        #[doc = "Trial ended."]
        TrialEnded,
        #[serde(rename = "RENEWAL_WITH_TYPE_CANCEL")]
        #[doc = "Entitlement renewal was canceled."]
        RenewalWithTypeCancel,
        #[serde(rename = "PENDING_TOS_ACCEPTANCE")]
        #[doc = "Entitlement was automatically suspended on creation for pending ToS acceptance on customer."]
        PendingTosAcceptance,
        #[serde(rename = "OTHER")]
        #[doc = "Other reasons (internal reasons, abuse, etc.)."]
        Other,
    }
    impl ::std::default::Default for GoogleCloudChannelV1alpha1EntitlementSuspensionReasonsEnum {
        fn default() -> Self {
            Self::SuspensionReasonUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents Pub/Sub message content describing entitlement update."]
    pub struct GoogleCloudChannelV1alpha1EntitlementEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entitlement")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of an entitlement of the form: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}"]
        pub entitlement: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of event which happened on the entitlement."]
        pub event_type:
            ::std::option::Option<GoogleCloudChannelV1alpha1EntitlementEventEventTypeEnum>,
    }
    impl GoogleCloudChannelV1alpha1EntitlementEvent {
        pub fn builder() -> GoogleCloudChannelV1alpha1EntitlementEventBuilder {
            GoogleCloudChannelV1alpha1EntitlementEventBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of event which happened on the entitlement."]
    pub enum GoogleCloudChannelV1alpha1EntitlementEventEventTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Default value. This state doesn't show unless an error occurs."]
        TypeUnspecified,
        #[serde(rename = "CREATED")]
        #[doc = "A new entitlement was created."]
        Created,
        #[serde(rename = "PRICE_PLAN_SWITCHED")]
        #[doc = "The offer type associated with an entitlement was changed. This is not triggered if an entitlement converts from a commit offer to a flexible offer as part of a renewal."]
        PricePlanSwitched,
        #[serde(rename = "COMMITMENT_CHANGED")]
        #[doc = "Annual commitment for a commit plan was changed."]
        CommitmentChanged,
        #[serde(rename = "RENEWED")]
        #[doc = "An annual entitlement was renewed."]
        Renewed,
        #[serde(rename = "SUSPENDED")]
        #[doc = "Entitlement was suspended."]
        Suspended,
        #[serde(rename = "ACTIVATED")]
        #[doc = "Entitlement was unsuspended."]
        Activated,
        #[serde(rename = "CANCELLED")]
        #[doc = "Entitlement was cancelled."]
        Cancelled,
        #[serde(rename = "SKU_CHANGED")]
        #[doc = "Entitlement was upgraded or downgraded (e.g. from Google Workspace Business Standard to Google Workspace Business Plus)."]
        SkuChanged,
        #[serde(rename = "RENEWAL_SETTING_CHANGED")]
        #[doc = "The renewal settings of an entitlement has changed."]
        RenewalSettingChanged,
        #[serde(rename = "PAID_SERVICE_STARTED")]
        #[doc = "Paid service has started on trial entitlement."]
        PaidServiceStarted,
        #[serde(rename = "LICENSE_ASSIGNMENT_CHANGED")]
        #[doc = "License was assigned to or revoked from a user."]
        LicenseAssignmentChanged,
    }
    impl ::std::default::Default for GoogleCloudChannelV1alpha1EntitlementEventEventTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides contextual information about a google.longrunning.Operation."]
    pub struct GoogleCloudChannelV1alpha1OperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The RPC that initiated this Long Running Operation."]
        pub operation_type:
            ::std::option::Option<GoogleCloudChannelV1alpha1OperationMetadataOperationTypeEnum>,
    }
    impl GoogleCloudChannelV1alpha1OperationMetadata {
        pub fn builder() -> GoogleCloudChannelV1alpha1OperationMetadataBuilder {
            GoogleCloudChannelV1alpha1OperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The RPC that initiated this Long Running Operation."]
    pub enum GoogleCloudChannelV1alpha1OperationMetadataOperationTypeEnum {
        #[serde(rename = "OPERATION_TYPE_UNSPECIFIED")]
        #[doc = "Default value. This state doesn't show unless an error occurs."]
        OperationTypeUnspecified,
        #[serde(rename = "CREATE_ENTITLEMENT")]
        #[doc = "Long Running Operation was triggered by CreateEntitlement."]
        CreateEntitlement,
        #[serde(rename = "CHANGE_QUANTITY")]
        #[doc = "Long Running Operation was triggered by ChangeQuantity."]
        ChangeQuantity,
        #[serde(rename = "CHANGE_RENEWAL_SETTINGS")]
        #[doc = "Long Running Operation was triggered by ChangeRenewalSettings."]
        ChangeRenewalSettings,
        #[serde(rename = "CHANGE_PLAN")]
        #[doc = "Long Running Operation was triggered by ChangePlan."]
        ChangePlan,
        #[serde(rename = "START_PAID_SERVICE")]
        #[doc = "Long Running Operation was triggered by StartPaidService."]
        StartPaidService,
        #[serde(rename = "CHANGE_SKU")]
        #[doc = "Long Running Operation was triggered by ChangeSku."]
        ChangeSku,
        #[serde(rename = "ACTIVATE_ENTITLEMENT")]
        #[doc = "Long Running Operation was triggered by ActivateEntitlement."]
        ActivateEntitlement,
        #[serde(rename = "SUSPEND_ENTITLEMENT")]
        #[doc = "Long Running Operation was triggered by SuspendEntitlement."]
        SuspendEntitlement,
        #[serde(rename = "CANCEL_ENTITLEMENT")]
        #[doc = "Long Running Operation was triggered by CancelEntitlement."]
        CancelEntitlement,
        #[serde(rename = "TRANSFER_ENTITLEMENTS")]
        #[doc = "Long Running Operation was triggered by TransferEntitlements."]
        TransferEntitlements,
        #[serde(rename = "TRANSFER_ENTITLEMENTS_TO_GOOGLE")]
        #[doc = "Long Running Operation was triggered by TransferEntitlementsToGoogle."]
        TransferEntitlementsToGoogle,
        #[serde(rename = "CHANGE_OFFER")]
        #[doc = "Long Running Operation was triggered by ChangeOffer."]
        ChangeOffer,
        #[serde(rename = "CHANGE_PARAMETERS")]
        #[doc = "Long Running Operation was triggered by ChangeParameters."]
        ChangeParameters,
        #[serde(rename = "PROVISION_CLOUD_IDENTITY")]
        #[doc = "Long Running Operation was triggered by ProvisionCloudIdentity."]
        ProvisionCloudIdentity,
    }
    impl ::std::default::Default for GoogleCloudChannelV1alpha1OperationMetadataOperationTypeEnum {
        fn default() -> Self {
            Self::OperationTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Definition for extended entitlement parameters."]
    pub struct GoogleCloudChannelV1alpha1Parameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "editable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Specifies whether this parameter is allowed to be changed. For example, for a Google Workspace Business Starter entitlement in commitment plan, num_units is editable when entitlement is active."]
        pub editable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the parameter."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of the parameter."]
        pub value: ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1alpha1Value>>,
    }
    impl GoogleCloudChannelV1alpha1Parameter {
        pub fn builder() -> GoogleCloudChannelV1alpha1ParameterBuilder {
            GoogleCloudChannelV1alpha1ParameterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents period in days/months/years."]
    pub struct GoogleCloudChannelV1alpha1Period {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total duration of Period Type defined."]
        pub duration: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "periodType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Period Type."]
        pub period_type: ::std::option::Option<GoogleCloudChannelV1alpha1PeriodPeriodTypeEnum>,
    }
    impl GoogleCloudChannelV1alpha1Period {
        pub fn builder() -> GoogleCloudChannelV1alpha1PeriodBuilder {
            GoogleCloudChannelV1alpha1PeriodBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Period Type."]
    pub enum GoogleCloudChannelV1alpha1PeriodPeriodTypeEnum {
        #[serde(rename = "PERIOD_TYPE_UNSPECIFIED")]
        #[doc = "Not used."]
        PeriodTypeUnspecified,
        #[serde(rename = "DAY")]
        #[doc = "Day."]
        Day,
        #[serde(rename = "MONTH")]
        #[doc = "Month."]
        Month,
        #[serde(rename = "YEAR")]
        #[doc = "Year."]
        Year,
    }
    impl ::std::default::Default for GoogleCloudChannelV1alpha1PeriodPeriodTypeEnum {
        fn default() -> Self {
            Self::PeriodTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Service provisioned for an entitlement."]
    pub struct GoogleCloudChannelV1alpha1ProvisionedService {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The product pertaining to the provisioning resource as specified in the Offer."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provisioningId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Provisioning ID of the entitlement. For Google Workspace, this would be the underlying Subscription ID."]
        pub provisioning_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skuId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The SKU pertaining to the provisioning resource as specified in the Offer."]
        pub sku_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1alpha1ProvisionedService {
        pub fn builder() -> GoogleCloudChannelV1alpha1ProvisionedServiceBuilder {
            GoogleCloudChannelV1alpha1ProvisionedServiceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Renewal settings for renewable Offers."]
    pub struct GoogleCloudChannelV1alpha1RenewalSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableCommitment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, disables commitment-based offer on renewal and switches to flexible or pay as you go. Deprecated: Use `payment_plan` instead."]
        pub disable_commitment: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableRenewal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If false, the plan will be completed at the end date."]
        pub enable_renewal: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paymentCycle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes how frequently the reseller will be billed, such as once per month."]
        pub payment_cycle:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1alpha1Period>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paymentOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set if enable_renewal=true. Deprecated: Use `payment_cycle` instead."]
        pub payment_option:
            ::std::option::Option<GoogleCloudChannelV1alpha1RenewalSettingsPaymentOptionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paymentPlan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes how a reseller will be billed."]
        pub payment_plan:
            ::std::option::Option<GoogleCloudChannelV1alpha1RenewalSettingsPaymentPlanEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resizeUnitCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true and enable_renewal = true, the unit (for example seats or licenses) will be set to the number of active units at renewal time."]
        pub resize_unit_count: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudChannelV1alpha1RenewalSettings {
        pub fn builder() -> GoogleCloudChannelV1alpha1RenewalSettingsBuilder {
            GoogleCloudChannelV1alpha1RenewalSettingsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Set if enable_renewal=true. Deprecated: Use `payment_cycle` instead."]
    pub enum GoogleCloudChannelV1alpha1RenewalSettingsPaymentOptionEnum {
        #[serde(rename = "PAYMENT_OPTION_UNSPECIFIED")]
        #[doc = "Default value. This state doesn't show unless an error occurs."]
        PaymentOptionUnspecified,
        #[serde(rename = "ANNUAL")]
        #[doc = "Paid in yearly installments."]
        Annual,
        #[serde(rename = "MONTHLY")]
        #[doc = "Paid in monthly installments."]
        Monthly,
    }
    impl ::std::default::Default for GoogleCloudChannelV1alpha1RenewalSettingsPaymentOptionEnum {
        fn default() -> Self {
            Self::PaymentOptionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Describes how a reseller will be billed."]
    pub enum GoogleCloudChannelV1alpha1RenewalSettingsPaymentPlanEnum {
        #[serde(rename = "PAYMENT_PLAN_UNSPECIFIED")]
        #[doc = "Not used."]
        PaymentPlanUnspecified,
        #[serde(rename = "COMMITMENT")]
        #[doc = "Commitment."]
        Commitment,
        #[serde(rename = "FLEXIBLE")]
        #[doc = "No commitment."]
        Flexible,
        #[serde(rename = "FREE")]
        #[doc = "Free."]
        Free,
        #[serde(rename = "TRIAL")]
        #[doc = "Trial."]
        Trial,
        #[serde(rename = "OFFLINE")]
        #[doc = "Price and ordering not available through API."]
        Offline,
    }
    impl ::std::default::Default for GoogleCloudChannelV1alpha1RenewalSettingsPaymentPlanEnum {
        fn default() -> Self {
            Self::PaymentPlanUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents information which resellers will get as part of notification from Cloud Pub/Sub."]
    pub struct GoogleCloudChannelV1alpha1SubscriberEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerEvent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Customer event send as part of Pub/Sub event to partners."]
        pub customer_event:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1alpha1CustomerEvent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entitlementEvent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entitlement event send as part of Pub/Sub event to partners."]
        pub entitlement_event:
            ::std::option::Option<::std::boxed::Box<GoogleCloudChannelV1alpha1EntitlementEvent>>,
    }
    impl GoogleCloudChannelV1alpha1SubscriberEvent {
        pub fn builder() -> GoogleCloudChannelV1alpha1SubscriberEventBuilder {
            GoogleCloudChannelV1alpha1SubscriberEventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for CloudChannelService.TransferEntitlements. This will be put into the response field of google.longrunning.Operation."]
    pub struct GoogleCloudChannelV1alpha1TransferEntitlementsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entitlements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entitlements that have been transferred."]
        pub entitlements: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudChannelV1alpha1Entitlement>>,
        >,
    }
    impl GoogleCloudChannelV1alpha1TransferEntitlementsResponse {
        pub fn builder() -> GoogleCloudChannelV1alpha1TransferEntitlementsResponseBuilder {
            GoogleCloudChannelV1alpha1TransferEntitlementsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings for trial offers."]
    pub struct GoogleCloudChannelV1alpha1TrialSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date when the trial ends. The value is in milliseconds using the UNIX Epoch format. See an example [Epoch converter](https://www.epochconverter.com)."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trial")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines if the entitlement is in a trial or not: * `true` - The entitlement is in trial. * `false` - The entitlement is not in trial."]
        pub trial: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudChannelV1alpha1TrialSettings {
        pub fn builder() -> GoogleCloudChannelV1alpha1TrialSettingsBuilder {
            GoogleCloudChannelV1alpha1TrialSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data type and value of a parameter."]
    pub struct GoogleCloudChannelV1alpha1Value {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "doubleValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a double value."]
        pub double_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "int64Value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents an int64 value."]
        pub int64_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protoValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents an 'Any' proto value."]
        pub proto_value:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a string value."]
        pub string_value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudChannelV1alpha1Value {
        pub fn builder() -> GoogleCloudChannelV1alpha1ValueBuilder {
            GoogleCloudChannelV1alpha1ValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for Operations.CancelOperation."]
    pub struct GoogleLongrunningCancelOperationRequest {}
    impl GoogleLongrunningCancelOperationRequest {
        pub fn builder() -> GoogleLongrunningCancelOperationRequestBuilder {
            GoogleLongrunningCancelOperationRequestBuilder::default()
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
    #[doc = "Represents an amount of money with its currency type."]
    pub struct GoogleTypeMoney {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The three-letter currency code defined in ISO 4217."]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nanos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
        pub nanos: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "units")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The whole units of the amount. For example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
        pub units: ::std::option::Option<::std::string::String>,
    }
    impl GoogleTypeMoney {
        pub fn builder() -> GoogleTypeMoneyBuilder {
            GoogleTypeMoneyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a postal address, e.g. for postal delivery or payments addresses. Given a postal address, a postal service can deliver items to a premise, P.O. Box or similar. It is not intended to model geographical locations (roads, towns, mountains). In typical usage an address would be created via user input or from importing existing data, depending on the type of process. Advice on address input / editing: - Use an i18n-ready address widget such as https://github.com/google/libaddressinput) - Users should not be presented with UI elements for input or editing of fields outside countries where that field is used. For more guidance on how to use this schema, please see: https://support.google.com/business/answer/6397478"]
    pub struct GoogleTypePostalAddress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addressLines")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unstructured address lines describing the lower levels of an address. Because values in address_lines do not have type information and may sometimes contain multiple values in a single field (e.g. \"Austin, TX\"), it is important that the line order is clear. The order of address lines should be \"envelope order\" for the country/region of the address. In places where this can vary (e.g. Japan), address_language is used to make it explicit (e.g. \"ja\" for large-to-small ordering and \"ja-Latn\" or \"en\" for small-to-large). This way, the most specific line of an address can be selected based on the language. The minimum permitted structural representation of an address consists of a region_code with all remaining information placed in the address_lines. It would be possible to format such an address very approximately without geocoding, but no semantic reasoning could be made about any of the address components until it was at least partially resolved. Creating an address only containing a region_code and address_lines, and then geocoding is the recommended way to handle completely unstructured addresses (as opposed to guessing which parts of the address should be localities or administrative areas)."]
        pub address_lines: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "administrativeArea")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Highest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state, a province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community (e.g. \"Barcelona\" and not \"Catalonia\"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland this should be left unpopulated."]
        pub administrative_area: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. BCP-47 language code of the contents of this address (if known). This is often the UI language of the input form or is expected to match one of the languages used in the address' country/region, or their transliterated equivalents. This can affect formatting in certain countries, but is not critical to the correctness of the data and will never affect any validation or other non-formatting related operations. If this value is not known, it should be omitted (rather than specifying a possibly incorrect default). Examples: \"zh-Hant\", \"ja\", \"ja-Latn\", \"en\"."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Generally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world where localities are not well defined or do not fit into this structure well, leave locality empty and use address_lines."]
        pub locality: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "organization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The name of the organization at the address."]
        pub organization: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Postal code of the address. Not all countries use or require postal codes to be present, but where they are used, they may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.)."]
        pub postal_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recipients")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The recipient at the address. This field may, under certain circumstances, contain multiline information. For example, it might contain \"care of\" information."]
        pub recipients: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to ensure the value is correct. See http://cldr.unicode.org/ and http://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: \"CH\" for Switzerland."]
        pub region_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The schema revision of the `PostalAddress`. This must be set to 0, which is the latest revision. All new revisions **must** be backward compatible with old revisions."]
        pub revision: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortingCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Additional, country-specific, sorting code. This is not used in most regions. Where it is used, the value is either a string like \"CEDEX\", optionally followed by a number (e.g. \"CEDEX 7\"), or just a number alone, representing the \"sector code\" (Jamaica), \"delivery area indicator\" (Malawi) or \"post office indicator\" (e.g. Côte d'Ivoire)."]
        pub sorting_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sublocality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Sublocality of the address. For example, this can be neighborhoods, boroughs, districts."]
        pub sublocality: ::std::option::Option<::std::string::String>,
    }
    impl GoogleTypePostalAddress {
        pub fn builder() -> GoogleTypePostalAddressBuilder {
            GoogleTypePostalAddressBuilder::default()
        }
    }
}
