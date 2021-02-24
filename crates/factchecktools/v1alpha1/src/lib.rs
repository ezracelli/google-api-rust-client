#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the claim."]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1Claim {
    #[serde(rename = "claimDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date that the claim was made."]
    pub claim_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "claimReview")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "One or more reviews of this claim (namely, a fact-checking article)."]
    pub claim_review: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleFactcheckingFactchecktoolsV1alpha1ClaimReview>>,
    >,
    #[serde(rename = "claimant")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A person or organization stating the claim. For instance, \"John Doe\"."]
    pub claimant: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The claim text. For instance, \"Crime has doubled in the last 2 years.\""]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the claim author."]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimAuthor {
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Corresponds to `ClaimReview.itemReviewed.author.image`."]
    pub image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "jobTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Corresponds to `ClaimReview.itemReviewed.author.jobTitle`."]
    pub job_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A person or organization stating the claim. For instance, \"John Doe\". Corresponds to `ClaimReview.itemReviewed.author.name`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sameAs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Corresponds to `ClaimReview.itemReviewed.author.sameAs`."]
    pub same_as: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the claim rating."]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimRating {
    #[serde(rename = "bestRating")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For numeric ratings, the best value possible in the scale from worst to best. Corresponds to `ClaimReview.reviewRating.bestRating`."]
    pub best_rating: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Corresponds to `ClaimReview.reviewRating.image`."]
    pub image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ratingExplanation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Corresponds to `ClaimReview.reviewRating.ratingExplanation`."]
    pub rating_explanation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ratingValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A numeric rating of this claim, in the range worstRating — bestRating inclusive. Corresponds to `ClaimReview.reviewRating.ratingValue`."]
    pub rating_value: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "textualRating")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The truthfulness rating as a human-readible short word or phrase. Corresponds to `ClaimReview.reviewRating.alternateName`."]
    pub textual_rating: ::std::option::Option<::std::string::String>,
    #[serde(rename = "worstRating")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For numeric ratings, the worst value possible in the scale from worst to best. Corresponds to `ClaimReview.reviewRating.worstRating`."]
    pub worst_rating: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about a claim review."]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimReview {
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language this review was written in. For instance, \"en\" or \"de\"."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publisher")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The publisher of this claim review."]
    pub publisher:
        ::std::option::Option<::std::boxed::Box<GoogleFactcheckingFactchecktoolsV1alpha1Publisher>>,
    #[serde(rename = "reviewDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date the claim was reviewed."]
    pub review_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "textualRating")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Textual rating. For instance, \"Mostly false\"."]
    pub textual_rating: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of this claim review, if it can be determined."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of this claim review."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the claim review author."]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewAuthor {
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Corresponds to `ClaimReview.author.image`."]
    pub image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the organization that is publishing the fact check. Corresponds to `ClaimReview.author.name`."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Fields for an individual `ClaimReview` element. Except for sub-messages that group fields together, each of these fields correspond those in https://schema.org/ClaimReview. We list the precise mapping for each field."]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkup {
    #[serde(rename = "claimAppearances")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of links to works in which this claim appears, aside from the one specified in `claim_first_appearance`. Corresponds to `ClaimReview.itemReviewed[@type=Claim].appearance.url`."]
    pub claim_appearances: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "claimAuthor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Info about the author of this claim."]
    pub claim_author: ::std::option::Option<
        ::std::boxed::Box<GoogleFactcheckingFactchecktoolsV1alpha1ClaimAuthor>,
    >,
    #[serde(rename = "claimDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date when the claim was made or entered public discourse. Corresponds to `ClaimReview.itemReviewed.datePublished`."]
    pub claim_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "claimFirstAppearance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to a work in which this claim first appears. Corresponds to `ClaimReview.itemReviewed[@type=Claim].firstAppearance.url`."]
    pub claim_first_appearance: ::std::option::Option<::std::string::String>,
    #[serde(rename = "claimLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location where this claim was made. Corresponds to `ClaimReview.itemReviewed.name`."]
    pub claim_location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "claimReviewed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short summary of the claim being evaluated. Corresponds to `ClaimReview.claimReviewed`."]
    pub claim_reviewed: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rating")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Info about the rating of this claim review."]
    pub rating: ::std::option::Option<
        ::std::boxed::Box<GoogleFactcheckingFactchecktoolsV1alpha1ClaimRating>,
    >,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is optional, and will default to the page URL. We provide this field to allow you the override the default value, but the only permitted override is the page URL plus an optional anchor link (\"page jump\"). Corresponds to `ClaimReview.url`"]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Holds one or more instances of `ClaimReview` markup for a webpage."]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkupPage {
    #[serde(rename = "claimReviewAuthor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Info about the author of this claim review. Similar to the above, semantically these are page-level fields, and each `ClaimReview` on this page will contain the same values."]
    pub claim_review_author: ::std::option::Option<
        ::std::boxed::Box<GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewAuthor>,
    >,
    #[serde(rename = "claimReviewMarkups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of individual claim reviews for this page. Each item in the list corresponds to one `ClaimReview` element."]
    pub claim_review_markups: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkup>,
        >,
    >,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this `ClaimReview` markup page resource, in the form of `pages/{page_id}`. Except for update requests, this field is output-only and should not be set by the user."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pageUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the page associated with this `ClaimReview` markup. While every individual `ClaimReview` has its own URL field, semantically this is a page-level field, and each `ClaimReview` on this page will use this value unless individually overridden. Corresponds to `ClaimReview.url`"]
    pub page_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publishDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date when the fact check was published. Similar to the URL, semantically this is a page-level field, and each `ClaimReview` on this page will contain the same value. Corresponds to `ClaimReview.datePublished`"]
    pub publish_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version ID for this markup. Except for update requests, this field is output-only and should not be set by the user."]
    pub version_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response from searching fact-checked claims."]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1FactCheckedClaimSearchResponse {
    #[serde(rename = "claims")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of claims and all of their associated information."]
    pub claims: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleFactcheckingFactchecktoolsV1alpha1Claim>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The next pagination token in the Search response. It should be used as the `page_token` for the following request. An empty value means no more results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response from listing `ClaimReview` markup."]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1ListClaimReviewMarkupPagesResponse {
    #[serde(rename = "claimReviewMarkupPages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result list of pages of `ClaimReview` markup."]
    pub claim_review_markup_pages: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkupPage>,
        >,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The next pagination token in the Search response. It should be used as the `page_token` for the following request. An empty value means no more results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the publisher."]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1Publisher {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this publisher. For instance, \"Awesome Fact Checks\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "site")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Host-level site name, without the protocol or \"www\" prefix. For instance, \"awesomefactchecks.com\". This value of this field is based purely on the claim review URL."]
    pub site: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct GoogleProtobufEmpty {}
