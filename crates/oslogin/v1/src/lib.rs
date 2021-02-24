#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response message for importing an SSH public key."]
pub struct ImportSshPublicKeyResponse {
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detailed information about import results."]
    pub details: ::std::option::Option<::std::string::String>,
    #[serde(rename = "loginProfile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The login profile information for the user."]
    pub login_profile: ::std::option::Option<::std::boxed::Box<LoginProfile>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The user profile information used for logging in to a virtual machine on Google Compute Engine."]
pub struct LoginProfile {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A unique user ID."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "posixAccounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of POSIX accounts associated with the user."]
    pub posix_accounts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PosixAccount>>>,
    #[serde(rename = "sshPublicKeys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A map from SSH public key fingerprint to the associated key object."]
    pub ssh_public_keys: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<SshPublicKey>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The POSIX account information associated with a Google account."]
pub struct PosixAccount {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A POSIX account identifier."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gecos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The GECOS (user information) entry for this account."]
    pub gecos: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default group ID."]
    pub gid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "homeDirectory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The path to the home directory for this account."]
    pub home_directory: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The canonical resource name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operatingSystemType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The operating system type where this account applies."]
    pub operating_system_type: ::std::option::Option<PosixAccountOperatingSystemTypeEnum>,
    #[serde(rename = "primary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only one POSIX account can be marked as primary."]
    pub primary: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "shell")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The path to the logic shell for this account."]
    pub shell: ::std::option::Option<::std::string::String>,
    #[serde(rename = "systemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "System identifier for which account the username or uid applies to. By default, the empty value is used."]
    pub system_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user ID."]
    pub uid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The username of the POSIX account."]
    pub username: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The operating system type where this account applies."]
pub enum PosixAccountOperatingSystemTypeEnum {
    #[serde(rename = "OPERATING_SYSTEM_TYPE_UNSPECIFIED")]
    #[doc = "The operating system type associated with the user account information is unspecified."]
    OperatingSystemTypeUnspecified,
    #[serde(rename = "LINUX")]
    #[doc = "Linux user account information."]
    Linux,
    #[serde(rename = "WINDOWS")]
    #[doc = "Windows user account information."]
    Windows,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The SSH public key information associated with a Google account."]
pub struct SshPublicKey {
    #[serde(rename = "expirationTimeUsec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An expiration time in microseconds since epoch."]
    pub expiration_time_usec: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The SHA-256 fingerprint of the SSH public key."]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Public key text in SSH format, defined by RFC4253 section 6.6."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The canonical resource name."]
    pub name: ::std::option::Option<::std::string::String>,
}
