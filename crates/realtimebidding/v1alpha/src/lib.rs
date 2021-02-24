#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The bidding function to be executed as part of the TURTLEDOVE simulation experiment bidding flow."]
pub struct BiddingFunction {
    #[serde(rename = "biddingFunction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The raw Javascript source code of the bidding function. The function takes in a Javascript object, `inputs`, that contains the following named fields: `openrtbContextualBidRequest` OR `googleContextualBidRequest`, `customContextualSignal`, `interestBasedBidData`, `interestGroupData`, `recentImpressionAges`, and returns the bid price CPM (double). Example: ``` /* Returns a bid price CPM (double). * * @param {Object} inputs an object with the * following named fields: * - openrtbContextualBidRequest * OR googleContextualBidRequest * - customContextualSignal * - interestBasedBidData * - interestGroupData * - recentImpressionAges */ function biddingFunction(inputs) { ... return inputs.interestBasedBidData.cpm * inputs.customContextualSignals.placementMultiplier; } ```"]
    pub bidding_function: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the bidding function that must follow the pattern: `bidders/{bidder_account_id}/biddingFunctions/{bidding_function_name}`."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response containing a list of a bidder's bidding functions."]
pub struct ListBiddingFunctionsResponse {
    #[serde(rename = "biddingFunctions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of a bidder's bidding functions."]
    pub bidding_functions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BiddingFunction>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token which can be passed to a subsequent call to the `ListBiddingFunctions` method to retrieve the next page of results in ListBiddingFunctionsRequest.pageToken."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
