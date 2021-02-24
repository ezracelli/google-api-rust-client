#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metric on a particular delivery error type."]
pub struct DeliveryError {
    #[serde(rename = "errorClass")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The class of delivery error."]
    pub error_class: ::std::option::Option<DeliveryErrorErrorClassEnum>,
    #[serde(rename = "errorRatio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ratio of messages where the error occurred vs all authenticated traffic."]
    pub error_ratio: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "errorType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of delivery error."]
    pub error_type: ::std::option::Option<DeliveryErrorErrorTypeEnum>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A registered domain resource in the Postmaster API."]
pub struct Domain {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp when the user registered this domain. Assigned by the server."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the Domain. Domain names have the form `domains/{domain_name}`, where domain_name is the fully qualified domain name (i.e., mymail.mydomain.com)."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User’s permission for this domain. Assigned by the server."]
    pub permission: ::std::option::Option<DomainPermissionEnum>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "[Feedback loop](https://support.google.com/mail/answer/6254652) identifier information."]
pub struct FeedbackLoop {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feedback loop identifier that uniquely identifies individual campaigns."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "spamRatio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ratio of user marked spam messages with the identifier vs the total number of inboxed messages with that identifier."]
    pub spam_ratio: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "IP Reputation information for a set of IPs in a specific reputation category."]
pub struct IpReputation {
    #[serde(rename = "ipCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of unique IPs in this reputation category. This metric only pertains to traffic that passed [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/)."]
    pub ip_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reputation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reputation category this IP reputation represents."]
    pub reputation: ::std::option::Option<IpReputationReputationEnum>,
    #[serde(rename = "sampleIps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A sample of IPs in this reputation category."]
    pub sample_ips: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListDomains."]
pub struct ListDomainsResponse {
    #[serde(rename = "domains")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of domains."]
    pub domains: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Domain>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListTrafficStats."]
pub struct ListTrafficStatsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trafficStats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of TrafficStats."]
    pub traffic_stats: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TrafficStats>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Email traffic statistics pertaining to a specific date."]
pub struct TrafficStats {
    #[serde(rename = "deliveryErrors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Delivery errors for the domain. This metric only pertains to traffic that passed [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/)."]
    pub delivery_errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeliveryError>>>,
    #[serde(rename = "dkimSuccessRatio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ratio of mail that successfully authenticated with DKIM vs. all mail that attempted to authenticate with [DKIM](http://www.dkim.org/). Spoofed mail is excluded."]
    pub dkim_success_ratio: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "dmarcSuccessRatio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ratio of mail that passed [DMARC](https://dmarc.org/) alignment checks vs all mail received from the domain that successfully authenticated with either of [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/)."]
    pub dmarc_success_ratio: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "domainReputation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reputation of the domain."]
    pub domain_reputation: ::std::option::Option<TrafficStatsDomainReputationEnum>,
    #[serde(rename = "inboundEncryptionRatio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ratio of incoming mail (to Gmail), that passed secure transport (TLS) vs all mail received from that domain. This metric only pertains to traffic that passed [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/)."]
    pub inbound_encryption_ratio: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "ipReputations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reputation information pertaining to the IP addresses of the email servers for the domain. There is exactly one entry for each reputation category except REPUTATION_CATEGORY_UNSPECIFIED."]
    pub ip_reputations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IpReputation>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the traffic statistics. Traffic statistic names have the form `domains/{domain}/trafficStats/{date}`, where domain_name is the fully qualified domain name (i.e., mymail.mydomain.com) of the domain this traffic statistics pertains to and date is the date in yyyymmdd format that these statistics corresponds to. For example: domains/mymail.mydomain.com/trafficStats/20160807"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outboundEncryptionRatio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ratio of outgoing mail (from Gmail) that was accepted over secure transport (TLS)."]
    pub outbound_encryption_ratio: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "spammyFeedbackLoops")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Spammy [Feedback loop identifiers] (https://support.google.com/mail/answer/6254652) with their individual spam rates. This metric only pertains to traffic that is authenticated by [DKIM](http://www.dkim.org/)."]
    pub spammy_feedback_loops:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FeedbackLoop>>>,
    #[serde(rename = "spfSuccessRatio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ratio of mail that successfully authenticated with SPF vs. all mail that attempted to authenticate with [SPF](http://www.openspf.org/). Spoofed mail is excluded."]
    pub spf_success_ratio: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "userReportedSpamRatio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ratio of user-report spam vs. email that was sent to the inbox. This metric only pertains to emails authenticated by [DKIM](http://www.dkim.org/)."]
    pub user_reported_spam_ratio: ::std::option::Option<::std::primitive::f64>,
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
