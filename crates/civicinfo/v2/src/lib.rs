#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes information about a regional election administrative area."]
pub struct AdministrationRegion {
    #[serde(rename = "electionAdministrationBody")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The election administration body for this area."]
    pub election_administration_body: ::std::option::Option<::std::boxed::Box<AdministrativeBody>>,
    #[serde(rename = "local_jurisdiction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The city or county that provides election information for this voter. This object can have the same elements as state."]
    pub local_jurisdiction: ::std::option::Option<::std::boxed::Box<AdministrationRegion>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the jurisdiction."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of sources for this area. If multiple sources are listed the data has been aggregated from those sources."]
    pub sources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Source>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about an election administrative body (e.g. County Board of Elections)."]
pub struct AdministrativeBody {
    #[serde(rename = "absenteeVotingInfoUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL provided by this administrative body for information on absentee voting."]
    pub absentee_voting_info_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ballotInfoUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL provided by this administrative body to give contest information to the voter."]
    pub ballot_info_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "correspondenceAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The mailing address of this administrative body."]
    pub correspondence_address: ::std::option::Option<::std::boxed::Box<SimpleAddressType>>,
    #[serde(rename = "electionInfoUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL provided by this administrative body for looking up general election information."]
    pub election_info_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "electionNoticeText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A last minute or emergency notification text provided by this administrative body."]
    pub election_notice_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "electionNoticeUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL provided by this administrative body for additional information related to the last minute or emergency notification."]
    pub election_notice_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "electionOfficials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The election officials for this election administrative body."]
    pub election_officials:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ElectionOfficial>>>,
    #[serde(rename = "electionRegistrationConfirmationUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL provided by this administrative body for confirming that the voter is registered to vote."]
    pub election_registration_confirmation_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "electionRegistrationUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL provided by this administrative body for looking up how to register to vote."]
    pub election_registration_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "electionRulesUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL provided by this administrative body describing election rules to the voter."]
    pub election_rules_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hoursOfOperation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of the hours of operation for this administrative body."]
    pub hours_of_operation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this election administrative body."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "physicalAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The physical address of this administrative body."]
    pub physical_address: ::std::option::Option<::std::boxed::Box<SimpleAddressType>>,
    #[serde(rename = "voter_services")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of the services this administrative body may provide."]
    pub voter_services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "votingLocationFinderUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL provided by this administrative body for looking up where to vote."]
    pub voting_location_finder_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about a candidate running for elected office."]
pub struct Candidate {
    #[serde(rename = "candidateUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL for the candidate's campaign web site."]
    pub candidate_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "channels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of known (social) media channels for this candidate."]
    pub channels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Channel>>>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address for the candidate's campaign."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The candidate's name. If this is a joint ticket it will indicate the name of the candidate at the top of a ticket followed by a / and that name of candidate at the bottom of the ticket. e.g. \"Mitt Romney / Paul Ryan\""]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderOnBallot")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The order the candidate appears on the ballot for this contest."]
    pub order_on_ballot: ::std::option::Option<::std::string::String>,
    #[serde(rename = "party")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full name of the party the candidate is a member of."]
    pub party: ::std::option::Option<::std::string::String>,
    #[serde(rename = "phone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The voice phone number for the candidate's campaign office."]
    pub phone: ::std::option::Option<::std::string::String>,
    #[serde(rename = "photoUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL for a photo of the candidate."]
    pub photo_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A social media or web channel for a candidate."]
pub struct Channel {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique public identifier for the candidate's channel."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of channel. The following is a list of types of channels, but is not exhaustive. More channel types may be added at a later time. One of: GooglePlus, YouTube, Facebook, Twitter"]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about a contest that appears on a voter's ballot."]
pub struct Contest {
    #[serde(rename = "ballotPlacement")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A number specifying the position of this contest on the voter's ballot."]
    pub ballot_placement: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ballotTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The official title on the ballot for this contest, only where available."]
    pub ballot_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "candidates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The candidate choices for this contest."]
    pub candidates: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Candidate>>>,
    #[serde(rename = "district")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the electoral district that this contest is in."]
    pub district: ::std::option::Option<::std::boxed::Box<ElectoralDistrict>>,
    #[serde(rename = "electorateSpecifications")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of any additional eligibility requirements for voting in this contest."]
    pub electorate_specifications: ::std::option::Option<::std::string::String>,
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The levels of government of the office for this contest. There may be more than one in cases where a jurisdiction effectively acts at two different levels of government; for example, the mayor of the District of Columbia acts at \"locality\" level, but also effectively at both \"administrative-area-2\" and \"administrative-area-1\"."]
    pub level: ::std::option::Option<::std::vec::Vec<ContestLevelEnum>>,
    #[serde(rename = "numberElected")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of candidates that will be elected to office in this contest."]
    pub number_elected: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numberVotingFor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of candidates that a voter may vote for in this contest."]
    pub number_voting_for: ::std::option::Option<::std::string::String>,
    #[serde(rename = "office")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the office for this contest."]
    pub office: ::std::option::Option<::std::string::String>,
    #[serde(rename = "primaryParties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this is a partisan election, the name of the party/parties it is for."]
    pub primary_parties: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "primaryParty")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[DEPRECATED] If this is a partisan election, the name of the party it is for. This field as deprecated in favor of the array \"primaryParties\", as contests may contain more than one party."]
    #[deprecated]
    pub primary_party: ::std::option::Option<::std::string::String>,
    #[serde(rename = "referendumBallotResponses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of ballot responses for the referendum. A ballot response represents a line on the ballot. Common examples might include \"yes\" or \"no\" for referenda. This field is only populated for contests of type 'Referendum'."]
    pub referendum_ballot_responses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "referendumBrief")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies a short summary of the referendum that is typically on the ballot below the title but above the text. This field is only populated for contests of type 'Referendum'."]
    pub referendum_brief: ::std::option::Option<::std::string::String>,
    #[serde(rename = "referendumConStatement")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A statement in opposition to the referendum. It does not necessarily appear on the ballot. This field is only populated for contests of type 'Referendum'."]
    pub referendum_con_statement: ::std::option::Option<::std::string::String>,
    #[serde(rename = "referendumEffectOfAbstain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies what effect abstaining (not voting) on the proposition will have (i.e. whether abstaining is considered a vote against it). This field is only populated for contests of type 'Referendum'."]
    pub referendum_effect_of_abstain: ::std::option::Option<::std::string::String>,
    #[serde(rename = "referendumPassageThreshold")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The threshold of votes that the referendum needs in order to pass, e.g. \"two-thirds\". This field is only populated for contests of type 'Referendum'."]
    pub referendum_passage_threshold: ::std::option::Option<::std::string::String>,
    #[serde(rename = "referendumProStatement")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A statement in favor of the referendum. It does not necessarily appear on the ballot. This field is only populated for contests of type 'Referendum'."]
    pub referendum_pro_statement: ::std::option::Option<::std::string::String>,
    #[serde(rename = "referendumSubtitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A brief description of the referendum. This field is only populated for contests of type 'Referendum'."]
    pub referendum_subtitle: ::std::option::Option<::std::string::String>,
    #[serde(rename = "referendumText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full text of the referendum. This field is only populated for contests of type 'Referendum'."]
    pub referendum_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "referendumTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the referendum (e.g. 'Proposition 42'). This field is only populated for contests of type 'Referendum'."]
    pub referendum_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "referendumUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the referendum. This field is only populated for contests of type 'Referendum'."]
    pub referendum_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "roles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The roles which this office fulfills."]
    pub roles: ::std::option::Option<::std::vec::Vec<ContestRolesEnum>>,
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of sources for this contest. If multiple sources are listed, the data has been aggregated from those sources."]
    pub sources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Source>>>,
    #[serde(rename = "special")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "\"Yes\" or \"No\" depending on whether this a contest being held outside the normal election cycle."]
    pub special: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of contest. Usually this will be 'General', 'Primary', or 'Run-off' for contests with candidates. For referenda this will be 'Referendum'. For Retention contests this will typically be 'Retention'."]
    pub _type: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The result of a division search query."]
pub struct DivisionSearchResponse {
    #[serde(rename = "kind")]
    #[serde(default = "division_search_response_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"civicinfo#divisionSearchResponse\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DivisionSearchResult>>>,
}
mod division_search_response_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("civicinfo#divisionSearchResponse")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a political geographic division that matches the requested query."]
pub struct DivisionSearchResult {
    #[serde(rename = "aliases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Other Open Civic Data identifiers that refer to the same division -- for example, those that refer to other political divisions whose boundaries are defined to be coterminous with this one. For example, ocd-division/country:us/state:wy will include an alias of ocd-division/country:us/state:wy/cd:1, since Wyoming has only one Congressional district."]
    pub aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the division."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ocdId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique Open Civic Data identifier for this division"]
    pub ocd_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the election that was queried."]
pub struct Election {
    #[serde(rename = "electionDay")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Day of the election in YYYY-MM-DD format."]
    pub election_day: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique ID of this election."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A displayable name for the election."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ocdDivisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The political division of the election. Represented as an OCD Division ID. Voters within these political jurisdictions are covered by this election. This is typically a state such as ocd-division/country:us/state:ca or for the midterms or general election the entire US (i.e. ocd-division/country:us)."]
    pub ocd_division_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about individual election officials."]
pub struct ElectionOfficial {
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of the election official."]
    pub email_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "faxNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fax number of the election official."]
    pub fax_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full name of the election official."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "officePhoneNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The office phone number of the election official."]
    pub office_phone_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the election official."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The list of elections available for this version of the API."]
pub struct ElectionsQueryResponse {
    #[serde(rename = "elections")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of available elections"]
    pub elections: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Election>>>,
    #[serde(rename = "kind")]
    #[serde(default = "elections_query_response_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"civicinfo#electionsQueryResponse\"."]
    pub kind: ::std::string::String,
}
mod elections_query_response_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("civicinfo#electionsQueryResponse")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes the geographic scope of a contest."]
pub struct ElectoralDistrict {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An identifier for this district, relative to its scope. For example, the 34th State Senate district would have id \"34\" and a scope of stateUpper."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the district."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The geographic scope of this district. If unspecified the district's geography is not known. One of: national, statewide, congressional, stateUpper, stateLower, countywide, judicial, schoolBoard, cityWide, township, countyCouncil, cityCouncil, ward, special"]
    pub scope: ::std::option::Option<ElectoralDistrictScopeEnum>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a political geography."]
pub struct GeographicDivision {
    #[serde(rename = "alsoKnownAs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Any other valid OCD IDs that refer to the same division.\\n\\nBecause OCD IDs are meant to be human-readable and at least somewhat predictable, there are occasionally several identifiers for a single division. These identifiers are defined to be equivalent to one another, and one is always indicated as the primary identifier. The primary identifier will be returned in ocd_id above, and any other equivalent valid identifiers will be returned in this list.\\n\\nFor example, if this division's OCD ID is ocd-division/country:us/district:dc, this will contain ocd-division/country:us/state:dc."]
    pub also_known_as: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the division."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "officeIndices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of indices in the offices array, one for each office elected from this division. Will only be present if includeOffices was true (or absent) in the request."]
    pub office_indices: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about an Office held by one or more Officials."]
pub struct Office {
    #[serde(rename = "divisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The OCD ID of the division with which this office is associated."]
    pub division_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "levels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The levels of government of which this office is part. There may be more than one in cases where a jurisdiction effectively acts at two different levels of government; for example, the mayor of the District of Columbia acts at \"locality\" level, but also effectively at both \"administrative-area-2\" and \"administrative-area-1\"."]
    pub levels: ::std::option::Option<::std::vec::Vec<OfficeLevelsEnum>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human-readable name of the office."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "officialIndices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of indices in the officials array of people who presently hold this office."]
    pub official_indices: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "roles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The roles which this office fulfills. Roles are not meant to be exhaustive, or to exactly specify the entire set of responsibilities of a given office, but are meant to be rough categories that are useful for general selection from or sorting of a list of offices."]
    pub roles: ::std::option::Option<::std::vec::Vec<OfficeRolesEnum>>,
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of sources for this office. If multiple sources are listed, the data has been aggregated from those sources."]
    pub sources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Source>>>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about a person holding an elected office."]
pub struct Official {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Addresses at which to contact the official."]
    pub address: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SimpleAddressType>>>,
    #[serde(rename = "channels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of known (social) media channels for this official."]
    pub channels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Channel>>>,
    #[serde(rename = "emails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The direct email addresses for the official."]
    pub emails: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The official's name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "party")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full name of the party the official belongs to."]
    pub party: ::std::option::Option<::std::string::String>,
    #[serde(rename = "phones")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The official's public contact phone numbers."]
    pub phones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "photoUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL for a photo of the official."]
    pub photo_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "urls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The official's public website URLs."]
    pub urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A location where a voter can vote. This may be an early vote site, an election day voting location, or a drop off location for a completed ballot."]
pub struct PollingLocation {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The address of the location."]
    pub address: ::std::option::Option<::std::boxed::Box<SimpleAddressType>>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last date that this early vote site or drop off location may be used. This field is not populated for polling locations."]
    pub end_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "latitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Latitude of the location, in degrees north of the equator. Note this field may not be available for some locations."]
    pub latitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "longitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Longitude of the location, in degrees east of the Prime Meridian. Note this field may not be available for some locations."]
    pub longitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the early vote site or drop off location. This field is not populated for polling locations."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Notes about this location (e.g. accessibility ramp or entrance to use)."]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pollingHours")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of when this location is open."]
    pub polling_hours: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of sources for this location. If multiple sources are listed the data has been aggregated from those sources."]
    pub sources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Source>>>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The first date that this early vote site or drop off location may be used. This field is not populated for polling locations."]
    pub start_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "voterServices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The services provided by this early vote site or drop off location. This field is not populated for polling locations."]
    pub voter_services: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepresentativeInfoData {
    #[serde(rename = "divisions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A map of political geographic divisions that contain the requested address, keyed by the unique Open Civic Data identifier for this division."]
    pub divisions: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<GeographicDivision>>,
    >,
    #[serde(rename = "offices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Elected offices referenced by the divisions listed above. Will only be present if includeOffices was true in the request."]
    pub offices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Office>>>,
    #[serde(rename = "officials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Officials holding the offices listed above. Will only be present if includeOffices was true in the request."]
    pub officials: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Official>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The result of a representative info lookup query."]
pub struct RepresentativeInfoResponse {
    #[serde(rename = "divisions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A map of political geographic divisions that contain the requested address, keyed by the unique Open Civic Data identifier for this division."]
    pub divisions: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<GeographicDivision>>,
    >,
    #[serde(rename = "kind")]
    #[serde(default = "representative_info_response_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"civicinfo#representativeInfoResponse\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "normalizedInput")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The normalized version of the requested address"]
    pub normalized_input: ::std::option::Option<::std::boxed::Box<SimpleAddressType>>,
    #[serde(rename = "offices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Elected offices referenced by the divisions listed above. Will only be present if includeOffices was true in the request."]
    pub offices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Office>>>,
    #[serde(rename = "officials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Officials holding the offices listed above. Will only be present if includeOffices was true in the request."]
    pub officials: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Official>>>,
}
mod representative_info_response_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("civicinfo#representativeInfoResponse")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A simple representation of an address."]
pub struct SimpleAddressType {
    #[serde(rename = "city")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The city or town for the address."]
    pub city: ::std::option::Option<::std::string::String>,
    #[serde(rename = "line1")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The street name and number of this address."]
    pub line1: ::std::option::Option<::std::string::String>,
    #[serde(rename = "line2")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The second line the address, if needed."]
    pub line2: ::std::option::Option<::std::string::String>,
    #[serde(rename = "line3")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The third line of the address, if needed."]
    pub line3: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the location."]
    pub location_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The US two letter state abbreviation of the address."]
    pub state: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zip")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The US Postal Zip Code of the address."]
    pub zip: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about the data source for the element containing it."]
pub struct Source {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the data source."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "official")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this data comes from an official government source."]
    pub official: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The result of a voter info lookup query."]
pub struct VoterInfoResponse {
    #[serde(rename = "contests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contests that will appear on the voter's ballot."]
    pub contests: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Contest>>>,
    #[serde(rename = "dropOffLocations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations where a voter is eligible to drop off a completed ballot. The voter must have received and completed a ballot prior to arriving at the location. The location may not have ballots available on the premises. These locations could be open on or before election day as indicated in the pollingHours field."]
    pub drop_off_locations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PollingLocation>>>,
    #[serde(rename = "earlyVoteSites")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations where the voter is eligible to vote early, prior to election day."]
    pub early_vote_sites:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PollingLocation>>>,
    #[serde(rename = "election")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The election that was queried."]
    pub election: ::std::option::Option<::std::boxed::Box<Election>>,
    #[serde(rename = "kind")]
    #[serde(default = "voter_info_response_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"civicinfo#voterInfoResponse\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "mailOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies whether voters in the precinct vote only by mailing their ballots (with the possible option of dropping off their ballots as well)."]
    pub mail_only: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "normalizedInput")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The normalized version of the requested address"]
    pub normalized_input: ::std::option::Option<::std::boxed::Box<SimpleAddressType>>,
    #[serde(rename = "otherElections")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When there are multiple elections for a voter address, the otherElections field is populated in the API response and there are two possibilities: 1. If the earliest election is not the intended election, specify the election ID of the desired election in a second API request using the electionId field. 2. If these elections occur on the same day, the API doesn?t return any polling location, contest, or election official information to ensure that an additional query is made. For user-facing applications, we recommend displaying these elections to the user to disambiguate. A second API request using the electionId field should be made for the election that is relevant to the user."]
    pub other_elections: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Election>>>,
    #[serde(rename = "pollingLocations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations where the voter is eligible to vote on election day."]
    pub polling_locations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PollingLocation>>>,
    #[serde(rename = "precinctId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub precinct_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Local Election Information for the state that the voter votes in. For the US, there will only be one element in this array."]
    pub state: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AdministrationRegion>>>,
}
mod voter_info_response_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("civicinfo#voterInfoResponse")
    }
}
