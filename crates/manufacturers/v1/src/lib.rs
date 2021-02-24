#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Attributes of the product. For more information, see https://support.google.com/manufacturers/answer/6124116."]
pub struct Attributes {
    #[serde(rename = "additionalImageLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The additional images of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#addlimage."]
    pub additional_image_link: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Image>>>,
    #[serde(rename = "ageGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target age group of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#agegroup."]
    pub age_group: ::std::option::Option<::std::string::String>,
    #[serde(rename = "brand")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The brand name of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#brand."]
    pub brand: ::std::option::Option<::std::string::String>,
    #[serde(rename = "capacity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The capacity of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#capacity."]
    pub capacity: ::std::option::Option<::std::boxed::Box<Capacity>>,
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The color of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#color."]
    pub color: ::std::option::Option<::std::string::String>,
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The count of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#count."]
    pub count: ::std::option::Option<::std::boxed::Box<Count>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#description."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disclosureDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The disclosure date of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#disclosure."]
    pub disclosure_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "excludedDestination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of excluded destinations."]
    pub excluded_destination: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "featureDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The rich format description of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#featuredesc."]
    pub feature_description:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FeatureDescription>>>,
    #[serde(rename = "flavor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The flavor of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#flavor."]
    pub flavor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The format of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#format."]
    pub format: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gender")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target gender of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#gender."]
    pub gender: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gtin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Global Trade Item Number (GTIN) of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#gtin."]
    pub gtin: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "imageLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The image of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#image."]
    pub image_link: ::std::option::Option<::std::boxed::Box<Image>>,
    #[serde(rename = "includedDestination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of included destinations."]
    pub included_destination: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "itemGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The item group id of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#itemgroupid."]
    pub item_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "material")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The material of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#material."]
    pub material: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mpn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Manufacturer Part Number (MPN) of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#mpn."]
    pub mpn: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pattern")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pattern of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#pattern."]
    pub pattern: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productDetail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The details of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productdetail."]
    pub product_detail: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductDetail>>>,
    #[serde(rename = "productHighlight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The product highlights. For more information, see https://support.google.com/manufacturers/answer/10066942"]
    pub product_highlight: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "productLine")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the group of products related to the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productline."]
    pub product_line: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The canonical name of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productname."]
    pub product_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productPageUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the detail page of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productpage."]
    pub product_page_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type or category of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#producttype."]
    pub product_type: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "releaseDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The release date of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#release."]
    pub release_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "richProductContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rich product content. For more information, see https://support.google.com/manufacturers/answer/9389865"]
    pub rich_product_content: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "scent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The scent of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#scent."]
    pub scent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#size."]
    pub size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sizeSystem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size system of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#sizesystem."]
    pub size_system: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sizeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size type of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#sizetype."]
    pub size_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "suggestedRetailPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested retail price (MSRP) of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#price."]
    pub suggested_retail_price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "targetClientId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target client id. Should only be used in the accounts of the data partners."]
    pub target_client_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "theme")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The theme of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#theme."]
    pub theme: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#title."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "videoLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The videos of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#video."]
    pub video_link: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The capacity of a product. For more information, see https://support.google.com/manufacturers/answer/6124116#capacity."]
pub struct Capacity {
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unit of the capacity, i.e., MB, GB, or TB."]
    pub unit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The numeric value of the capacity."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The number of products in a single package. For more information, see https://support.google.com/manufacturers/answer/6124116#count."]
pub struct Count {
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unit in which these products are counted."]
    pub unit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The numeric value of the number of products in a package."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The destination status."]
pub struct DestinationStatus {
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the destination."]
    pub destination: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the destination."]
    pub status: ::std::option::Option<DestinationStatusStatusEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The status of the destination."]
pub enum DestinationStatusStatusEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "Unspecified status, never used."]
    Unknown,
    #[serde(rename = "ACTIVE")]
    #[doc = "The product is used for this destination."]
    Active,
    #[serde(rename = "PENDING")]
    #[doc = "The decision is still pending."]
    Pending,
    #[serde(rename = "DISAPPROVED")]
    #[doc = "The product is disapproved. Please look at the issues."]
    Disapproved,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A feature description of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#featuredesc."]
pub struct FeatureDescription {
    #[serde(rename = "headline")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short description of the feature."]
    pub headline: ::std::option::Option<::std::string::String>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional image describing the feature."]
    pub image: ::std::option::Option<::std::boxed::Box<Image>>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A detailed description of the feature."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An image."]
pub struct Image {
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the image. For crawled images, this is the provided URL. For uploaded images, this is a serving URL from Google if the image has been processed successfully."]
    pub image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the image. @OutputOnly"]
    pub status: ::std::option::Option<ImageStatusEnum>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the image, i.e., crawled or uploaded. @OutputOnly"]
    pub _type: ::std::option::Option<ImageTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The status of the image. @OutputOnly"]
pub enum ImageStatusEnum {
    #[serde(rename = "STATUS_UNSPECIFIED")]
    #[doc = "The image status is unspecified. Should not be used."]
    StatusUnspecified,
    #[serde(rename = "PENDING_PROCESSING")]
    #[doc = "The image was uploaded and is being processed."]
    PendingProcessing,
    #[serde(rename = "PENDING_CRAWL")]
    #[doc = "The image crawl is still pending."]
    PendingCrawl,
    #[serde(rename = "OK")]
    #[doc = "The image was processed and it meets the requirements."]
    Ok,
    #[serde(rename = "ROBOTED")]
    #[doc = "The image URL is protected by robots.txt file and cannot be crawled."]
    Roboted,
    #[serde(rename = "XROBOTED")]
    #[doc = "The image URL is protected by X-Robots-Tag and cannot be crawled."]
    Xroboted,
    #[serde(rename = "CRAWL_ERROR")]
    #[doc = "There was an error while crawling the image."]
    CrawlError,
    #[serde(rename = "PROCESSING_ERROR")]
    #[doc = "The image cannot be processed."]
    ProcessingError,
    #[serde(rename = "DECODING_ERROR")]
    #[doc = "The image cannot be decoded."]
    DecodingError,
    #[serde(rename = "TOO_BIG")]
    #[doc = "The image is too big."]
    TooBig,
    #[serde(rename = "CRAWL_SKIPPED")]
    #[doc = "The image was manually overridden and will not be crawled."]
    CrawlSkipped,
    #[serde(rename = "HOSTLOADED")]
    #[doc = "The image crawl was postponed to avoid overloading the host."]
    Hostloaded,
    #[serde(rename = "HTTP_404")]
    #[doc = "The image URL returned a \"404 Not Found\" error."]
    Http404,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the image, i.e., crawled or uploaded. @OutputOnly"]
pub enum ImageTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Type is unspecified. Should not be used."]
    TypeUnspecified,
    #[serde(rename = "CRAWLED")]
    #[doc = "The image was crawled from a provided URL."]
    Crawled,
    #[serde(rename = "UPLOADED")]
    #[doc = "The image was uploaded."]
    Uploaded,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Product issue."]
pub struct Issue {
    #[serde(rename = "attribute")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If present, the attribute that triggered the issue. For more information about attributes, see https://support.google.com/manufacturers/answer/6124116."]
    pub attribute: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Longer description of the issue focused on how to resolve it."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The destination this issue applies to."]
    pub destination: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "What needs to happen to resolve the issue."]
    pub resolution: ::std::option::Option<IssueResolutionEnum>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The severity of the issue."]
    pub severity: ::std::option::Option<IssueSeverityEnum>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp when this issue appeared."]
    pub timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Short title describing the nature of the issue."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The server-generated type of the issue, for example, “INCORRECT_TEXT_FORMATTING”, “IMAGE_NOT_SERVEABLE”, etc."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "What needs to happen to resolve the issue."]
pub enum IssueResolutionEnum {
    #[serde(rename = "RESOLUTION_UNSPECIFIED")]
    #[doc = "Unspecified resolution, never used."]
    ResolutionUnspecified,
    #[serde(rename = "USER_ACTION")]
    #[doc = "The user who provided the data must act in order to resolve the issue (for example by correcting some data)."]
    UserAction,
    #[serde(rename = "PENDING_PROCESSING")]
    #[doc = "The issue will be resolved automatically (for example image crawl or Google review). No action is required now. Resolution might lead to another issue (for example if crawl fails)."]
    PendingProcessing,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The severity of the issue."]
pub enum IssueSeverityEnum {
    #[serde(rename = "SEVERITY_UNSPECIFIED")]
    #[doc = "Unspecified severity, never used."]
    SeverityUnspecified,
    #[serde(rename = "ERROR")]
    #[doc = "Error severity. The issue prevents the usage of the whole item."]
    Error,
    #[serde(rename = "WARNING")]
    #[doc = "Warning severity. The issue is either one that prevents the usage of the attribute that triggered it or one that will soon prevent the usage of the whole item."]
    Warning,
    #[serde(rename = "INFO")]
    #[doc = "Info severity. The issue is one that doesn't require immediate attention. It is, for example, used to communicate which attributes are still pending review."]
    Info,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListProductsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of product statuses."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "products")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of the products."]
    pub products: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Product>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A price."]
pub struct Price {
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The numeric value of the price."]
    pub amount: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The currency in which the price is denoted."]
    pub currency: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Product data."]
pub struct Product {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Attributes of the product uploaded to the Manufacturer Center. Manually edited attributes are taken into account."]
    pub attributes: ::std::option::Option<::std::boxed::Box<Attributes>>,
    #[serde(rename = "contentLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content language of the product as a two-letter ISO 639-1 language code (for example, en)."]
    pub content_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destinationStatuses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the destinations."]
    pub destination_statuses:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DestinationStatus>>>,
    #[serde(rename = "issues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A server-generated list of issues associated with the product."]
    pub issues: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Issue>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name in the format `{target_country}:{content_language}:{product_id}`. `target_country` - The target country of the product as a CLDR territory code (for example, US). `content_language` - The content language of the product as a two-letter ISO 639-1 language code (for example, en). `product_id` - The ID of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#id."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parent ID in the format `accounts/{account_id}`. `account_id` - The ID of the Manufacturer Center account."]
    pub parent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#id."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetCountry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target country of the product as a CLDR territory code (for example, US)."]
    pub target_country: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A product detail of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productdetail."]
pub struct ProductDetail {
    #[serde(rename = "attributeName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the attribute."]
    pub attribute_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "attributeValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of the attribute."]
    pub attribute_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sectionName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short section name that can be reused between multiple product details."]
    pub section_name: ::std::option::Option<::std::string::String>,
}
