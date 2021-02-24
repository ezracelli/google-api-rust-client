#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Representation of a license assignment."]
pub struct LicenseAssignment {
    #[serde(rename = "etags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the resource."]
    pub etags: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "license_assignment_defaults :: kind")]
    #[doc = "Identifies the resource as a LicenseAssignment, which is `licensing#licenseAssignment`."]
    pub kind: ::std::string::String,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A product's unique identifier. For more information about products in this version of the API, see Product and SKU IDs."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display Name of the product."]
    pub product_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Link to this page."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "skuId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A product SKU's unique identifier. For more information about available SKUs in this version of the API, see Products and SKUs."]
    pub sku_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "skuName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display Name of the sku of the product."]
    pub sku_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's current primary email address. If the user's email address changes, use the new email address in your API requests. Since a `userId` is subject to change, do not use a `userId` value as a key for persistent data. This key could break if the current user's email address changes. If the `userId` is suspended, the license status changes."]
    pub user_id: ::std::option::Option<::std::string::String>,
}
mod license_assignment_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("licensing#licenseAssignment")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Representation of a license assignment."]
pub struct LicenseAssignmentInsert {
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Email id of the user"]
    pub user_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LicenseAssignmentList {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The LicenseAssignments in this page of results."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LicenseAssignment>>>,
    #[serde(rename = "kind")]
    #[serde(default = "license_assignment_list_defaults :: kind")]
    #[doc = "Identifies the resource as a collection of LicenseAssignments."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token that you must submit in a subsequent request to retrieve additional license results matching your query parameters. The `maxResults` query string is related to the `nextPageToken` since `maxResults` determines how many entries are returned on each next page."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod license_assignment_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("licensing#licenseAssignmentList")
    }
}
