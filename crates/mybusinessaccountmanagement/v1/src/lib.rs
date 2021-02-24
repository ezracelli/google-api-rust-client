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
                    #[doc = "Optional. A filter constraining the accounts to return. The response includes only entries that match the filter. If `filter` is empty, then no constraints are applied and all accounts (paginated) are retrieved for the requested account. For example, a request with the filter `type=USER_GROUP` will only return user groups. The `type` field is the only supported filter."]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. How many accounts to fetch per page. The minimum supported page_size is 2. The default and maximum is 20."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. If specified, the next page of accounts is retrieved. The `pageToken` is returned when a call to `accounts.list` returns more results than can fit into the requested page size."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "parentAccount")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. The resource name of the account for which the list of directly accessible accounts is to be retrieved. This only makes sense for Organizations and User Groups. If empty, will return `ListAccounts` for the authenticated user. `accounts/{account_id}`."]
                    pub parent_account: ::std::option::Option<::std::string::String>,
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "updateMask")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. The specific fields that should be updated. The only editable field is `accountName`."]
                    pub update_mask: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "validateOnly")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. If true, the request is validated without actually updating the account."]
                    pub validate_only: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
        pub mod resources {
            pub mod admins {
                pub mod methods {
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
                            #[doc = "Required. The specific fields that should be updated. The only editable field is role."]
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
            pub mod invitations {
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
                            #[doc = "Optional. Filtering the response is supported via the Invitation.target_type field."]
                            pub filter: ::std::option::Option<::std::string::String>,
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
    pub mod locations {
        pub mod resources {
            pub mod admins {
                pub mod methods {
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
                            #[doc = "Required. The specific fields that should be updated. The only editable field is role."]
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
        }
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for AccessControl.AcceptInvitation."]
    pub struct AcceptInvitationRequest {}
    impl AcceptInvitationRequest {
        pub fn builder() -> AcceptInvitationRequestBuilder {
            AcceptInvitationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An account is a container for your location. If you are the only user who manages locations for your business, you can use your personal Google Account. To share management of locations with multiple users, [create a business account] (https://support.google.com/business/answer/6085339?ref_topic=6085325)."]
    pub struct Account {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the account. For an account of type `PERSONAL`, this is the first and last name of the user account."]
        pub account_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Account reference number if provisioned."]
        pub account_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. The resource name, in the format `accounts/{account_id}`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "organizationInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Additional info for an organization. This is populated only for an organization account."]
        pub organization_info: ::std::option::Option<::std::boxed::Box<OrganizationInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissionLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Specifies the permission level the user has for this account."]
        pub permission_level: ::std::option::Option<AccountPermissionLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryOwner")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. The resource name of the account which will be the primary owner of the account being created. It should be of the form `accounts/{account_id}/`."]
        pub primary_owner: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Specifies the AccountRole of this account."]
        pub role: ::std::option::Option<AccountRoleEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Contains the type of account. Accounts of type PERSONAL and ORGANIZATION cannot be created using this API."]
        pub _type: ::std::option::Option<AccountTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verificationState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If verified, future locations that are created are automatically connected to Google Maps, and have Google+ pages created, without requiring moderation."]
        pub verification_state: ::std::option::Option<AccountVerificationStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vettedState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Indicates whether the account is vetted by Google. A vetted account is able to verify locations via the VETTED_PARTNER method."]
        pub vetted_state: ::std::option::Option<AccountVettedStateEnum>,
    }
    impl Account {
        pub fn builder() -> AccountBuilder {
            AccountBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Specifies the permission level the user has for this account."]
    pub enum AccountPermissionLevelEnum {
        #[serde(rename = "PERMISSION_LEVEL_UNSPECIFIED")]
        #[doc = "Not specified."]
        PermissionLevelUnspecified,
        #[serde(rename = "OWNER_LEVEL")]
        #[doc = "The user has owner level permission."]
        OwnerLevel,
        #[serde(rename = "MEMBER_LEVEL")]
        #[doc = "The user has member level permission."]
        MemberLevel,
    }
    impl ::std::default::Default for AccountPermissionLevelEnum {
        fn default() -> Self {
            Self::PermissionLevelUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Specifies the AccountRole of this account."]
    pub enum AccountRoleEnum {
        #[serde(rename = "ACCOUNT_ROLE_UNSPECIFIED")]
        #[doc = "Not specified."]
        AccountRoleUnspecified,
        #[serde(rename = "PRIMARY_OWNER")]
        #[doc = "The user is the primary owner this account."]
        PrimaryOwner,
        #[serde(rename = "OWNER")]
        #[doc = "The user owner of the account."]
        Owner,
        #[serde(rename = "MANAGER")]
        #[doc = "The user can manage this account."]
        Manager,
        #[serde(rename = "SITE_MANAGER")]
        #[doc = "The user can manage a limited set of features for the account."]
        SiteManager,
    }
    impl ::std::default::Default for AccountRoleEnum {
        fn default() -> Self {
            Self::AccountRoleUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Contains the type of account. Accounts of type PERSONAL and ORGANIZATION cannot be created using this API."]
    pub enum AccountTypeEnum {
        #[serde(rename = "ACCOUNT_TYPE_UNSPECIFIED")]
        #[doc = "Not specified."]
        AccountTypeUnspecified,
        #[serde(rename = "PERSONAL")]
        #[doc = "An end-user account."]
        Personal,
        #[serde(rename = "LOCATION_GROUP")]
        #[doc = "A group of Locations. For more information, see the [help center article] (https://support.google.com/business/answer/6085326)"]
        LocationGroup,
        #[serde(rename = "USER_GROUP")]
        #[doc = "A User Group for segregating organization staff in groups. For more information, see the [help center article](https://support.google.com/business/answer/7655731)"]
        UserGroup,
        #[serde(rename = "ORGANIZATION")]
        #[doc = "An organization representing a company. For more information, see the [help center article](https://support.google.com/business/answer/7663063)"]
        Organization,
    }
    impl ::std::default::Default for AccountTypeEnum {
        fn default() -> Self {
            Self::AccountTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. If verified, future locations that are created are automatically connected to Google Maps, and have Google+ pages created, without requiring moderation."]
    pub enum AccountVerificationStateEnum {
        #[serde(rename = "VERIFICATION_STATE_UNSPECIFIED")]
        #[doc = "Not specified."]
        VerificationStateUnspecified,
        #[serde(rename = "VERIFIED")]
        #[doc = "Verified account."]
        Verified,
        #[serde(rename = "UNVERIFIED")]
        #[doc = "Account that is not verified, and verification has not been requested."]
        Unverified,
        #[serde(rename = "VERIFICATION_REQUESTED")]
        #[doc = "Account that is not verified, but verification has been requested."]
        VerificationRequested,
    }
    impl ::std::default::Default for AccountVerificationStateEnum {
        fn default() -> Self {
            Self::VerificationStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Indicates whether the account is vetted by Google. A vetted account is able to verify locations via the VETTED_PARTNER method."]
    pub enum AccountVettedStateEnum {
        #[serde(rename = "VETTED_STATE_UNSPECIFIED")]
        #[doc = "Not Specified"]
        VettedStateUnspecified,
        #[serde(rename = "NOT_VETTED")]
        #[doc = "The account is not vetted by Google."]
        NotVetted,
        #[serde(rename = "VETTED")]
        #[doc = "The account is vetted by Google and in a valid state. An account is automatically vetted if it has direct access to a vetted group account."]
        Vetted,
        #[serde(rename = "INVALID")]
        #[doc = "The account is vetted but in an invalid state. The account will behave like an unvetted account."]
        Invalid,
    }
    impl ::std::default::Default for AccountVettedStateEnum {
        fn default() -> Self {
            Self::VettedStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An administrator of an Account or a location."]
    pub struct Admin {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "admin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The name of the admin. When making the initial invitation, this is the invitee's email address. On `GET` calls, the user's email address is returned if the invitation is still pending. Otherwise, it contains the user's first and last names. This field is only needed to be set during admin creation."]
        pub admin: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. The resource name. For account admins, this is in the form: `accounts/{account_id}/admins/{admin_id}` For location admins, this is in the form: `locations/{location_id}/admins/{admin_id}` This field will be ignored if set during admin creation."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pendingInvitation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Indicates whether this admin has a pending invitation for the specified resource."]
        pub pending_invitation: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Specifies the role that this admin uses with the specified Account or Location."]
        pub role: ::std::option::Option<AdminRoleEnum>,
    }
    impl Admin {
        pub fn builder() -> AdminBuilder {
            AdminBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Specifies the role that this admin uses with the specified Account or Location."]
    pub enum AdminRoleEnum {
        #[serde(rename = "ADMIN_ROLE_UNSPECIFIED")]
        #[doc = "Not specified."]
        AdminRoleUnspecified,
        #[serde(rename = "PRIMARY_OWNER")]
        #[doc = "The admin has owner-level access and is the primary owner. (Displays as 'Primary Owner' in UI)."]
        PrimaryOwner,
        #[serde(rename = "OWNER")]
        #[doc = "The admin has owner-level access. (Displays as 'Owner' in UI)."]
        Owner,
        #[serde(rename = "MANAGER")]
        #[doc = "The admin has managerial access."]
        Manager,
        #[serde(rename = "SITE_MANAGER")]
        #[doc = "The admin can manage social (Google+) pages. (Displays as 'Site Manager' in UI). This API doesn't allow creating an account admin with a SITE_MANAGER role."]
        SiteManager,
    }
    impl ::std::default::Default for AdminRoleEnum {
        fn default() -> Self {
            Self::AdminRoleUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for AccessControl.DeclineInvitation."]
    pub struct DeclineInvitationRequest {}
    impl DeclineInvitationRequest {
        pub fn builder() -> DeclineInvitationRequestBuilder {
            DeclineInvitationRequestBuilder::default()
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
    #[doc = "Represents a pending invitation."]
    pub struct Invitation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The resource name for the invitation. `accounts/{account_id}/invitations/{invitation_id}`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The invited role on the account."]
        pub role: ::std::option::Option<InvitationRoleEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetAccount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sparsely populated account this invitation is for."]
        pub target_account: ::std::option::Option<::std::boxed::Box<Account>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target location this invitation is for."]
        pub target_location: ::std::option::Option<::std::boxed::Box<TargetLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Specifies which target types should appear in the response."]
        pub target_type: ::std::option::Option<InvitationTargetTypeEnum>,
    }
    impl Invitation {
        pub fn builder() -> InvitationBuilder {
            InvitationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The invited role on the account."]
    pub enum InvitationRoleEnum {
        #[serde(rename = "ADMIN_ROLE_UNSPECIFIED")]
        #[doc = "Not specified."]
        AdminRoleUnspecified,
        #[serde(rename = "PRIMARY_OWNER")]
        #[doc = "The admin has owner-level access and is the primary owner. (Displays as 'Primary Owner' in UI)."]
        PrimaryOwner,
        #[serde(rename = "OWNER")]
        #[doc = "The admin has owner-level access. (Displays as 'Owner' in UI)."]
        Owner,
        #[serde(rename = "MANAGER")]
        #[doc = "The admin has managerial access."]
        Manager,
        #[serde(rename = "SITE_MANAGER")]
        #[doc = "The admin can manage social (Google+) pages. (Displays as 'Site Manager' in UI). This API doesn't allow creating an account admin with a SITE_MANAGER role."]
        SiteManager,
    }
    impl ::std::default::Default for InvitationRoleEnum {
        fn default() -> Self {
            Self::AdminRoleUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Specifies which target types should appear in the response."]
    pub enum InvitationTargetTypeEnum {
        #[serde(rename = "TARGET_TYPE_UNSPECIFIED")]
        #[doc = "Set when target type is unspecified."]
        TargetTypeUnspecified,
        #[serde(rename = "ACCOUNTS_ONLY")]
        #[doc = "List invitations only for targets of type Account."]
        AccountsOnly,
        #[serde(rename = "LOCATIONS_ONLY")]
        #[doc = "List invitations only for targets of type Location."]
        LocationsOnly,
    }
    impl ::std::default::Default for InvitationTargetTypeEnum {
        fn default() -> Self {
            Self::TargetTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for AccessControl.ListAccountAdmins."]
    pub struct ListAccountAdminsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountAdmins")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A collection of Admin instances."]
        pub account_admins: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Admin>>>,
    }
    impl ListAccountAdminsResponse {
        pub fn builder() -> ListAccountAdminsResponseBuilder {
            ListAccountAdminsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for Accounts.ListAccounts."]
    pub struct ListAccountsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A collection of accounts to which the user has access. The personal account of the user doing the query will always be the first item of the result, unless it is filtered out."]
        pub accounts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Account>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the number of accounts exceeds the requested page size, this field is populated with a token to fetch the next page of accounts on a subsequent call to `accounts.list`. If there are no more accounts, this field is not present in the response."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListAccountsResponse {
        pub fn builder() -> ListAccountsResponseBuilder {
            ListAccountsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for AccessControl.ListInvitations."]
    pub struct ListInvitationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invitations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A collection of invitations that are pending for the account. The number of invitations listed here cannot exceed 1000."]
        pub invitations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Invitation>>>,
    }
    impl ListInvitationsResponse {
        pub fn builder() -> ListInvitationsResponseBuilder {
            ListInvitationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for AccessControl.ListLocationAdmins."]
    pub struct ListLocationAdminsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "admins")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A collection of Admins."]
        pub admins: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Admin>>>,
    }
    impl ListLocationAdminsResponse {
        pub fn builder() -> ListLocationAdminsResponseBuilder {
            ListLocationAdminsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional information stored for an organization."]
    pub struct OrganizationInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The postal address for the account."]
        pub address: ::std::option::Option<::std::boxed::Box<PostalAddress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The contact number for the organization."]
        pub phone_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "registeredDomain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The registered domain for the account."]
        pub registered_domain: ::std::option::Option<::std::string::String>,
    }
    impl OrganizationInfo {
        pub fn builder() -> OrganizationInfoBuilder {
            OrganizationInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a postal address, e.g. for postal delivery or payments addresses. Given a postal address, a postal service can deliver items to a premise, P.O. Box or similar. It is not intended to model geographical locations (roads, towns, mountains). In typical usage an address would be created via user input or from importing existing data, depending on the type of process. Advice on address input / editing: - Use an i18n-ready address widget such as https://github.com/google/libaddressinput) - Users should not be presented with UI elements for input or editing of fields outside countries where that field is used. For more guidance on how to use this schema, please see: https://support.google.com/business/answer/6397478"]
    pub struct PostalAddress {
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
    impl PostalAddress {
        pub fn builder() -> PostalAddressBuilder {
            PostalAddressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a target location for a pending invitation."]
    pub struct TargetLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The address of the location to which the user is invited."]
        pub address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the location to which the user is invited."]
        pub location_name: ::std::option::Option<::std::string::String>,
    }
    impl TargetLocation {
        pub fn builder() -> TargetLocationBuilder {
            TargetLocationBuilder::default()
        }
    }
}
