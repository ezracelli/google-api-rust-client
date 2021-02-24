#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Promotion result."]
pub struct Promotion {
    #[serde(rename = "bodyLines")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An array of block objects for this promotion. See [Google WebSearch Protocol reference](https://developers.google.com/custom-search/docs/xml_results) for more information."]
    pub body_lines: ::std::option::Option<::std::vec::Vec<PromotionBodyLines>>,
    #[serde(rename = "displayLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An abridged version of this search's result URL, e.g. www.example.com."]
    pub display_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "htmlTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the promotion, in HTML."]
    pub html_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Image belonging to a promotion."]
    pub image: ::std::option::Option<PromotionImage>,
    #[serde(rename = "link")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the promotion."]
    pub link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the promotion."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Block object belonging to a promotion."]
pub struct PromotionBodyLines {
    #[serde(rename = "htmlTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The block object's text in HTML, if it has text."]
    pub html_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "link")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The anchor text of the block object's link, if it has a link."]
    pub link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The block object's text, if it has text."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the block object's link, if it has one."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Image belonging to a promotion."]
pub struct PromotionImage {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Image height in pixels."]
    pub height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of the image for this promotion link."]
    pub source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Image width in pixels."]
    pub width: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A custom search result."]
pub struct Result {
    #[serde(rename = "cacheId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the ID of Google's cached version of the search result."]
    pub cache_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An abridged version of this search result’s URL, e.g. www.example.com."]
    pub display_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The file format of the search result."]
    pub file_format: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formattedUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL displayed after the snippet for each search result."]
    pub formatted_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "htmlFormattedUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HTML-formatted URL displayed after the snippet for each search result."]
    pub html_formatted_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "htmlSnippet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The snippet of the search result, in HTML."]
    pub html_snippet: ::std::option::Option<::std::string::String>,
    #[serde(rename = "htmlTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the search result, in HTML."]
    pub html_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Image belonging to a custom search result."]
    pub image: ::std::option::Option<ResultImage>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique identifier for the type of current object. For this API, it is `customsearch#result.`"]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Encapsulates all information about [refinement labels](https://developers.google.com/custom-search/docs/xml_results)."]
    pub labels: ::std::option::Option<::std::vec::Vec<ResultLabels>>,
    #[serde(rename = "link")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full URL to which the search result is pointing, e.g. http://www.example.com/foo/bar."]
    pub link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MIME type of the search result."]
    pub mime: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pagemap")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains [PageMap](https://developers.google.com/custom-search/docs/structured_data#pagemaps) information for this search result."]
    pub pagemap: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "snippet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The snippet of the search result, in plain text."]
    pub snippet: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the search result, in plain text."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Image belonging to a custom search result."]
pub struct ResultImage {
    #[serde(rename = "byteSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of the image, in pixels."]
    pub byte_size: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "contextLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL pointing to the webpage hosting the image."]
    pub context_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The height of the image, in pixels."]
    pub height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "thumbnailHeight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The height of the thumbnail image, in pixels."]
    pub thumbnail_height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "thumbnailLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL to the thumbnail image."]
    pub thumbnail_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thumbnailWidth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width of the thumbnail image, in pixels."]
    pub thumbnail_width: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width of the image, in pixels."]
    pub width: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Refinement label associated with a custom search result."]
pub struct ResultLabels {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name of a refinement label. This is the name you should display in your user interface."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "label_with_op")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Refinement label and the associated refinement operation."]
    pub label_with_op: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of a refinement label, which you can use to refine searches. Don't display this in your user interface; instead, use displayName."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response to a custom search request."]
pub struct Search {
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata and refinements associated with the given search engine, including: * The name of the search engine that was used for the query. * A set of [facet objects](https://developers.google.com/custom-search/docs/refinements#create) (refinements) you can use for refining a search."]
    pub context: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current set of custom search results."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Result>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier for the type of current object. For this API, it is customsearch#search."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "promotions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of [promotions](https://developers.google.com/custom-search/docs/promotions). Present only if the custom search engine's configuration files define any promotions for the given query."]
    pub promotions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Promotion>>>,
    #[serde(rename = "queries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Query metadata for the previous, current, and next pages of results."]
    pub queries: ::std::option::Option<SearchQueries>,
    #[serde(rename = "searchInformation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about a search operation."]
    pub search_information: ::std::option::Option<SearchSearchInformation>,
    #[serde(rename = "spelling")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Spell correction information for a query."]
    pub spelling: ::std::option::Option<SearchSpelling>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OpenSearch template and URL."]
    pub url: ::std::option::Option<SearchUrl>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Query metadata for the previous, current, and next pages of results."]
pub struct SearchQueries {
    #[serde(rename = "nextPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata representing the next page of results, if applicable."]
    pub next_page: ::std::option::Option<::std::vec::Vec<SearchQueriesNextPage>>,
    #[serde(rename = "previousPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata representing the previous page of results, if applicable."]
    pub previous_page: ::std::option::Option<::std::vec::Vec<SearchQueriesPreviousPage>>,
    #[serde(rename = "request")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata representing the current request."]
    pub request: ::std::option::Option<::std::vec::Vec<SearchQueriesRequest>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Custom search request metadata."]
pub struct SearchQueriesNextPage {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of search results returned in this set."]
    pub count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "cr")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/xml_results#booleanOperators) in the `cr` parameter's value. Google WebSearch determines the country of a document by analyzing the following: * The top-level domain (TLD) of the document's URL. * The geographic location of the web server's IP address. See [Country (cr) Parameter Values](https://developers.google.com/custom-search/docs/xml_results#countryCollections) for a list of valid values for this parameter."]
    pub cr: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cx")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of an engine created using the Programmable Search Engine [Control Panel](https://programmablesearchengine.google.com/). This is a custom property not defined in the OpenSearch spec. This parameter is **required**."]
    pub cx: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dateRestrict")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years."]
    pub date_restrict: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disableCnTwTranslation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enables or disables the [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/xml_results#chineseSearch) feature. Supported values are: * `0`: enabled (default) * `1`: disabled"]
    pub disable_cn_tw_translation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "exactTerms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies a phrase that all documents in the search results must contain."]
    pub exact_terms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "excludeTerms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies a word or phrase that should not appear in any documents in the search results."]
    pub exclude_terms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to files of a specified extension. Filetypes supported by Google include: * Adobe Portable Document Format (`pdf`) * Adobe PostScript (`ps`) * Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`) * Lotus WordPro (`lwp`) * Macwrite (`mw`) * Microsoft Excel (`xls`) * Microsoft PowerPoint (`ppt`) * Microsoft Word (`doc`) * Microsoft Works (`wks`, `wps`, `wdb`) * Microsoft Write (`wri`) * Rich Text Format (`rtf`) * Shockwave Flash (`swf`) * Text (`ans`, `txt`). Additional filetypes may be added in the future. An up-to-date list can always be found in Google's [file type FAQ](https://support.google.com/webmasters/answer/35287)."]
    pub file_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Activates or deactivates the automatic filtering of Google search results. See [Automatic Filtering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering) for more information about Google's search results filters. Valid values for this parameter are: * `0`: Disabled * `1`: Enabled (default) **Note**: By default, Google applies filtering to all search results to improve the quality of those results."]
    pub filter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Boosts search results whose country of origin matches the parameter value. See [Country Codes](https://developers.google.com/custom-search/docs/xml_results#countryCodes) for a list of valid values. Specifying a `gl` parameter value in WebSearch requests should improve the relevance of results. This is particularly true for international customers and, even more specifically, for customers in English-speaking countries other than the United States."]
    pub gl: ::std::option::Option<::std::string::String>,
    #[serde(rename = "googleHost")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the Google domain (for example, google.com, google.de, or google.fr) to which the search should be limited."]
    pub google_host: ::std::option::Option<::std::string::String>,
    #[serde(rename = "highRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the ending value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
    pub high_range: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the interface language (host language) of your user interface. Explicitly setting this parameter improves the performance and the quality of your search results. See the [Interface Languages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages) for a list of supported languages."]
    pub hl: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hq")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Appends the specified query terms to the query, as if they were combined with a logical `AND` operator."]
    pub hq: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imgColorType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to images of a specified color type. Supported values are: * `mono` (black and white) * `gray` (grayscale) * `color` (color)"]
    pub img_color_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imgDominantColor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to images with a specific dominant color. Supported values are: * `red` * `orange` * `yellow` * `green` * `teal` * `blue` * `purple` * `pink` * `white` * `gray` * `black` * `brown`"]
    pub img_dominant_color: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imgSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to images of a specified size. Supported values are: * `icon` (small) * `small | medium | large | xlarge` (medium) * `xxlarge` (large) * `huge` (extra-large)"]
    pub img_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imgType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to images of a specified type. Supported values are: * `clipart` (Clip art) * `face` (Face) * `lineart` (Line drawing) * `photo` (Photo) * `animated` (Animated) * `stock` (Stock)"]
    pub img_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inputEncoding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The character encoding supported for search requests."]
    pub input_encoding: ::std::option::Option<::std::string::String>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language of the search results."]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "linkSite")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies that all results should contain a link to a specific URL."]
    pub link_site: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lowRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the starting value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
    pub low_range: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orTerms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms. You can also use the [Boolean OR](https://developers.google.com/custom-search/docs/xml_results#BooleanOrqt) query term for this type of query."]
    pub or_terms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outputEncoding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The character encoding supported for search results."]
    pub output_encoding: ::std::option::Option<::std::string::String>,
    #[serde(rename = "relatedSite")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies that all search results should be pages that are related to the specified URL. The parameter value should be a URL."]
    pub related_site: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rights")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filters based on licensing. Supported values include: * `cc_publicdomain` * `cc_attribute` * `cc_sharealike` * `cc_noncommercial` * `cc_nonderived`"]
    pub rights: ::std::option::Option<::std::string::String>,
    #[serde(rename = "safe")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the [SafeSearch level](https://developers.google.com/custom-search/docs/xml_results#safeSearchLevels) used for filtering out adult results. This is a custom property not defined in the OpenSearch spec. Valid parameter values are: * `\"off\"`: Disable SafeSearch * `\"active\"`: Enable SafeSearch"]
    pub safe: ::std::option::Option<::std::string::String>,
    #[serde(rename = "searchTerms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The search terms entered by the user."]
    pub search_terms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "searchType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allowed values are `web` or `image`. If unspecified, results are limited to webpages."]
    pub search_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "siteSearch")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to URLs from a specified site."]
    pub site_search: ::std::option::Option<::std::string::String>,
    #[serde(rename = "siteSearchFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies whether to include or exclude results from the site named in the `sitesearch` parameter. Supported values are: * `i`: include content from site * `e`: exclude content from site"]
    pub site_search_filter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sort")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies that results should be sorted according to the specified expression. For example, sort by date."]
    pub sort: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The index of the current set of search results into the total set of results, where the index of the first result is 1."]
    pub start_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page number of this set of results, where the page length is set by the `count` property."]
    pub start_page: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of the query."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Estimated number of total search results. May not be accurate."]
    pub total_results: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Custom search request metadata."]
pub struct SearchQueriesPreviousPage {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of search results returned in this set."]
    pub count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "cr")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/xml_results#booleanOperators) in the `cr` parameter's value. Google WebSearch determines the country of a document by analyzing the following: * The top-level domain (TLD) of the document's URL. * The geographic location of the web server's IP address. See [Country (cr) Parameter Values](https://developers.google.com/custom-search/docs/xml_results#countryCollections) for a list of valid values for this parameter."]
    pub cr: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cx")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of an engine created using the Programmable Search Engine [Control Panel](https://programmablesearchengine.google.com/). This is a custom property not defined in the OpenSearch spec. This parameter is **required**."]
    pub cx: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dateRestrict")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years."]
    pub date_restrict: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disableCnTwTranslation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enables or disables the [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/xml_results#chineseSearch) feature. Supported values are: * `0`: enabled (default) * `1`: disabled"]
    pub disable_cn_tw_translation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "exactTerms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies a phrase that all documents in the search results must contain."]
    pub exact_terms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "excludeTerms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies a word or phrase that should not appear in any documents in the search results."]
    pub exclude_terms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to files of a specified extension. Filetypes supported by Google include: * Adobe Portable Document Format (`pdf`) * Adobe PostScript (`ps`) * Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`) * Lotus WordPro (`lwp`) * Macwrite (`mw`) * Microsoft Excel (`xls`) * Microsoft PowerPoint (`ppt`) * Microsoft Word (`doc`) * Microsoft Works (`wks`, `wps`, `wdb`) * Microsoft Write (`wri`) * Rich Text Format (`rtf`) * Shockwave Flash (`swf`) * Text (`ans`, `txt`). Additional filetypes may be added in the future. An up-to-date list can always be found in Google's [file type FAQ](https://support.google.com/webmasters/answer/35287)."]
    pub file_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Activates or deactivates the automatic filtering of Google search results. See [Automatic Filtering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering) for more information about Google's search results filters. Valid values for this parameter are: * `0`: Disabled * `1`: Enabled (default) **Note**: By default, Google applies filtering to all search results to improve the quality of those results."]
    pub filter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Boosts search results whose country of origin matches the parameter value. See [Country Codes](https://developers.google.com/custom-search/docs/xml_results#countryCodes) for a list of valid values. Specifying a `gl` parameter value in WebSearch requests should improve the relevance of results. This is particularly true for international customers and, even more specifically, for customers in English-speaking countries other than the United States."]
    pub gl: ::std::option::Option<::std::string::String>,
    #[serde(rename = "googleHost")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the Google domain (for example, google.com, google.de, or google.fr) to which the search should be limited."]
    pub google_host: ::std::option::Option<::std::string::String>,
    #[serde(rename = "highRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the ending value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
    pub high_range: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the interface language (host language) of your user interface. Explicitly setting this parameter improves the performance and the quality of your search results. See the [Interface Languages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages) for a list of supported languages."]
    pub hl: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hq")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Appends the specified query terms to the query, as if they were combined with a logical `AND` operator."]
    pub hq: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imgColorType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to images of a specified color type. Supported values are: * `mono` (black and white) * `gray` (grayscale) * `color` (color)"]
    pub img_color_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imgDominantColor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to images with a specific dominant color. Supported values are: * `red` * `orange` * `yellow` * `green` * `teal` * `blue` * `purple` * `pink` * `white` * `gray` * `black` * `brown`"]
    pub img_dominant_color: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imgSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to images of a specified size. Supported values are: * `icon` (small) * `small | medium | large | xlarge` (medium) * `xxlarge` (large) * `huge` (extra-large)"]
    pub img_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imgType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to images of a specified type. Supported values are: * `clipart` (Clip art) * `face` (Face) * `lineart` (Line drawing) * `photo` (Photo) * `animated` (Animated) * `stock` (Stock)"]
    pub img_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inputEncoding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The character encoding supported for search requests."]
    pub input_encoding: ::std::option::Option<::std::string::String>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language of the search results."]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "linkSite")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies that all results should contain a link to a specific URL."]
    pub link_site: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lowRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the starting value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
    pub low_range: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orTerms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms. You can also use the [Boolean OR](https://developers.google.com/custom-search/docs/xml_results#BooleanOrqt) query term for this type of query."]
    pub or_terms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outputEncoding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The character encoding supported for search results."]
    pub output_encoding: ::std::option::Option<::std::string::String>,
    #[serde(rename = "relatedSite")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies that all search results should be pages that are related to the specified URL. The parameter value should be a URL."]
    pub related_site: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rights")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filters based on licensing. Supported values include: * `cc_publicdomain` * `cc_attribute` * `cc_sharealike` * `cc_noncommercial` * `cc_nonderived`"]
    pub rights: ::std::option::Option<::std::string::String>,
    #[serde(rename = "safe")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the [SafeSearch level](https://developers.google.com/custom-search/docs/xml_results#safeSearchLevels) used for filtering out adult results. This is a custom property not defined in the OpenSearch spec. Valid parameter values are: * `\"off\"`: Disable SafeSearch * `\"active\"`: Enable SafeSearch"]
    pub safe: ::std::option::Option<::std::string::String>,
    #[serde(rename = "searchTerms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The search terms entered by the user."]
    pub search_terms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "searchType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allowed values are `web` or `image`. If unspecified, results are limited to webpages."]
    pub search_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "siteSearch")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to URLs from a specified site."]
    pub site_search: ::std::option::Option<::std::string::String>,
    #[serde(rename = "siteSearchFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies whether to include or exclude results from the site named in the `sitesearch` parameter. Supported values are: * `i`: include content from site * `e`: exclude content from site"]
    pub site_search_filter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sort")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies that results should be sorted according to the specified expression. For example, sort by date."]
    pub sort: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The index of the current set of search results into the total set of results, where the index of the first result is 1."]
    pub start_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page number of this set of results, where the page length is set by the `count` property."]
    pub start_page: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of the query."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Estimated number of total search results. May not be accurate."]
    pub total_results: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Custom search request metadata."]
pub struct SearchQueriesRequest {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of search results returned in this set."]
    pub count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "cr")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/xml_results#booleanOperators) in the `cr` parameter's value. Google WebSearch determines the country of a document by analyzing the following: * The top-level domain (TLD) of the document's URL. * The geographic location of the web server's IP address. See [Country (cr) Parameter Values](https://developers.google.com/custom-search/docs/xml_results#countryCollections) for a list of valid values for this parameter."]
    pub cr: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cx")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of an engine created using the Programmable Search Engine [Control Panel](https://programmablesearchengine.google.com/). This is a custom property not defined in the OpenSearch spec. This parameter is **required**."]
    pub cx: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dateRestrict")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years."]
    pub date_restrict: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disableCnTwTranslation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enables or disables the [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/xml_results#chineseSearch) feature. Supported values are: * `0`: enabled (default) * `1`: disabled"]
    pub disable_cn_tw_translation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "exactTerms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies a phrase that all documents in the search results must contain."]
    pub exact_terms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "excludeTerms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies a word or phrase that should not appear in any documents in the search results."]
    pub exclude_terms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to files of a specified extension. Filetypes supported by Google include: * Adobe Portable Document Format (`pdf`) * Adobe PostScript (`ps`) * Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`) * Lotus WordPro (`lwp`) * Macwrite (`mw`) * Microsoft Excel (`xls`) * Microsoft PowerPoint (`ppt`) * Microsoft Word (`doc`) * Microsoft Works (`wks`, `wps`, `wdb`) * Microsoft Write (`wri`) * Rich Text Format (`rtf`) * Shockwave Flash (`swf`) * Text (`ans`, `txt`). Additional filetypes may be added in the future. An up-to-date list can always be found in Google's [file type FAQ](https://support.google.com/webmasters/answer/35287)."]
    pub file_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Activates or deactivates the automatic filtering of Google search results. See [Automatic Filtering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering) for more information about Google's search results filters. Valid values for this parameter are: * `0`: Disabled * `1`: Enabled (default) **Note**: By default, Google applies filtering to all search results to improve the quality of those results."]
    pub filter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Boosts search results whose country of origin matches the parameter value. See [Country Codes](https://developers.google.com/custom-search/docs/xml_results#countryCodes) for a list of valid values. Specifying a `gl` parameter value in WebSearch requests should improve the relevance of results. This is particularly true for international customers and, even more specifically, for customers in English-speaking countries other than the United States."]
    pub gl: ::std::option::Option<::std::string::String>,
    #[serde(rename = "googleHost")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the Google domain (for example, google.com, google.de, or google.fr) to which the search should be limited."]
    pub google_host: ::std::option::Option<::std::string::String>,
    #[serde(rename = "highRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the ending value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
    pub high_range: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the interface language (host language) of your user interface. Explicitly setting this parameter improves the performance and the quality of your search results. See the [Interface Languages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages) for a list of supported languages."]
    pub hl: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hq")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Appends the specified query terms to the query, as if they were combined with a logical `AND` operator."]
    pub hq: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imgColorType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to images of a specified color type. Supported values are: * `mono` (black and white) * `gray` (grayscale) * `color` (color)"]
    pub img_color_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imgDominantColor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to images with a specific dominant color. Supported values are: * `red` * `orange` * `yellow` * `green` * `teal` * `blue` * `purple` * `pink` * `white` * `gray` * `black` * `brown`"]
    pub img_dominant_color: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imgSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to images of a specified size. Supported values are: * `icon` (small) * `small | medium | large | xlarge` (medium) * `xxlarge` (large) * `huge` (extra-large)"]
    pub img_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imgType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to images of a specified type. Supported values are: * `clipart` (Clip art) * `face` (Face) * `lineart` (Line drawing) * `photo` (Photo) * `animated` (Animated) * `stock` (Stock)"]
    pub img_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inputEncoding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The character encoding supported for search requests."]
    pub input_encoding: ::std::option::Option<::std::string::String>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language of the search results."]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "linkSite")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies that all results should contain a link to a specific URL."]
    pub link_site: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lowRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the starting value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
    pub low_range: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orTerms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms. You can also use the [Boolean OR](https://developers.google.com/custom-search/docs/xml_results#BooleanOrqt) query term for this type of query."]
    pub or_terms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outputEncoding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The character encoding supported for search results."]
    pub output_encoding: ::std::option::Option<::std::string::String>,
    #[serde(rename = "relatedSite")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies that all search results should be pages that are related to the specified URL. The parameter value should be a URL."]
    pub related_site: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rights")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filters based on licensing. Supported values include: * `cc_publicdomain` * `cc_attribute` * `cc_sharealike` * `cc_noncommercial` * `cc_nonderived`"]
    pub rights: ::std::option::Option<::std::string::String>,
    #[serde(rename = "safe")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the [SafeSearch level](https://developers.google.com/custom-search/docs/xml_results#safeSearchLevels) used for filtering out adult results. This is a custom property not defined in the OpenSearch spec. Valid parameter values are: * `\"off\"`: Disable SafeSearch * `\"active\"`: Enable SafeSearch"]
    pub safe: ::std::option::Option<::std::string::String>,
    #[serde(rename = "searchTerms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The search terms entered by the user."]
    pub search_terms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "searchType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allowed values are `web` or `image`. If unspecified, results are limited to webpages."]
    pub search_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "siteSearch")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts results to URLs from a specified site."]
    pub site_search: ::std::option::Option<::std::string::String>,
    #[serde(rename = "siteSearchFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies whether to include or exclude results from the site named in the `sitesearch` parameter. Supported values are: * `i`: include content from site * `e`: exclude content from site"]
    pub site_search_filter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sort")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies that results should be sorted according to the specified expression. For example, sort by date."]
    pub sort: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The index of the current set of search results into the total set of results, where the index of the first result is 1."]
    pub start_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page number of this set of results, where the page length is set by the `count` property."]
    pub start_page: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of the query."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Estimated number of total search results. May not be accurate."]
    pub total_results: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata about a search operation."]
pub struct SearchSearchInformation {
    #[serde(rename = "formattedSearchTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time taken for the server to return search results, formatted according to locale style."]
    pub formatted_search_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formattedTotalResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of search results, formatted according to locale style."]
    pub formatted_total_results: ::std::option::Option<::std::string::String>,
    #[serde(rename = "searchTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time taken for the server to return search results."]
    pub search_time: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "totalResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of search results returned by the query."]
    pub total_results: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Spell correction information for a query."]
pub struct SearchSpelling {
    #[serde(rename = "correctedQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The corrected query."]
    pub corrected_query: ::std::option::Option<::std::string::String>,
    #[serde(rename = "htmlCorrectedQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The corrected query, formatted in HTML."]
    pub html_corrected_query: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "OpenSearch template and URL."]
pub struct SearchUrl {
    #[serde(rename = "template")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The actual [OpenSearch template](http://www.opensearch.org/specifications/opensearch/1.1#opensearch_url_template_syntax) for this API."]
    pub template: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MIME type of the OpenSearch URL template for the Custom Search JSON API."]
    pub _type: ::std::option::Option<::std::string::String>,
}
