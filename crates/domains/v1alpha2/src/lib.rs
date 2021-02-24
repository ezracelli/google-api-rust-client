#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { \"audit_configs\": [ { \"service\": \"allServices\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" }, { \"log_type\": \"ADMIN_READ\" } ] }, { \"service\": \"sampleservice.googleapis.com\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\" }, { \"log_type\": \"DATA_WRITE\", \"exempted_members\": [ \"user:aliya@example.com\" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging."]
pub struct AuditConfig {
    #[serde(rename = "auditLogConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for logging of each type of permission."]
    pub audit_log_configs:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditLogConfig>>>,
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services."]
    pub service: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Provides the configuration for logging a type of permissions. Example: { \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging."]
pub struct AuditLogConfig {
    #[serde(rename = "exemptedMembers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members."]
    pub exempted_members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "logType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The log type that this config enables."]
    pub log_type: ::std::option::Option<AuditLogConfigLogTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The log type that this config enables."]
pub enum AuditLogConfigLogTypeEnum {
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines an authorization code."]
pub struct AuthorizationCode {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Authorization Code in ASCII. It can be used to transfer the domain to or from another registrar."]
    pub code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Associates `members` with a `role`."]
pub struct Binding {
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the members in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub condition: ::std::option::Option<::std::boxed::Box<Expr>>,
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the identities requesting access for a Cloud Platform resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. "]
    pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Role that is assigned to `members`. For example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
    pub role: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the `ConfigureContactSettings` method."]
pub struct ConfigureContactSettingsRequest {
    #[serde(rename = "contactNotices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of contact notices that the caller acknowledges. The notices needed here depend on the values specified in `contact_settings`."]
    pub contact_notices:
        ::std::option::Option<::std::vec::Vec<ConfigureContactSettingsRequestContactNoticesEnum>>,
    #[serde(rename = "contactSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fields of the `ContactSettings` to update."]
    pub contact_settings: ::std::option::Option<::std::boxed::Box<ContactSettings>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The field mask describing which fields to update as a comma-separated list. For example, if only the registrant contact is being updated, the `update_mask` would be `\"registrant_contact\"`."]
    pub update_mask: ::std::option::Option<::std::string::String>,
    #[serde(rename = "validateOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Validate the request without actually updating the contact settings."]
    pub validate_only: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ConfigureContactSettingsRequestContactNoticesEnum {
    #[serde(rename = "CONTACT_NOTICE_UNSPECIFIED")]
    #[doc = "The notice is undefined."]
    ContactNoticeUnspecified,
    #[serde(rename = "PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT")]
    #[doc = "Required when setting the `privacy` field of `ContactSettings` to `PUBLIC_CONTACT_DATA`, which exposes contact data publicly."]
    PublicContactDataAcknowledgement,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the `ConfigureDnsSettings` method."]
pub struct ConfigureDnsSettingsRequest {
    #[serde(rename = "dnsSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fields of the `DnsSettings` to update."]
    pub dns_settings: ::std::option::Option<::std::boxed::Box<DnsSettings>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The field mask describing which fields to update as a comma-separated list. For example, if only the name servers are being updated for an existing Custom DNS configuration, the `update_mask` would be `\"custom_dns.name_servers\"`. When changing the DNS provider from one type to another, pass the new provider's field name as part of the field mask. For example, when changing from a Google Domains DNS configuration to a Custom DNS configuration, the `update_mask` would be `\"custom_dns\"`. //"]
    pub update_mask: ::std::option::Option<::std::string::String>,
    #[serde(rename = "validateOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Validate the request without actually updating the DNS settings."]
    pub validate_only: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the `ConfigureManagementSettings` method."]
pub struct ConfigureManagementSettingsRequest {
    #[serde(rename = "managementSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fields of the `ManagementSettings` to update."]
    pub management_settings: ::std::option::Option<::std::boxed::Box<ManagementSettings>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The field mask describing which fields to update as a comma-separated list. For example, if only the transfer lock is being updated, the `update_mask` would be `\"transfer_lock_state\"`."]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details required for a contact associated with a `Registration`."]
pub struct Contact {
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Email address of the contact."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "faxNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fax number of the contact in international format. For example, `\"+1-800-555-0123\"`."]
    pub fax_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "phoneNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Phone number of the contact in international format. For example, `\"+1-800-555-0123\"`."]
    pub phone_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postalAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Postal address of the contact."]
    pub postal_address: ::std::option::Option<::std::boxed::Box<PostalAddress>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines the contact information associated with a `Registration`. [ICANN](https://icann.org/) requires all domain names to have associated contact information. The `registrant_contact` is considered the domain's legal owner, and often the other contacts are identical."]
pub struct ContactSettings {
    #[serde(rename = "adminContact")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The administrative contact for the `Registration`."]
    pub admin_contact: ::std::option::Option<::std::boxed::Box<Contact>>,
    #[serde(rename = "privacy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Privacy setting for the contacts associated with the `Registration`."]
    pub privacy: ::std::option::Option<ContactSettingsPrivacyEnum>,
    #[serde(rename = "registrantContact")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The registrant contact for the `Registration`. *Caution: Anyone with access to this email address, phone number, and/or postal address can take control of the domain.* *Warning: For new `Registration`s, the registrant will receive an email confirmation that they must complete within 15 days to avoid domain suspension.*"]
    pub registrant_contact: ::std::option::Option<::std::boxed::Box<Contact>>,
    #[serde(rename = "technicalContact")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The technical contact for the `Registration`."]
    pub technical_contact: ::std::option::Option<::std::boxed::Box<Contact>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Privacy setting for the contacts associated with the `Registration`."]
pub enum ContactSettingsPrivacyEnum {
    #[serde(rename = "CONTACT_PRIVACY_UNSPECIFIED")]
    #[doc = "The contact privacy settings are undefined."]
    ContactPrivacyUnspecified,
    #[serde(rename = "PUBLIC_CONTACT_DATA")]
    #[doc = "All the data from `ContactSettings` is publicly available. When setting this option, you must also provide a `PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT` in the `contact_notices` field of the request."]
    PublicContactData,
    #[serde(rename = "PRIVATE_CONTACT_DATA")]
    #[doc = "None of the data from `ContactSettings` is publicly available. Instead, proxy contact data is published for your domain. Email sent to the proxy email address is forwarded to the registrant's email address. Cloud Domains provides this privacy proxy service at no additional cost."]
    PrivateContactData,
    #[serde(rename = "REDACTED_CONTACT_DATA")]
    #[doc = "Some data from `ContactSettings` is publicly available. The actual information redacted depends on the domain. For details, see [the registration privacy article](https://support.google.com/domains/answer/3251242)."]
    RedactedContactData,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for an arbitrary DNS provider."]
pub struct CustomDns {
    #[serde(rename = "dsRecords")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of DS records for this domain, which are used to enable DNSSEC. The domain's DNS provider can provide the values to set here. If this field is empty, DNSSEC is disabled."]
    pub ds_records: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DsRecord>>>,
    #[serde(rename = "nameServers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A list of name servers that store the DNS zone for this domain. Each name server is a domain name, with Unicode domain names expressed in Punycode format."]
    pub name_servers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines the DNS configuration of a `Registration`, including name servers, DNSSEC, and glue records."]
pub struct DnsSettings {
    #[serde(rename = "customDns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An arbitrary DNS provider identified by its name servers."]
    pub custom_dns: ::std::option::Option<::std::boxed::Box<CustomDns>>,
    #[serde(rename = "glueRecords")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of glue records for this `Registration`. Commonly empty."]
    pub glue_records: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GlueRecord>>>,
    #[serde(rename = "googleDomainsDns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The free DNS zone provided by [Google Domains](https://domains.google/)."]
    pub google_domains_dns: ::std::option::Option<::std::boxed::Box<GoogleDomainsDns>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines a Delegation Signer (DS) record, which is needed to enable DNSSEC for a domain. It contains a digest (hash) of a DNSKEY record that must be present in the domain's DNS zone."]
pub struct DsRecord {
    #[serde(rename = "algorithm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The algorithm used to generate the referenced DNSKEY."]
    pub algorithm: ::std::option::Option<DsRecordAlgorithmEnum>,
    #[serde(rename = "digest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The digest generated from the referenced DNSKEY."]
    pub digest: ::std::option::Option<::std::string::String>,
    #[serde(rename = "digestType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The hash function used to generate the digest of the referenced DNSKEY."]
    pub digest_type: ::std::option::Option<DsRecordDigestTypeEnum>,
    #[serde(rename = "keyTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The key tag of the record. Must be set in range 0 -- 65535."]
    pub key_tag: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The algorithm used to generate the referenced DNSKEY."]
pub enum DsRecordAlgorithmEnum {
    #[serde(rename = "ALGORITHM_UNSPECIFIED")]
    #[doc = "The algorithm is unspecified."]
    AlgorithmUnspecified,
    #[serde(rename = "DSA")]
    #[doc = "DSA/SHA1. Not recommended for new deployments."]
    Dsa,
    #[serde(rename = "ECC")]
    #[doc = "ECC. Not recommended for new deployments."]
    Ecc,
    #[serde(rename = "RSASHA1")]
    #[doc = "RSA/SHA-1. Not recommended for new deployments."]
    Rsasha1,
    #[serde(rename = "DSANSEC3SHA1")]
    #[doc = "DSA-NSEC3-SHA1. Not recommended for new deployments."]
    Dsansec3Sha1,
    #[serde(rename = "RSASHA1NSEC3SHA1")]
    #[doc = "RSA/SHA1-NSEC3-SHA1. Not recommended for new deployments."]
    Rsasha1Nsec3Sha1,
    #[serde(rename = "RSASHA256")]
    #[doc = "RSA/SHA-256."]
    Rsasha256,
    #[serde(rename = "RSASHA512")]
    #[doc = "RSA/SHA-512."]
    Rsasha512,
    #[serde(rename = "ECCGOST")]
    #[doc = "GOST R 34.10-2001."]
    Eccgost,
    #[serde(rename = "ECDSAP256SHA256")]
    #[doc = "ECDSA Curve P-256 with SHA-256."]
    Ecdsap256Sha256,
    #[serde(rename = "ECDSAP384SHA384")]
    #[doc = "ECDSA Curve P-384 with SHA-384."]
    Ecdsap384Sha384,
    #[serde(rename = "ED25519")]
    #[doc = "Ed25519."]
    Ed25519,
    #[serde(rename = "ED448")]
    #[doc = "Ed448."]
    Ed448,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The hash function used to generate the digest of the referenced DNSKEY."]
pub enum DsRecordDigestTypeEnum {
    #[serde(rename = "DIGEST_TYPE_UNSPECIFIED")]
    #[doc = "The DigestType is unspecified."]
    DigestTypeUnspecified,
    #[serde(rename = "SHA1")]
    #[doc = "SHA-1. Not recommended for new deployments."]
    Sha1,
    #[serde(rename = "SHA256")]
    #[doc = "SHA-256."]
    Sha256,
    #[serde(rename = "GOST3411")]
    #[doc = "GOST R 34.11-94."]
    Gost3411,
    #[serde(rename = "SHA384")]
    #[doc = "SHA-384."]
    Sha384,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the `ExportRegistration` method."]
pub struct ExportRegistrationRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() < 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\" expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description: \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information."]
pub struct Expr {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expression")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines a host on your domain that is a DNS name server for your domain and/or other domains. Glue records are a way of making the IP address of a name server known, even when it serves DNS queries for its parent domain. For example, when `ns.example.com` is a name server for `example.com`, the host `ns.example.com` must have a glue record to break the circular DNS reference."]
pub struct GlueRecord {
    #[serde(rename = "hostName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Domain name of the host in Punycode format."]
    pub host_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ipv4Addresses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of IPv4 addresses corresponding to this host in the standard decimal format (e.g. `198.51.100.1`). At least one of `ipv4_address` and `ipv6_address` must be set."]
    pub ipv4_addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "ipv6Addresses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of IPv6 addresses corresponding to this host in the standard hexadecimal format (e.g. `2001:db8::`). At least one of `ipv4_address` and `ipv6_address` must be set."]
    pub ipv6_addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for using the free DNS zone provided by Google Domains as a `Registration`'s `dns_provider`. You cannot configure the DNS zone itself using the API. To configure the DNS zone, go to [Google Domains](https://domains.google/)."]
pub struct GoogleDomainsDns {
    #[serde(rename = "dsRecords")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The list of DS records published for this domain. The list is automatically populated when `ds_state` is `DS_RECORDS_PUBLISHED`, otherwise it remains empty."]
    pub ds_records: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DsRecord>>>,
    #[serde(rename = "dsState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The state of DS records for this domain. Used to enable or disable automatic DNSSEC."]
    pub ds_state: ::std::option::Option<GoogleDomainsDnsDsStateEnum>,
    #[serde(rename = "nameServers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A list of name servers that store the DNS zone for this domain. Each name server is a domain name, with Unicode domain names expressed in Punycode format. This field is automatically populated with the name servers assigned to the Google Domains DNS zone."]
    pub name_servers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The state of DS records for this domain. Used to enable or disable automatic DNSSEC."]
pub enum GoogleDomainsDnsDsStateEnum {
    #[serde(rename = "DS_STATE_UNSPECIFIED")]
    #[doc = "DS state is unspecified."]
    DsStateUnspecified,
    #[serde(rename = "DS_RECORDS_UNPUBLISHED")]
    #[doc = "DNSSEC is disabled for this domain. No DS records for this domain are published in the parent DNS zone."]
    DsRecordsUnpublished,
    #[serde(rename = "DS_RECORDS_PUBLISHED")]
    #[doc = "DNSSEC is enabled for this domain. Appropriate DS records for this domain are published in the parent DNS zone. This option is valid only if the DNS zone referenced in the `Registration`'s `dns_provider` field is already DNSSEC-signed."]
    DsRecordsPublished,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Locations.ListLocations."]
pub struct ListLocationsResponse {
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of locations that matches the specified filter in the request."]
    pub locations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Location>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The standard List next-page token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Operations.ListOperations."]
pub struct ListOperationsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The standard List next-page token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of operations that matches the specified filter in the request."]
    pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `ListRegistrations` method."]
pub struct ListRegistrationsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When present, there are more results to retrieve. Set `page_token` to this value on a subsequent call to get the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "registrations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of `Registration`s."]
    pub registrations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Registration>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A resource that represents Google Cloud Platform location."]
pub struct Location {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The friendly name for this location, typically a nearby city name. For example, \"Tokyo\"."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cross-service attributes for the location. For example {\"cloud.googleapis.com/region\": \"us-east1\"}"]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The canonical id for this location. For example: `\"us-east1\"`."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Service-specific metadata. For example the available capacity at the given location."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name for the location, which may vary between implementations. For example: `\"projects/example-project/locations/us-east1\"`"]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines renewal, billing, and transfer settings for a `Registration`."]
pub struct ManagementSettings {
    #[serde(rename = "renewalMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The renewal method for this `Registration`."]
    pub renewal_method: ::std::option::Option<ManagementSettingsRenewalMethodEnum>,
    #[serde(rename = "transferLockState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Controls whether the domain can be transferred to another registrar."]
    pub transfer_lock_state: ::std::option::Option<ManagementSettingsTransferLockStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The renewal method for this `Registration`."]
pub enum ManagementSettingsRenewalMethodEnum {
    #[serde(rename = "RENEWAL_METHOD_UNSPECIFIED")]
    #[doc = "The renewal method is undefined."]
    RenewalMethodUnspecified,
    #[serde(rename = "AUTOMATIC_RENEWAL")]
    #[doc = "The domain is automatically renewed each year . To disable automatic renewals, export the domain by calling `ExportRegistration` ."]
    AutomaticRenewal,
    #[serde(rename = "MANUAL_RENEWAL")]
    #[doc = "The domain must be explicitly renewed each year before its `expire_time`. This option is only available when the `Registration` is in state `EXPORTED`. To manage the domain's current billing and renewal settings, go to [Google Domains](https://domains.google/)."]
    ManualRenewal,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Controls whether the domain can be transferred to another registrar."]
pub enum ManagementSettingsTransferLockStateEnum {
    #[serde(rename = "TRANSFER_LOCK_STATE_UNSPECIFIED")]
    #[doc = "The state is unspecified."]
    TransferLockStateUnspecified,
    #[serde(rename = "UNLOCKED")]
    #[doc = "The domain is unlocked and can be transferred to another registrar."]
    Unlocked,
    #[serde(rename = "LOCKED")]
    #[doc = "The domain is locked and cannot be transferred to another registrar."]
    Locked,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an amount of money with its currency type."]
pub struct Money {
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The three-letter currency code defined in ISO 4217."]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nanos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
    pub nanos: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "units")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The whole units of the amount. For example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
    pub units: ::std::option::Option<::std::string::String>,
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
#[doc = "Represents the metadata of the long-running operation. Output only."]
pub struct OperationMetadata {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API version used to start the operation."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time the operation was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time the operation finished running."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statusDetail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human-readable status of the operation, if any."]
    pub status_detail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Server-defined resource path for the target of the operation."]
    pub target: ::std::option::Option<::std::string::String>,
    #[serde(rename = "verb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the verb executed by the operation."]
    pub verb: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/)."]
pub struct Policy {
    #[serde(rename = "auditConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies cloud audit logging configuration for this policy."]
    pub audit_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditConfig>>>,
    #[serde(rename = "bindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member."]
    pub bindings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Binding>>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a postal address, e.g. for postal delivery or payments addresses. Given a postal address, a postal service can deliver items to a premise, P.O. Box or similar. It is not intended to model geographical locations (roads, towns, mountains). In typical usage an address would be created via user input or from importing existing data, depending on the type of process. Advice on address input / editing: - Use an i18n-ready address widget such as https://github.com/google/libaddressinput) - Users should not be presented with UI elements for input or editing of fields outside countries where that field is used. For more guidance on how to use this schema, please see: https://support.google.com/business/answer/6397478"]
pub struct PostalAddress {
    #[serde(rename = "addressLines")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unstructured address lines describing the lower levels of an address. Because values in address_lines do not have type information and may sometimes contain multiple values in a single field (e.g. \"Austin, TX\"), it is important that the line order is clear. The order of address lines should be \"envelope order\" for the country/region of the address. In places where this can vary (e.g. Japan), address_language is used to make it explicit (e.g. \"ja\" for large-to-small ordering and \"ja-Latn\" or \"en\" for small-to-large). This way, the most specific line of an address can be selected based on the language. The minimum permitted structural representation of an address consists of a region_code with all remaining information placed in the address_lines. It would be possible to format such an address very approximately without geocoding, but no semantic reasoning could be made about any of the address components until it was at least partially resolved. Creating an address only containing a region_code and address_lines, and then geocoding is the recommended way to handle completely unstructured addresses (as opposed to guessing which parts of the address should be localities or administrative areas)."]
    pub address_lines: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "administrativeArea")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Highest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state, a province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community (e.g. \"Barcelona\" and not \"Catalonia\"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland this should be left unpopulated."]
    pub administrative_area: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. BCP-47 language code of the contents of this address (if known). This is often the UI language of the input form or is expected to match one of the languages used in the address' country/region, or their transliterated equivalents. This can affect formatting in certain countries, but is not critical to the correctness of the data and will never affect any validation or other non-formatting related operations. If this value is not known, it should be omitted (rather than specifying a possibly incorrect default). Examples: \"zh-Hant\", \"ja\", \"ja-Latn\", \"en\"."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locality")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Generally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world where localities are not well defined or do not fit into this structure well, leave locality empty and use address_lines."]
    pub locality: ::std::option::Option<::std::string::String>,
    #[serde(rename = "organization")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The name of the organization at the address."]
    pub organization: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Postal code of the address. Not all countries use or require postal codes to be present, but where they are used, they may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.)."]
    pub postal_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "recipients")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The recipient at the address. This field may, under certain circumstances, contain multiline information. For example, it might contain \"care of\" information."]
    pub recipients: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "regionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to ensure the value is correct. See http://cldr.unicode.org/ and http://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: \"CH\" for Switzerland."]
    pub region_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The schema revision of the `PostalAddress`. This must be set to 0, which is the latest revision. All new revisions **must** be backward compatible with old revisions."]
    pub revision: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "sortingCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Additional, country-specific, sorting code. This is not used in most regions. Where it is used, the value is either a string like \"CEDEX\", optionally followed by a number (e.g. \"CEDEX 7\"), or just a number alone, representing the \"sector code\" (Jamaica), \"delivery area indicator\" (Malawi) or \"post office indicator\" (e.g. Côte d'Ivoire)."]
    pub sorting_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sublocality")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Sublocality of the address. For example, this can be neighborhoods, boroughs, districts."]
    pub sublocality: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the `RegisterDomain` method."]
pub struct RegisterDomainRequest {
    #[serde(rename = "contactNotices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of contact notices that the caller acknowledges. The notices needed here depend on the values specified in `registration.contact_settings`."]
    pub contact_notices:
        ::std::option::Option<::std::vec::Vec<RegisterDomainRequestContactNoticesEnum>>,
    #[serde(rename = "domainNotices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of domain notices that you acknowledge. Call `RetrieveRegisterParameters` to see the notices that need acknowledgement."]
    pub domain_notices:
        ::std::option::Option<::std::vec::Vec<RegisterDomainRequestDomainNoticesEnum>>,
    #[serde(rename = "registration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The complete `Registration` resource to be created."]
    pub registration: ::std::option::Option<::std::boxed::Box<Registration>>,
    #[serde(rename = "validateOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When true, only validation will be performed, without actually registering the domain. Follows: https://cloud.google.com/apis/design/design_patterns#request_validation"]
    pub validate_only: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "yearlyPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Yearly price to register or renew the domain. The value that should be put here can be obtained from RetrieveRegisterParameters or SearchDomains calls."]
    pub yearly_price: ::std::option::Option<::std::boxed::Box<Money>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum RegisterDomainRequestContactNoticesEnum {
    #[serde(rename = "CONTACT_NOTICE_UNSPECIFIED")]
    #[doc = "The notice is undefined."]
    ContactNoticeUnspecified,
    #[serde(rename = "PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT")]
    #[doc = "Required when setting the `privacy` field of `ContactSettings` to `PUBLIC_CONTACT_DATA`, which exposes contact data publicly."]
    PublicContactDataAcknowledgement,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum RegisterDomainRequestDomainNoticesEnum {
    #[serde(rename = "DOMAIN_NOTICE_UNSPECIFIED")]
    #[doc = "The notice is undefined."]
    DomainNoticeUnspecified,
    #[serde(rename = "HSTS_PRELOADED")]
    #[doc = "Indicates that the domain is preloaded on the HTTP Strict Transport Security list in browsers. Serving a website on such domain requires an SSL certificate. For details, see [how to get an SSL certificate](https://support.google.com/domains/answer/7638036)."]
    HstsPreloaded,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Parameters required to register a new domain."]
pub struct RegisterParameters {
    #[serde(rename = "availability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the domain is available for registration. This value is accurate when obtained by calling `RetrieveRegisterParameters`, but is approximate when obtained by calling `SearchDomains`."]
    pub availability: ::std::option::Option<RegisterParametersAvailabilityEnum>,
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The domain name. Unicode domain names are expressed in Punycode format."]
    pub domain_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "domainNotices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Notices about special properties of the domain."]
    pub domain_notices: ::std::option::Option<::std::vec::Vec<RegisterParametersDomainNoticesEnum>>,
    #[serde(rename = "supportedPrivacy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contact privacy options that the domain supports."]
    pub supported_privacy:
        ::std::option::Option<::std::vec::Vec<RegisterParametersSupportedPrivacyEnum>>,
    #[serde(rename = "yearlyPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Price to register or renew the domain for one year."]
    pub yearly_price: ::std::option::Option<::std::boxed::Box<Money>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Indicates whether the domain is available for registration. This value is accurate when obtained by calling `RetrieveRegisterParameters`, but is approximate when obtained by calling `SearchDomains`."]
pub enum RegisterParametersAvailabilityEnum {
    #[serde(rename = "AVAILABILITY_UNSPECIFIED")]
    #[doc = "The availability is unspecified."]
    AvailabilityUnspecified,
    #[serde(rename = "AVAILABLE")]
    #[doc = "The domain is available for registration."]
    Available,
    #[serde(rename = "UNAVAILABLE")]
    #[doc = "The domain is not available for registration. Generally this means it is already registered to another party."]
    Unavailable,
    #[serde(rename = "UNSUPPORTED")]
    #[doc = "The domain is not currently supported by Cloud Domains, but may be available elsewhere."]
    Unsupported,
    #[serde(rename = "UNKNOWN")]
    #[doc = "Cloud Domains is unable to determine domain availability, generally due to system maintenance at the domain name registry."]
    Unknown,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum RegisterParametersDomainNoticesEnum {
    #[serde(rename = "DOMAIN_NOTICE_UNSPECIFIED")]
    #[doc = "The notice is undefined."]
    DomainNoticeUnspecified,
    #[serde(rename = "HSTS_PRELOADED")]
    #[doc = "Indicates that the domain is preloaded on the HTTP Strict Transport Security list in browsers. Serving a website on such domain requires an SSL certificate. For details, see [how to get an SSL certificate](https://support.google.com/domains/answer/7638036)."]
    HstsPreloaded,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum RegisterParametersSupportedPrivacyEnum {
    #[serde(rename = "CONTACT_PRIVACY_UNSPECIFIED")]
    #[doc = "The contact privacy settings are undefined."]
    ContactPrivacyUnspecified,
    #[serde(rename = "PUBLIC_CONTACT_DATA")]
    #[doc = "All the data from `ContactSettings` is publicly available. When setting this option, you must also provide a `PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT` in the `contact_notices` field of the request."]
    PublicContactData,
    #[serde(rename = "PRIVATE_CONTACT_DATA")]
    #[doc = "None of the data from `ContactSettings` is publicly available. Instead, proxy contact data is published for your domain. Email sent to the proxy email address is forwarded to the registrant's email address. Cloud Domains provides this privacy proxy service at no additional cost."]
    PrivateContactData,
    #[serde(rename = "REDACTED_CONTACT_DATA")]
    #[doc = "Some data from `ContactSettings` is publicly available. The actual information redacted depends on the domain. For details, see [the registration privacy article](https://support.google.com/domains/answer/3251242)."]
    RedactedContactData,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The `Registration` resource facilitates managing and configuring domain name registrations. To create a new `Registration` resource, find a suitable domain name by calling the `SearchDomains` method with a query to see available domain name options. After choosing a name, call `RetrieveRegisterParameters` to ensure availability and obtain information like pricing, which is needed to build a call to `RegisterDomain`. "]
pub struct Registration {
    #[serde(rename = "contactSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Settings for contact information linked to the `Registration`. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureContactSettings` method."]
    pub contact_settings: ::std::option::Option<::std::boxed::Box<ContactSettings>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The creation timestamp of the `Registration` resource."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dnsSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings controlling the DNS configuration of the `Registration`. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureDnsSettings` method."]
    pub dns_settings: ::std::option::Option<::std::boxed::Box<DnsSettings>>,
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The domain name. Unicode domain names must be expressed in Punycode format."]
    pub domain_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The expiration timestamp of the `Registration`."]
    pub expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "issues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The set of issues with the `Registration` that require attention."]
    pub issues: ::std::option::Option<::std::vec::Vec<RegistrationIssuesEnum>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set of labels associated with the `Registration`."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "managementSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings for management of the `Registration`, including renewal, billing, and transfer. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureManagementSettings` method."]
    pub management_settings: ::std::option::Option<::std::boxed::Box<ManagementSettings>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Name of the `Registration` resource, in the format `projects/*/locations/*/registrations/`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pendingContactSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Pending contact settings for the `Registration`. Updates to the `contact_settings` field that change its `registrant_contact` or `privacy` fields require email confirmation by the `registrant_contact` before taking effect. This field is set only if there are pending updates to the `contact_settings` that have not yet been confirmed. To confirm the changes, the `registrant_contact` must follow the instructions in the email they receive."]
    pub pending_contact_settings: ::std::option::Option<::std::boxed::Box<ContactSettings>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The state of the `Registration`"]
    pub state: ::std::option::Option<RegistrationStateEnum>,
    #[serde(rename = "supportedPrivacy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Set of options for the `contact_settings.privacy` field that this `Registration` supports."]
    pub supported_privacy: ::std::option::Option<::std::vec::Vec<RegistrationSupportedPrivacyEnum>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum RegistrationIssuesEnum {
    #[serde(rename = "ISSUE_UNSPECIFIED")]
    #[doc = "The issue is undefined."]
    IssueUnspecified,
    #[serde(rename = "CONTACT_SUPPORT")]
    #[doc = "Contact the Cloud Support team to resolve a problem with this domain."]
    ContactSupport,
    #[serde(rename = "UNVERIFIED_EMAIL")]
    #[doc = "[ICANN](https://icann.org/) requires verification of the email address in the `Registration`'s `contact_settings.registrant_contact` field. To verify the email address, follow the instructions in the email the `registrant_contact` receives following registration. If you do not complete email verification within 15 days of registration, the domain is suspended. To resend the verification email, call ConfigureContactSettings and provide the current `registrant_contact.email`."]
    UnverifiedEmail,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The state of the `Registration`"]
pub enum RegistrationStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The state is undefined."]
    StateUnspecified,
    #[serde(rename = "REGISTRATION_PENDING")]
    #[doc = "The domain is being registered."]
    RegistrationPending,
    #[serde(rename = "REGISTRATION_FAILED")]
    #[doc = "The domain registration failed. You can delete resources in this state to allow registration to be retried."]
    RegistrationFailed,
    #[serde(rename = "ACTIVE")]
    #[doc = "The domain is registered and operational. The domain renews automatically as long as it remains in this state."]
    Active,
    #[serde(rename = "SUSPENDED")]
    #[doc = "The domain is suspended and inoperative. For more details, see the `issues` field."]
    Suspended,
    #[serde(rename = "EXPORTED")]
    #[doc = "The domain has been exported from Cloud Domains to [Google Domains](https://domains.google/). You can no longer update it with this API, and information shown about it may be stale. Without further action, domains in this state expire at their `expire_time`. You can delete the resource after the `expire_time` has passed."]
    Exported,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum RegistrationSupportedPrivacyEnum {
    #[serde(rename = "CONTACT_PRIVACY_UNSPECIFIED")]
    #[doc = "The contact privacy settings are undefined."]
    ContactPrivacyUnspecified,
    #[serde(rename = "PUBLIC_CONTACT_DATA")]
    #[doc = "All the data from `ContactSettings` is publicly available. When setting this option, you must also provide a `PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT` in the `contact_notices` field of the request."]
    PublicContactData,
    #[serde(rename = "PRIVATE_CONTACT_DATA")]
    #[doc = "None of the data from `ContactSettings` is publicly available. Instead, proxy contact data is published for your domain. Email sent to the proxy email address is forwarded to the registrant's email address. Cloud Domains provides this privacy proxy service at no additional cost."]
    PrivateContactData,
    #[serde(rename = "REDACTED_CONTACT_DATA")]
    #[doc = "Some data from `ContactSettings` is publicly available. The actual information redacted depends on the domain. For details, see [the registration privacy article](https://support.google.com/domains/answer/3251242)."]
    RedactedContactData,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the `ResetAuthorizationCode` method."]
pub struct ResetAuthorizationCodeRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `RetrieveRegisterParameters` method."]
pub struct RetrieveRegisterParametersResponse {
    #[serde(rename = "registerParameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameters to use when calling the `RegisterDomain` method."]
    pub register_parameters: ::std::option::Option<::std::boxed::Box<RegisterParameters>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `SearchDomains` method."]
pub struct SearchDomainsResponse {
    #[serde(rename = "registerParameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Results of the domain name search."]
    pub register_parameters:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RegisterParameters>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `SetIamPolicy` method."]
pub struct SetIamPolicyRequest {
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them."]
    pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: \"bindings, etag\"`"]
    pub update_mask: ::std::option::Option<::std::string::String>,
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
#[doc = "Request message for `TestIamPermissions` method."]
pub struct TestIamPermissionsRequest {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of permissions to check for the `resource`. Permissions with wildcards (such as '*' or 'storage.*') are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions)."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for `TestIamPermissions` method."]
pub struct TestIamPermissionsResponse {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is allowed."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
