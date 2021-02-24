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
    pub mod claims {
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
                    #[serde(rename = "languageCode")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". Can be used to restrict results by language, though we do not currently consider the region."]
                    pub language_code: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxAgeDays")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum age of the returned search results, in days. Age is determined by either claim date or review date, whichever is newer."]
                    pub max_age_days: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "offset")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "An integer that specifies the current offset (that is, starting result location) in search results. This field is only considered if `page_token` is unset. For example, 0 means to return results starting from the first matching result, and 10 means to return from the 11th result."]
                    pub offset: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The pagination size. We will return up to that many results. Defaults to 10 if not set."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The pagination token. You may provide the `next_page_token` returned from a previous List request, if any, in order to get the next page. All other fields must have the same values as in the previous request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "query")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Textual query string. Required unless `review_publisher_site_filter` is specified."]
                    pub query: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "reviewPublisherSiteFilter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The review publisher site to filter results by, e.g. nytimes.com."]
                    pub review_publisher_site_filter: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod pages {
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
                    #[serde(rename = "offset")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "An integer that specifies the current offset (that is, starting result location) in search results. This field is only considered if `page_token` is unset, and if the request is not for a specific URL. For example, 0 means to return results starting from the first matching result, and 10 means to return from the 11th result."]
                    pub offset: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "organization")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The organization for which we want to fetch markups for. For instance, \"site.com\". Cannot be specified along with an URL."]
                    pub organization: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The pagination size. We will return up to that many results. Defaults to 10 if not set. Has no effect if a URL is requested."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The pagination token. You may provide the `next_page_token` returned from a previous List request, if any, in order to get the next page. All other fields must have the same values as in the previous request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "url")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The URL from which to get `ClaimReview` markup. There will be at most one result. If markup is associated with a more canonical version of the URL provided, we will return that URL instead. Cannot be specified along with an organization."]
                    pub url: ::std::option::Option<::std::string::String>,
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
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the claim."]
    pub struct GoogleFactcheckingFactchecktoolsV1alpha1Claim {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claimDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date that the claim was made."]
        pub claim_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claimReview")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One or more reviews of this claim (namely, a fact-checking article)."]
        pub claim_review: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleFactcheckingFactchecktoolsV1alpha1ClaimReview>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claimant")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A person or organization stating the claim. For instance, \"John Doe\"."]
        pub claimant: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The claim text. For instance, \"Crime has doubled in the last 2 years.\""]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFactcheckingFactchecktoolsV1alpha1Claim {
        pub fn builder() -> GoogleFactcheckingFactchecktoolsV1alpha1ClaimBuilder {
            GoogleFactcheckingFactchecktoolsV1alpha1ClaimBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the claim author."]
    pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimAuthor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Corresponds to `ClaimReview.itemReviewed.author.image`."]
        pub image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Corresponds to `ClaimReview.itemReviewed.author.jobTitle`."]
        pub job_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A person or organization stating the claim. For instance, \"John Doe\". Corresponds to `ClaimReview.itemReviewed.author.name`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sameAs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Corresponds to `ClaimReview.itemReviewed.author.sameAs`."]
        pub same_as: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFactcheckingFactchecktoolsV1alpha1ClaimAuthor {
        pub fn builder() -> GoogleFactcheckingFactchecktoolsV1alpha1ClaimAuthorBuilder {
            GoogleFactcheckingFactchecktoolsV1alpha1ClaimAuthorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the claim rating."]
    pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimRating {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bestRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For numeric ratings, the best value possible in the scale from worst to best. Corresponds to `ClaimReview.reviewRating.bestRating`."]
        pub best_rating: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Corresponds to `ClaimReview.reviewRating.image`."]
        pub image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ratingExplanation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Corresponds to `ClaimReview.reviewRating.ratingExplanation`."]
        pub rating_explanation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ratingValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A numeric rating of this claim, in the range worstRating — bestRating inclusive. Corresponds to `ClaimReview.reviewRating.ratingValue`."]
        pub rating_value: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textualRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The truthfulness rating as a human-readible short word or phrase. Corresponds to `ClaimReview.reviewRating.alternateName`."]
        pub textual_rating: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "worstRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For numeric ratings, the worst value possible in the scale from worst to best. Corresponds to `ClaimReview.reviewRating.worstRating`."]
        pub worst_rating: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleFactcheckingFactchecktoolsV1alpha1ClaimRating {
        pub fn builder() -> GoogleFactcheckingFactchecktoolsV1alpha1ClaimRatingBuilder {
            GoogleFactcheckingFactchecktoolsV1alpha1ClaimRatingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a claim review."]
    pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimReview {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language this review was written in. For instance, \"en\" or \"de\"."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publisher")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The publisher of this claim review."]
        pub publisher: ::std::option::Option<
            ::std::boxed::Box<GoogleFactcheckingFactchecktoolsV1alpha1Publisher>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reviewDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date the claim was reviewed."]
        pub review_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textualRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Textual rating. For instance, \"Mostly false\"."]
        pub textual_rating: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of this claim review, if it can be determined."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of this claim review."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFactcheckingFactchecktoolsV1alpha1ClaimReview {
        pub fn builder() -> GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewBuilder {
            GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the claim review author."]
    pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewAuthor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Corresponds to `ClaimReview.author.image`."]
        pub image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the organization that is publishing the fact check. Corresponds to `ClaimReview.author.name`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewAuthor {
        pub fn builder() -> GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewAuthorBuilder {
            GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewAuthorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Fields for an individual `ClaimReview` element. Except for sub-messages that group fields together, each of these fields correspond those in https://schema.org/ClaimReview. We list the precise mapping for each field."]
    pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claimAppearances")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of links to works in which this claim appears, aside from the one specified in `claim_first_appearance`. Corresponds to `ClaimReview.itemReviewed[@type=Claim].appearance.url`."]
        pub claim_appearances: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claimAuthor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Info about the author of this claim."]
        pub claim_author: ::std::option::Option<
            ::std::boxed::Box<GoogleFactcheckingFactchecktoolsV1alpha1ClaimAuthor>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claimDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date when the claim was made or entered public discourse. Corresponds to `ClaimReview.itemReviewed.datePublished`."]
        pub claim_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claimFirstAppearance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to a work in which this claim first appears. Corresponds to `ClaimReview.itemReviewed[@type=Claim].firstAppearance.url`."]
        pub claim_first_appearance: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claimLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location where this claim was made. Corresponds to `ClaimReview.itemReviewed.name`."]
        pub claim_location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claimReviewed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short summary of the claim being evaluated. Corresponds to `ClaimReview.claimReviewed`."]
        pub claim_reviewed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Info about the rating of this claim review."]
        pub rating: ::std::option::Option<
            ::std::boxed::Box<GoogleFactcheckingFactchecktoolsV1alpha1ClaimRating>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is optional, and will default to the page URL. We provide this field to allow you the override the default value, but the only permitted override is the page URL plus an optional anchor link (\"page jump\"). Corresponds to `ClaimReview.url`"]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkup {
        pub fn builder() -> GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkupBuilder {
            GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Holds one or more instances of `ClaimReview` markup for a webpage."]
    pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkupPage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claimReviewAuthor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Info about the author of this claim review. Similar to the above, semantically these are page-level fields, and each `ClaimReview` on this page will contain the same values."]
        pub claim_review_author: ::std::option::Option<
            ::std::boxed::Box<GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewAuthor>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claimReviewMarkups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of individual claim reviews for this page. Each item in the list corresponds to one `ClaimReview` element."]
        pub claim_review_markups: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkup>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this `ClaimReview` markup page resource, in the form of `pages/{page_id}`. Except for update requests, this field is output-only and should not be set by the user."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the page associated with this `ClaimReview` markup. While every individual `ClaimReview` has its own URL field, semantically this is a page-level field, and each `ClaimReview` on this page will use this value unless individually overridden. Corresponds to `ClaimReview.url`"]
        pub page_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date when the fact check was published. Similar to the URL, semantically this is a page-level field, and each `ClaimReview` on this page will contain the same value. Corresponds to `ClaimReview.datePublished`"]
        pub publish_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version ID for this markup. Except for update requests, this field is output-only and should not be set by the user."]
        pub version_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkupPage {
        pub fn builder() -> GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkupPageBuilder {
            GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkupPageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response from searching fact-checked claims."]
    pub struct GoogleFactcheckingFactchecktoolsV1alpha1FactCheckedClaimSearchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claims")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of claims and all of their associated information."]
        pub claims: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleFactcheckingFactchecktoolsV1alpha1Claim>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The next pagination token in the Search response. It should be used as the `page_token` for the following request. An empty value means no more results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFactcheckingFactchecktoolsV1alpha1FactCheckedClaimSearchResponse {
        pub fn builder(
        ) -> GoogleFactcheckingFactchecktoolsV1alpha1FactCheckedClaimSearchResponseBuilder {
            GoogleFactcheckingFactchecktoolsV1alpha1FactCheckedClaimSearchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response from listing `ClaimReview` markup."]
    pub struct GoogleFactcheckingFactchecktoolsV1alpha1ListClaimReviewMarkupPagesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claimReviewMarkupPages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result list of pages of `ClaimReview` markup."]
        pub claim_review_markup_pages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkupPage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The next pagination token in the Search response. It should be used as the `page_token` for the following request. An empty value means no more results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFactcheckingFactchecktoolsV1alpha1ListClaimReviewMarkupPagesResponse {
        pub fn builder(
        ) -> GoogleFactcheckingFactchecktoolsV1alpha1ListClaimReviewMarkupPagesResponseBuilder
        {
            GoogleFactcheckingFactchecktoolsV1alpha1ListClaimReviewMarkupPagesResponseBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the publisher."]
    pub struct GoogleFactcheckingFactchecktoolsV1alpha1Publisher {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this publisher. For instance, \"Awesome Fact Checks\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "site")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Host-level site name, without the protocol or \"www\" prefix. For instance, \"awesomefactchecks.com\". This value of this field is based purely on the claim review URL."]
        pub site: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFactcheckingFactchecktoolsV1alpha1Publisher {
        pub fn builder() -> GoogleFactcheckingFactchecktoolsV1alpha1PublisherBuilder {
            GoogleFactcheckingFactchecktoolsV1alpha1PublisherBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct GoogleProtobufEmpty {}
    impl GoogleProtobufEmpty {
        pub fn builder() -> GoogleProtobufEmptyBuilder {
            GoogleProtobufEmptyBuilder::default()
        }
    }
}
