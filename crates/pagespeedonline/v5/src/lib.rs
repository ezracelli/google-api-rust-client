#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A light reference to an audit by id, used to group and weight audits in a given category."]
pub struct AuditRefs {
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The category group that the audit belongs to (optional)."]
    pub group: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The audit ref id."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "weight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The weight this audit's score has on the overall category score."]
    pub weight: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A proportion of data in the total distribution, bucketed by a min/max percentage. Each bucket's range is bounded by min <= x < max, In millisecond."]
pub struct Bucket {
    #[serde(rename = "max")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Upper bound for a bucket's range."]
    pub max: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "min")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Lower bound for a bucket's range."]
    pub min: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "proportion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The proportion of data in this bucket."]
    pub proportion: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The categories in a Lighthouse run."]
pub struct Categories {
    #[serde(rename = "accessibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The accessibility category, containing all accessibility related audits."]
    pub accessibility: ::std::option::Option<::std::boxed::Box<LighthouseCategoryV5>>,
    #[serde(rename = "best-practices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The best practices category, containing all best practices related audits."]
    pub best_practices: ::std::option::Option<::std::boxed::Box<LighthouseCategoryV5>>,
    #[serde(rename = "performance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The performance category, containing all performance related audits."]
    pub performance: ::std::option::Option<::std::boxed::Box<LighthouseCategoryV5>>,
    #[serde(rename = "pwa")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Progressive-Web-App (PWA) category, containing all pwa related audits."]
    pub pwa: ::std::option::Option<::std::boxed::Box<LighthouseCategoryV5>>,
    #[serde(rename = "seo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Search-Engine-Optimization (SEO) category, containing all seo related audits."]
    pub seo: ::std::option::Option<::std::boxed::Box<LighthouseCategoryV5>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message containing a category"]
pub struct CategoryGroupV5 {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of what the category is grouping"]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human readable title of the group"]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message containing the configuration settings for the Lighthouse run."]
pub struct ConfigSettings {
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How Lighthouse was run, e.g. from the Chrome extension or from the npm module."]
    pub channel: ::std::option::Option<::std::string::String>,
    #[serde(rename = "emulatedFormFactor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The form factor the emulation should use. This field is deprecated, form_factor should be used instead."]
    pub emulated_form_factor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formFactor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How Lighthouse should interpret this run in regards to scoring performance metrics and skipping mobile-only tests in desktop."]
    pub form_factor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The locale setting."]
    pub locale: ::std::option::Option<::std::string::String>,
    #[serde(rename = "onlyCategories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of categories of audits the run should conduct."]
    pub only_categories: ::std::option::Option<::serde_json::Value>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message containing environment configuration for a Lighthouse run."]
pub struct Environment {
    #[serde(rename = "benchmarkIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The benchmark index number that indicates rough device class."]
    pub benchmark_index: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "hostUserAgent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user agent string of the version of Chrome used."]
    pub host_user_agent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "networkUserAgent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user agent string that was sent over the network."]
    pub network_user_agent: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message containing the i18n data for the LHR - Version 1."]
pub struct I18n {
    #[serde(rename = "rendererFormattedStrings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Internationalized strings that are formatted to the locale in configSettings."]
    pub renderer_formatted_strings:
        ::std::option::Option<::std::boxed::Box<RendererFormattedStrings>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An audit's result object in a Lighthouse result."]
pub struct LighthouseAuditResultV5 {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of the audit."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Freeform details section of the audit."]
    pub details: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "displayValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value that should be displayed on the UI for this audit."]
    pub display_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An error message from a thrown error inside the audit."]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "explanation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An explanation of the errors in the audit."]
    pub explanation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The audit's id."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numericValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A numeric value that has a meaning specific to the audit, e.g. the number of nodes in the DOM or the timestamp of a specific load event. More information can be found in the audit details, if present."]
    pub numeric_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The score of the audit, can be null."]
    pub score: ::std::option::Option<::serde_json::Value>,
    #[serde(rename = "scoreDisplayMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The enumerated score display mode."]
    pub score_display_mode: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human readable title."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Possible warnings that occurred in the audit, can be null."]
    pub warnings: ::std::option::Option<::serde_json::Value>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Lighthouse category."]
pub struct LighthouseCategoryV5 {
    #[serde(rename = "auditRefs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An array of references to all the audit members of this category."]
    pub audit_refs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditRefs>>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A more detailed description of the category and its importance."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The string identifier of the category."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "manualDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description for the manual audits in the category."]
    pub manual_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The overall score of the category, the weighted average of all its audits. (The category's score, can be null.)"]
    pub score: ::std::option::Option<::serde_json::Value>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human-friendly name of the category."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Lighthouse result object."]
pub struct LighthouseResultV5 {
    #[serde(rename = "audits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Map of audits in the LHR."]
    pub audits: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<LighthouseAuditResultV5>>,
    >,
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Map of categories in the LHR."]
    pub categories: ::std::option::Option<::std::boxed::Box<Categories>>,
    #[serde(rename = "categoryGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Map of category groups in the LHR."]
    pub category_groups: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<CategoryGroupV5>>,
    >,
    #[serde(rename = "configSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration settings for this LHR."]
    pub config_settings: ::std::option::Option<::std::boxed::Box<ConfigSettings>>,
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Environment settings that were used when making this LHR."]
    pub environment: ::std::option::Option<::std::boxed::Box<Environment>>,
    #[serde(rename = "fetchTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time that this run was fetched."]
    pub fetch_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "finalUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The final resolved url that was audited."]
    pub final_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "i18n")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The internationalization strings that are required to render the LHR."]
    pub i18n: ::std::option::Option<::std::boxed::Box<I18n>>,
    #[serde(rename = "lighthouseVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The lighthouse version that was used to generate this LHR."]
    pub lighthouse_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestedUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The original requested url."]
    pub requested_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "runWarnings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of all run warnings in the LHR. Will always output to at least `[]`."]
    pub run_warnings: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
    #[serde(rename = "runtimeError")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A top-level error message that, if present, indicates a serious enough problem that this Lighthouse result may need to be discarded."]
    pub runtime_error: ::std::option::Option<::std::boxed::Box<RuntimeError>>,
    #[serde(rename = "stackPacks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Stack Pack advice strings."]
    pub stack_packs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StackPack>>>,
    #[serde(rename = "timing")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timing information for this LHR."]
    pub timing: ::std::option::Option<::std::boxed::Box<Timing>>,
    #[serde(rename = "userAgent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user agent that was used to run this LHR."]
    pub user_agent: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The CrUX loading experience object that contains CrUX data breakdowns."]
pub struct PagespeedApiLoadingExperienceV5 {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The url, pattern or origin which the metrics are on."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "initial_url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The requested URL, which may differ from the resolved \"id\"."]
    pub initial_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The map of ."]
    pub metrics: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<UserPageLoadMetricV5>>,
    >,
    #[serde(rename = "origin_fallback")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if the result is an origin fallback from a page, false otherwise."]
    pub origin_fallback: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "overall_category")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human readable speed \"category\" of the id."]
    pub overall_category: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Pagespeed API response object."]
pub struct PagespeedApiPagespeedResponseV5 {
    #[serde(rename = "analysisUTCTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The UTC timestamp of this analysis."]
    pub analysis_utc_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "captchaResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The captcha verify result"]
    pub captcha_result: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Canonicalized and final URL for the document, after following page redirects (if any)."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Kind of result."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lighthouseResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Lighthouse response for the audit url as an object."]
    pub lighthouse_result: ::std::option::Option<::std::boxed::Box<LighthouseResultV5>>,
    #[serde(rename = "loadingExperience")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metrics of end users' page loading experience."]
    pub loading_experience:
        ::std::option::Option<::std::boxed::Box<PagespeedApiLoadingExperienceV5>>,
    #[serde(rename = "originLoadingExperience")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metrics of the aggregated page loading experience of the origin"]
    pub origin_loading_experience:
        ::std::option::Option<::std::boxed::Box<PagespeedApiLoadingExperienceV5>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of PageSpeed used to generate these results."]
    pub version: ::std::option::Option<::std::boxed::Box<PagespeedVersion>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Pagespeed Version object."]
pub struct PagespeedVersion {
    #[serde(rename = "major")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The major version number of PageSpeed used to generate these results."]
    pub major: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minor version number of PageSpeed used to generate these results."]
    pub minor: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message holding the formatted strings used in the renderer."]
pub struct RendererFormattedStrings {
    #[serde(rename = "auditGroupExpandTooltip")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tooltip text on an expandable chevron icon."]
    pub audit_group_expand_tooltip: ::std::option::Option<::std::string::String>,
    #[serde(rename = "crcInitialNavigation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The label for the initial request in a critical request chain."]
    pub crc_initial_navigation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "crcLongestDurationLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The label for values shown in the summary of critical request chains."]
    pub crc_longest_duration_label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errorLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The label shown next to an audit or metric that has had an error."]
    pub error_label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errorMissingAuditInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error string shown next to an erroring audit."]
    pub error_missing_audit_info: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labDataTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the lab data performance category."]
    pub lab_data_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lsPerformanceCategoryDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The disclaimer shown under performance explaning that the network can vary."]
    pub ls_performance_category_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "manualAuditsGroupTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The heading shown above a list of audits that were not computerd in the run."]
    pub manual_audits_group_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notApplicableAuditsGroupTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The heading shown above a list of audits that do not apply to a page."]
    pub not_applicable_audits_group_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "opportunityResourceColumnLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The heading for the estimated page load savings opportunity of an audit."]
    pub opportunity_resource_column_label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "opportunitySavingsColumnLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The heading for the estimated page load savings of opportunity audits."]
    pub opportunity_savings_column_label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "passedAuditsGroupTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The heading that is shown above a list of audits that are passing."]
    pub passed_audits_group_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scorescaleLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The label that explains the score gauges scale (0-49, 50-89, 90-100)."]
    pub scorescale_label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "toplevelWarningsMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The label shown preceding important warnings that may have invalidated an entire report."]
    pub toplevel_warnings_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "varianceDisclaimer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The disclaimer shown below a performance metric value."]
    pub variance_disclaimer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "warningHeader")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The label shown above a bulleted list of warnings."]
    pub warning_header: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message containing a runtime error config."]
pub struct RuntimeError {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The enumerated Lighthouse Error code."]
    pub code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A human readable message explaining the error code."]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message containing Stack Pack information."]
pub struct StackPack {
    #[serde(rename = "descriptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The stack pack advice strings."]
    pub descriptions:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "iconDataURL")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The stack pack icon data uri."]
    pub icon_data_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The stack pack id."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The stack pack title."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message containing the performance timing data for the Lighthouse run."]
pub struct Timing {
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total duration of Lighthouse's run."]
    pub total: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A CrUX metric object for a single metric and form factor."]
pub struct UserPageLoadMetricV5 {
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The category of the specific time metric."]
    pub category: ::std::option::Option<::std::string::String>,
    #[serde(rename = "distributions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metric distributions. Proportions should sum up to 1."]
    pub distributions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Bucket>>>,
    #[serde(rename = "formFactor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies the form factor of the metric being collected."]
    pub form_factor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "median")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The median number of the metric, in millisecond."]
    pub median: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "metricId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies the type of the metric."]
    pub metric_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "percentile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "We use this field to store certain percentile value for this metric. For v4, this field contains pc50. For v5, this field contains pc90."]
    pub percentile: ::std::option::Option<::std::primitive::i64>,
}
