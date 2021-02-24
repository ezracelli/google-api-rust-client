#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Change represents a set of ResourceRecordSet additions and deletions applied atomically to a ManagedZone. ResourceRecordSets within a ManagedZone are modified by creating a new Change element in the Changes collection. In turn the Changes collection also records the past modifications to the ResourceRecordSets in a ManagedZone. The current state of the ManagedZone is the sum effect of applying all Change elements in the Changes collection in sequence."]
pub struct Change {
    #[serde(rename = "additions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Which ResourceRecordSets to add?"]
    pub additions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResourceRecordSet>>>,
    #[serde(rename = "deletions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Which ResourceRecordSets to remove? Must match existing data exactly."]
    pub deletions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResourceRecordSet>>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier for the resource; defined by the server (output only)."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isServing")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the DNS queries for the zone will be served."]
    pub is_serving: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(default = "change_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time that this operation was started by the server (output only). This is in RFC3339 text format."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the operation (output only). A status of \"done\" means that the request to update the authoritative servers has been sent but the servers might not be updated yet."]
    pub status: ::std::option::Option<ChangeStatusEnum>,
}
mod change_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#change")
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Status of the operation (output only). A status of \"done\" means that the request to update the authoritative servers has been sent but the servers might not be updated yet."]
pub enum ChangeStatusEnum {
    #[serde(rename = "pending")]
    #[doc = ""]
    Pending,
    #[serde(rename = "done")]
    #[doc = ""]
    Done,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response to a request to enumerate Changes to a ResourceRecordSets collection."]
pub struct ChangesListResponse {
    #[serde(rename = "changes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The requested changes."]
    pub changes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Change>>>,
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
    #[serde(rename = "kind")]
    #[serde(default = "changes_list_response_defaults :: kind")]
    #[doc = "Type of resource."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your pagination token. In this way you can retrieve the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. There is no way to retrieve a \"snapshot\" of collections larger than the maximum page size."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod changes_list_response_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#changesListResponse")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A DNSSEC key pair."]
pub struct DnsKey {
    #[serde(rename = "algorithm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "String mnemonic specifying the DNSSEC algorithm of this key. Immutable after creation time."]
    pub algorithm: ::std::option::Option<DnsKeyAlgorithmEnum>,
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time that this resource was created in the control plane. This is in RFC3339 text format. Output only."]
    pub creation_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the resource's function."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "digests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cryptographic hashes of the DNSKEY resource record associated with this DnsKey. These digests are needed to construct a DS record that points at this DNS key. Output only."]
    pub digests: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DnsKeyDigest>>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier for the resource; defined by the server (output only)."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Active keys are used to sign subsequent changes to the ManagedZone. Inactive keys will still be present as DNSKEY Resource Records for the use of resolvers validating existing signatures."]
    pub is_active: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "keyLength")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Length of the key in bits. Specified at creation time, then immutable."]
    pub key_length: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "keyTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The key tag is a non-cryptographic hash of the a DNSKEY resource record associated with this DnsKey. The key tag can be used to identify a DNSKEY more quickly (but it is not a unique identifier). In particular, the key tag is used in a parent zone's DS record to point at the DNSKEY in this child ManagedZone. The key tag is a number in the range [0, 65535] and the algorithm to calculate it is specified in RFC4034 Appendix B. Output only."]
    pub key_tag: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "kind")]
    #[serde(default = "dns_key_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "publicKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Base64 encoded public half of this key. Output only."]
    pub public_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "One of \"KEY_SIGNING\" or \"ZONE_SIGNING\". Keys of type KEY_SIGNING have the Secure Entry Point flag set and, when active, are used to sign only resource record sets of type DNSKEY. Otherwise, the Secure Entry Point flag is cleared and this key is used to sign only resource record sets of other types. Immutable after creation time."]
    pub _type: ::std::option::Option<DnsKeyTypeEnum>,
}
mod dns_key_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#dnsKey")
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "String mnemonic specifying the DNSSEC algorithm of this key. Immutable after creation time."]
pub enum DnsKeyAlgorithmEnum {
    #[serde(rename = "rsasha1")]
    #[doc = ""]
    Rsasha1,
    #[serde(rename = "rsasha256")]
    #[doc = ""]
    Rsasha256,
    #[serde(rename = "rsasha512")]
    #[doc = ""]
    Rsasha512,
    #[serde(rename = "ecdsap256sha256")]
    #[doc = ""]
    Ecdsap256sha256,
    #[serde(rename = "ecdsap384sha384")]
    #[doc = ""]
    Ecdsap384sha384,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "One of \"KEY_SIGNING\" or \"ZONE_SIGNING\". Keys of type KEY_SIGNING have the Secure Entry Point flag set and, when active, are used to sign only resource record sets of type DNSKEY. Otherwise, the Secure Entry Point flag is cleared and this key is used to sign only resource record sets of other types. Immutable after creation time."]
pub enum DnsKeyTypeEnum {
    #[serde(rename = "keySigning")]
    #[doc = ""]
    KeySigning,
    #[serde(rename = "zoneSigning")]
    #[doc = ""]
    ZoneSigning,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DnsKeyDigest {
    #[serde(rename = "digest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The base-16 encoded bytes of this digest. Suitable for use in a DS resource record."]
    pub digest: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the algorithm used to calculate this digest."]
    pub _type: ::std::option::Option<DnsKeyDigestTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Specifies the algorithm used to calculate this digest."]
pub enum DnsKeyDigestTypeEnum {
    #[serde(rename = "sha1")]
    #[doc = ""]
    Sha1,
    #[serde(rename = "sha256")]
    #[doc = ""]
    Sha256,
    #[serde(rename = "sha384")]
    #[doc = ""]
    Sha384,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Parameters for DnsKey key generation. Used for generating initial keys for a new ManagedZone and as default when adding a new DnsKey."]
pub struct DnsKeySpec {
    #[serde(rename = "algorithm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "String mnemonic specifying the DNSSEC algorithm of this key."]
    pub algorithm: ::std::option::Option<DnsKeySpecAlgorithmEnum>,
    #[serde(rename = "keyLength")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Length of the keys in bits."]
    pub key_length: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "keyType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies whether this is a key signing key (KSK) or a zone signing key (ZSK). Key signing keys have the Secure Entry Point flag set and, when active, are only used to sign resource record sets of type DNSKEY. Zone signing keys do not have the Secure Entry Point flag set and are used to sign all other types of resource record sets."]
    pub key_type: ::std::option::Option<DnsKeySpecKeyTypeEnum>,
    #[serde(rename = "kind")]
    #[serde(default = "dns_key_spec_defaults :: kind")]
    pub kind: ::std::string::String,
}
mod dns_key_spec_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#dnsKeySpec")
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "String mnemonic specifying the DNSSEC algorithm of this key."]
pub enum DnsKeySpecAlgorithmEnum {
    #[serde(rename = "rsasha1")]
    #[doc = ""]
    Rsasha1,
    #[serde(rename = "rsasha256")]
    #[doc = ""]
    Rsasha256,
    #[serde(rename = "rsasha512")]
    #[doc = ""]
    Rsasha512,
    #[serde(rename = "ecdsap256sha256")]
    #[doc = ""]
    Ecdsap256sha256,
    #[serde(rename = "ecdsap384sha384")]
    #[doc = ""]
    Ecdsap384sha384,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Specifies whether this is a key signing key (KSK) or a zone signing key (ZSK). Key signing keys have the Secure Entry Point flag set and, when active, are only used to sign resource record sets of type DNSKEY. Zone signing keys do not have the Secure Entry Point flag set and are used to sign all other types of resource record sets."]
pub enum DnsKeySpecKeyTypeEnum {
    #[serde(rename = "keySigning")]
    #[doc = ""]
    KeySigning,
    #[serde(rename = "zoneSigning")]
    #[doc = ""]
    ZoneSigning,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response to a request to enumerate DnsKeys in a ManagedZone."]
pub struct DnsKeysListResponse {
    #[serde(rename = "dnsKeys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The requested resources."]
    pub dns_keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DnsKey>>>,
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
    #[serde(rename = "kind")]
    #[serde(default = "dns_keys_list_response_defaults :: kind")]
    #[doc = "Type of resource."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your pagination token. In this way you can retrieve the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. There is no way to retrieve a \"snapshot\" of collections larger than the maximum page size."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod dns_keys_list_response_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#dnsKeysListResponse")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A zone is a subtree of the DNS namespace under one administrative responsibility. A ManagedZone is a resource that represents a DNS zone hosted by the Cloud DNS service."]
pub struct ManagedZone {
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time that this resource was created on the server. This is in RFC3339 text format. Output only."]
    pub creation_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the managed zone's function."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dnsName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The DNS name of this managed zone, for instance \"example.com.\"."]
    pub dns_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dnssecConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DNSSEC configuration."]
    pub dnssec_config: ::std::option::Option<::std::boxed::Box<ManagedZoneDnsSecConfig>>,
    #[serde(rename = "forwardingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The presence for this field indicates that outbound forwarding is enabled for this zone. The value of this field contains the set of destinations to forward to."]
    pub forwarding_config: ::std::option::Option<::std::boxed::Box<ManagedZoneForwardingConfig>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier for the resource; defined by the server (output only)"]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "managed_zone_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User labels."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User assigned name for this resource. Must be unique within the project. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nameServerSet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optionally specifies the NameServerSet for this ManagedZone. A NameServerSet is a set of DNS name servers that all host the same ManagedZones. Most users will leave this field unset. If you need to use this field, please reach out to your account team."]
    pub name_server_set: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nameServers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Delegate your managed_zone to these virtual name servers; defined by the server (output only)"]
    pub name_servers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "peeringConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The presence of this field indicates that DNS Peering is enabled for this zone. The value of this field contains the network to peer with."]
    pub peering_config: ::std::option::Option<::std::boxed::Box<ManagedZonePeeringConfig>>,
    #[serde(rename = "privateVisibilityConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For privately visible zones, the set of Virtual Private Cloud resources that the zone is visible from."]
    pub private_visibility_config:
        ::std::option::Option<::std::boxed::Box<ManagedZonePrivateVisibilityConfig>>,
    #[serde(rename = "reverseLookupConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The presence of this field indicates that this is a managed reverse lookup zone and Cloud DNS resolves reverse lookup queries using automatically configured records for VPC resources. This only applies to networks listed under private_visibility_config."]
    pub reverse_lookup_config:
        ::std::option::Option<::std::boxed::Box<ManagedZoneReverseLookupConfig>>,
    #[serde(rename = "serviceDirectoryConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field links to the associated service directory namespace. This field should not be set for public zones or forwarding zones."]
    pub service_directory_config:
        ::std::option::Option<::std::boxed::Box<ManagedZoneServiceDirectoryConfig>>,
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources."]
    pub visibility: ::std::option::Option<ManagedZoneVisibilityEnum>,
}
mod managed_zone_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#managedZone")
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources."]
pub enum ManagedZoneVisibilityEnum {
    #[serde(rename = "public")]
    #[doc = ""]
    Public,
    #[serde(rename = "private")]
    #[doc = ""]
    Private,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedZoneDnsSecConfig {
    #[serde(rename = "defaultKeySpecs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies parameters for generating initial DnsKeys for this ManagedZone. Can only be changed while the state is OFF."]
    pub default_key_specs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DnsKeySpec>>>,
    #[serde(rename = "kind")]
    #[serde(default = "managed_zone_dns_sec_config_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "nonExistence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the mechanism for authenticated denial-of-existence responses. Can only be changed while the state is OFF."]
    pub non_existence: ::std::option::Option<ManagedZoneDnsSecConfigNonExistenceEnum>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies whether DNSSEC is enabled, and what mode it is in."]
    pub state: ::std::option::Option<ManagedZoneDnsSecConfigStateEnum>,
}
mod managed_zone_dns_sec_config_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#managedZoneDnsSecConfig")
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Specifies the mechanism for authenticated denial-of-existence responses. Can only be changed while the state is OFF."]
pub enum ManagedZoneDnsSecConfigNonExistenceEnum {
    #[serde(rename = "nsec")]
    #[doc = ""]
    Nsec,
    #[serde(rename = "nsec3")]
    #[doc = ""]
    Nsec3,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Specifies whether DNSSEC is enabled, and what mode it is in."]
pub enum ManagedZoneDnsSecConfigStateEnum {
    #[serde(rename = "off")]
    #[doc = "DNSSEC is disabled; the zone is not signed."]
    Off,
    #[serde(rename = "on")]
    #[doc = "DNSSEC is enabled; the zone is signed and fully managed."]
    On,
    #[serde(rename = "transfer")]
    #[doc = "DNSSEC is enabled, but in a \"transfer\" mode."]
    Transfer,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedZoneForwardingConfig {
    #[serde(rename = "kind")]
    #[serde(default = "managed_zone_forwarding_config_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "targetNameServers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of target name servers to forward to. Cloud DNS selects the best available name server if more than one target is given."]
    pub target_name_servers: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<ManagedZoneForwardingConfigNameServerTarget>>,
    >,
}
mod managed_zone_forwarding_config_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#managedZoneForwardingConfig")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedZoneForwardingConfigNameServerTarget {
    #[serde(rename = "forwardingPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Forwarding path for this NameServerTarget. If unset or set to DEFAULT, Cloud DNS makes forwarding decisions based on address ranges; that is, RFC1918 addresses go to the VPC, non-RFC1918 addresses go to the internet. When set to PRIVATE, Cloud DNS always sends queries through VPC for this target."]
    pub forwarding_path:
        ::std::option::Option<ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum>,
    #[serde(rename = "ipv4Address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IPv4 address of a target name server."]
    pub ipv4_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "managed_zone_forwarding_config_name_server_target_defaults :: kind")]
    pub kind: ::std::string::String,
}
mod managed_zone_forwarding_config_name_server_target_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#managedZoneForwardingConfigNameServerTarget")
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Forwarding path for this NameServerTarget. If unset or set to DEFAULT, Cloud DNS makes forwarding decisions based on address ranges; that is, RFC1918 addresses go to the VPC, non-RFC1918 addresses go to the internet. When set to PRIVATE, Cloud DNS always sends queries through VPC for this target."]
pub enum ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum {
    #[serde(rename = "default")]
    #[doc = "Cloud DNS makes forwarding decisions based on address ranges; that is, RFC1918 addresses forward to the target through the VPC and non-RFC1918 addresses forward to the target through the internet"]
    Default,
    #[serde(rename = "private")]
    #[doc = "Cloud DNS always forwards to this target through the VPC."]
    Private,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedZoneOperationsListResponse {
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
    #[serde(rename = "kind")]
    #[serde(default = "managed_zone_operations_list_response_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your page token. In this way you can retrieve the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. There is no way to retrieve a consistent snapshot of a collection larger than the maximum page size."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The operation resources."]
    pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
}
mod managed_zone_operations_list_response_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#managedZoneOperationsListResponse")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedZonePeeringConfig {
    #[serde(rename = "kind")]
    #[serde(default = "managed_zone_peering_config_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "targetNetwork")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The network with which to peer."]
    pub target_network:
        ::std::option::Option<::std::boxed::Box<ManagedZonePeeringConfigTargetNetwork>>,
}
mod managed_zone_peering_config_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#managedZonePeeringConfig")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedZonePeeringConfigTargetNetwork {
    #[serde(rename = "deactivateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the zone was deactivated, in RFC 3339 date-time format. An empty string indicates that the peering connection is active. The producer network can deactivate a zone. The zone is automatically deactivated if the producer network that the zone targeted is deleted. Output only."]
    pub deactivate_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "managed_zone_peering_config_target_network_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "networkUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fully qualified URL of the VPC network to forward queries to. This should be formatted like https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}"]
    pub network_url: ::std::option::Option<::std::string::String>,
}
mod managed_zone_peering_config_target_network_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#managedZonePeeringConfigTargetNetwork")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedZonePrivateVisibilityConfig {
    #[serde(rename = "kind")]
    #[serde(default = "managed_zone_private_visibility_config_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "networks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of VPC networks that can see this zone."]
    pub networks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<ManagedZonePrivateVisibilityConfigNetwork>>,
    >,
}
mod managed_zone_private_visibility_config_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#managedZonePrivateVisibilityConfig")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedZonePrivateVisibilityConfigNetwork {
    #[serde(rename = "kind")]
    #[serde(default = "managed_zone_private_visibility_config_network_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "networkUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fully qualified URL of the VPC network to bind to. This should be formatted like https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}"]
    pub network_url: ::std::option::Option<::std::string::String>,
}
mod managed_zone_private_visibility_config_network_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#managedZonePrivateVisibilityConfigNetwork")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedZoneReverseLookupConfig {
    #[serde(rename = "kind")]
    #[serde(default = "managed_zone_reverse_lookup_config_defaults :: kind")]
    pub kind: ::std::string::String,
}
mod managed_zone_reverse_lookup_config_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#managedZoneReverseLookupConfig")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about Service Directory-backed zones."]
pub struct ManagedZoneServiceDirectoryConfig {
    #[serde(rename = "kind")]
    #[serde(default = "managed_zone_service_directory_config_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains information about the namespace associated with the zone."]
    pub namespace:
        ::std::option::Option<::std::boxed::Box<ManagedZoneServiceDirectoryConfigNamespace>>,
}
mod managed_zone_service_directory_config_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#managedZoneServiceDirectoryConfig")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedZoneServiceDirectoryConfigNamespace {
    #[serde(rename = "deletionTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time that the namespace backing this zone was deleted, empty string if it still exists. This is in RFC3339 text format. Output only."]
    pub deletion_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "managed_zone_service_directory_config_namespace_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "namespaceUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fully qualified URL of the namespace associated with the zone. This should be formatted like https://servicedirectory.googleapis.com/v1/projects/{project}/locations/{location}/namespaces/{namespace}"]
    pub namespace_url: ::std::option::Option<::std::string::String>,
}
mod managed_zone_service_directory_config_namespace_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#managedZoneServiceDirectoryConfigNamespace")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedZonesListResponse {
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
    #[serde(rename = "kind")]
    #[serde(default = "managed_zones_list_response_defaults :: kind")]
    #[doc = "Type of resource."]
    pub kind: ::std::string::String,
    #[serde(rename = "managedZones")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The managed zone resources."]
    pub managed_zones: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ManagedZone>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your page token. In this way you can retrieve the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. There is no way to retrieve a consistent snapshot of a collection larger than the maximum page size."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod managed_zones_list_response_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#managedZonesListResponse")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An operation represents a successful mutation performed on a Cloud DNS resource. Operations provide: - An audit log of server resource mutations. - A way to recover/retry API calls in the case where the response is never received by the caller. Use the caller specified client_operation_id."]
pub struct Operation {
    #[serde(rename = "dnsKeyContext")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only populated if the operation targeted a DnsKey (output only)."]
    pub dns_key_context: ::std::option::Option<::std::boxed::Box<OperationDnsKeyContext>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier for the resource. This is the client_operation_id if the client specified it when the mutation was initiated, otherwise, it is generated by the server. The name must be 1-63 characters long and match the regular expression [-a-z0-9]? (output only)"]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "operation_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time that this operation was started by the server. This is in RFC3339 text format (output only)."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the operation. Can be one of the following: \"PENDING\" or \"DONE\" (output only). A status of \"DONE\" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet."]
    pub status: ::std::option::Option<OperationStatusEnum>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the operation. Operations include insert, update, and delete (output only)."]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User who requested the operation, for example: user@example.com. cloud-dns-system for operations automatically done by the system. (output only)"]
    pub user: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zoneContext")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only populated if the operation targeted a ManagedZone (output only)."]
    pub zone_context: ::std::option::Option<::std::boxed::Box<OperationManagedZoneContext>>,
}
mod operation_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#operation")
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Status of the operation. Can be one of the following: \"PENDING\" or \"DONE\" (output only). A status of \"DONE\" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet."]
pub enum OperationStatusEnum {
    #[serde(rename = "pending")]
    #[doc = ""]
    Pending,
    #[serde(rename = "done")]
    #[doc = ""]
    Done,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationDnsKeyContext {
    #[serde(rename = "newValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The post-operation DnsKey resource."]
    pub new_value: ::std::option::Option<::std::boxed::Box<DnsKey>>,
    #[serde(rename = "oldValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pre-operation DnsKey resource."]
    pub old_value: ::std::option::Option<::std::boxed::Box<DnsKey>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationManagedZoneContext {
    #[serde(rename = "newValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The post-operation ManagedZone resource."]
    pub new_value: ::std::option::Option<::std::boxed::Box<ManagedZone>>,
    #[serde(rename = "oldValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pre-operation ManagedZone resource."]
    pub old_value: ::std::option::Option<::std::boxed::Box<ManagedZone>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PoliciesListResponse {
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
    #[serde(rename = "kind")]
    #[serde(default = "policies_list_response_defaults :: kind")]
    #[doc = "Type of resource."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your page token. In this way you can retrieve the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. There is no way to retrieve a consistent snapshot of a collection larger than the maximum page size."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The policy resources."]
    pub policies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Policy>>>,
}
mod policies_list_response_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#policiesListResponse")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PoliciesPatchResponse {
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PoliciesUpdateResponse {
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A policy is a collection of DNS rules applied to one or more Virtual Private Cloud resources."]
pub struct Policy {
    #[serde(rename = "alternativeNameServerConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sets an alternative name server for the associated networks. When specified, all DNS queries are forwarded to a name server that you choose. Names such as .internal are not available when an alternative name server is specified."]
    pub alternative_name_server_config:
        ::std::option::Option<::std::boxed::Box<PolicyAlternativeNameServerConfig>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the policy's function."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enableInboundForwarding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allows networks bound to this policy to receive DNS queries sent by VMs or applications over VPN connections. When enabled, a virtual IP address is allocated from each of the sub-networks that are bound to this policy."]
    pub enable_inbound_forwarding: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enableLogging")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Controls whether logging is enabled for the networks bound to this policy. Defaults to no logging if not set."]
    pub enable_logging: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier for the resource; defined by the server (output only)."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "policy_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-assigned name for this policy."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "networks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of network names specifying networks to which this policy is applied."]
    pub networks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PolicyNetwork>>>,
}
mod policy_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#policy")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyAlternativeNameServerConfig {
    #[serde(rename = "kind")]
    #[serde(default = "policy_alternative_name_server_config_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "targetNameServers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sets an alternative name server for the associated networks. When specified, all DNS queries are forwarded to a name server that you choose. Names such as .internal are not available when an alternative name server is specified."]
    pub target_name_servers: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<PolicyAlternativeNameServerConfigTargetNameServer>>,
    >,
}
mod policy_alternative_name_server_config_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#policyAlternativeNameServerConfig")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyAlternativeNameServerConfigTargetNameServer {
    #[serde(rename = "forwardingPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Forwarding path for this TargetNameServer. If unset or set to DEFAULT, Cloud DNS makes forwarding decision based on address ranges; that is, RFC1918 addresses go to the VPC, non-RFC1918 addresses go to the internet. When set to PRIVATE, Cloud DNS always sends queries through VPC for this target."]
    pub forwarding_path:
        ::std::option::Option<PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum>,
    #[serde(rename = "ipv4Address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IPv4 address to forward to."]
    pub ipv4_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "policy_alternative_name_server_config_target_name_server_defaults :: kind")]
    pub kind: ::std::string::String,
}
mod policy_alternative_name_server_config_target_name_server_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#policyAlternativeNameServerConfigTargetNameServer")
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Forwarding path for this TargetNameServer. If unset or set to DEFAULT, Cloud DNS makes forwarding decision based on address ranges; that is, RFC1918 addresses go to the VPC, non-RFC1918 addresses go to the internet. When set to PRIVATE, Cloud DNS always sends queries through VPC for this target."]
pub enum PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum {
    #[serde(rename = "default")]
    #[doc = "Cloud DNS will make forwarding decision based on address ranges; that is, RFC1918 addresses forward to the target through the VPC and non-RFC1918 addresses forward to the target through the internet"]
    Default,
    #[serde(rename = "private")]
    #[doc = "Cloud DNS will always forward to this target through the VPC."]
    Private,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyNetwork {
    #[serde(rename = "kind")]
    #[serde(default = "policy_network_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "networkUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fully qualified URL of the VPC network to bind to. This should be formatted like https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}"]
    pub network_url: ::std::option::Option<::std::string::String>,
}
mod policy_network_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#policyNetwork")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A project resource. The project is a top level container for resources including Cloud DNS ManagedZones. Projects can be created only in the APIs console."]
pub struct Project {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User assigned unique identifier for the resource (output only)."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "project_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "number")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique numeric identifier for the resource; defined by the server (output only)."]
    pub number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quota")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Quotas assigned to this project (output only)."]
    pub quota: ::std::option::Option<::std::boxed::Box<Quota>>,
}
mod project_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#project")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Limits associated with a Project."]
pub struct Quota {
    #[serde(rename = "dnsKeysPerManagedZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum allowed number of DnsKeys per ManagedZone."]
    pub dns_keys_per_managed_zone: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "kind")]
    #[serde(default = "quota_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "managedZones")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum allowed number of managed zones in the project."]
    pub managed_zones: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "managedZonesPerNetwork")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum allowed number of managed zones which can be attached to a network."]
    pub managed_zones_per_network: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "networksPerManagedZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum allowed number of networks to which a privately scoped zone can be attached."]
    pub networks_per_managed_zone: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "networksPerPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum allowed number of networks per policy."]
    pub networks_per_policy: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum allowed number of policies per project."]
    pub policies: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "resourceRecordsPerRrset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum allowed number of ResourceRecords per ResourceRecordSet."]
    pub resource_records_per_rrset: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "rrsetAdditionsPerChange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum allowed number of ResourceRecordSets to add per ChangesCreateRequest."]
    pub rrset_additions_per_change: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "rrsetDeletionsPerChange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum allowed number of ResourceRecordSets to delete per ChangesCreateRequest."]
    pub rrset_deletions_per_change: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "rrsetsPerManagedZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum allowed number of ResourceRecordSets per zone in the project."]
    pub rrsets_per_managed_zone: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "targetNameServersPerManagedZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum allowed number of target name servers per managed forwarding zone."]
    pub target_name_servers_per_managed_zone: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "targetNameServersPerPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum allowed number of alternative target name servers per policy."]
    pub target_name_servers_per_policy: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "totalRrdataSizePerChange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum allowed size for total rrdata in one ChangesCreateRequest in bytes."]
    pub total_rrdata_size_per_change: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "whitelistedKeySpecs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DNSSEC algorithm and key length types that can be used for DnsKeys."]
    pub whitelisted_key_specs:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DnsKeySpec>>>,
}
mod quota_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#quota")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A unit of data that will be returned by the DNS servers."]
pub struct ResourceRecordSet {
    #[serde(rename = "kind")]
    #[serde(default = "resource_record_set_defaults :: kind")]
    pub kind: ::std::string::String,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For example, www.example.com."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rrdatas")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "As defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1) -- see examples."]
    pub rrdatas: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "signatureRrdatas")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "As defined in RFC 4034 (section 3.2)."]
    pub signature_rrdatas: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "ttl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of seconds that this ResourceRecordSet can be cached by resolvers."]
    pub ttl: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of a supported record type. See the list of Supported DNS record types."]
    pub _type: ::std::option::Option<::std::string::String>,
}
mod resource_record_set_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#resourceRecordSet")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceRecordSetsListResponse {
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
    #[serde(rename = "kind")]
    #[serde(default = "resource_record_sets_list_response_defaults :: kind")]
    #[doc = "Type of resource."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your pagination token. This lets you retrieve complete contents of even larger collections, one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of elements returned are an inconsistent view of the collection. You cannot retrieve a consistent snapshot of a collection larger than the maximum page size."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rrsets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource record set resources."]
    pub rrsets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResourceRecordSet>>>,
}
mod resource_record_sets_list_response_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("dns#resourceRecordSetsListResponse")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Elements common to every response."]
pub struct ResponseHeader {
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For mutating operation requests that completed successfully. This is the client_operation_id if the client specified it, otherwise it is generated by the server (output only)."]
    pub operation_id: ::std::option::Option<::std::string::String>,
}
