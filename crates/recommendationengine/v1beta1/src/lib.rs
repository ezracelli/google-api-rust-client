#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message that represents an arbitrary HTTP body. It should only be used for payload formats that can't be represented as JSON, such as raw binary or an HTML page. This message can be used both in streaming and non-streaming API methods in the request as well as the response. It can be used as a top-level request field, which is convenient if one wants to extract parameters from either the URL or HTTP template into the request fields and also want access to the raw HTTP body. Example: message GetResourceRequest { // A unique request id. string request_id = 1; // The raw HTTP body is bound to this field. google.api.HttpBody http_body = 2; } service ResourceService { rpc GetResource(GetResourceRequest) returns (google.api.HttpBody); rpc UpdateResource(google.api.HttpBody) returns (google.protobuf.Empty); } Example with streaming methods: service CaldavService { rpc GetCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); rpc UpdateCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); } Use of this type only changes how the request and response bodies are handled, all other features will continue to work unchanged."]
pub struct GoogleApiHttpBody {
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HTTP Content-Type header value specifying the content type of the body."]
    pub content_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HTTP request/response body as raw binary."]
    pub data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "extensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Application specific response metadata. Must be set in the first response for streaming APIs."]
    pub extensions: ::std::option::Option<
        ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for TriggerCatalogRejoin method."]
pub struct GoogleCloudRecommendationengineV1alphaRejoinCatalogMetadata {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for TriggerCatalogRejoin method."]
pub struct GoogleCloudRecommendationengineV1alphaRejoinCatalogResponse {
    #[serde(rename = "rejoinedUserEventsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of user events that were joined with latest catalog items."]
    pub rejoined_user_events_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata associated with a tune operation."]
pub struct GoogleCloudRecommendationengineV1alphaTuningMetadata {
    #[serde(rename = "recommendationModel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the recommendation model that this tune applies to. Format: projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/eventStores/{event_store_id}/recommendationModels/{recommendation_model_id}"]
    pub recommendation_model: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response associated with a tune operation."]
pub struct GoogleCloudRecommendationengineV1alphaTuningResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "BigQuery source import data from."]
pub struct GoogleCloudRecommendationengineV1beta1BigQuerySource {
    #[serde(rename = "dataSchema")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The schema to use when parsing the data from the source. Supported values for catalog imports: 1: \"catalog_recommendations_ai\" using https://cloud.google.com/recommendations-ai/docs/upload-catalog#json (Default for catalogItems.import) 2: \"catalog_merchant_center\" using https://cloud.google.com/recommendations-ai/docs/upload-catalog#mc Supported values for user event imports: 1: \"user_events_recommendations_ai\" using https://cloud.google.com/recommendations-ai/docs/manage-user-events#import (Default for userEvents.import) 2. \"user_events_ga360\" using https://support.google.com/analytics/answer/3437719?hl=en"]
    pub data_schema: ::std::option::Option<::std::string::String>,
    #[serde(rename = "datasetId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The BigQuery data set to copy the data from."]
    pub dataset_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gcsStagingDir")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Intermediate Cloud Storage directory used for the import. Can be specified if one wants to have the BigQuery export to a specific Cloud Storage directory."]
    pub gcs_staging_dir: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The project id (can be project # or id) that the BigQuery source is in. If not specified, inherits the project id from the parent request."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tableId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The BigQuery table to copy the data from."]
    pub table_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The catalog configuration. Next ID: 5."]
pub struct GoogleCloudRecommendationengineV1beta1Catalog {
    #[serde(rename = "catalogItemLevelConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The catalog item level configuration."]
    pub catalog_item_level_config: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfig>,
    >,
    #[serde(rename = "defaultEventStoreId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ID of the default event store."]
    pub default_event_store_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The catalog display name."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fully qualified resource name of the catalog."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The inline source for the input config for ImportCatalogItems method."]
pub struct GoogleCloudRecommendationengineV1beta1CatalogInlineSource {
    #[serde(rename = "catalogItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A list of catalog items to update/create. Recommended max of 10k items."]
    pub catalog_items: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudRecommendationengineV1beta1CatalogItem>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "CatalogItem captures all metadata information of items to be recommended."]
pub struct GoogleCloudRecommendationengineV1beta1CatalogItem {
    #[serde(rename = "categoryHierarchies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Catalog item categories. This field is repeated for supporting one catalog item belonging to several parallel category hierarchies. For example, if a shoes product belongs to both [\"Shoes & Accessories\" -> \"Shoes\"] and [\"Sports & Fitness\" -> \"Athletic Clothing\" -> \"Shoes\"], it could be represented as: \"categoryHierarchies\": [ { \"categories\": [\"Shoes & Accessories\", \"Shoes\"]}, { \"categories\": [\"Sports & Fitness\", \"Athletic Clothing\", \"Shoes\"] } ]"]
    pub category_hierarchies: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1CatalogItemCategoryHierarchy>,
        >,
    >,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Catalog item description. UTF-8 encoded string with a length limit of 5 KiB."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Catalog item identifier. UTF-8 encoded string with a length limit of 128 bytes. This id must be unique among all catalog items within the same catalog. It should also be used when logging user events in order for the user events to be joined with the Catalog."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "itemAttributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Highly encouraged. Extra catalog item attributes to be included in the recommendation model. For example, for retail products, this could include the store name, vendor, style, color, etc. These are very strong signals for recommendation model, thus we highly recommend providing the item attributes here."]
    pub item_attributes:
        ::std::option::Option<::std::boxed::Box<GoogleCloudRecommendationengineV1beta1FeatureMap>>,
    #[serde(rename = "itemGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Variant group identifier for prediction results. UTF-8 encoded string with a length limit of 128 bytes. This field must be enabled before it can be used. [Learn more](/recommendations-ai/docs/catalog#item-group-id)."]
    pub item_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Deprecated. The model automatically detects the text language. Your catalog can include text in different languages, but duplicating catalog items to provide text in multiple languages can result in degraded model performance."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Metadata specific to retail products."]
    pub product_metadata: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1ProductCatalogItem>,
    >,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Filtering tags associated with the catalog item. Each tag should be a UTF-8 encoded string with a length limit of 1 KiB. This tag can be used for filtering recommendation results by passing the tag as part of the predict request filter."]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Catalog item title. UTF-8 encoded string with a length limit of 1 KiB."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Category represents catalog item category hierarchy."]
pub struct GoogleCloudRecommendationengineV1beta1CatalogItemCategoryHierarchy {
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Catalog item categories. Each category should be a UTF-8 encoded string with a length limit of 2 KiB. Note that the order in the list denotes the specificity (from least to most specific)."]
    pub categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configures the catalog level that users send events to, and the level at which predictions are made."]
pub struct GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfig {
    #[serde(rename = "eventItemLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Level of the catalog at which events are uploaded. See https://cloud.google.com/recommendations-ai/docs/catalog#catalog-levels for more details."]
    pub event_item_level: ::std::option::Option<
        GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigEventItemLevelEnum,
    >,
    #[serde(rename = "predictItemLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Level of the catalog at which predictions are made. See https://cloud.google.com/recommendations-ai/docs/catalog#catalog-levels for more details."]
    pub predict_item_level: ::std::option::Option<
        GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigPredictItemLevelEnum,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Level of the catalog at which events are uploaded. See https://cloud.google.com/recommendations-ai/docs/catalog#catalog-levels for more details."]
pub enum GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigEventItemLevelEnum {
    #[serde(rename = "CATALOG_ITEM_LEVEL_UNSPECIFIED")]
    #[doc = "Unknown value - should never be used."]
    CatalogItemLevelUnspecified,
    #[serde(rename = "VARIANT")]
    #[doc = "Catalog items are at variant level."]
    Variant,
    #[serde(rename = "MASTER")]
    #[doc = "Catalog items are at master level."]
    Master,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Level of the catalog at which predictions are made. See https://cloud.google.com/recommendations-ai/docs/catalog#catalog-levels for more details."]
pub enum GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfigPredictItemLevelEnum {
    #[serde(rename = "CATALOG_ITEM_LEVEL_UNSPECIFIED")]
    #[doc = "Unknown value - should never be used."]
    CatalogItemLevelUnspecified,
    #[serde(rename = "VARIANT")]
    #[doc = "Catalog items are at variant level."]
    Variant,
    #[serde(rename = "MASTER")]
    #[doc = "Catalog items are at master level."]
    Master,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for the `CreatePredictionApiKeyRegistration` method."]
pub struct GoogleCloudRecommendationengineV1beta1CreatePredictionApiKeyRegistrationRequest {
    #[serde(rename = "predictionApiKeyRegistration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The prediction API key registration."]
    pub prediction_api_key_registration: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1PredictionApiKeyRegistration>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "User event details shared by all recommendation types."]
pub struct GoogleCloudRecommendationengineV1beta1EventDetail {
    #[serde(rename = "eventAttributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Extra user event features to include in the recommendation model. For product recommendation, an example of extra user information is traffic_channel, i.e. how user arrives at the site. Users can arrive at the site by coming to the site directly, or coming through Google search, and etc."]
    pub event_attributes:
        ::std::option::Option<::std::boxed::Box<GoogleCloudRecommendationengineV1beta1FeatureMap>>,
    #[serde(rename = "experimentIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A list of identifiers for the independent experiment groups this user event belongs to. This is used to distinguish between user events associated with different experiment setups (e.g. using Recommendation Engine system, using different recommendation models)."]
    pub experiment_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "pageViewId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A unique id of a web page view. This should be kept the same for all user events triggered from the same pageview. For example, an item detail page view could trigger multiple events as the user is browsing the page. The `pageViewId` property should be kept the same for all these events so that they can be grouped together properly. This `pageViewId` will be automatically generated if using the JavaScript pixel."]
    pub page_view_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "recommendationToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Recommendation token included in the recommendation prediction response. This field enables accurate attribution of recommendation model performance. This token enables us to accurately attribute page view or purchase back to the event and the particular predict response containing this clicked/purchased item. If user clicks on product K in the recommendation results, pass the `PredictResponse.recommendationToken` property as a url parameter to product K's page. When recording events on product K's page, log the PredictResponse.recommendation_token to this field. Optional, but highly encouraged for user events that are the result of a recommendation prediction query."]
    pub recommendation_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "referrerUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The referrer url of the current page. When using the JavaScript pixel, this value is filled in automatically."]
    pub referrer_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Complete url (window.location.href) of the user's current page. When using the JavaScript pixel, this value is filled in automatically. Maximum length 5KB."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "FeatureMap represents extra features that customers want to include in the recommendation model for catalogs/user events as categorical/numerical features."]
pub struct GoogleCloudRecommendationengineV1beta1FeatureMap {
    #[serde(rename = "categoricalFeatures")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Categorical features that can take on one of a limited number of possible values. Some examples would be the brand/maker of a product, or country of a customer. Feature names and values must be UTF-8 encoded strings. For example: `{ \"colors\": {\"value\": [\"yellow\", \"green\"]}, \"sizes\": {\"value\":[\"S\", \"M\"]}`"]
    pub categorical_features: ::std::option::Option<
        ::std::collections::BTreeMap<
            String,
            ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1FeatureMapStringList>,
        >,
    >,
    #[serde(rename = "numericalFeatures")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Numerical features. Some examples would be the height/weight of a product, or age of a customer. Feature names must be UTF-8 encoded strings. For example: `{ \"lengths_cm\": {\"value\":[2.3, 15.4]}, \"heights_cm\": {\"value\":[8.1, 6.4]} }`"]
    pub numerical_features: ::std::option::Option<
        ::std::collections::BTreeMap<
            String,
            ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1FeatureMapFloatList>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of float features."]
pub struct GoogleCloudRecommendationengineV1beta1FeatureMapFloatList {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Float feature value."]
    pub value: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of string features."]
pub struct GoogleCloudRecommendationengineV1beta1FeatureMapStringList {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "String feature value with a length limit of 128 bytes."]
    pub value: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Google Cloud Storage location for input content. format."]
pub struct GoogleCloudRecommendationengineV1beta1GcsSource {
    #[serde(rename = "inputUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Google Cloud Storage URIs to input files. URI can be up to 2000 characters long. URIs can match the full object path (for example, gs://bucket/directory/object.json) or a pattern matching one or more files, such as gs://bucket/directory/*.json. A request can contain at most 100 files, and each file can be up to 2 GB. See [Importing catalog information](/recommendations-ai/docs/upload-catalog) for the expected file format and setup instructions."]
    pub input_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "jsonSchema")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The schema to use when parsing the data from the source. Supported values for catalog imports: 1: \"catalog_recommendations_ai\" using https://cloud.google.com/recommendations-ai/docs/upload-catalog#json (Default for catalogItems.import) 2: \"catalog_merchant_center\" using https://cloud.google.com/recommendations-ai/docs/upload-catalog#mc Supported values for user events imports: 1: \"user_events_recommendations_ai\" using https://cloud.google.com/recommendations-ai/docs/manage-user-events#import (Default for userEvents.import) 2. \"user_events_ga360\" using https://support.google.com/analytics/answer/3437719?hl=en"]
    pub json_schema: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Catalog item thumbnail/detail image."]
pub struct GoogleCloudRecommendationengineV1beta1Image {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Height of the image in number of pixels."]
    pub height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. URL of the image with a length limit of 5 KiB."]
    pub uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Width of the image in number of pixels."]
    pub width: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for Import methods."]
pub struct GoogleCloudRecommendationengineV1beta1ImportCatalogItemsRequest {
    #[serde(rename = "errorsConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The desired location of errors incurred during the Import."]
    pub errors_config: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1ImportErrorsConfig>,
    >,
    #[serde(rename = "inputConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The desired input location of the data."]
    pub input_config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudRecommendationengineV1beta1InputConfig>>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Unique identifier provided by client, within the ancestor dataset scope. Ensures idempotency and used for request deduplication. Server-generated if unspecified. Up to 128 characters long. This is returned as google.longrunning.Operation.name in the response."]
    pub request_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates which fields in the provided imported 'items' to update. If not set, will by default update all fields."]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response of the ImportCatalogItemsRequest. If the long running operation is done, then this message is returned by the google.longrunning.Operations.response field if the operation was successful."]
pub struct GoogleCloudRecommendationengineV1beta1ImportCatalogItemsResponse {
    #[serde(rename = "errorSamples")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A sample of errors encountered while processing the request."]
    pub error_samples: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    #[serde(rename = "errorsConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Echoes the destination for the complete errors in the request if set."]
    pub errors_config: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1ImportErrorsConfig>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration of destination for Import related errors."]
pub struct GoogleCloudRecommendationengineV1beta1ImportErrorsConfig {
    #[serde(rename = "gcsPrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Cloud Storage path for import errors. This must be an empty, existing Cloud Storage bucket. Import errors will be written to a file in this bucket, one per line, as a JSON-encoded `google.rpc.Status` message."]
    pub gcs_prefix: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata related to the progress of the Import operation. This will be returned by the google.longrunning.Operation.metadata field."]
pub struct GoogleCloudRecommendationengineV1beta1ImportMetadata {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Operation create time."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "failureCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Count of entries that encountered errors while processing."]
    pub failure_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the operation."]
    pub operation_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Id of the request / operation. This is parroting back the requestId that was passed in the request."]
    pub request_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "successCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Count of entries that were processed successfully."]
    pub success_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Operation last update time. If the operation is done, this is also the finish time."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for the ImportUserEvents request."]
pub struct GoogleCloudRecommendationengineV1beta1ImportUserEventsRequest {
    #[serde(rename = "errorsConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The desired location of errors incurred during the Import."]
    pub errors_config: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1ImportErrorsConfig>,
    >,
    #[serde(rename = "inputConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The desired input location of the data."]
    pub input_config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudRecommendationengineV1beta1InputConfig>>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Unique identifier provided by client, within the ancestor dataset scope. Ensures idempotency for expensive long running operations. Server-generated if unspecified. Up to 128 characters long. This is returned as google.longrunning.Operation.name in the response. Note that this field must not be set if the desired input config is catalog_inline_source."]
    pub request_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response of the ImportUserEventsRequest. If the long running operation was successful, then this message is returned by the google.longrunning.Operations.response field if the operation was successful."]
pub struct GoogleCloudRecommendationengineV1beta1ImportUserEventsResponse {
    #[serde(rename = "errorSamples")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A sample of errors encountered while processing the request."]
    pub error_samples: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    #[serde(rename = "errorsConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Echoes the destination for the complete errors if this field was set in the request."]
    pub errors_config: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1ImportErrorsConfig>,
    >,
    #[serde(rename = "importSummary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Aggregated statistics of user event import status."]
    pub import_summary: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1UserEventImportSummary>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The input config source."]
pub struct GoogleCloudRecommendationengineV1beta1InputConfig {
    #[serde(rename = "bigQuerySource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "BigQuery input source."]
    pub big_query_source: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1BigQuerySource>,
    >,
    #[serde(rename = "catalogInlineSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Inline source for the input content for Catalog items."]
    pub catalog_inline_source: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1CatalogInlineSource>,
    >,
    #[serde(rename = "gcsSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Cloud Storage location for the input content."]
    pub gcs_source:
        ::std::option::Option<::std::boxed::Box<GoogleCloudRecommendationengineV1beta1GcsSource>>,
    #[serde(rename = "userEventInlineSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Inline source for the input content for UserEvents."]
    pub user_event_inline_source: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1UserEventInlineSource>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListCatalogItems method."]
pub struct GoogleCloudRecommendationengineV1beta1ListCatalogItemsResponse {
    #[serde(rename = "catalogItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The catalog items."]
    pub catalog_items: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudRecommendationengineV1beta1CatalogItem>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If empty, the list is complete. If nonempty, the token to pass to the next request's ListCatalogItemRequest.page_token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListCatalogs method."]
pub struct GoogleCloudRecommendationengineV1beta1ListCatalogsResponse {
    #[serde(rename = "catalogs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. All the customer's catalogs."]
    pub catalogs: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudRecommendationengineV1beta1Catalog>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token, if not returned indicates the last page."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the `ListPredictionApiKeyRegistrations`."]
pub struct GoogleCloudRecommendationengineV1beta1ListPredictionApiKeyRegistrationsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If empty, the list is complete. If nonempty, pass the token to the next request's `ListPredictionApiKeysRegistrationsRequest.pageToken`."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "predictionApiKeyRegistrations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of registered API keys."]
    pub prediction_api_key_registrations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1PredictionApiKeyRegistration>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListUserEvents method."]
pub struct GoogleCloudRecommendationengineV1beta1ListUserEventsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If empty, the list is complete. If nonempty, the token to pass to the next request's ListUserEvents.page_token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user events."]
    pub user_events: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudRecommendationengineV1beta1UserEvent>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for Predict method."]
pub struct GoogleCloudRecommendationengineV1beta1PredictRequest {
    #[serde(rename = "dryRun")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Use dryRun mode for this prediction query. If set to true, a dummy model will be used that returns arbitrary catalog items. Note that the dryRun mode should only be used for testing the API, or if the model is not ready."]
    pub dry_run: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Filter for restricting prediction results. Accepts values for tags and the `filterOutOfStockItems` flag. * Tag expressions. Restricts predictions to items that match all of the specified tags. Boolean operators `OR` and `NOT` are supported if the expression is enclosed in parentheses, and must be separated from the tag values by a space. `-\"tagA\"` is also supported and is equivalent to `NOT \"tagA\"`. Tag values must be double quoted UTF-8 encoded strings with a size limit of 1 KiB. * filterOutOfStockItems. Restricts predictions to items that do not have a stockState value of OUT_OF_STOCK. Examples: * tag=(\"Red\" OR \"Blue\") tag=\"New-Arrival\" tag=(NOT \"promotional\") * filterOutOfStockItems tag=(-\"promotional\") * filterOutOfStockItems If your filter blocks all prediction results, nothing will be returned. If you want generic (unfiltered) popular items to be returned instead, set `strictFiltering` to false in `PredictRequest.params`."]
    pub filter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The labels for the predict request. * Label keys can contain lowercase letters, digits and hyphens, must start with a letter, and must end with a letter or digit. * Non-zero label values can contain lowercase letters, digits and hyphens, must start with a letter, and must end with a letter or digit. * No more than 64 labels can be associated with a given request. See https://goo.gl/xmQnxf for more information on and examples of labels."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Maximum number of results to return per page. Set this property to the number of prediction results required. If zero, the service will choose a reasonable default."]
    pub page_size: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The previous PredictResponse.next_page_token."]
    pub page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "params")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Additional domain specific parameters for the predictions. Allowed values: * `returnCatalogItem`: Boolean. If set to true, the associated catalogItem object will be returned in the `PredictResponse.PredictionResult.itemMetadata` object in the method response. * `returnItemScore`: Boolean. If set to true, the prediction 'score' corresponding to each returned item will be set in the `metadata` field in the prediction response. The given 'score' indicates the probability of an item being clicked/purchased given the user's context and history. * `strictFiltering`: Boolean. True by default. If set to false, the service will return generic (unfiltered) popular items instead of empty if your filter blocks all prediction results."]
    pub params: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "userEvent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Context about the user, what they are looking at and what action they took to trigger the predict request. Note that this user event detail won't be ingested to userEvent logs. Thus, a separate userEvent write request is required for event logging."]
    pub user_event:
        ::std::option::Option<::std::boxed::Box<GoogleCloudRecommendationengineV1beta1UserEvent>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for predict method."]
pub struct GoogleCloudRecommendationengineV1beta1PredictResponse {
    #[serde(rename = "dryRun")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if the dryRun property was set in the request."]
    pub dry_run: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "itemsMissingInCatalog")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IDs of items in the request that were missing from the catalog."]
    pub items_missing_in_catalog: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional domain specific prediction response metadata."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If empty, the list is complete. If nonempty, the token to pass to the next request's PredictRequest.page_token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "recommendationToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique recommendation token. This should be included in the user event logs resulting from this recommendation, which enables accurate attribution of recommendation model performance."]
    pub recommendation_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of recommended items. The order represents the ranking (from the most relevant item to the least)."]
    pub results: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<
                GoogleCloudRecommendationengineV1beta1PredictResponsePredictionResult,
            >,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "PredictionResult represents the recommendation prediction results."]
pub struct GoogleCloudRecommendationengineV1beta1PredictResponsePredictionResult {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the recommended catalog item"]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "itemMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional item metadata / annotations. Possible values: * `catalogItem`: JSON representation of the catalogItem. Will be set if `returnCatalogItem` is set to true in `PredictRequest.params`. * `score`: Prediction score in double value. Will be set if `returnItemScore` is set to true in `PredictRequest.params`."]
    pub item_metadata:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Registered Api Key."]
pub struct GoogleCloudRecommendationengineV1beta1PredictionApiKeyRegistration {
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API key."]
    pub api_key: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ProductCatalogItem captures item metadata specific to retail products."]
pub struct GoogleCloudRecommendationengineV1beta1ProductCatalogItem {
    #[serde(rename = "availableQuantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The available quantity of the item."]
    pub available_quantity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "canonicalProductUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Canonical URL directly linking to the item detail page with a length limit of 5 KiB.."]
    pub canonical_product_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "costs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A map to pass the costs associated with the product. For example: {\"manufacturing\": 45.5} The profit of selling this item is computed like so: * If 'exactPrice' is provided, profit = displayPrice - sum(costs) * If 'priceRange' is provided, profit = minPrice - sum(costs)"]
    pub costs: ::std::option::Option<::std::collections::BTreeMap<String, ::std::primitive::f64>>,
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Only required if the price is set. Currency code for price/costs. Use three-character ISO-4217 code."]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "exactPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The exact product price."]
    pub exact_price: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1ProductCatalogItemExactPrice>,
    >,
    #[serde(rename = "images")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Product images for the catalog item."]
    pub images: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudRecommendationengineV1beta1Image>>,
    >,
    #[serde(rename = "priceRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The product price range."]
    pub price_range: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1ProductCatalogItemPriceRange>,
    >,
    #[serde(rename = "stockState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Online stock state of the catalog item. Default is `IN_STOCK`."]
    pub stock_state: ::std::option::Option<
        GoogleCloudRecommendationengineV1beta1ProductCatalogItemStockStateEnum,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Online stock state of the catalog item. Default is `IN_STOCK`."]
pub enum GoogleCloudRecommendationengineV1beta1ProductCatalogItemStockStateEnum {
    #[serde(rename = "STOCK_STATE_UNSPECIFIED")]
    #[doc = "Default item stock status. Should never be used."]
    StockStateUnspecified,
    #[serde(rename = "IN_STOCK")]
    #[doc = "Item in stock."]
    InStock,
    #[serde(rename = "OUT_OF_STOCK")]
    #[doc = "Item out of stock."]
    OutOfStock,
    #[serde(rename = "PREORDER")]
    #[doc = "Item that is in pre-order state."]
    Preorder,
    #[serde(rename = "BACKORDER")]
    #[doc = "Item that is back-ordered (i.e. temporarily out of stock)."]
    Backorder,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Exact product price."]
pub struct GoogleCloudRecommendationengineV1beta1ProductCatalogItemExactPrice {
    #[serde(rename = "displayPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Display price of the product."]
    pub display_price: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "originalPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Price of the product without any discount. If zero, by default set to be the 'displayPrice'."]
    pub original_price: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Product price range when there are a range of prices for different variations of the same product."]
pub struct GoogleCloudRecommendationengineV1beta1ProductCatalogItemPriceRange {
    #[serde(rename = "max")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The maximum product price."]
    pub max: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "min")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The minimum product price."]
    pub min: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Detailed product information associated with a user event."]
pub struct GoogleCloudRecommendationengineV1beta1ProductDetail {
    #[serde(rename = "availableQuantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Quantity of the products in stock when a user event happens. Optional. If provided, this overrides the available quantity in Catalog for this event. and can only be set if `stock_status` is set to `IN_STOCK`. Note that if an item is out of stock, you must set the `stock_state` field to be `OUT_OF_STOCK`. Leaving this field unspecified / as zero is not sufficient to mark the item out of stock."]
    pub available_quantity: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Currency code for price/costs. Use three-character ISO-4217 code. Required only if originalPrice or displayPrice is set."]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Display price of the product (e.g. discounted price). If provided, this will override the display price in Catalog for this product."]
    pub display_price: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Catalog item ID. UTF-8 encoded string with a length limit of 128 characters."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "itemAttributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Extra features associated with a product in the user event."]
    pub item_attributes:
        ::std::option::Option<::std::boxed::Box<GoogleCloudRecommendationengineV1beta1FeatureMap>>,
    #[serde(rename = "originalPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Original price of the product. If provided, this will override the original price in Catalog for this product."]
    pub original_price: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Quantity of the product associated with the user event. For example, this field will be 2 if two products are added to the shopping cart for `add-to-cart` event. Required for `add-to-cart`, `add-to-list`, `remove-from-cart`, `checkout-start`, `purchase-complete`, `refund` event types."]
    pub quantity: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "stockState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Item stock state. If provided, this overrides the stock state in Catalog for items in this event."]
    pub stock_state:
        ::std::option::Option<GoogleCloudRecommendationengineV1beta1ProductDetailStockStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Item stock state. If provided, this overrides the stock state in Catalog for items in this event."]
pub enum GoogleCloudRecommendationengineV1beta1ProductDetailStockStateEnum {
    #[serde(rename = "STOCK_STATE_UNSPECIFIED")]
    #[doc = "Default item stock status. Should never be used."]
    StockStateUnspecified,
    #[serde(rename = "IN_STOCK")]
    #[doc = "Item in stock."]
    InStock,
    #[serde(rename = "OUT_OF_STOCK")]
    #[doc = "Item out of stock."]
    OutOfStock,
    #[serde(rename = "PREORDER")]
    #[doc = "Item that is in pre-order state."]
    Preorder,
    #[serde(rename = "BACKORDER")]
    #[doc = "Item that is back-ordered (i.e. temporarily out of stock)."]
    Backorder,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ProductEventDetail captures user event information specific to retail products."]
pub struct GoogleCloudRecommendationengineV1beta1ProductEventDetail {
    #[serde(rename = "cartId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The id or name of the associated shopping cart. This id is used to associate multiple items added or present in the cart before purchase. This can only be set for `add-to-cart`, `remove-from-cart`, `checkout-start`, `purchase-complete`, or `shopping-cart-page-view` events."]
    pub cart_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "listId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for `add-to-list` and `remove-from-list` events. The id or name of the list that the item is being added to or removed from. Other event types should not set this field."]
    pub list_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pageCategories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for `category-page-view` events. At least one of search_query or page_categories is required for `search` events. Other event types should not set this field. The categories associated with a category page. Category pages include special pages such as sales or promotions. For instance, a special sale page may have the category hierarchy: categories : [\"Sales\", \"2017 Black Friday Deals\"]."]
    pub page_categories: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1CatalogItemCategoryHierarchy>,
        >,
    >,
    #[serde(rename = "productDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The main product details related to the event. This field is required for the following event types: * `add-to-cart` * `add-to-list` * `checkout-start` * `detail-page-view` * `purchase-complete` * `refund` * `remove-from-cart` * `remove-from-list` This field is optional for the following event types: * `page-visit` * `shopping-cart-page-view` - note that 'product_details' should be set for this unless the shopping cart is empty. * `search` (highly encouraged) In a `search` event, this field represents the products returned to the end user on the current page (the end user may have not finished broswing the whole page yet). When a new page is returned to the end user, after pagination/filtering/ordering even for the same query, a new SEARCH event with different product_details is desired. The end user may have not finished broswing the whole page yet. This field is not allowed for the following event types: * `category-page-view` * `home-page-view`"]
    pub product_details: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudRecommendationengineV1beta1ProductDetail>>,
    >,
    #[serde(rename = "purchaseTransaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A transaction represents the entire purchase transaction. Required for `purchase-complete` events. Optional for `checkout-start` events. Other event types should not set this field."]
    pub purchase_transaction: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1PurchaseTransaction>,
    >,
    #[serde(rename = "searchQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "At least one of search_query or page_categories is required for `search` events. Other event types should not set this field. The user's search query as UTF-8 encoded text with a length limit of 5 KiB."]
    pub search_query: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A transaction represents the entire purchase transaction."]
pub struct GoogleCloudRecommendationengineV1beta1PurchaseTransaction {
    #[serde(rename = "costs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. All the costs associated with the product. These can be manufacturing costs, shipping expenses not borne by the end user, or any other costs. Total product cost such that profit = revenue - (sum(taxes) + sum(costs)) If product_cost is not set, then profit = revenue - tax - shipping - sum(CatalogItem.costs). If CatalogItem.cost is not specified for one of the items, CatalogItem.cost based profit *cannot* be calculated for this Transaction."]
    pub costs: ::std::option::Option<::std::collections::BTreeMap<String, ::std::primitive::f64>>,
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Currency code. Use three-character ISO-4217 code. This field is not required if the event type is `refund`."]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The transaction ID with a length limit of 128 bytes."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revenue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Total revenue or grand total associated with the transaction. This value include shipping, tax, or other adjustments to total revenue that you want to include as part of your revenue calculations. This field is not required if the event type is `refund`."]
    pub revenue: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "taxes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. All the taxes associated with the transaction."]
    pub taxes: ::std::option::Option<::std::collections::BTreeMap<String, ::std::primitive::f64>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata related to the progress of the PurgeUserEvents operation. This will be returned by the google.longrunning.Operation.metadata field."]
pub struct GoogleCloudRecommendationengineV1beta1PurgeUserEventsMetadata {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Operation create time."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the request / operation."]
    pub operation_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for PurgeUserEvents method."]
pub struct GoogleCloudRecommendationengineV1beta1PurgeUserEventsRequest {
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The filter string to specify the events to be deleted. Empty string filter is not allowed. The eligible fields for filtering are: * `eventType`: UserEvent.eventType field of type string. * `eventTime`: in ISO 8601 \"zulu\" format. * `visitorId`: field of type string. Specifying this will delete all events associated with a visitor. * `userId`: field of type string. Specifying this will delete all events associated with a user. Examples: * Deleting all events in a time range: `eventTime > \"2012-04-23T18:25:43.511Z\" eventTime < \"2012-04-23T18:30:43.511Z\"` * Deleting specific eventType in time range: `eventTime > \"2012-04-23T18:25:43.511Z\" eventType = \"detail-page-view\"` * Deleting all events for a specific visitor: `visitorId = \"visitor1024\"` The filtering fields are assumed to have an implicit AND."]
    pub filter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The default value is false. Override this flag to true to actually perform the purge. If the field is not set to true, a sampling of events to be deleted will be returned."]
    pub force: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response of the PurgeUserEventsRequest. If the long running operation is successfully done, then this message is returned by the google.longrunning.Operations.response field."]
pub struct GoogleCloudRecommendationengineV1beta1PurgeUserEventsResponse {
    #[serde(rename = "purgedEventsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total count of events purged as a result of the operation."]
    pub purged_events_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userEventsSample")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A sampling of events deleted (or will be deleted) depending on the `force` property in the request. Max of 500 items will be returned."]
    pub user_events_sample: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudRecommendationengineV1beta1UserEvent>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for RejoinUserEvents method."]
pub struct GoogleCloudRecommendationengineV1beta1RejoinUserEventsMetadata {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for CatalogRejoin method."]
pub struct GoogleCloudRecommendationengineV1beta1RejoinUserEventsRequest {
    #[serde(rename = "userEventRejoinScope")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of the catalog rejoin to define the scope and range of the user events to be rejoined with catalog items."]
    pub user_event_rejoin_scope: ::std::option::Option<
        GoogleCloudRecommendationengineV1beta1RejoinUserEventsRequestUserEventRejoinScopeEnum,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of the catalog rejoin to define the scope and range of the user events to be rejoined with catalog items."]
pub enum GoogleCloudRecommendationengineV1beta1RejoinUserEventsRequestUserEventRejoinScopeEnum {
    #[serde(rename = "USER_EVENT_REJOIN_SCOPE_UNSPECIFIED")]
    #[doc = "Rejoin catalogs with all events including both joined events and unjoined events."]
    UserEventRejoinScopeUnspecified,
    #[serde(rename = "JOINED_EVENTS")]
    #[doc = "Only rejoin catalogs with joined events."]
    JoinedEvents,
    #[serde(rename = "UNJOINED_EVENTS")]
    #[doc = "Only rejoin catalogs with unjoined events."]
    UnjoinedEvents,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for RejoinUserEvents method."]
pub struct GoogleCloudRecommendationengineV1beta1RejoinUserEventsResponse {
    #[serde(rename = "rejoinedUserEventsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of user events that were joined with latest catalog items."]
    pub rejoined_user_events_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "UserEvent captures all metadata information recommendation engine needs to know about how end users interact with customers' website."]
pub struct GoogleCloudRecommendationengineV1beta1UserEvent {
    #[serde(rename = "eventDetail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. User event detailed information common across different recommendation types."]
    pub event_detail:
        ::std::option::Option<::std::boxed::Box<GoogleCloudRecommendationengineV1beta1EventDetail>>,
    #[serde(rename = "eventSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. This field should *not* be set when using JavaScript pixel or the Recommendations AI Tag. Defaults to `EVENT_SOURCE_UNSPECIFIED`."]
    pub event_source:
        ::std::option::Option<GoogleCloudRecommendationengineV1beta1UserEventEventSourceEnum>,
    #[serde(rename = "eventTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Only required for ImportUserEvents method. Timestamp of user event created."]
    pub event_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "eventType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. User event type. Allowed values are: * `add-to-cart` Products being added to cart. * `add-to-list` Items being added to a list (shopping list, favorites etc). * `category-page-view` Special pages such as sale or promotion pages viewed. * `checkout-start` User starting a checkout process. * `detail-page-view` Products detail page viewed. * `home-page-view` Homepage viewed. * `page-visit` Generic page visits not included in the event types above. * `purchase-complete` User finishing a purchase. * `refund` Purchased items being refunded or returned. * `remove-from-cart` Products being removed from cart. * `remove-from-list` Items being removed from a list. * `search` Product search. * `shopping-cart-page-view` User viewing a shopping cart. * `impression` List of items displayed. Used by Google Tag Manager."]
    pub event_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productEventDetail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Retail product specific user event metadata. This field is required for the following event types: * `add-to-cart` * `add-to-list` * `category-page-view` * `checkout-start` * `detail-page-view` * `purchase-complete` * `refund` * `remove-from-cart` * `remove-from-list` * `search` This field is optional for the following event types: * `page-visit` * `shopping-cart-page-view` - note that 'product_event_detail' should be set for this unless the shopping cart is empty. This field is not allowed for the following event types: * `home-page-view`"]
    pub product_event_detail: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudRecommendationengineV1beta1ProductEventDetail>,
    >,
    #[serde(rename = "userInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. User information."]
    pub user_info:
        ::std::option::Option<::std::boxed::Box<GoogleCloudRecommendationengineV1beta1UserInfo>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. This field should *not* be set when using JavaScript pixel or the Recommendations AI Tag. Defaults to `EVENT_SOURCE_UNSPECIFIED`."]
pub enum GoogleCloudRecommendationengineV1beta1UserEventEventSourceEnum {
    #[serde(rename = "EVENT_SOURCE_UNSPECIFIED")]
    #[doc = "Unspecified event source."]
    EventSourceUnspecified,
    #[serde(rename = "AUTOML")]
    #[doc = "The event is ingested via a javascript pixel or Recommendations AI Tag through automl datalayer or JS Macros."]
    Automl,
    #[serde(rename = "ECOMMERCE")]
    #[doc = "The event is ingested via Recommendations AI Tag through Enhanced Ecommerce datalayer."]
    Ecommerce,
    #[serde(rename = "BATCH_UPLOAD")]
    #[doc = "The event is ingested via Import user events API."]
    BatchUpload,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A summary of import result. The UserEventImportSummary summarizes the import status for user events."]
pub struct GoogleCloudRecommendationengineV1beta1UserEventImportSummary {
    #[serde(rename = "joinedEventsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Count of user events imported with complete existing catalog information."]
    pub joined_events_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unjoinedEventsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Count of user events imported, but with catalog information not found in the imported catalog."]
    pub unjoined_events_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The inline source for the input config for ImportUserEvents method."]
pub struct GoogleCloudRecommendationengineV1beta1UserEventInlineSource {
    #[serde(rename = "userEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A list of user events to import. Recommended max of 10k items."]
    pub user_events: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudRecommendationengineV1beta1UserEvent>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information of end users."]
pub struct GoogleCloudRecommendationengineV1beta1UserInfo {
    #[serde(rename = "directUserRequest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates if the request is made directly from the end user in which case the user_agent and ip_address fields can be populated from the HTTP request. This should *not* be set when using the javascript pixel. This flag should be set only if the API request is made directly from the end user such as a mobile app (and not if a gateway or a server is processing and pushing the user events)."]
    pub direct_user_request: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. IP address of the user. This could be either IPv4 (e.g. 104.133.9.80) or IPv6 (e.g. 2001:0db8:85a3:0000:0000:8a2e:0370:7334). This should *not* be set when using the javascript pixel or if `direct_user_request` is set. Used to extract location information for personalization."]
    pub ip_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userAgent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. User agent as included in the HTTP header. UTF-8 encoded string with a length limit of 1 KiB. This should *not* be set when using the JavaScript pixel or if `directUserRequest` is set."]
    pub user_agent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Unique identifier for logged-in user with a length limit of 128 bytes. Required only for logged-in users."]
    pub user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "visitorId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A unique identifier for tracking visitors with a length limit of 128 bytes. For example, this could be implemented with a http cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor log in/out of the website. Maximum length 128 bytes. Cannot be empty."]
    pub visitor_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Operations.ListOperations."]
pub struct GoogleLongrunningListOperationsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The standard List next-page token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of operations that matches the specified filter in the request."]
    pub operations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleLongrunningOperation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This resource represents a long-running operation that is the result of a network API call."]
pub struct GoogleLongrunningOperation {
    #[serde(rename = "done")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
    pub done: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error result of the operation in case of failure or cancellation."]
    pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
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
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct GoogleProtobufEmpty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
pub struct GoogleRpcStatus {
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