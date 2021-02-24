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
    pub mod divisions {
        pub mod methods {
            pub mod search {
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
                    #[serde(rename = "query")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The search query. Queries can cover any parts of a OCD ID or a human readable division name. All words given in the query are treated as required patterns. In addition to that, most query operators of the Apache Lucene library are supported. See http://lucene.apache.org/core/2_9_4/queryparsersyntax.html"]
                    pub query: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod elections {
        pub mod methods {
            pub mod voter_info_query {
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
                    #[serde(rename = "address")]
                    #[doc = "The registered address of the voter to look up."]
                    pub address: ::std::string::String,
                    #[builder(
                        default = "{ query_parameters_defaults :: election_id () }",
                        setter(into)
                    )]
                    #[serde(rename = "electionId")]
                    #[serde(default = "query_parameters_defaults :: election_id")]
                    #[doc = "The unique ID of the election to look up. A list of election IDs can be obtained at https://www.googleapis.com/civicinfo/{version}/elections. If no election ID is specified in the query and there is more than one election with data for the given voter, the additional elections are provided in the otherElections response field."]
                    pub election_id: ::std::string::String,
                    #[builder(
                        default = "{ query_parameters_defaults :: official_only () }",
                        setter(into)
                    )]
                    #[serde(rename = "officialOnly")]
                    #[serde(default = "query_parameters_defaults :: official_only")]
                    #[doc = "If set to true, only data from official state sources will be returned."]
                    pub official_only: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: return_all_available_data () }",
                        setter(into)
                    )]
                    #[serde(rename = "returnAllAvailableData")]
                    #[serde(default = "query_parameters_defaults :: return_all_available_data")]
                    #[doc = "If set to true, the query will return the success code and include any partial information when it is unable to determine a matching address or unable to determine the election for electionId=0 queries."]
                    pub return_all_available_data: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn election_id() -> ::std::string::String {
                        String::from("0")
                    }
                    pub fn official_only() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn return_all_available_data() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
            }
        }
    }
    pub mod representatives {
        pub mod methods {
            pub mod representative_info_by_address {
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
                    #[serde(rename = "address")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The address to look up. May only be specified if the field ocdId is not given in the URL"]
                    pub address: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: include_offices () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeOffices")]
                    #[serde(default = "query_parameters_defaults :: include_offices")]
                    #[doc = "Whether to return information about offices and officials. If false, only the top-level district information will be returned."]
                    pub include_offices: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "levels")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A list of office levels to filter by. Only offices that serve at least one of these levels will be returned. Divisions that don't contain a matching office will not be returned."]
                    pub levels: ::std::option::Option<QueryParametersLevelsEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "roles")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A list of office roles to filter by. Only offices fulfilling one of these roles will be returned. Divisions that don't contain a matching office will not be returned."]
                    pub roles: ::std::option::Option<QueryParametersRolesEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn include_offices() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "A list of office levels to filter by. Only offices that serve at least one of these levels will be returned. Divisions that don't contain a matching office will not be returned."]
                pub enum QueryParametersLevelsEnum {
                    #[serde(rename = "international")]
                    #[doc = ""]
                    International,
                    #[serde(rename = "country")]
                    #[doc = ""]
                    Country,
                    #[serde(rename = "administrativeArea1")]
                    #[doc = ""]
                    AdministrativeArea1,
                    #[serde(rename = "regional")]
                    #[doc = ""]
                    Regional,
                    #[serde(rename = "administrativeArea2")]
                    #[doc = ""]
                    AdministrativeArea2,
                    #[serde(rename = "locality")]
                    #[doc = ""]
                    Locality,
                    #[serde(rename = "subLocality1")]
                    #[doc = ""]
                    SubLocality1,
                    #[serde(rename = "subLocality2")]
                    #[doc = ""]
                    SubLocality2,
                    #[serde(rename = "special")]
                    #[doc = ""]
                    Special,
                }
                impl ::std::default::Default for QueryParametersLevelsEnum {
                    fn default() -> Self {
                        Self::International
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "A list of office roles to filter by. Only offices fulfilling one of these roles will be returned. Divisions that don't contain a matching office will not be returned."]
                pub enum QueryParametersRolesEnum {
                    #[serde(rename = "headOfState")]
                    #[doc = ""]
                    HeadOfState,
                    #[serde(rename = "headOfGovernment")]
                    #[doc = ""]
                    HeadOfGovernment,
                    #[serde(rename = "deputyHeadOfGovernment")]
                    #[doc = ""]
                    DeputyHeadOfGovernment,
                    #[serde(rename = "governmentOfficer")]
                    #[doc = ""]
                    GovernmentOfficer,
                    #[serde(rename = "executiveCouncil")]
                    #[doc = ""]
                    ExecutiveCouncil,
                    #[serde(rename = "legislatorUpperBody")]
                    #[doc = ""]
                    LegislatorUpperBody,
                    #[serde(rename = "legislatorLowerBody")]
                    #[doc = ""]
                    LegislatorLowerBody,
                    #[serde(rename = "highestCourtJudge")]
                    #[doc = ""]
                    HighestCourtJudge,
                    #[serde(rename = "judge")]
                    #[doc = ""]
                    Judge,
                    #[serde(rename = "schoolBoard")]
                    #[doc = ""]
                    SchoolBoard,
                    #[serde(rename = "specialPurposeOfficer")]
                    #[doc = ""]
                    SpecialPurposeOfficer,
                }
                impl ::std::default::Default for QueryParametersRolesEnum {
                    fn default() -> Self {
                        Self::HeadOfState
                    }
                }
            }
            pub mod representative_info_by_division {
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
                    #[serde(rename = "levels")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A list of office levels to filter by. Only offices that serve at least one of these levels will be returned. Divisions that don't contain a matching office will not be returned."]
                    pub levels: ::std::option::Option<QueryParametersLevelsEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "recursive")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If true, information about all divisions contained in the division requested will be included as well. For example, if querying ocd-division/country:us/district:dc, this would also return all DC's wards and ANCs."]
                    pub recursive: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "roles")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A list of office roles to filter by. Only offices fulfilling one of these roles will be returned. Divisions that don't contain a matching office will not be returned."]
                    pub roles: ::std::option::Option<QueryParametersRolesEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "A list of office levels to filter by. Only offices that serve at least one of these levels will be returned. Divisions that don't contain a matching office will not be returned."]
                pub enum QueryParametersLevelsEnum {
                    #[serde(rename = "international")]
                    #[doc = ""]
                    International,
                    #[serde(rename = "country")]
                    #[doc = ""]
                    Country,
                    #[serde(rename = "administrativeArea1")]
                    #[doc = ""]
                    AdministrativeArea1,
                    #[serde(rename = "regional")]
                    #[doc = ""]
                    Regional,
                    #[serde(rename = "administrativeArea2")]
                    #[doc = ""]
                    AdministrativeArea2,
                    #[serde(rename = "locality")]
                    #[doc = ""]
                    Locality,
                    #[serde(rename = "subLocality1")]
                    #[doc = ""]
                    SubLocality1,
                    #[serde(rename = "subLocality2")]
                    #[doc = ""]
                    SubLocality2,
                    #[serde(rename = "special")]
                    #[doc = ""]
                    Special,
                }
                impl ::std::default::Default for QueryParametersLevelsEnum {
                    fn default() -> Self {
                        Self::International
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "A list of office roles to filter by. Only offices fulfilling one of these roles will be returned. Divisions that don't contain a matching office will not be returned."]
                pub enum QueryParametersRolesEnum {
                    #[serde(rename = "headOfState")]
                    #[doc = ""]
                    HeadOfState,
                    #[serde(rename = "headOfGovernment")]
                    #[doc = ""]
                    HeadOfGovernment,
                    #[serde(rename = "deputyHeadOfGovernment")]
                    #[doc = ""]
                    DeputyHeadOfGovernment,
                    #[serde(rename = "governmentOfficer")]
                    #[doc = ""]
                    GovernmentOfficer,
                    #[serde(rename = "executiveCouncil")]
                    #[doc = ""]
                    ExecutiveCouncil,
                    #[serde(rename = "legislatorUpperBody")]
                    #[doc = ""]
                    LegislatorUpperBody,
                    #[serde(rename = "legislatorLowerBody")]
                    #[doc = ""]
                    LegislatorLowerBody,
                    #[serde(rename = "highestCourtJudge")]
                    #[doc = ""]
                    HighestCourtJudge,
                    #[serde(rename = "judge")]
                    #[doc = ""]
                    Judge,
                    #[serde(rename = "schoolBoard")]
                    #[doc = ""]
                    SchoolBoard,
                    #[serde(rename = "specialPurposeOfficer")]
                    #[doc = ""]
                    SpecialPurposeOfficer,
                }
                impl ::std::default::Default for QueryParametersRolesEnum {
                    fn default() -> Self {
                        Self::HeadOfState
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
    #[doc = "Describes information about a regional election administrative area."]
    pub struct AdministrationRegion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "electionAdministrationBody")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The election administration body for this area."]
        pub election_administration_body:
            ::std::option::Option<::std::boxed::Box<AdministrativeBody>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "local_jurisdiction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The city or county that provides election information for this voter. This object can have the same elements as state."]
        pub local_jurisdiction: ::std::option::Option<::std::boxed::Box<AdministrationRegion>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the jurisdiction."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of sources for this area. If multiple sources are listed the data has been aggregated from those sources."]
        pub sources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Source>>>,
    }
    impl AdministrationRegion {
        pub fn builder() -> AdministrationRegionBuilder {
            AdministrationRegionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about an election administrative body (e.g. County Board of Elections)."]
    pub struct AdministrativeBody {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "absenteeVotingInfoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL provided by this administrative body for information on absentee voting."]
        pub absentee_voting_info_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ballotInfoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL provided by this administrative body to give contest information to the voter."]
        pub ballot_info_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "correspondenceAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The mailing address of this administrative body."]
        pub correspondence_address: ::std::option::Option<::std::boxed::Box<SimpleAddressType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "electionInfoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL provided by this administrative body for looking up general election information."]
        pub election_info_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "electionNoticeText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A last minute or emergency notification text provided by this administrative body."]
        pub election_notice_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "electionNoticeUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL provided by this administrative body for additional information related to the last minute or emergency notification."]
        pub election_notice_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "electionOfficials")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The election officials for this election administrative body."]
        pub election_officials:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ElectionOfficial>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "electionRegistrationConfirmationUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL provided by this administrative body for confirming that the voter is registered to vote."]
        pub election_registration_confirmation_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "electionRegistrationUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL provided by this administrative body for looking up how to register to vote."]
        pub election_registration_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "electionRulesUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL provided by this administrative body describing election rules to the voter."]
        pub election_rules_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hoursOfOperation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of the hours of operation for this administrative body."]
        pub hours_of_operation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this election administrative body."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "physicalAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The physical address of this administrative body."]
        pub physical_address: ::std::option::Option<::std::boxed::Box<SimpleAddressType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "voter_services")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of the services this administrative body may provide."]
        pub voter_services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "votingLocationFinderUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL provided by this administrative body for looking up where to vote."]
        pub voting_location_finder_url: ::std::option::Option<::std::string::String>,
    }
    impl AdministrativeBody {
        pub fn builder() -> AdministrativeBodyBuilder {
            AdministrativeBodyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a candidate running for elected office."]
    pub struct Candidate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "candidateUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL for the candidate's campaign web site."]
        pub candidate_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of known (social) media channels for this candidate."]
        pub channels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Channel>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address for the candidate's campaign."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The candidate's name. If this is a joint ticket it will indicate the name of the candidate at the top of a ticket followed by a / and that name of candidate at the bottom of the ticket. e.g. \"Mitt Romney / Paul Ryan\""]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orderOnBallot")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The order the candidate appears on the ballot for this contest."]
        pub order_on_ballot: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "party")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full name of the party the candidate is a member of."]
        pub party: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The voice phone number for the candidate's campaign office."]
        pub phone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL for a photo of the candidate."]
        pub photo_url: ::std::option::Option<::std::string::String>,
    }
    impl Candidate {
        pub fn builder() -> CandidateBuilder {
            CandidateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A social media or web channel for a candidate."]
    pub struct Channel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique public identifier for the candidate's channel."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of channel. The following is a list of types of channels, but is not exhaustive. More channel types may be added at a later time. One of: GooglePlus, YouTube, Facebook, Twitter"]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl Channel {
        pub fn builder() -> ChannelBuilder {
            ChannelBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a contest that appears on a voter's ballot."]
    pub struct Contest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ballotPlacement")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A number specifying the position of this contest on the voter's ballot."]
        pub ballot_placement: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ballotTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The official title on the ballot for this contest, only where available."]
        pub ballot_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "candidates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The candidate choices for this contest."]
        pub candidates: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Candidate>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "district")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the electoral district that this contest is in."]
        pub district: ::std::option::Option<::std::boxed::Box<ElectoralDistrict>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "electorateSpecifications")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of any additional eligibility requirements for voting in this contest."]
        pub electorate_specifications: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "level")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The levels of government of the office for this contest. There may be more than one in cases where a jurisdiction effectively acts at two different levels of government; for example, the mayor of the District of Columbia acts at \"locality\" level, but also effectively at both \"administrative-area-2\" and \"administrative-area-1\"."]
        pub level: ::std::option::Option<::std::vec::Vec<ContestLevelEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numberElected")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of candidates that will be elected to office in this contest."]
        pub number_elected: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numberVotingFor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of candidates that a voter may vote for in this contest."]
        pub number_voting_for: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "office")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the office for this contest."]
        pub office: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryParties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this is a partisan election, the name of the party/parties it is for."]
        pub primary_parties: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryParty")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[DEPRECATED] If this is a partisan election, the name of the party it is for. This field as deprecated in favor of the array \"primaryParties\", as contests may contain more than one party."]
        #[deprecated]
        pub primary_party: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referendumBallotResponses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of ballot responses for the referendum. A ballot response represents a line on the ballot. Common examples might include \"yes\" or \"no\" for referenda. This field is only populated for contests of type 'Referendum'."]
        pub referendum_ballot_responses:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referendumBrief")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies a short summary of the referendum that is typically on the ballot below the title but above the text. This field is only populated for contests of type 'Referendum'."]
        pub referendum_brief: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referendumConStatement")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A statement in opposition to the referendum. It does not necessarily appear on the ballot. This field is only populated for contests of type 'Referendum'."]
        pub referendum_con_statement: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referendumEffectOfAbstain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies what effect abstaining (not voting) on the proposition will have (i.e. whether abstaining is considered a vote against it). This field is only populated for contests of type 'Referendum'."]
        pub referendum_effect_of_abstain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referendumPassageThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The threshold of votes that the referendum needs in order to pass, e.g. \"two-thirds\". This field is only populated for contests of type 'Referendum'."]
        pub referendum_passage_threshold: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referendumProStatement")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A statement in favor of the referendum. It does not necessarily appear on the ballot. This field is only populated for contests of type 'Referendum'."]
        pub referendum_pro_statement: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referendumSubtitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A brief description of the referendum. This field is only populated for contests of type 'Referendum'."]
        pub referendum_subtitle: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referendumText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full text of the referendum. This field is only populated for contests of type 'Referendum'."]
        pub referendum_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referendumTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the referendum (e.g. 'Proposition 42'). This field is only populated for contests of type 'Referendum'."]
        pub referendum_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referendumUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to the referendum. This field is only populated for contests of type 'Referendum'."]
        pub referendum_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "roles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The roles which this office fulfills."]
        pub roles: ::std::option::Option<::std::vec::Vec<ContestRolesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of sources for this contest. If multiple sources are listed, the data has been aggregated from those sources."]
        pub sources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Source>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "special")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "\"Yes\" or \"No\" depending on whether this a contest being held outside the normal election cycle."]
        pub special: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of contest. Usually this will be 'General', 'Primary', or 'Run-off' for contests with candidates. For referenda this will be 'Referendum'. For Retention contests this will typically be 'Retention'."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl Contest {
        pub fn builder() -> ContestBuilder {
            ContestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum ContestLevelEnum {
        #[serde(rename = "international")]
        #[doc = ""]
        International,
        #[serde(rename = "country")]
        #[doc = ""]
        Country,
        #[serde(rename = "administrativeArea1")]
        #[doc = ""]
        AdministrativeArea1,
        #[serde(rename = "regional")]
        #[doc = ""]
        Regional,
        #[serde(rename = "administrativeArea2")]
        #[doc = ""]
        AdministrativeArea2,
        #[serde(rename = "locality")]
        #[doc = ""]
        Locality,
        #[serde(rename = "subLocality1")]
        #[doc = ""]
        SubLocality1,
        #[serde(rename = "subLocality2")]
        #[doc = ""]
        SubLocality2,
        #[serde(rename = "special")]
        #[doc = ""]
        Special,
    }
    impl ::std::default::Default for ContestLevelEnum {
        fn default() -> Self {
            Self::International
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum ContestRolesEnum {
        #[serde(rename = "headOfState")]
        #[doc = ""]
        HeadOfState,
        #[serde(rename = "headOfGovernment")]
        #[doc = ""]
        HeadOfGovernment,
        #[serde(rename = "deputyHeadOfGovernment")]
        #[doc = ""]
        DeputyHeadOfGovernment,
        #[serde(rename = "governmentOfficer")]
        #[doc = ""]
        GovernmentOfficer,
        #[serde(rename = "executiveCouncil")]
        #[doc = ""]
        ExecutiveCouncil,
        #[serde(rename = "legislatorUpperBody")]
        #[doc = ""]
        LegislatorUpperBody,
        #[serde(rename = "legislatorLowerBody")]
        #[doc = ""]
        LegislatorLowerBody,
        #[serde(rename = "highestCourtJudge")]
        #[doc = ""]
        HighestCourtJudge,
        #[serde(rename = "judge")]
        #[doc = ""]
        Judge,
        #[serde(rename = "schoolBoard")]
        #[doc = ""]
        SchoolBoard,
        #[serde(rename = "specialPurposeOfficer")]
        #[doc = ""]
        SpecialPurposeOfficer,
    }
    impl ::std::default::Default for ContestRolesEnum {
        fn default() -> Self {
            Self::HeadOfState
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of a division search query."]
    pub struct DivisionSearchResponse {
        #[builder(
            default = "{ division_search_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "division_search_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"civicinfo#divisionSearchResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub results:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DivisionSearchResult>>>,
    }
    impl DivisionSearchResponse {
        pub fn builder() -> DivisionSearchResponseBuilder {
            DivisionSearchResponseBuilder::default()
        }
    }
    mod division_search_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("civicinfo#divisionSearchResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a political geographic division that matches the requested query."]
    pub struct DivisionSearchResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aliases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Other Open Civic Data identifiers that refer to the same division -- for example, those that refer to other political divisions whose boundaries are defined to be coterminous with this one. For example, ocd-division/country:us/state:wy will include an alias of ocd-division/country:us/state:wy/cd:1, since Wyoming has only one Congressional district."]
        pub aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the division."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ocdId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique Open Civic Data identifier for this division"]
        pub ocd_id: ::std::option::Option<::std::string::String>,
    }
    impl DivisionSearchResult {
        pub fn builder() -> DivisionSearchResultBuilder {
            DivisionSearchResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the election that was queried."]
    pub struct Election {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "electionDay")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Day of the election in YYYY-MM-DD format."]
        pub election_day: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID of this election."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A displayable name for the election."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ocdDivisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The political division of the election. Represented as an OCD Division ID. Voters within these political jurisdictions are covered by this election. This is typically a state such as ocd-division/country:us/state:ca or for the midterms or general election the entire US (i.e. ocd-division/country:us)."]
        pub ocd_division_id: ::std::option::Option<::std::string::String>,
    }
    impl Election {
        pub fn builder() -> ElectionBuilder {
            ElectionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about individual election officials."]
    pub struct ElectionOfficial {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of the election official."]
        pub email_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faxNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fax number of the election official."]
        pub fax_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full name of the election official."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "officePhoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The office phone number of the election official."]
        pub office_phone_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the election official."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl ElectionOfficial {
        pub fn builder() -> ElectionOfficialBuilder {
            ElectionOfficialBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The list of elections available for this version of the API."]
    pub struct ElectionsQueryResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "elections")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of available elections"]
        pub elections: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Election>>>,
        #[builder(
            default = "{ elections_query_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "elections_query_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"civicinfo#electionsQueryResponse\"."]
        pub kind: ::std::string::String,
    }
    impl ElectionsQueryResponse {
        pub fn builder() -> ElectionsQueryResponseBuilder {
            ElectionsQueryResponseBuilder::default()
        }
    }
    mod elections_query_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("civicinfo#electionsQueryResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes the geographic scope of a contest."]
    pub struct ElectoralDistrict {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An identifier for this district, relative to its scope. For example, the 34th State Senate district would have id \"34\" and a scope of stateUpper."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the district."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scope")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The geographic scope of this district. If unspecified the district's geography is not known. One of: national, statewide, congressional, stateUpper, stateLower, countywide, judicial, schoolBoard, cityWide, township, countyCouncil, cityCouncil, ward, special"]
        pub scope: ::std::option::Option<ElectoralDistrictScopeEnum>,
    }
    impl ElectoralDistrict {
        pub fn builder() -> ElectoralDistrictBuilder {
            ElectoralDistrictBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The geographic scope of this district. If unspecified the district's geography is not known. One of: national, statewide, congressional, stateUpper, stateLower, countywide, judicial, schoolBoard, cityWide, township, countyCouncil, cityCouncil, ward, special"]
    pub enum ElectoralDistrictScopeEnum {
        #[serde(rename = "statewide")]
        #[doc = ""]
        Statewide,
        #[serde(rename = "congressional")]
        #[doc = ""]
        Congressional,
        #[serde(rename = "stateUpper")]
        #[doc = ""]
        StateUpper,
        #[serde(rename = "stateLower")]
        #[doc = ""]
        StateLower,
        #[serde(rename = "countywide")]
        #[doc = ""]
        Countywide,
        #[serde(rename = "judicial")]
        #[doc = ""]
        Judicial,
        #[serde(rename = "schoolBoard")]
        #[doc = ""]
        SchoolBoard,
        #[serde(rename = "citywide")]
        #[doc = ""]
        Citywide,
        #[serde(rename = "special")]
        #[doc = ""]
        Special,
        #[serde(rename = "countyCouncil")]
        #[doc = ""]
        CountyCouncil,
        #[serde(rename = "township")]
        #[doc = ""]
        Township,
        #[serde(rename = "ward")]
        #[doc = ""]
        Ward,
        #[serde(rename = "cityCouncil")]
        #[doc = ""]
        CityCouncil,
        #[serde(rename = "national")]
        #[doc = ""]
        National,
    }
    impl ::std::default::Default for ElectoralDistrictScopeEnum {
        fn default() -> Self {
            Self::Statewide
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a political geography."]
    pub struct GeographicDivision {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alsoKnownAs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Any other valid OCD IDs that refer to the same division.\\n\\nBecause OCD IDs are meant to be human-readable and at least somewhat predictable, there are occasionally several identifiers for a single division. These identifiers are defined to be equivalent to one another, and one is always indicated as the primary identifier. The primary identifier will be returned in ocd_id above, and any other equivalent valid identifiers will be returned in this list.\\n\\nFor example, if this division's OCD ID is ocd-division/country:us/district:dc, this will contain ocd-division/country:us/state:dc."]
        pub also_known_as: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the division."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "officeIndices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of indices in the offices array, one for each office elected from this division. Will only be present if includeOffices was true (or absent) in the request."]
        pub office_indices: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    }
    impl GeographicDivision {
        pub fn builder() -> GeographicDivisionBuilder {
            GeographicDivisionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about an Office held by one or more Officials."]
    pub struct Office {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "divisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The OCD ID of the division with which this office is associated."]
        pub division_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "levels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The levels of government of which this office is part. There may be more than one in cases where a jurisdiction effectively acts at two different levels of government; for example, the mayor of the District of Columbia acts at \"locality\" level, but also effectively at both \"administrative-area-2\" and \"administrative-area-1\"."]
        pub levels: ::std::option::Option<::std::vec::Vec<OfficeLevelsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The human-readable name of the office."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "officialIndices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of indices in the officials array of people who presently hold this office."]
        pub official_indices: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "roles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The roles which this office fulfills. Roles are not meant to be exhaustive, or to exactly specify the entire set of responsibilities of a given office, but are meant to be rough categories that are useful for general selection from or sorting of a list of offices."]
        pub roles: ::std::option::Option<::std::vec::Vec<OfficeRolesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of sources for this office. If multiple sources are listed, the data has been aggregated from those sources."]
        pub sources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Source>>>,
    }
    impl Office {
        pub fn builder() -> OfficeBuilder {
            OfficeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum OfficeLevelsEnum {
        #[serde(rename = "international")]
        #[doc = ""]
        International,
        #[serde(rename = "country")]
        #[doc = ""]
        Country,
        #[serde(rename = "administrativeArea1")]
        #[doc = ""]
        AdministrativeArea1,
        #[serde(rename = "regional")]
        #[doc = ""]
        Regional,
        #[serde(rename = "administrativeArea2")]
        #[doc = ""]
        AdministrativeArea2,
        #[serde(rename = "locality")]
        #[doc = ""]
        Locality,
        #[serde(rename = "subLocality1")]
        #[doc = ""]
        SubLocality1,
        #[serde(rename = "subLocality2")]
        #[doc = ""]
        SubLocality2,
        #[serde(rename = "special")]
        #[doc = ""]
        Special,
    }
    impl ::std::default::Default for OfficeLevelsEnum {
        fn default() -> Self {
            Self::International
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum OfficeRolesEnum {
        #[serde(rename = "headOfState")]
        #[doc = ""]
        HeadOfState,
        #[serde(rename = "headOfGovernment")]
        #[doc = ""]
        HeadOfGovernment,
        #[serde(rename = "deputyHeadOfGovernment")]
        #[doc = ""]
        DeputyHeadOfGovernment,
        #[serde(rename = "governmentOfficer")]
        #[doc = ""]
        GovernmentOfficer,
        #[serde(rename = "executiveCouncil")]
        #[doc = ""]
        ExecutiveCouncil,
        #[serde(rename = "legislatorUpperBody")]
        #[doc = ""]
        LegislatorUpperBody,
        #[serde(rename = "legislatorLowerBody")]
        #[doc = ""]
        LegislatorLowerBody,
        #[serde(rename = "highestCourtJudge")]
        #[doc = ""]
        HighestCourtJudge,
        #[serde(rename = "judge")]
        #[doc = ""]
        Judge,
        #[serde(rename = "schoolBoard")]
        #[doc = ""]
        SchoolBoard,
        #[serde(rename = "specialPurposeOfficer")]
        #[doc = ""]
        SpecialPurposeOfficer,
    }
    impl ::std::default::Default for OfficeRolesEnum {
        fn default() -> Self {
            Self::HeadOfState
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a person holding an elected office."]
    pub struct Official {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Addresses at which to contact the official."]
        pub address: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SimpleAddressType>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of known (social) media channels for this official."]
        pub channels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Channel>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The direct email addresses for the official."]
        pub emails: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The official's name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "party")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full name of the party the official belongs to."]
        pub party: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phones")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The official's public contact phone numbers."]
        pub phones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL for a photo of the official."]
        pub photo_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "urls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The official's public website URLs."]
        pub urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Official {
        pub fn builder() -> OfficialBuilder {
            OfficialBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A location where a voter can vote. This may be an early vote site, an election day voting location, or a drop off location for a completed ballot."]
    pub struct PollingLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The address of the location."]
        pub address: ::std::option::Option<::std::boxed::Box<SimpleAddressType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last date that this early vote site or drop off location may be used. This field is not populated for polling locations."]
        pub end_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Latitude of the location, in degrees north of the equator. Note this field may not be available for some locations."]
        pub latitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Longitude of the location, in degrees east of the Prime Meridian. Note this field may not be available for some locations."]
        pub longitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the early vote site or drop off location. This field is not populated for polling locations."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Notes about this location (e.g. accessibility ramp or entrance to use)."]
        pub notes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pollingHours")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of when this location is open."]
        pub polling_hours: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of sources for this location. If multiple sources are listed the data has been aggregated from those sources."]
        pub sources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Source>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The first date that this early vote site or drop off location may be used. This field is not populated for polling locations."]
        pub start_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "voterServices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The services provided by this early vote site or drop off location. This field is not populated for polling locations."]
        pub voter_services: ::std::option::Option<::std::string::String>,
    }
    impl PollingLocation {
        pub fn builder() -> PollingLocationBuilder {
            PollingLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct RepresentativeInfoData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "divisions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of political geographic divisions that contain the requested address, keyed by the unique Open Civic Data identifier for this division."]
        pub divisions: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<GeographicDivision>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Elected offices referenced by the divisions listed above. Will only be present if includeOffices was true in the request."]
        pub offices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Office>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "officials")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Officials holding the offices listed above. Will only be present if includeOffices was true in the request."]
        pub officials: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Official>>>,
    }
    impl RepresentativeInfoData {
        pub fn builder() -> RepresentativeInfoDataBuilder {
            RepresentativeInfoDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of a representative info lookup query."]
    pub struct RepresentativeInfoResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "divisions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of political geographic divisions that contain the requested address, keyed by the unique Open Civic Data identifier for this division."]
        pub divisions: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<GeographicDivision>>,
        >,
        #[builder(
            default = "{ representative_info_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "representative_info_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"civicinfo#representativeInfoResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedInput")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normalized version of the requested address"]
        pub normalized_input: ::std::option::Option<::std::boxed::Box<SimpleAddressType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Elected offices referenced by the divisions listed above. Will only be present if includeOffices was true in the request."]
        pub offices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Office>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "officials")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Officials holding the offices listed above. Will only be present if includeOffices was true in the request."]
        pub officials: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Official>>>,
    }
    impl RepresentativeInfoResponse {
        pub fn builder() -> RepresentativeInfoResponseBuilder {
            RepresentativeInfoResponseBuilder::default()
        }
    }
    mod representative_info_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("civicinfo#representativeInfoResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A simple representation of an address."]
    pub struct SimpleAddressType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "city")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The city or town for the address."]
        pub city: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "line1")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The street name and number of this address."]
        pub line1: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "line2")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The second line the address, if needed."]
        pub line2: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "line3")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The third line of the address, if needed."]
        pub line3: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the location."]
        pub location_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The US two letter state abbreviation of the address."]
        pub state: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zip")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The US Postal Zip Code of the address."]
        pub zip: ::std::option::Option<::std::string::String>,
    }
    impl SimpleAddressType {
        pub fn builder() -> SimpleAddressTypeBuilder {
            SimpleAddressTypeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains information about the data source for the element containing it."]
    pub struct Source {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the data source."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "official")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this data comes from an official government source."]
        pub official: ::std::option::Option<::std::primitive::bool>,
    }
    impl Source {
        pub fn builder() -> SourceBuilder {
            SourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of a voter info lookup query."]
    pub struct VoterInfoResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contests that will appear on the voter's ballot."]
        pub contests: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Contest>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dropOffLocations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Locations where a voter is eligible to drop off a completed ballot. The voter must have received and completed a ballot prior to arriving at the location. The location may not have ballots available on the premises. These locations could be open on or before election day as indicated in the pollingHours field."]
        pub drop_off_locations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PollingLocation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "earlyVoteSites")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Locations where the voter is eligible to vote early, prior to election day."]
        pub early_vote_sites:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PollingLocation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "election")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The election that was queried."]
        pub election: ::std::option::Option<::std::boxed::Box<Election>>,
        #[builder(default = "{ voter_info_response_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "voter_info_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"civicinfo#voterInfoResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mailOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies whether voters in the precinct vote only by mailing their ballots (with the possible option of dropping off their ballots as well)."]
        pub mail_only: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedInput")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normalized version of the requested address"]
        pub normalized_input: ::std::option::Option<::std::boxed::Box<SimpleAddressType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "otherElections")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When there are multiple elections for a voter address, the otherElections field is populated in the API response and there are two possibilities: 1. If the earliest election is not the intended election, specify the election ID of the desired election in a second API request using the electionId field. 2. If these elections occur on the same day, the API doesn?t return any polling location, contest, or election official information to ensure that an additional query is made. For user-facing applications, we recommend displaying these elections to the user to disambiguate. A second API request using the electionId field should be made for the election that is relevant to the user."]
        pub other_elections: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Election>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pollingLocations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Locations where the voter is eligible to vote on election day."]
        pub polling_locations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PollingLocation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "precinctId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub precinct_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Local Election Information for the state that the voter votes in. For the US, there will only be one element in this array."]
        pub state: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AdministrationRegion>>>,
    }
    impl VoterInfoResponse {
        pub fn builder() -> VoterInfoResponseBuilder {
            VoterInfoResponseBuilder::default()
        }
    }
    mod voter_info_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("civicinfo#voterInfoResponse")
        }
    }
}
