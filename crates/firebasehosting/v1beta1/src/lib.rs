#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains metadata about the user who performed an action, such as creating a release or finalizing a version."]
pub struct ActingUser {
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of the user when the user performed the action."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A profile image URL for the user. May not be present if the user has changed their email address or deleted their account."]
    pub image_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a DNS certificate challenge."]
pub struct CertDnsChallenge {
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The domain name upon which the DNS challenge must be satisfied."]
    pub domain_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value that must be present as a TXT record on the domain name to satisfy the challenge."]
    pub token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an HTTP certificate challenge."]
pub struct CertHttpChallenge {
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL path on which to serve the specified token to satisfy the certificate challenge."]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token to serve at the specified URL path to satisfy the certificate challenge."]
    pub token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A `Channel` represents a stream of releases for a site. All sites have a default `live` channel that serves content to the live Firebase-provided domains and any connected custom domains."]
pub struct Channel {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which the channel was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the channel will be automatically deleted. If null, the channel will not be automatically deleted. This field is present in output whether set directly or via the `ttl` field."]
    pub expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Text labels used for extra metadata and/or filtering."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fully-qualified identifier of the Channel."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "release")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current release for the channel, if any."]
    pub release: ::std::option::Option<::std::boxed::Box<Release>>,
    #[serde(rename = "retainedReleaseCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of previous releases to retain on the channel for rollback or other purposes. Must be a number between 1-100. Defaults to 10 for new channels."]
    pub retained_release_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "ttl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input only. A time-to-live for this channel. Sets `expire_time` to the provided duration past the time of the request."]
    pub ttl: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which the channel was last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The URL at which the channel can be viewed. For the `live` channel, the content of the current release may also be visible at other URLs."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request sent to CloneVersion."]
pub struct CloneVersionRequest {
    #[serde(rename = "exclude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If provided, only paths that do not match any of the regexes in this list will be included in the new version."]
    pub exclude: ::std::option::Option<::std::boxed::Box<PathFilter>>,
    #[serde(rename = "finalize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, immediately finalize the version after cloning is complete."]
    pub finalize: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "include")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If provided, only paths that match one or more regexes in this list will be included in the new version."]
    pub include: ::std::option::Option<::std::boxed::Box<PathFilter>>,
    #[serde(rename = "sourceVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the version to be cloned, in the format: `sites/{site}/versions/{version}`"]
    pub source_version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A configured rewrite that directs requests to a Cloud Run service. If the Cloud Run service does not exist when setting or updating your Firebase Hosting configuration, then the request fails. Any errors from the Cloud Run service are passed to the end user (for example, if you delete a service, any requests directed to that service receive a `404` error)."]
pub struct CloudRunRewrite {
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. User-provided region where the Cloud Run service is hosted. Defaults to `us-central1` if not supplied."]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. User-defined ID of the Cloud Run service."]
    pub service_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The intended behavior and status information of a domain."]
pub struct Domain {
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The domain name of the association."]
    pub domain_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "domainRedirect")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, the domain should redirect with the provided parameters."]
    pub domain_redirect: ::std::option::Option<::std::boxed::Box<DomainRedirect>>,
    #[serde(rename = "provisioning")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Information about the provisioning of certificates and the health of the DNS resolution for the domain."]
    pub provisioning: ::std::option::Option<::std::boxed::Box<DomainProvisioning>>,
    #[serde(rename = "site")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The site name of the association."]
    pub site: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Additional status of the domain association."]
    pub status: ::std::option::Option<DomainStatusEnum>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which the domain was last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Additional status of the domain association."]
pub enum DomainStatusEnum {
    #[serde(rename = "DOMAIN_STATUS_UNSPECIFIED")]
    #[doc = "Unspecified domain association status."]
    DomainStatusUnspecified,
    #[serde(rename = "DOMAIN_CHANGE_PENDING")]
    #[doc = "An external operation is in progress on the domain association and no further operations can be performed until it is complete. Formerly used for metabase updates. Not currently used"]
    DomainChangePending,
    #[serde(rename = "DOMAIN_ACTIVE")]
    #[doc = "The domain association is active and no additional action is required."]
    DomainActive,
    #[serde(rename = "DOMAIN_VERIFICATION_REQUIRED")]
    #[doc = "The domain was previously verified in the legacy system. User must reverify the domain through the ownership service."]
    DomainVerificationRequired,
    #[serde(rename = "DOMAIN_VERIFICATION_LOST")]
    #[doc = "The domain verification has been lost and the domain is in the grace period before being removed from the Firebase Hosting site."]
    DomainVerificationLost,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The current certificate provisioning status information for a domain."]
pub struct DomainProvisioning {
    #[serde(rename = "certChallengeDiscoveredTxt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The TXT records (for the certificate challenge) that were found at the last DNS fetch."]
    pub cert_challenge_discovered_txt:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "certChallengeDns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The DNS challenge for generating a certificate."]
    pub cert_challenge_dns: ::std::option::Option<::std::boxed::Box<CertDnsChallenge>>,
    #[serde(rename = "certChallengeHttp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HTTP challenge for generating a certificate."]
    pub cert_challenge_http: ::std::option::Option<::std::boxed::Box<CertHttpChallenge>>,
    #[serde(rename = "certStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The certificate provisioning status; updated when Firebase Hosting provisions an SSL certificate for the domain."]
    pub cert_status: ::std::option::Option<DomainProvisioningCertStatusEnum>,
    #[serde(rename = "discoveredIps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IPs found at the last DNS fetch."]
    pub discovered_ips: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "dnsFetchTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the last DNS fetch occurred."]
    pub dns_fetch_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dnsStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The DNS record match status as of the last DNS fetch."]
    pub dns_status: ::std::option::Option<DomainProvisioningDnsStatusEnum>,
    #[serde(rename = "expectedIps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of IPs to which the domain is expected to resolve."]
    pub expected_ips: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The certificate provisioning status; updated when Firebase Hosting provisions an SSL certificate for the domain."]
pub enum DomainProvisioningCertStatusEnum {
    #[serde(rename = "CERT_STATUS_UNSPECIFIED")]
    #[doc = "Unspecified certificate provisioning status."]
    CertStatusUnspecified,
    #[serde(rename = "CERT_PENDING")]
    #[doc = "Waiting for certificate challenge to be created."]
    CertPending,
    #[serde(rename = "CERT_MISSING")]
    #[doc = "Waiting for certificate challenge to be met."]
    CertMissing,
    #[serde(rename = "CERT_PROCESSING")]
    #[doc = "Certificate challenge met; attempting to acquire/propagate certificate."]
    CertProcessing,
    #[serde(rename = "CERT_PROPAGATING")]
    #[doc = "Certificate obtained; propagating to the CDN."]
    CertPropagating,
    #[serde(rename = "CERT_ACTIVE")]
    #[doc = "Certificate provisioned and deployed across the CDN."]
    CertActive,
    #[serde(rename = "CERT_ERROR")]
    #[doc = "Certificate provisioning failed in a non-recoverable manner."]
    CertError,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The DNS record match status as of the last DNS fetch."]
pub enum DomainProvisioningDnsStatusEnum {
    #[serde(rename = "DNS_STATUS_UNSPECIFIED")]
    #[doc = "Unspecified DNS status."]
    DnsStatusUnspecified,
    #[serde(rename = "DNS_PENDING")]
    #[doc = "No DNS records have been specified for this domain yet."]
    DnsPending,
    #[serde(rename = "DNS_MISSING")]
    #[doc = "None of the required DNS records have been detected on the domain."]
    DnsMissing,
    #[serde(rename = "DNS_PARTIAL_MATCH")]
    #[doc = "Some of the required DNS records were detected, but not all of them. No extra (non-required) DNS records were detected."]
    DnsPartialMatch,
    #[serde(rename = "DNS_MATCH")]
    #[doc = "All required DNS records were detected. No extra (non-required) DNS records were detected."]
    DnsMatch,
    #[serde(rename = "DNS_EXTRANEOUS_MATCH")]
    #[doc = "The domain has at least one of the required DNS records, and it has at least one extra (non-required) DNS record."]
    DnsExtraneousMatch,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines the behavior of a domain-level redirect. Domain redirects preserve the path of the redirect but replace the requested domain with the one specified in the redirect configuration."]
pub struct DomainRedirect {
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The domain name to redirect to."]
    pub domain_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The redirect status code."]
    pub _type: ::std::option::Option<DomainRedirectTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The redirect status code."]
pub enum DomainRedirectTypeEnum {
    #[serde(rename = "REDIRECT_TYPE_UNSPECIFIED")]
    #[doc = "The default redirect type; should not be intentionlly used."]
    RedirectTypeUnspecified,
    #[serde(rename = "MOVED_PERMANENTLY")]
    #[doc = "The redirect will respond with an HTTP status code of `301 Moved Permanently`."]
    MovedPermanently,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A [`Header`](https://firebase.google.com/docs/hosting/full-config#headers) specifies a URL pattern that, if matched to the request URL path, triggers Hosting to apply the specified custom response headers."]
pub struct Header {
    #[serde(rename = "glob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-supplied [glob](https://firebase.google.com/docs/hosting/full-config#glob_pattern_matching) to match against the request URL path."]
    pub glob: ::std::option::Option<::std::string::String>,
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The additional headers to add to the response."]
    pub headers: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "regex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-supplied RE2 regular expression to match against the request URL path."]
    pub regex: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "If provided, i18n rewrites are enabled."]
pub struct I18nConfig {
    #[serde(rename = "root")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The user-supplied path where country and language specific content will be looked for within the public directory."]
    pub root: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response returned by ListChannels."]
pub struct ListChannelsResponse {
    #[serde(rename = "channels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of channels."]
    pub channels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Channel>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If there are additional releases remaining beyond the ones in this response, then supply this token in the next [`list`](../sites.channels/list) call to continue with the next set of releases."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response to listing Domains."]
pub struct ListDomainsResponse {
    #[serde(rename = "domains")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of domains, if any exist."]
    pub domains: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Domain>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token, if more results exist."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListReleasesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token, if more results exist beyond the ones in this response. Include this token in your next call to `ListReleases`. Page tokens are short-lived and should not be stored."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "releases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of hashes of files that still need to be uploaded, if any exist."]
    pub releases: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Release>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListVersionFilesResponse {
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = " The list of paths to the hashes of the files in the specified version."]
    pub files: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<VersionFile>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token, if more results exist beyond the ones in this response. Include this token in your next call to `ListVersionFiles`. Page tokens are short-lived and should not be stored."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListVersionsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token, if more results exist"]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "versions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of versions, if any exist."]
    pub versions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Version>>>,
}
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
#[doc = "A representation of filter path."]
pub struct PathFilter {
    #[serde(rename = "regexes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An array of regexes to filter by."]
    pub regexes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PopulateVersionFilesRequest {
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of file paths to the hashes corresponding to assets that should be added to the version. A file path to an empty hash will remove the path from the version. Calculate a hash by Gzipping the file then taking the SHA256 hash of the newly compressed file."]
    pub files: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PopulateVersionFilesResponse {
    #[serde(rename = "uploadRequiredHashes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content hashes of the specified files that need to be uploaded to the specified URL."]
    pub upload_required_hashes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "uploadUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL to which the files should be uploaded, in the format: \"https://upload-firebasehosting.googleapis.com/upload/sites/SITE_NAME /versions/VERSION_ID/files\" Perform a multipart `POST` of the Gzipped file contents to the URL using a forward slash and the hash of the file appended to the end."]
    pub upload_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Version preview configuration. If active and unexpired, this version will be accessible via a custom URL even if it is not the currently released version. Deprecated in favor of site channels."]
pub struct PreviewConfig {
    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, preview URLs are enabled for this version."]
    pub active: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the expiration time for previewing this version; preview URL requests received after this time will 404."]
    pub expire_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A [`Redirect`](https://firebase.google.com/docs/hosting/full-config#redirects) specifies a URL pattern that, if matched to the request URL path, triggers Hosting to respond with a redirect to the specified destination path."]
pub struct Redirect {
    #[serde(rename = "glob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-supplied [glob](https://firebase.google.com/docs/hosting/full-config#glob_pattern_matching) to match against the request URL path."]
    pub glob: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The value to put in the HTTP location header of the response. The location can contain capture group values from the pattern using a `:` prefix to identify the segment and an optional `*` to capture the rest of the URL. For example: \"glob\": \"/:capture*\", \"statusCode\": 301, \"location\": \"https://example.com/foo/:capture\""]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "regex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-supplied RE2 regular expression to match against the request URL path."]
    pub regex: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The status HTTP code to return in the response. It must be a valid 3xx status code."]
    pub status_code: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A `Release` is a particular [collection of configurations and files](sites.versions) that is set to be public at a particular time."]
pub struct Release {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The deploy description when the release was created. The value can be up to 512 characters."]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique identifier for the release, in the format: sites/ SITE_NAME/releases/RELEASE_ID This name is provided in the response body when you call [`CreateRelease`](sites.releases/create)."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "releaseTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which the version is set to be public."]
    pub release_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "releaseUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Identifies the user who created the release."]
    pub release_user: ::std::option::Option<::std::boxed::Box<ActingUser>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Explains the reason for the release. Specify a value for this field only when creating a `SITE_DISABLE` type release."]
    pub _type: ::std::option::Option<ReleaseTypeEnum>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The configuration and content that was released."]
    pub version: ::std::option::Option<::std::boxed::Box<Version>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Explains the reason for the release. Specify a value for this field only when creating a `SITE_DISABLE` type release."]
pub enum ReleaseTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "An unspecified type. Indicates that a version was released. This is the default value when no other `type` is explicitly specified."]
    TypeUnspecified,
    #[serde(rename = "DEPLOY")]
    #[doc = "A version was uploaded to Firebase Hosting and released."]
    Deploy,
    #[serde(rename = "ROLLBACK")]
    #[doc = "The release points back to a previously deployed version."]
    Rollback,
    #[serde(rename = "SITE_DISABLE")]
    #[doc = "The release prevents the site from serving content. Firebase Hosting acts as if the site never existed."]
    SiteDisable,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A [`Rewrite`](https://firebase.google.com/docs/hosting/full-config#rewrites) specifies a URL pattern that, if matched to the request URL path, triggers Hosting to respond as if the service were given the specified destination URL."]
pub struct Rewrite {
    #[serde(rename = "dynamicLinks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request will be forwarded to Firebase Dynamic Links."]
    pub dynamic_links: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "function")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The function to proxy requests to. Must match the exported function name exactly."]
    pub function: ::std::option::Option<::std::string::String>,
    #[serde(rename = "glob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-supplied [glob](https://firebase.google.com/docs/hosting/full-config#glob_pattern_matching) to match against the request URL path."]
    pub glob: ::std::option::Option<::std::string::String>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL path to rewrite the request to."]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "regex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-supplied RE2 regular expression to match against the request URL path."]
    pub regex: ::std::option::Option<::std::string::String>,
    #[serde(rename = "run")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request will be forwarded to Cloud Run."]
    pub run: ::std::option::Option<::std::boxed::Box<CloudRunRewrite>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The configuration for how incoming requests to a site should be routed and processed before serving content. The URL request paths are matched against the specified URL patterns in the configuration, then Hosting applies the applicable configuration according to a specific [priority order](https://firebase.google.com/docs/hosting/full-config#hosting_priority_order)."]
pub struct ServingConfig {
    #[serde(rename = "appAssociation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How to handle well known App Association files."]
    pub app_association: ::std::option::Option<ServingConfigAppAssociationEnum>,
    #[serde(rename = "cleanUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines whether to drop the file extension from uploaded files."]
    pub clean_urls: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An array of objects, where each object specifies a URL pattern that, if matched to the request URL path, triggers Hosting to apply the specified custom response headers."]
    pub headers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Header>>>,
    #[serde(rename = "i18n")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Defines i18n rewrite behavior."]
    pub i18n: ::std::option::Option<::std::boxed::Box<I18nConfig>>,
    #[serde(rename = "redirects")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An array of objects (called redirect rules), where each rule specifies a URL pattern that, if matched to the request URL path, triggers Hosting to respond with a redirect to the specified destination path."]
    pub redirects: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Redirect>>>,
    #[serde(rename = "rewrites")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An array of objects (called rewrite rules), where each rule specifies a URL pattern that, if matched to the request URL path, triggers Hosting to respond as if the service were given the specified destination URL."]
    pub rewrites: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Rewrite>>>,
    #[serde(rename = "trailingSlashBehavior")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines how to handle a trailing slash in the URL path."]
    pub trailing_slash_behavior: ::std::option::Option<ServingConfigTrailingSlashBehaviorEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "How to handle well known App Association files."]
pub enum ServingConfigAppAssociationEnum {
    #[serde(rename = "AUTO")]
    #[doc = "The app association files will be automatically created from the apps that exist in the Firebase project."]
    Auto,
    #[serde(rename = "NONE")]
    #[doc = "No special handling of the app association files will occur, these paths will result in a 404 unless caught with a Rewrite."]
    None,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Defines how to handle a trailing slash in the URL path."]
pub enum ServingConfigTrailingSlashBehaviorEnum {
    #[serde(rename = "TRAILING_SLASH_BEHAVIOR_UNSPECIFIED")]
    #[doc = "No behavior is specified. Files are served at their exact location only, and trailing slashes are only added to directory indexes."]
    TrailingSlashBehaviorUnspecified,
    #[serde(rename = "ADD")]
    #[doc = "Trailing slashes are _added_ to directory indexes as well as to any URL path not ending in a file extension."]
    Add,
    #[serde(rename = "REMOVE")]
    #[doc = "Trailing slashes are _removed_ from directory indexes as well as from any URL path not ending in a file extension."]
    Remove,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A `SiteConfig` contains metadata associated with a specific site that controls Firebase Hosting serving behavior"]
pub struct SiteConfig {
    #[serde(rename = "cloudLoggingEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not web requests made by site visitors are logged via Cloud Logging."]
    pub cloud_logging_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "maxVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of FINALIZED versions that will be held for a site before automatic deletion. When a new version is deployed, content for versions in storage in excess of this number will be deleted, and will no longer be billed for storage usage. Oldest versions will be deleted first; sites are created with an unlimited number of max_versions by default."]
    pub max_versions: ::std::option::Option<::std::string::String>,
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
#[doc = "A `Version` is a configuration and a collection of static files which determine how a site is displayed."]
pub struct Version {
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for the behavior of the site. This configuration exists in the [`firebase.json`](https://firebase.google.com/docs/cli/#the_firebasejson_file) file."]
    pub config: ::std::option::Option<::std::boxed::Box<ServingConfig>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which the version was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Identifies the user who created the version."]
    pub create_user: ::std::option::Option<::std::boxed::Box<ActingUser>>,
    #[serde(rename = "deleteTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which the version was `DELETED`."]
    pub delete_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deleteUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Identifies the user who `DELETED` the version."]
    pub delete_user: ::std::option::Option<::std::boxed::Box<ActingUser>>,
    #[serde(rename = "fileCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The total number of files associated with the version. This value is calculated after a version is `FINALIZED`."]
    pub file_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "finalizeTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which the version was `FINALIZED`."]
    pub finalize_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "finalizeUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Identifies the user who `FINALIZED` the version."]
    pub finalize_user: ::std::option::Option<::std::boxed::Box<ActingUser>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The labels used for extra metadata and/or filtering."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier for a version, in the format: sites/SITE_NAME /versions/VERSION_ID This name is provided in the response body when you call [`CreateVersion`](sites.versions/create)."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "preview")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated in favor of site channels. Version preview configuration for the site version. This configuration specifies whether previewing is enabled for this site version. Version previews allow you to preview your site at a custom URL before releasing it as the live version."]
    pub preview: ::std::option::Option<::std::boxed::Box<PreviewConfig>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The deploy status of a version. For a successful deploy, call [`CreateVersion`](sites.versions/create) to make a new version (`CREATED` status), [upload all desired files](sites.versions/populateFiles) to the version, then [update](sites.versions/patch) the version to the `FINALIZED` status. Note that if you leave the version in the `CREATED` state for more than 12 hours, the system will automatically mark the version as `ABANDONED`. You can also change the status of a version to `DELETED` by calling [`DeleteVersion`](sites.versions/delete)."]
    pub status: ::std::option::Option<VersionStatusEnum>,
    #[serde(rename = "versionBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The total stored bytesize of the version. This value is calculated after a version is `FINALIZED`."]
    pub version_bytes: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The deploy status of a version. For a successful deploy, call [`CreateVersion`](sites.versions/create) to make a new version (`CREATED` status), [upload all desired files](sites.versions/populateFiles) to the version, then [update](sites.versions/patch) the version to the `FINALIZED` status. Note that if you leave the version in the `CREATED` state for more than 12 hours, the system will automatically mark the version as `ABANDONED`. You can also change the status of a version to `DELETED` by calling [`DeleteVersion`](sites.versions/delete)."]
pub enum VersionStatusEnum {
    #[serde(rename = "VERSION_STATUS_UNSPECIFIED")]
    #[doc = "The default status; should not be intentionally used."]
    VersionStatusUnspecified,
    #[serde(rename = "CREATED")]
    #[doc = "The version has been created, and content is currently being added to the version."]
    Created,
    #[serde(rename = "FINALIZED")]
    #[doc = "All content has been added to the version, and the version can no longer be changed."]
    Finalized,
    #[serde(rename = "DELETED")]
    #[doc = "The version has been deleted."]
    Deleted,
    #[serde(rename = "ABANDONED")]
    #[doc = "The version was not updated to `FINALIZED` within 12 hours and was automatically deleted."]
    Abandoned,
    #[serde(rename = "EXPIRED")]
    #[doc = "The version is outside the site-configured limit for the number of retained versions, so the version's content is scheduled for deletion."]
    Expired,
    #[serde(rename = "CLONING")]
    #[doc = "The version is being cloned from another version. All content is still being copied over."]
    Cloning,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A static content file that is part of a version."]
pub struct VersionFile {
    #[serde(rename = "hash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The SHA256 content hash of the file."]
    pub hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI at which the file's content should display."]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current status of a particular file in the specified version. The value will be either `pending upload` or `uploaded`."]
    pub status: ::std::option::Option<VersionFileStatusEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current status of a particular file in the specified version. The value will be either `pending upload` or `uploaded`."]
pub enum VersionFileStatusEnum {
    #[serde(rename = "STATUS_UNSPECIFIED")]
    #[doc = "The default status; should not be intentionally used."]
    StatusUnspecified,
    #[serde(rename = "EXPECTED")]
    #[doc = "The file has been included in the version and is expected to be uploaded in the near future."]
    Expected,
    #[serde(rename = "ACTIVE")]
    #[doc = "The file has already been uploaded to Firebase Hosting."]
    Active,
}
