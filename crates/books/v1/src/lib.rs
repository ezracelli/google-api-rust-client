#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Annotation {
    #[serde(rename = "afterSelectedText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Anchor text after excerpt. For requests, if the user bookmarked a screen that has no flowing text on it, then this field should be empty."]
    pub after_selected_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "beforeSelectedText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Anchor text before excerpt. For requests, if the user bookmarked a screen that has no flowing text on it, then this field should be empty."]
    pub before_selected_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clientVersionRanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selection ranges sent from the client."]
    pub client_version_ranges: ::std::option::Option<AnnotationClientVersionRanges>,
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp for the created time of this annotation."]
    pub created: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currentVersionRanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selection ranges for the most recent content version."]
    pub current_version_ranges: ::std::option::Option<AnnotationCurrentVersionRanges>,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-created data for this annotation."]
    pub data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that this annotation is deleted."]
    pub deleted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "highlightStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The highlight style for this annotation."]
    pub highlight_style: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Id of this annotation, in the form of a GUID."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "layerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The layer this annotation is for."]
    pub layer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "layerSummary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub layer_summary: ::std::option::Option<AnnotationLayerSummary>,
    #[serde(rename = "pageIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pages that this annotation spans."]
    pub page_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "selectedText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Excerpt from the volume."]
    pub selected_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to this resource."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp for the last time this annotation was modified."]
    pub updated: ::std::option::Option<::std::string::String>,
    #[serde(rename = "volumeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The volume that this annotation belongs to."]
    pub volume_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Selection ranges sent from the client."]
pub struct AnnotationClientVersionRanges {
    #[serde(rename = "cfiRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Range in CFI format for this annotation sent by client."]
    pub cfi_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
    #[serde(rename = "contentVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content version the client sent in."]
    pub content_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gbImageRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Range in GB image format for this annotation sent by client."]
    pub gb_image_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
    #[serde(rename = "gbTextRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Range in GB text format for this annotation sent by client."]
    pub gb_text_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
    #[serde(rename = "imageCfiRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Range in image CFI format for this annotation sent by client."]
    pub image_cfi_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Selection ranges for the most recent content version."]
pub struct AnnotationCurrentVersionRanges {
    #[serde(rename = "cfiRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Range in CFI format for this annotation for version above."]
    pub cfi_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
    #[serde(rename = "contentVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content version applicable to ranges below."]
    pub content_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gbImageRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Range in GB image format for this annotation for version above."]
    pub gb_image_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
    #[serde(rename = "gbTextRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Range in GB text format for this annotation for version above."]
    pub gb_text_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
    #[serde(rename = "imageCfiRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Range in image CFI format for this annotation for version above."]
    pub image_cfi_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnnotationLayerSummary {
    #[serde(rename = "allowedCharacterCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum allowed characters on this layer, especially for the \"copy\" layer."]
    pub allowed_character_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "limitType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of limitation on this layer. \"limited\" or \"unlimited\" for the \"copy\" layer."]
    pub limit_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "remainingCharacterCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Remaining allowed characters on this layer, especially for the \"copy\" layer."]
    pub remaining_character_count: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Annotations {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of annotations."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Annotation>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to pass in for pagination for the next page. This will not be present if this request does not have more results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of annotations found. This may be greater than the number of notes returned in this response if results have been paginated."]
    pub total_items: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnnotationsSummary {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "layers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub layers: ::std::option::Option<::std::vec::Vec<AnnotationsSummaryLayers>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnnotationsSummaryLayers {
    #[serde(rename = "allowedCharacterCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_character_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "layerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub layer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "limitType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "remainingCharacterCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub remaining_character_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub updated: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Annotationsdata {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of Annotation Data."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GeoAnnotationdata>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type"]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to pass in for pagination for the next page. This will not be present if this request does not have more results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of volume annotations found."]
    pub total_items: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BooksAnnotationsRange {
    #[serde(rename = "endOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The offset from the ending position."]
    pub end_offset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endPosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ending position for the range."]
    pub end_position: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The offset from the starting position."]
    pub start_offset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startPosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The starting position for the range."]
    pub start_position: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BooksCloudloadingResource {
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub author: ::std::option::Option<::std::string::String>,
    #[serde(rename = "processingState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub processing_state: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "volumeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub volume_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BooksVolumesRecommendedRateResponse {
    #[serde(rename = "consistency_token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub consistency_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bookshelf {
    #[serde(rename = "access")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this bookshelf is PUBLIC or PRIVATE."]
    pub access: ::std::option::Option<::std::string::String>,
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Created time for this bookshelf (formatted UTC timestamp with millisecond resolution)."]
    pub created: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of this bookshelf."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Id of this bookshelf, only unique by user."]
    pub id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type for bookshelf metadata."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to this resource."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of this bookshelf."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last modified time of this bookshelf (formatted UTC timestamp with millisecond resolution)."]
    pub updated: ::std::option::Option<::std::string::String>,
    #[serde(rename = "volumeCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of volumes in this bookshelf."]
    pub volume_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "volumesLastUpdated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last time a volume was added or removed from this bookshelf (formatted UTC timestamp with millisecond resolution)."]
    pub volumes_last_updated: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bookshelves {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of bookshelves."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Bookshelf>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Category {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of onboarding categories."]
    pub items: ::std::option::Option<::std::vec::Vec<CategoryItems>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CategoryItems {
    #[serde(rename = "badgeUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub badge_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "categoryId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub category_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConcurrentAccessRestriction {
    #[serde(rename = "deviceAllowed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether access is granted for this (user, device, volume)."]
    pub device_allowed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxConcurrentDevices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number of concurrent access licenses for this volume."]
    pub max_concurrent_devices: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error/warning message."]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nonce")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Client nonce for verification. Download access and client-validation only."]
    pub nonce: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error/warning reason code."]
    pub reason_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "restricted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this volume has any concurrent access restrictions."]
    pub restricted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "signature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Response signature."]
    pub signature: ::std::option::Option<::std::string::String>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Client app identifier for verification. Download access and client-validation only."]
    pub source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeWindowSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time in seconds for license auto-expiration."]
    pub time_window_seconds: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "volumeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies the volume for which this entry applies."]
    pub volume_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictionaryAnnotationdata {
    #[serde(rename = "annotationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of annotation this data is for."]
    pub annotation_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "JSON encoded data for this dictionary annotation data. Emitted with name 'data' in JSON output. Either this or geo_data will be populated."]
    pub data: ::std::option::Option<::std::boxed::Box<Dictlayerdata>>,
    #[serde(rename = "encodedData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Base64 encoded data for this annotation data."]
    pub encoded_data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique id for this annotation data."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource Type"]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "layerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Layer id for this data. *"]
    pub layer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL for this resource. *"]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp for the last time this data was updated. (RFC 3339 UTC date-time format)."]
    pub updated: ::std::option::Option<::std::string::String>,
    #[serde(rename = "volumeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The volume id for this data. *"]
    pub volume_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dictlayerdata {
    #[serde(rename = "common")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub common: ::std::option::Option<DictlayerdataCommon>,
    #[serde(rename = "dict")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dict: ::std::option::Option<DictlayerdataDict>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictlayerdataCommon {
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display title and localized canonical name to use when searching for this entity on Google search."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictlayerdataDict {
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source, url and attribution for this dictionary data."]
    pub source: ::std::option::Option<DictlayerdataDictSource>,
    #[serde(rename = "words")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub words: ::std::option::Option<::std::vec::Vec<DictlayerdataDictWords>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The source, url and attribution for this dictionary data."]
pub struct DictlayerdataDictSource {
    #[serde(rename = "attribution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub attribution: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictlayerdataDictWords {
    #[serde(rename = "derivatives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub derivatives: ::std::option::Option<::std::vec::Vec<DictlayerdataDictWordsDerivatives>>,
    #[serde(rename = "examples")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub examples: ::std::option::Option<::std::vec::Vec<DictlayerdataDictWordsExamples>>,
    #[serde(rename = "senses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub senses: ::std::option::Option<::std::vec::Vec<DictlayerdataDictWordsSenses>>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The words with different meanings but not related words, e.g. \"go\" (game) and \"go\" (verb)."]
    pub source: ::std::option::Option<DictlayerdataDictWordsSource>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictlayerdataDictWordsDerivatives {
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<DictlayerdataDictWordsDerivativesSource>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictlayerdataDictWordsDerivativesSource {
    #[serde(rename = "attribution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub attribution: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictlayerdataDictWordsExamples {
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<DictlayerdataDictWordsExamplesSource>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictlayerdataDictWordsExamplesSource {
    #[serde(rename = "attribution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub attribution: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictlayerdataDictWordsSenses {
    #[serde(rename = "conjugations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub conjugations:
        ::std::option::Option<::std::vec::Vec<DictlayerdataDictWordsSensesConjugations>>,
    #[serde(rename = "definitions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub definitions:
        ::std::option::Option<::std::vec::Vec<DictlayerdataDictWordsSensesDefinitions>>,
    #[serde(rename = "partOfSpeech")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub part_of_speech: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pronunciation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pronunciation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pronunciationUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pronunciation_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<DictlayerdataDictWordsSensesSource>,
    #[serde(rename = "syllabification")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub syllabification: ::std::option::Option<::std::string::String>,
    #[serde(rename = "synonyms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub synonyms: ::std::option::Option<::std::vec::Vec<DictlayerdataDictWordsSensesSynonyms>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictlayerdataDictWordsSensesConjugations {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictlayerdataDictWordsSensesDefinitions {
    #[serde(rename = "definition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub definition: ::std::option::Option<::std::string::String>,
    #[serde(rename = "examples")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub examples:
        ::std::option::Option<::std::vec::Vec<DictlayerdataDictWordsSensesDefinitionsExamples>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictlayerdataDictWordsSensesDefinitionsExamples {
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<DictlayerdataDictWordsSensesDefinitionsExamplesSource>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictlayerdataDictWordsSensesDefinitionsExamplesSource {
    #[serde(rename = "attribution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub attribution: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictlayerdataDictWordsSensesSource {
    #[serde(rename = "attribution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub attribution: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictlayerdataDictWordsSensesSynonyms {
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<DictlayerdataDictWordsSensesSynonymsSource>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictlayerdataDictWordsSensesSynonymsSource {
    #[serde(rename = "attribution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub attribution: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The words with different meanings but not related words, e.g. \"go\" (game) and \"go\" (verb)."]
pub struct DictlayerdataDictWordsSource {
    #[serde(rename = "attribution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub attribution: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Discoveryclusters {
    #[serde(rename = "clusters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub clusters: ::std::option::Option<::std::vec::Vec<DiscoveryclustersClusters>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resorce type."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalClusters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub total_clusters: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiscoveryclustersClusters {
    #[serde(rename = "banner_with_content_container")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub banner_with_content_container:
        ::std::option::Option<DiscoveryclustersClustersBannerWithContentContainer>,
    #[serde(rename = "subTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sub_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalVolumes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub total_volumes: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub uid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "volumes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub volumes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Volume>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiscoveryclustersClustersBannerWithContentContainer {
    #[serde(rename = "fillColorArgb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub fill_color_argb: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maskColorArgb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mask_color_argb: ::std::option::Option<::std::string::String>,
    #[serde(rename = "moreButtonText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub more_button_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "moreButtonUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub more_button_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "textColorArgb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text_color_argb: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DownloadAccessRestriction {
    #[serde(rename = "deviceAllowed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If restricted, whether access is granted for this (user, device, volume)."]
    pub device_allowed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "downloadsAcquired")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If restricted, the number of content download licenses already acquired (including the requesting client, if licensed)."]
    pub downloads_acquired: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "justAcquired")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If deviceAllowed, whether access was just acquired with this request."]
    pub just_acquired: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxDownloadDevices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If restricted, the maximum number of content download licenses for this volume."]
    pub max_download_devices: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error/warning message."]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nonce")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Client nonce for verification. Download access and client-validation only."]
    pub nonce: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error/warning reason code. Additional codes may be added in the future. 0 OK 100 ACCESS_DENIED_PUBLISHER_LIMIT 101 ACCESS_DENIED_LIMIT 200 WARNING_USED_LAST_ACCESS"]
    pub reason_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "restricted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this volume has any download access restrictions."]
    pub restricted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "signature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Response signature."]
    pub signature: ::std::option::Option<::std::string::String>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Client app identifier for verification. Download access and client-validation only."]
    pub source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "volumeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies the volume for which this entry applies."]
    pub volume_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DownloadAccesses {
    #[serde(rename = "downloadAccessList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of download access responses."]
    pub download_access_list:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DownloadAccessRestriction>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyInfo {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "membership")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Family membership info of the user that made the request."]
    pub membership: ::std::option::Option<FamilyInfoMembership>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Family membership info of the user that made the request."]
pub struct FamilyInfoMembership {
    #[serde(rename = "acquirePermission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restrictions on user buying and acquiring content."]
    pub acquire_permission: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ageGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The age group of the user."]
    pub age_group: ::std::option::Option<::std::string::String>,
    #[serde(rename = "allowedMaturityRating")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum allowed maturity rating for the user."]
    pub allowed_maturity_rating: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isInFamily")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_in_family: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The role of the user in the family."]
    pub role: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeoAnnotationdata {
    #[serde(rename = "annotationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of annotation this data is for."]
    pub annotation_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "JSON encoded data for this geo annotation data. Emitted with name 'data' in JSON output. Either this or dict_data will be populated."]
    pub data: ::std::option::Option<::std::boxed::Box<Geolayerdata>>,
    #[serde(rename = "encodedData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Base64 encoded data for this annotation data."]
    pub encoded_data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique id for this annotation data."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource Type"]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "layerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Layer id for this data. *"]
    pub layer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL for this resource. *"]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp for the last time this data was updated. (RFC 3339 UTC date-time format)."]
    pub updated: ::std::option::Option<::std::string::String>,
    #[serde(rename = "volumeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The volume id for this data. *"]
    pub volume_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Geolayerdata {
    #[serde(rename = "common")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub common: ::std::option::Option<GeolayerdataCommon>,
    #[serde(rename = "geo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub geo: ::std::option::Option<GeolayerdataGeo>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeolayerdataCommon {
    #[serde(rename = "lang")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language of the information url and description."]
    pub lang: ::std::option::Option<::std::string::String>,
    #[serde(rename = "previewImageUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL for the preview image information."]
    pub preview_image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "snippet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description for this location."]
    pub snippet: ::std::option::Option<::std::string::String>,
    #[serde(rename = "snippetUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL for information for this location. Ex: wikipedia link."]
    pub snippet_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display title and localized canonical name to use when searching for this entity on Google search."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeolayerdataGeo {
    #[serde(rename = "boundary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The boundary of the location as a set of loops containing pairs of latitude, longitude coordinates."]
    pub boundary: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "cachePolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cache policy active for this data. EX: UNRESTRICTED, RESTRICTED, NEVER"]
    pub cache_policy: ::std::option::Option<::std::string::String>,
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The country code of the location."]
    pub country_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "latitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The latitude of the location."]
    pub latitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "longitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The longitude of the location."]
    pub longitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "mapType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of map that should be used for this location. EX: HYBRID, ROADMAP, SATELLITE, TERRAIN"]
    pub map_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "viewport")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The viewport for showing this location. This is a latitude, longitude rectangle."]
    pub viewport: ::std::option::Option<GeolayerdataGeoViewport>,
    #[serde(rename = "zoom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Zoom level to use for the map. Zoom levels between 0 (the lowest zoom level, in which the entire world can be seen on one map) to 21+ (down to individual buildings). See: https: //developers.google.com/maps/documentation/staticmaps/#Zoomlevels"]
    pub zoom: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The viewport for showing this location. This is a latitude, longitude rectangle."]
pub struct GeolayerdataGeoViewport {
    #[serde(rename = "hi")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hi: ::std::option::Option<GeolayerdataGeoViewportHi>,
    #[serde(rename = "lo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lo: ::std::option::Option<GeolayerdataGeoViewportLo>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeolayerdataGeoViewportHi {
    #[serde(rename = "latitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub latitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "longitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub longitude: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeolayerdataGeoViewportLo {
    #[serde(rename = "latitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub latitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "longitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub longitude: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Layersummaries {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of layer summary items."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Layersummary>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of layer summaries found."]
    pub total_items: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Layersummary {
    #[serde(rename = "annotationCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of annotations for this layer."]
    pub annotation_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "annotationTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of annotation types contained for this layer."]
    pub annotation_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "annotationsDataLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Link to get data for this annotation."]
    pub annotations_data_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "annotationsLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The link to get the annotations for this layer."]
    pub annotations_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content version this resource is for."]
    pub content_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dataCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of data items for this layer."]
    pub data_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique id of this layer summary."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource Type"]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "layerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The layer id for this summary."]
    pub layer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to this resource."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp for the last time an item in this layer was updated. (RFC 3339 UTC date-time format)."]
    pub updated: ::std::option::Option<::std::string::String>,
    #[serde(rename = "volumeAnnotationsVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current version of this layer's volume annotations. Note that this version applies only to the data in the books.layers.volumeAnnotations.* responses. The actual annotation data is versioned separately."]
    pub volume_annotations_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "volumeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The volume id this resource is for."]
    pub volume_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of offline dictionary metadata."]
    pub items: ::std::option::Option<::std::vec::Vec<MetadataItems>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MetadataItems {
    #[serde(rename = "download_url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub download_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "encrypted_key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub encrypted_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Notification {
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub body: ::std::option::Option<::std::string::String>,
    #[serde(rename = "crmExperimentIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of crm experiment ids."]
    pub crm_experiment_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "doc_id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub doc_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "doc_type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub doc_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dont_show_notification")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dont_show_notification: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "iconUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub icon_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "is_document_mature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_document_mature: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notificationGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub notification_group: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notification_type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub notification_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pcampaign_id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pcampaign_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "show_notification_settings_action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub show_notification_settings_action: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "targetUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeToExpireMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub time_to_expire_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Offers {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of offers."]
    pub items: ::std::option::Option<::std::vec::Vec<OffersItems>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OffersItems {
    #[serde(rename = "artUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub art_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gservicesKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub gservices_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub items: ::std::option::Option<::std::vec::Vec<OffersItemsItems>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OffersItemsItems {
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub author: ::std::option::Option<::std::string::String>,
    #[serde(rename = "canonicalVolumeLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub canonical_volume_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "coverUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cover_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "volumeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub volume_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadingPosition {
    #[serde(rename = "epubCfiPosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Position in an EPUB as a CFI."]
    pub epub_cfi_position: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gbImagePosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Position in a volume for image-based content."]
    pub gb_image_position: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gbTextPosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Position in a volume for text-based content."]
    pub gb_text_position: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type for a reading position."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pdfPosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Position in a PDF file."]
    pub pdf_position: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp when this reading position was last updated (formatted UTC timestamp with millisecond resolution)."]
    pub updated: ::std::option::Option<::std::string::String>,
    #[serde(rename = "volumeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Volume id associated with this reading position."]
    pub volume_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestAccessData {
    #[serde(rename = "concurrentAccess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A concurrent access response."]
    pub concurrent_access: ::std::option::Option<::std::boxed::Box<ConcurrentAccessRestriction>>,
    #[serde(rename = "downloadAccess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A download access response."]
    pub download_access: ::std::option::Option<::std::boxed::Box<DownloadAccessRestriction>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Review {
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Author of this review."]
    pub author: ::std::option::Option<ReviewAuthor>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Review text."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date of this review."]
    pub date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fullTextUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL for the full review text, for reviews gathered from the web."]
    pub full_text_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type for a review."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rating")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Star rating for this review. Possible values are ONE, TWO, THREE, FOUR, FIVE or NOT_RATED."]
    pub rating: ::std::option::Option<::std::string::String>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information regarding the source of this review, when the review is not from a Google Books user."]
    pub source: ::std::option::Option<ReviewSource>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title for this review."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Source type for this review. Possible values are EDITORIAL, WEB_USER or GOOGLE_USER."]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "volumeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Volume that this review is for."]
    pub volume_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Author of this review."]
pub struct ReviewAuthor {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this person."]
    pub display_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information regarding the source of this review, when the review is not from a Google Books user."]
pub struct ReviewSource {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the source."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "extraDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Extra text about the source of the review."]
    pub extra_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of the source of the review."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Series {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "series")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub series: ::std::option::Option<::std::vec::Vec<SeriesSeries>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeriesSeries {
    #[serde(rename = "bannerImageUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub banner_image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "eligibleForSubscription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub eligible_for_subscription: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isComplete")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_complete: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "seriesFormatType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub series_format_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "seriesId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub series_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "seriesSubscriptionReleaseInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub series_subscription_release_info:
        ::std::option::Option<SeriesSeriesSeriesSubscriptionReleaseInfo>,
    #[serde(rename = "seriesType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub series_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subscriptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subscription_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeriesSeriesSeriesSubscriptionReleaseInfo {
    #[serde(rename = "cancelTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cancel_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currentReleaseInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub current_release_info:
        ::std::option::Option<SeriesSeriesSeriesSubscriptionReleaseInfoCurrentReleaseInfo>,
    #[serde(rename = "nextReleaseInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_release_info:
        ::std::option::Option<SeriesSeriesSeriesSubscriptionReleaseInfoNextReleaseInfo>,
    #[serde(rename = "seriesSubscriptionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub series_subscription_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeriesSeriesSeriesSubscriptionReleaseInfoCurrentReleaseInfo {
    #[serde(rename = "amountInMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub amount_in_micros: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "releaseNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub release_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "releaseTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub release_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeriesSeriesSeriesSubscriptionReleaseInfoNextReleaseInfo {
    #[serde(rename = "amountInMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub amount_in_micros: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "releaseNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub release_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "releaseTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub release_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Seriesmembership {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resorce type."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "member")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub member: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Volume>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Usersettings {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notesExport")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User settings in sub-objects, each for different purposes."]
    pub notes_export: ::std::option::Option<UsersettingsNotesExport>,
    #[serde(rename = "notification")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub notification: ::std::option::Option<UsersettingsNotification>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "User settings in sub-objects, each for different purposes."]
pub struct UsersettingsNotesExport {
    #[serde(rename = "folderName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub folder_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsersettingsNotification {
    #[serde(rename = "matchMyInterests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub match_my_interests: ::std::option::Option<UsersettingsNotificationMatchMyInterests>,
    #[serde(rename = "moreFromAuthors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub more_from_authors: ::std::option::Option<UsersettingsNotificationMoreFromAuthors>,
    #[serde(rename = "moreFromSeries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub more_from_series: ::std::option::Option<UsersettingsNotificationMoreFromSeries>,
    #[serde(rename = "priceDrop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub price_drop: ::std::option::Option<UsersettingsNotificationPriceDrop>,
    #[serde(rename = "rewardExpirations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reward_expirations: ::std::option::Option<UsersettingsNotificationRewardExpirations>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsersettingsNotificationMatchMyInterests {
    #[serde(rename = "opted_state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub opted_state: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsersettingsNotificationMoreFromAuthors {
    #[serde(rename = "opted_state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub opted_state: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsersettingsNotificationMoreFromSeries {
    #[serde(rename = "opted_state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub opted_state: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsersettingsNotificationPriceDrop {
    #[serde(rename = "opted_state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub opted_state: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsersettingsNotificationRewardExpirations {
    #[serde(rename = "opted_state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub opted_state: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Volume {
    #[serde(rename = "accessInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Any information about a volume related to reading or obtaining that volume text. This information can depend on country (books may be public domain in one country but not in another, e.g.)."]
    pub access_info: ::std::option::Option<VolumeAccessInfo>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Opaque identifier for a specific version of a volume resource. (In LITE projection)"]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier for a volume. (In LITE projection.)"]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type for a volume. (In LITE projection.)"]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "layerInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "What layers exist in this volume and high level information about them."]
    pub layer_info: ::std::option::Option<VolumeLayerInfo>,
    #[serde(rename = "recommendedInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Recommendation related information for this volume."]
    pub recommended_info: ::std::option::Option<VolumeRecommendedInfo>,
    #[serde(rename = "saleInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Any information about a volume related to the eBookstore and/or purchaseability. This information can depend on the country where the request originates from (i.e. books may not be for sale in certain countries)."]
    pub sale_info: ::std::option::Option<VolumeSaleInfo>,
    #[serde(rename = "searchInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Search result information related to this volume."]
    pub search_info: ::std::option::Option<VolumeSearchInfo>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to this resource. (In LITE projection.)"]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User specific information related to this volume. (e.g. page this user last read or whether they purchased this book)"]
    pub user_info: ::std::option::Option<VolumeUserInfo>,
    #[serde(rename = "volumeInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "General volume information."]
    pub volume_info: ::std::option::Option<VolumeVolumeInfo>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Any information about a volume related to reading or obtaining that volume text. This information can depend on country (books may be public domain in one country but not in another, e.g.)."]
pub struct VolumeAccessInfo {
    #[serde(rename = "accessViewStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Combines the access and viewability of this volume into a single status field for this user. Values can be FULL_PURCHASED, FULL_PUBLIC_DOMAIN, SAMPLE or NONE. (In LITE projection.)"]
    pub access_view_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The two-letter ISO_3166-1 country code for which this access information is valid. (In LITE projection.)"]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "downloadAccess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about a volume's download license access restrictions."]
    pub download_access: ::std::option::Option<::std::boxed::Box<DownloadAccessRestriction>>,
    #[serde(rename = "driveImportedContentLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to the Google Drive viewer if this volume is uploaded by the user by selecting the file from Google Drive."]
    pub drive_imported_content_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "embeddable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this volume can be embedded in a viewport using the Embedded Viewer API."]
    pub embeddable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "epub")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about epub content. (In LITE projection.)"]
    pub epub: ::std::option::Option<VolumeAccessInfoEpub>,
    #[serde(rename = "explicitOfflineLicenseManagement")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this volume requires that the client explicitly request offline download license rather than have it done automatically when loading the content, if the client supports it."]
    pub explicit_offline_license_management: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "pdf")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about pdf content. (In LITE projection.)"]
    pub pdf: ::std::option::Option<VolumeAccessInfoPdf>,
    #[serde(rename = "publicDomain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not this book is public domain in the country listed above."]
    pub public_domain: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "quoteSharingAllowed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether quote sharing is allowed for this volume."]
    pub quote_sharing_allowed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "textToSpeechPermission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether text-to-speech is permitted for this volume. Values can be ALLOWED, ALLOWED_FOR_ACCESSIBILITY, or NOT_ALLOWED."]
    pub text_to_speech_permission: ::std::option::Option<::std::string::String>,
    #[serde(rename = "viewOrderUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For ordered but not yet processed orders, we give a URL that can be used to go to the appropriate Google Wallet page."]
    pub view_order_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "viewability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The read access of a volume. Possible values are PARTIAL, ALL_PAGES, NO_PAGES or UNKNOWN. This value depends on the country listed above. A value of PARTIAL means that the publisher has allowed some portion of the volume to be viewed publicly, without purchase. This can apply to eBooks as well as non-eBooks. Public domain books will always have a value of ALL_PAGES."]
    pub viewability: ::std::option::Option<::std::string::String>,
    #[serde(rename = "webReaderLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to read this volume on the Google Books site. Link will not allow users to read non-viewable volumes."]
    pub web_reader_link: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about epub content. (In LITE projection.)"]
pub struct VolumeAccessInfoEpub {
    #[serde(rename = "acsTokenLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to retrieve ACS token for epub download. (In LITE projection.)"]
    pub acs_token_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "downloadLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to download epub. (In LITE projection.)"]
    pub download_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isAvailable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Is a flowing text epub available either as public domain or for purchase. (In LITE projection.)"]
    pub is_available: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about pdf content. (In LITE projection.)"]
pub struct VolumeAccessInfoPdf {
    #[serde(rename = "acsTokenLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to retrieve ACS token for pdf download. (In LITE projection.)"]
    pub acs_token_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "downloadLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to download pdf. (In LITE projection.)"]
    pub download_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isAvailable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Is a scanned image pdf available either as public domain or for purchase. (In LITE projection.)"]
    pub is_available: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "What layers exist in this volume and high level information about them."]
pub struct VolumeLayerInfo {
    #[serde(rename = "layers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A layer should appear here if and only if the layer exists for this book."]
    pub layers: ::std::option::Option<::std::vec::Vec<VolumeLayerInfoLayers>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VolumeLayerInfoLayers {
    #[serde(rename = "layerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The layer id of this layer (e.g. \"geo\")."]
    pub layer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "volumeAnnotationsVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current version of this layer's volume annotations. Note that this version applies only to the data in the books.layers.volumeAnnotations.* responses. The actual annotation data is versioned separately."]
    pub volume_annotations_version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Recommendation related information for this volume."]
pub struct VolumeRecommendedInfo {
    #[serde(rename = "explanation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A text explaining why this volume is recommended."]
    pub explanation: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Any information about a volume related to the eBookstore and/or purchaseability. This information can depend on the country where the request originates from (i.e. books may not be for sale in certain countries)."]
pub struct VolumeSaleInfo {
    #[serde(rename = "buyLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to purchase this volume on the Google Books site. (In LITE projection)"]
    pub buy_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The two-letter ISO_3166-1 country code for which this sale information is valid. (In LITE projection.)"]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isEbook")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not this volume is an eBook (can be added to the My eBooks shelf)."]
    pub is_ebook: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "listPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Suggested retail price. (In LITE projection.)"]
    pub list_price: ::std::option::Option<VolumeSaleInfoListPrice>,
    #[serde(rename = "offers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Offers available for this volume (sales and rentals)."]
    pub offers: ::std::option::Option<::std::vec::Vec<VolumeSaleInfoOffers>>,
    #[serde(rename = "onSaleDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date on which this book is available for sale."]
    pub on_sale_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "retailPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The actual selling price of the book. This is the same as the suggested retail or list price unless there are offers or discounts on this volume. (In LITE projection.)"]
    pub retail_price: ::std::option::Option<VolumeSaleInfoRetailPrice>,
    #[serde(rename = "saleability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not this book is available for sale or offered for free in the Google eBookstore for the country listed above. Possible values are FOR_SALE, FOR_RENTAL_ONLY, FOR_SALE_AND_RENTAL, FREE, NOT_FOR_SALE, or FOR_PREORDER."]
    pub saleability: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Suggested retail price. (In LITE projection.)"]
pub struct VolumeSaleInfoListPrice {
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Amount in the currency listed below. (In LITE projection.)"]
    pub amount: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An ISO 4217, three-letter currency code. (In LITE projection.)"]
    pub currency_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VolumeSaleInfoOffers {
    #[serde(rename = "finskyOfferType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The finsky offer type (e.g., PURCHASE=0 RENTAL=3)"]
    pub finsky_offer_type: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "giftable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the offer is giftable."]
    pub giftable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "listPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Offer list (=undiscounted) price in Micros."]
    pub list_price: ::std::option::Option<VolumeSaleInfoOffersListPrice>,
    #[serde(rename = "rentalDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The rental duration (for rental offers only)."]
    pub rental_duration: ::std::option::Option<VolumeSaleInfoOffersRentalDuration>,
    #[serde(rename = "retailPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Offer retail (=discounted) price in Micros"]
    pub retail_price: ::std::option::Option<VolumeSaleInfoOffersRetailPrice>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Offer list (=undiscounted) price in Micros."]
pub struct VolumeSaleInfoOffersListPrice {
    #[serde(rename = "amountInMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub amount_in_micros: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub currency_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The rental duration (for rental offers only)."]
pub struct VolumeSaleInfoOffersRentalDuration {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub count: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub unit: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Offer retail (=discounted) price in Micros"]
pub struct VolumeSaleInfoOffersRetailPrice {
    #[serde(rename = "amountInMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub amount_in_micros: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub currency_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The actual selling price of the book. This is the same as the suggested retail or list price unless there are offers or discounts on this volume. (In LITE projection.)"]
pub struct VolumeSaleInfoRetailPrice {
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Amount in the currency listed below. (In LITE projection.)"]
    pub amount: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An ISO 4217, three-letter currency code. (In LITE projection.)"]
    pub currency_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Search result information related to this volume."]
pub struct VolumeSearchInfo {
    #[serde(rename = "textSnippet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A text snippet containing the search query."]
    pub text_snippet: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "User specific information related to this volume. (e.g. page this user last read or whether they purchased this book)"]
pub struct VolumeUserInfo {
    #[serde(rename = "acquiredTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp when this volume was acquired by the user. (RFC 3339 UTC date-time format) Acquiring includes purchase, user upload, receiving family sharing, etc."]
    pub acquired_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "acquisitionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How this volume was acquired."]
    pub acquisition_type: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "copy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Copy/Paste accounting information."]
    pub copy: ::std::option::Option<VolumeUserInfoCopy>,
    #[serde(rename = "entitlementType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this volume is purchased, sample, pd download etc."]
    pub entitlement_type: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "familySharing")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information on the ability to share with the family."]
    pub family_sharing: ::std::option::Option<VolumeUserInfoFamilySharing>,
    #[serde(rename = "isFamilySharedFromUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not the user shared this volume with the family."]
    pub is_family_shared_from_user: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isFamilySharedToUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not the user received this volume through family sharing."]
    pub is_family_shared_to_user: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isFamilySharingAllowed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated: Replaced by familySharing."]
    pub is_family_sharing_allowed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isFamilySharingDisabledByFop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated: Replaced by familySharing."]
    pub is_family_sharing_disabled_by_fop: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isInMyBooks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not this volume is currently in \"my books.\""]
    pub is_in_my_books: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isPreordered")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not this volume was pre-ordered by the authenticated user making the request. (In LITE projection.)"]
    pub is_preordered: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isPurchased")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not this volume was purchased by the authenticated user making the request. (In LITE projection.)"]
    pub is_purchased: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isUploaded")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not this volume was user uploaded."]
    pub is_uploaded: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "readingPosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's current reading position in the volume, if one is available. (In LITE projection.)"]
    pub reading_position: ::std::option::Option<::std::boxed::Box<ReadingPosition>>,
    #[serde(rename = "rentalPeriod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Period during this book is/was a valid rental."]
    pub rental_period: ::std::option::Option<VolumeUserInfoRentalPeriod>,
    #[serde(rename = "rentalState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this book is an active or an expired rental."]
    pub rental_state: ::std::option::Option<::std::string::String>,
    #[serde(rename = "review")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This user's review of this volume, if one exists."]
    pub review: ::std::option::Option<::std::boxed::Box<Review>>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp when this volume was last modified by a user action, such as a reading position update, volume purchase or writing a review. (RFC 3339 UTC date-time format)."]
    pub updated: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userUploadedVolumeInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_uploaded_volume_info: ::std::option::Option<VolumeUserInfoUserUploadedVolumeInfo>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Copy/Paste accounting information."]
pub struct VolumeUserInfoCopy {
    #[serde(rename = "allowedCharacterCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_character_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "limitType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "remainingCharacterCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub remaining_character_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub updated: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information on the ability to share with the family."]
pub struct VolumeUserInfoFamilySharing {
    #[serde(rename = "familyRole")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The role of the user in the family."]
    pub family_role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isSharingAllowed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not this volume can be shared with the family by the user. This includes sharing eligibility of both the volume and the user. If the value is true, the user can initiate a family sharing action."]
    pub is_sharing_allowed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isSharingDisabledByFop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not sharing this volume is temporarily disabled due to issues with the Family Wallet."]
    pub is_sharing_disabled_by_fop: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Period during this book is/was a valid rental."]
pub struct VolumeUserInfoRentalPeriod {
    #[serde(rename = "endUtcSec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_utc_sec: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startUtcSec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_utc_sec: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VolumeUserInfoUserUploadedVolumeInfo {
    #[serde(rename = "processingState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub processing_state: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "General volume information."]
pub struct VolumeVolumeInfo {
    #[serde(rename = "allowAnonLogging")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether anonymous logging should be allowed."]
    pub allow_anon_logging: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "authors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The names of the authors and/or editors for this volume. (In LITE projection)"]
    pub authors: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "averageRating")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The mean review rating for this volume. (min = 1.0, max = 5.0)"]
    pub average_rating: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "canonicalVolumeLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Canonical URL for a volume. (In LITE projection.)"]
    pub canonical_volume_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of subject categories, such as \"Fiction\", \"Suspense\", etc."]
    pub categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "comicsContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the volume has comics content."]
    pub comics_content: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "contentVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An identifier for the version of the volume content (text & images). (In LITE projection)"]
    pub content_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A synopsis of the volume. The text of the description is formatted in HTML and includes simple formatting elements, such as b, i, and br tags. (In LITE projection.)"]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Physical dimensions of this volume."]
    pub dimensions: ::std::option::Option<VolumeVolumeInfoDimensions>,
    #[serde(rename = "imageLinks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of image links for all the sizes that are available. (In LITE projection.)"]
    pub image_links: ::std::option::Option<VolumeVolumeInfoImageLinks>,
    #[serde(rename = "industryIdentifiers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Industry standard identifiers for this volume."]
    pub industry_identifiers:
        ::std::option::Option<::std::vec::Vec<VolumeVolumeInfoIndustryIdentifiers>>,
    #[serde(rename = "infoLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to view information about this volume on the Google Books site. (In LITE projection)"]
    pub info_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Best language for this volume (based on content). It is the two-letter ISO 639-1 code such as 'fr', 'en', etc."]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mainCategory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The main category to which this volume belongs. It will be the category from the categories list returned below that has the highest weight."]
    pub main_category: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maturityRating")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub maturity_rating: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pageCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of pages as per publisher metadata."]
    pub page_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "panelizationSummary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A top-level summary of the panelization info in this volume."]
    pub panelization_summary: ::std::option::Option<VolumeVolumeInfoPanelizationSummary>,
    #[serde(rename = "previewLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to preview this volume on the Google Books site."]
    pub preview_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "printType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of publication of this volume. Possible values are BOOK or MAGAZINE."]
    pub print_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "printedPageCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of printed pages in generated pdf representation."]
    pub printed_page_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "publishedDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date of publication. (In LITE projection.)"]
    pub published_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publisher")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Publisher of this volume. (In LITE projection.)"]
    pub publisher: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ratingsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of review ratings for this volume."]
    pub ratings_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "readingModes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reading modes available for this volume."]
    pub reading_modes: ::std::option::Option<VolumeVolumeInfoReadingModes>,
    #[serde(rename = "samplePageCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of sample pages as per publisher metadata."]
    pub sample_page_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "seriesInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub series_info: ::std::option::Option<::std::boxed::Box<Volumeseriesinfo>>,
    #[serde(rename = "subtitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Volume subtitle. (In LITE projection.)"]
    pub subtitle: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Volume title. (In LITE projection.)"]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Physical dimensions of this volume."]
pub struct VolumeVolumeInfoDimensions {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Height or length of this volume (in cm)."]
    pub height: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thickness")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Thickness of this volume (in cm)."]
    pub thickness: ::std::option::Option<::std::string::String>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Width of this volume (in cm)."]
    pub width: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of image links for all the sizes that are available. (In LITE projection.)"]
pub struct VolumeVolumeInfoImageLinks {
    #[serde(rename = "extraLarge")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Image link for extra large size (width of ~1280 pixels). (In LITE projection)"]
    pub extra_large: ::std::option::Option<::std::string::String>,
    #[serde(rename = "large")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Image link for large size (width of ~800 pixels). (In LITE projection)"]
    pub large: ::std::option::Option<::std::string::String>,
    #[serde(rename = "medium")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Image link for medium size (width of ~575 pixels). (In LITE projection)"]
    pub medium: ::std::option::Option<::std::string::String>,
    #[serde(rename = "small")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Image link for small size (width of ~300 pixels). (In LITE projection)"]
    pub small: ::std::option::Option<::std::string::String>,
    #[serde(rename = "smallThumbnail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Image link for small thumbnail size (width of ~80 pixels). (In LITE projection)"]
    pub small_thumbnail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thumbnail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Image link for thumbnail size (width of ~128 pixels). (In LITE projection)"]
    pub thumbnail: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VolumeVolumeInfoIndustryIdentifiers {
    #[serde(rename = "identifier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Industry specific volume identifier."]
    pub identifier: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier type. Possible values are ISBN_10, ISBN_13, ISSN and OTHER."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A top-level summary of the panelization info in this volume."]
pub struct VolumeVolumeInfoPanelizationSummary {
    #[serde(rename = "containsEpubBubbles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub contains_epub_bubbles: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "containsImageBubbles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub contains_image_bubbles: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "epubBubbleVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub epub_bubble_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imageBubbleVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_bubble_version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The reading modes available for this volume."]
pub struct VolumeVolumeInfoReadingModes {
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Volume2 {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of volumes."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Volume>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Volumeannotation {
    #[serde(rename = "annotationDataId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The annotation data id for this volume annotation."]
    pub annotation_data_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "annotationDataLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Link to get data for this annotation."]
    pub annotation_data_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "annotationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of annotation this is."]
    pub annotation_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentRanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content ranges to identify the selected text."]
    pub content_ranges: ::std::option::Option<VolumeannotationContentRanges>,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Data for this annotation."]
    pub data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that this annotation is deleted."]
    pub deleted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique id of this volume annotation."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource Type"]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "layerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Layer this annotation is for."]
    pub layer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pageIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pages the annotation spans."]
    pub page_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "selectedText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Excerpt from the volume."]
    pub selected_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to this resource."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp for the last time this anntoation was updated. (RFC 3339 UTC date-time format)."]
    pub updated: ::std::option::Option<::std::string::String>,
    #[serde(rename = "volumeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Volume this annotation is for."]
    pub volume_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The content ranges to identify the selected text."]
pub struct VolumeannotationContentRanges {
    #[serde(rename = "cfiRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Range in CFI format for this annotation for version above."]
    pub cfi_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
    #[serde(rename = "contentVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content version applicable to ranges below."]
    pub content_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gbImageRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Range in GB image format for this annotation for version above."]
    pub gb_image_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
    #[serde(rename = "gbTextRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Range in GB text format for this annotation for version above."]
    pub gb_text_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Volumeannotations {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of volume annotations."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Volumeannotation>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type"]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to pass in for pagination for the next page. This will not be present if this request does not have more results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of volume annotations found."]
    pub total_items: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version string for all of the volume annotations in this layer (not just the ones in this response). Note: the version string doesn't apply to the annotation data, just the information in this response (e.g. the location of annotations in the book)."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Volumes {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of volumes."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Volume>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of volumes found. This might be greater than the number of volumes returned in this response if results have been paginated."]
    pub total_items: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Volumeseriesinfo {
    #[serde(rename = "bookDisplayNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display number string. This should be used only for display purposes and the actual sequence should be inferred from the below orderNumber."]
    pub book_display_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource type."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shortSeriesBookTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Short book title in the context of the series."]
    pub short_series_book_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "volumeSeries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub volume_series: ::std::option::Option<::std::vec::Vec<VolumeseriesinfoVolumeSeries>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VolumeseriesinfoVolumeSeries {
    #[serde(rename = "issue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of issues. Applicable only for Collection Edition and Omnibus."]
    pub issue: ::std::option::Option<::std::vec::Vec<VolumeseriesinfoVolumeSeriesIssue>>,
    #[serde(rename = "orderNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The book order number in the series."]
    pub order_number: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "seriesBookType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The book type in the context of series. Examples - Single Issue, Collection Edition, etc."]
    pub series_book_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "seriesId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The series id."]
    pub series_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VolumeseriesinfoVolumeSeriesIssue {
    #[serde(rename = "issueDisplayNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub issue_display_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "issueOrderNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub issue_order_number: ::std::option::Option<::std::primitive::i64>,
}
