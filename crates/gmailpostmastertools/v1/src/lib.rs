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
    pub mod domains {
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
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Requested page size. Server may return fewer domains than requested. If unspecified, server will pick an appropriate default."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The next_page_token value returned from a previous List request, if any. This is the value of ListDomainsResponse.next_page_token returned from the previous call to `ListDomains` method."]
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
            pub mod traffic_stats {
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
                            #[serde(rename = "endDate.day")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
                            pub end_date_day: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "endDate.month")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
                            pub end_date_month: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "endDate.year")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
                            pub end_date_year: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Requested page size. Server may return fewer TrafficStats than requested. If unspecified, server will pick an appropriate default."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The next_page_token value returned from a previous List request, if any. This is the value of ListTrafficStatsResponse.next_page_token returned from the previous call to `ListTrafficStats` method."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startDate.day")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
                            pub start_date_day: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startDate.month")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
                            pub start_date_month: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startDate.year")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
                            pub start_date_year: ::std::option::Option<::std::primitive::i64>,
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
    #[doc = "Metric on a particular delivery error type."]
    pub struct DeliveryError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorClass")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The class of delivery error."]
        pub error_class: ::std::option::Option<DeliveryErrorErrorClassEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorRatio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ratio of messages where the error occurred vs all authenticated traffic."]
        pub error_ratio: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of delivery error."]
        pub error_type: ::std::option::Option<DeliveryErrorErrorTypeEnum>,
    }
    impl DeliveryError {
        pub fn builder() -> DeliveryErrorBuilder {
            DeliveryErrorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The class of delivery error."]
    pub enum DeliveryErrorErrorClassEnum {
        #[serde(rename = "DELIVERY_ERROR_CLASS_UNSPECIFIED")]
        #[doc = "The default value which should never be used explicitly."]
        DeliveryErrorClassUnspecified,
        #[serde(rename = "PERMANENT_ERROR")]
        #[doc = "Delivery of message has been rejected."]
        PermanentError,
        #[serde(rename = "TEMPORARY_ERROR")]
        #[doc = "Temporary failure of message delivery to the recipient."]
        TemporaryError,
    }
    impl ::std::default::Default for DeliveryErrorErrorClassEnum {
        fn default() -> Self {
            Self::DeliveryErrorClassUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of delivery error."]
    pub enum DeliveryErrorErrorTypeEnum {
        #[serde(rename = "DELIVERY_ERROR_TYPE_UNSPECIFIED")]
        #[doc = "The default value which should never be used explicitly."]
        DeliveryErrorTypeUnspecified,
        #[serde(rename = "RATE_LIMIT_EXCEEDED")]
        #[doc = "The Domain or IP is sending traffic at a suspiciously high rate, due to which temporary rate limits have been imposed. The limit will be lifted when Gmail is confident enough of the nature of the traffic."]
        RateLimitExceeded,
        #[serde(rename = "SUSPECTED_SPAM")]
        #[doc = "The traffic is suspected to be spam, by Gmail, for various reasons."]
        SuspectedSpam,
        #[serde(rename = "CONTENT_SPAMMY")]
        #[doc = "The traffic is suspected to be spammy, specific to the content."]
        ContentSpammy,
        #[serde(rename = "BAD_ATTACHMENT")]
        #[doc = "Traffic contains attachments not supported by Gmail."]
        BadAttachment,
        #[serde(rename = "BAD_DMARC_POLICY")]
        #[doc = "The sender domain has set up a DMARC rejection policy."]
        BadDmarcPolicy,
        #[serde(rename = "LOW_IP_REPUTATION")]
        #[doc = "The IP reputation of the sending IP is very low."]
        LowIpReputation,
        #[serde(rename = "LOW_DOMAIN_REPUTATION")]
        #[doc = "The Domain reputation of the sending domain is very low."]
        LowDomainReputation,
        #[serde(rename = "IP_IN_RBL")]
        #[doc = "The IP is listed in one or more public [Real-time Blackhole Lists](http://en.wikipedia.org/wiki/DNSBL). Work with the RBL to get your IP delisted."]
        IpInRbl,
        #[serde(rename = "DOMAIN_IN_RBL")]
        #[doc = "The Domain is listed in one or more public [Real-time Blackhole Lists](http://en.wikipedia.org/wiki/DNSBL). Work with the RBL to get your domain delisted."]
        DomainInRbl,
        #[serde(rename = "BAD_PTR_RECORD")]
        #[doc = "The sending IP is missing a [PTR record](https://support.google.com/domains/answer/3251147#ptr)."]
        BadPtrRecord,
    }
    impl ::std::default::Default for DeliveryErrorErrorTypeEnum {
        fn default() -> Self {
            Self::DeliveryErrorTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A registered domain resource in the Postmaster API."]
    pub struct Domain {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp when the user registered this domain. Assigned by the server."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the Domain. Domain names have the form `domains/{domain_name}`, where domain_name is the fully qualified domain name (i.e., mymail.mydomain.com)."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User’s permission for this domain. Assigned by the server."]
        pub permission: ::std::option::Option<DomainPermissionEnum>,
    }
    impl Domain {
        pub fn builder() -> DomainBuilder {
            DomainBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "User’s permission for this domain. Assigned by the server."]
    pub enum DomainPermissionEnum {
        #[serde(rename = "PERMISSION_UNSPECIFIED")]
        #[doc = "The default value and should never be used explicitly."]
        PermissionUnspecified,
        #[serde(rename = "OWNER")]
        #[doc = "User has read access to the domain and can share access with others."]
        Owner,
        #[serde(rename = "READER")]
        #[doc = "User has read access to the domain."]
        Reader,
        #[serde(rename = "NONE")]
        #[doc = "User doesn't have permission to access information about the domain. User did not verify ownership of domain nor was access granted by other domain owners."]
        None,
    }
    impl ::std::default::Default for DomainPermissionEnum {
        fn default() -> Self {
            Self::PermissionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "[Feedback loop](https://support.google.com/mail/answer/6254652) identifier information."]
    pub struct FeedbackLoop {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feedback loop identifier that uniquely identifies individual campaigns."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spamRatio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ratio of user marked spam messages with the identifier vs the total number of inboxed messages with that identifier."]
        pub spam_ratio: ::std::option::Option<::std::primitive::f64>,
    }
    impl FeedbackLoop {
        pub fn builder() -> FeedbackLoopBuilder {
            FeedbackLoopBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "IP Reputation information for a set of IPs in a specific reputation category."]
    pub struct IpReputation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of unique IPs in this reputation category. This metric only pertains to traffic that passed [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/)."]
        pub ip_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reputation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reputation category this IP reputation represents."]
        pub reputation: ::std::option::Option<IpReputationReputationEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampleIps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sample of IPs in this reputation category."]
        pub sample_ips: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl IpReputation {
        pub fn builder() -> IpReputationBuilder {
            IpReputationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The reputation category this IP reputation represents."]
    pub enum IpReputationReputationEnum {
        #[serde(rename = "REPUTATION_CATEGORY_UNSPECIFIED")]
        #[doc = "The default value which should never be used explicitly. This represents the state where no reputation information is available."]
        ReputationCategoryUnspecified,
        #[serde(rename = "HIGH")]
        #[doc = "Has a good track record of a very low spam rate, and complies with Gmail's sender guidelines. Mail will rarely be marked by the spam filter."]
        High,
        #[serde(rename = "MEDIUM")]
        #[doc = "Known to send good mail, but is prone to sending a low volume of spam intermittently. Most of the email from this entity will have a fair deliverability rate, except when there is a notable increase in spam levels."]
        Medium,
        #[serde(rename = "LOW")]
        #[doc = "Known to send a considerable volume of spam regularly, and mail from this sender will likely be marked as spam."]
        Low,
        #[serde(rename = "BAD")]
        #[doc = "History of sending an enormously high volume of spam. Mail coming from this entity will almost always be rejected at SMTP level or marked as spam."]
        Bad,
    }
    impl ::std::default::Default for IpReputationReputationEnum {
        fn default() -> Self {
            Self::ReputationCategoryUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListDomains."]
    pub struct ListDomainsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domains")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of domains."]
        pub domains: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Domain>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListDomainsResponse {
        pub fn builder() -> ListDomainsResponseBuilder {
            ListDomainsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListTrafficStats."]
    pub struct ListTrafficStatsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trafficStats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of TrafficStats."]
        pub traffic_stats: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TrafficStats>>>,
    }
    impl ListTrafficStatsResponse {
        pub fn builder() -> ListTrafficStatsResponseBuilder {
            ListTrafficStatsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Email traffic statistics pertaining to a specific date."]
    pub struct TrafficStats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliveryErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Delivery errors for the domain. This metric only pertains to traffic that passed [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/)."]
        pub delivery_errors:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeliveryError>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dkimSuccessRatio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ratio of mail that successfully authenticated with DKIM vs. all mail that attempted to authenticate with [DKIM](http://www.dkim.org/). Spoofed mail is excluded."]
        pub dkim_success_ratio: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dmarcSuccessRatio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ratio of mail that passed [DMARC](https://dmarc.org/) alignment checks vs all mail received from the domain that successfully authenticated with either of [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/)."]
        pub dmarc_success_ratio: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainReputation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reputation of the domain."]
        pub domain_reputation: ::std::option::Option<TrafficStatsDomainReputationEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inboundEncryptionRatio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ratio of incoming mail (to Gmail), that passed secure transport (TLS) vs all mail received from that domain. This metric only pertains to traffic that passed [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/)."]
        pub inbound_encryption_ratio: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipReputations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reputation information pertaining to the IP addresses of the email servers for the domain. There is exactly one entry for each reputation category except REPUTATION_CATEGORY_UNSPECIFIED."]
        pub ip_reputations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IpReputation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the traffic statistics. Traffic statistic names have the form `domains/{domain}/trafficStats/{date}`, where domain_name is the fully qualified domain name (i.e., mymail.mydomain.com) of the domain this traffic statistics pertains to and date is the date in yyyymmdd format that these statistics corresponds to. For example: domains/mymail.mydomain.com/trafficStats/20160807"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outboundEncryptionRatio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ratio of outgoing mail (from Gmail) that was accepted over secure transport (TLS)."]
        pub outbound_encryption_ratio: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spammyFeedbackLoops")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Spammy [Feedback loop identifiers] (https://support.google.com/mail/answer/6254652) with their individual spam rates. This metric only pertains to traffic that is authenticated by [DKIM](http://www.dkim.org/)."]
        pub spammy_feedback_loops:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FeedbackLoop>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spfSuccessRatio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ratio of mail that successfully authenticated with SPF vs. all mail that attempted to authenticate with [SPF](http://www.openspf.org/). Spoofed mail is excluded."]
        pub spf_success_ratio: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userReportedSpamRatio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ratio of user-report spam vs. email that was sent to the inbox. This metric only pertains to emails authenticated by [DKIM](http://www.dkim.org/)."]
        pub user_reported_spam_ratio: ::std::option::Option<::std::primitive::f64>,
    }
    impl TrafficStats {
        pub fn builder() -> TrafficStatsBuilder {
            TrafficStatsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Reputation of the domain."]
    pub enum TrafficStatsDomainReputationEnum {
        #[serde(rename = "REPUTATION_CATEGORY_UNSPECIFIED")]
        #[doc = "The default value which should never be used explicitly. This represents the state where no reputation information is available."]
        ReputationCategoryUnspecified,
        #[serde(rename = "HIGH")]
        #[doc = "Has a good track record of a very low spam rate, and complies with Gmail's sender guidelines. Mail will rarely be marked by the spam filter."]
        High,
        #[serde(rename = "MEDIUM")]
        #[doc = "Known to send good mail, but is prone to sending a low volume of spam intermittently. Most of the email from this entity will have a fair deliverability rate, except when there is a notable increase in spam levels."]
        Medium,
        #[serde(rename = "LOW")]
        #[doc = "Known to send a considerable volume of spam regularly, and mail from this sender will likely be marked as spam."]
        Low,
        #[serde(rename = "BAD")]
        #[doc = "History of sending an enormously high volume of spam. Mail coming from this entity will almost always be rejected at SMTP level or marked as spam."]
        Bad,
    }
    impl ::std::default::Default for TrafficStatsDomainReputationEnum {
        fn default() -> Self {
            Self::ReputationCategoryUnspecified
        }
    }
}
