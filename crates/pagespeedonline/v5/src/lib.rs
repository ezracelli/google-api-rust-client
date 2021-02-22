#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "Message containing the performance timing data for the Lighthouse run."]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Timing {
    #[doc = "The total duration of Lighthouse's run."]
    pub total: Option<f64>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "Message holding the formatted strings used in the renderer."]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RendererFormattedStrings {
    #[doc = "The tooltip text on an expandable chevron icon."]
    pub audit_group_expand_tooltip: Option<String>,
    #[doc = "The heading shown above a list of audits that were not computerd in the run."]
    pub manual_audits_group_title: Option<String>,
    #[doc = "The label shown next to an audit or metric that has had an error."]
    pub error_label: Option<String>,
    #[doc = "The label for values shown in the summary of critical request chains."]
    pub crc_longest_duration_label: Option<String>,
    #[doc = "The error string shown next to an erroring audit."]
    pub error_missing_audit_info: Option<String>,
    #[doc = "The disclaimer shown under performance explaning that the network can vary."]
    pub ls_performance_category_description: Option<String>,
    #[doc = "The heading for the estimated page load savings opportunity of an audit."]
    pub opportunity_resource_column_label: Option<String>,
    #[doc = "The label for the initial request in a critical request chain."]
    pub crc_initial_navigation: Option<String>,
    #[doc = "The heading that is shown above a list of audits that are passing."]
    pub passed_audits_group_title: Option<String>,
    #[doc = "The label shown above a bulleted list of warnings."]
    pub warning_header: Option<String>,
    #[doc = "The disclaimer shown below a performance metric value."]
    pub variance_disclaimer: Option<String>,
    #[doc = "The heading for the estimated page load savings of opportunity audits."]
    pub opportunity_savings_column_label: Option<String>,
    #[doc = "The label that explains the score gauges scale (0-49, 50-89, 90-100)."]
    pub scorescale_label: Option<String>,
    #[doc = "The heading shown above a list of audits that do not apply to a page."]
    pub not_applicable_audits_group_title: Option<String>,
    #[doc = "The title of the lab data performance category."]
    pub lab_data_title: Option<String>,
    #[doc = "The label shown preceding important warnings that may have invalidated an entire report."]
    pub toplevel_warnings_message: Option<String>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "A CrUX metric object for a single metric and form factor."]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct UserPageLoadMetricV5 {
    #[doc = "The median number of the metric, in millisecond."]
    pub median: Option<i64>,
    #[doc = "Identifies the type of the metric."]
    pub metric_id: Option<String>,
    #[doc = "The category of the specific time metric."]
    pub category: Option<String>,
    #[doc = "We use this field to store certain percentile value for this metric. For v4, this field contains pc50. For v5, this field contains pc90."]
    pub percentile: Option<i64>,
    #[doc = "Identifies the form factor of the metric being collected."]
    pub form_factor: Option<String>,
    #[doc = "Metric distributions. Proportions should sum up to 1."]
    pub distributions: Option<Vec<Bucket>>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "Message containing the configuration settings for the Lighthouse run."]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ConfigSettings {
    #[doc = "How Lighthouse should interpret this run in regards to scoring performance metrics and skipping mobile-only tests in desktop."]
    pub form_factor: Option<String>,
    #[doc = "How Lighthouse was run, e.g. from the Chrome extension or from the npm module."]
    pub channel: Option<String>,
    #[doc = "The form factor the emulation should use. This field is deprecated, form_factor should be used instead."]
    pub emulated_form_factor: Option<String>,
    #[doc = "The locale setting."]
    pub locale: Option<String>,
    #[doc = "List of categories of audits the run should conduct."]
    pub only_categories: Option<serde_json::Value>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "The Lighthouse result object."]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct LighthouseResultV5 {
    #[doc = "The final resolved url that was audited."]
    pub final_url: Option<String>,
    #[doc = "List of all run warnings in the LHR. Will always output to at least `[]`."]
    pub run_warnings: Option<Vec<serde_json::Value>>,
    #[doc = "The original requested url."]
    pub requested_url: Option<String>,
    #[doc = "Map of category groups in the LHR."]
    pub category_groups: Option<serde_json::Value>,
    #[doc = "Environment settings that were used when making this LHR."]
    pub environment: Option<Environment>,
    #[doc = "A top-level error message that, if present, indicates a serious enough problem that this Lighthouse result may need to be discarded."]
    pub runtime_error: Option<RuntimeError>,
    #[doc = "The internationalization strings that are required to render the LHR."]
    pub i18n: Option<I18n>,
    #[doc = "Timing information for this LHR."]
    pub timing: Option<Timing>,
    #[doc = "The Stack Pack advice strings."]
    pub stack_packs: Option<Vec<StackPack>>,
    #[doc = "The user agent that was used to run this LHR."]
    pub user_agent: Option<String>,
    #[doc = "Map of categories in the LHR."]
    pub categories: Option<Categories>,
    #[doc = "Map of audits in the LHR."]
    pub audits: Option<serde_json::Value>,
    #[doc = "The lighthouse version that was used to generate this LHR."]
    pub lighthouse_version: Option<String>,
    #[doc = "The time that this run was fetched."]
    pub fetch_time: Option<String>,
    #[doc = "The configuration settings for this LHR."]
    pub config_settings: Option<ConfigSettings>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "Message containing a runtime error config."]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeError {
    #[doc = "The enumerated Lighthouse Error code."]
    pub code: Option<String>,
    #[doc = "A human readable message explaining the error code."]
    pub message: Option<String>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "Message containing environment configuration for a Lighthouse run."]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    #[doc = "The benchmark index number that indicates rough device class."]
    pub benchmark_index: Option<f64>,
    #[doc = "The user agent string of the version of Chrome used."]
    pub host_user_agent: Option<String>,
    #[doc = "The user agent string that was sent over the network."]
    pub network_user_agent: Option<String>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "Message containing Stack Pack information."]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct StackPack {
    #[doc = "The stack pack title."]
    pub title: Option<String>,
    #[doc = "The stack pack advice strings."]
    pub descriptions: Option<serde_json::Value>,
    #[doc = "The stack pack id."]
    pub id: Option<String>,
    #[doc = "The stack pack icon data uri."]
    #[serde(rename = "iconDataURL")]
    pub icon_data_url: Option<String>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "Message containing a category"]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct CategoryGroupV5 {
    #[doc = "The human readable title of the group"]
    pub title: Option<String>,
    #[doc = "The description of what the category is grouping"]
    pub description: Option<String>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "The Pagespeed Version object."]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct PagespeedVersion {
    #[doc = "The minor version number of PageSpeed used to generate these results."]
    pub minor: Option<String>,
    #[doc = "The major version number of PageSpeed used to generate these results."]
    pub major: Option<String>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "A Lighthouse category."]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct LighthouseCategoryV5 {
    #[doc = "An array of references to all the audit members of this category."]
    pub audit_refs: Option<Vec<AuditRefs>>,
    #[doc = "The human-friendly name of the category."]
    pub title: Option<String>,
    #[doc = "A more detailed description of the category and its importance."]
    pub description: Option<String>,
    #[doc = "The string identifier of the category."]
    pub id: Option<String>,
    #[doc = "A description for the manual audits in the category."]
    pub manual_description: Option<String>,
    #[doc = "The overall score of the category, the weighted average of all its audits. (The category's score, can be null.)"]
    pub score: Option<serde_json::Value>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "The Pagespeed API response object."]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct PagespeedApiPagespeedResponseV5 {
    #[doc = "The UTC timestamp of this analysis."]
    #[serde(rename = "analysisUTCTimestamp")]
    pub analysis_utc_timestamp: Option<String>,
    #[doc = "Kind of result."]
    pub kind: Option<String>,
    #[doc = "Metrics of end users' page loading experience."]
    pub loading_experience: Option<PagespeedApiLoadingExperienceV5>,
    #[doc = "Metrics of the aggregated page loading experience of the origin"]
    pub origin_loading_experience: Option<PagespeedApiLoadingExperienceV5>,
    #[doc = "The captcha verify result"]
    pub captcha_result: Option<String>,
    #[doc = "The version of PageSpeed used to generate these results."]
    pub version: Option<PagespeedVersion>,
    #[doc = "Canonicalized and final URL for the document, after following page redirects (if any)."]
    pub id: Option<String>,
    #[doc = "Lighthouse response for the audit url as an object."]
    pub lighthouse_result: Option<LighthouseResultV5>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "An audit's result object in a Lighthouse result."]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct LighthouseAuditResultV5 {
    #[doc = "The value that should be displayed on the UI for this audit."]
    pub display_value: Option<String>,
    #[doc = "The audit's id."]
    pub id: Option<String>,
    #[doc = "The human readable title."]
    pub title: Option<String>,
    #[doc = "Freeform details section of the audit."]
    pub details: Option<serde_json::Value>,
    #[doc = "The enumerated score display mode."]
    pub score_display_mode: Option<String>,
    #[doc = "The description of the audit."]
    pub description: Option<String>,
    #[doc = "The score of the audit, can be null."]
    pub score: Option<serde_json::Value>,
    #[doc = "An error message from a thrown error inside the audit."]
    pub error_message: Option<String>,
    #[doc = "An explanation of the errors in the audit."]
    pub explanation: Option<String>,
    #[doc = "Possible warnings that occurred in the audit, can be null."]
    pub warnings: Option<serde_json::Value>,
    #[doc = "A numeric value that has a meaning specific to the audit, e.g. the number of nodes in the DOM or the timestamp of a specific load event. More information can be found in the audit details, if present."]
    pub numeric_value: Option<f64>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "A proportion of data in the total distribution, bucketed by a min/max percentage. Each bucket's range is bounded by min <= x < max, In millisecond."]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Bucket {
    #[doc = "The proportion of data in this bucket."]
    pub proportion: Option<f64>,
    #[doc = "Lower bound for a bucket's range."]
    pub min: Option<i64>,
    #[doc = "Upper bound for a bucket's range."]
    pub max: Option<i64>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "The categories in a Lighthouse run."]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Categories {
    #[doc = "The performance category, containing all performance related audits."]
    pub performance: Option<LighthouseCategoryV5>,
    #[doc = "The accessibility category, containing all accessibility related audits."]
    pub accessibility: Option<LighthouseCategoryV5>,
    #[doc = "The Progressive-Web-App (PWA) category, containing all pwa related audits."]
    pub pwa: Option<LighthouseCategoryV5>,
    #[doc = "The Search-Engine-Optimization (SEO) category, containing all seo related audits."]
    pub seo: Option<LighthouseCategoryV5>,
    #[doc = "The best practices category, containing all best practices related audits."]
    #[serde(rename = "best-practices")]
    pub best_practices: Option<LighthouseCategoryV5>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "Message containing the i18n data for the LHR - Version 1."]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct I18n {
    #[doc = "Internationalized strings that are formatted to the locale in configSettings."]
    pub renderer_formatted_strings: Option<RendererFormattedStrings>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "The CrUX loading experience object that contains CrUX data breakdowns."]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct PagespeedApiLoadingExperienceV5 {
    #[doc = "The url, pattern or origin which the metrics are on."]
    pub id: Option<String>,
    #[doc = "True if the result is an origin fallback from a page, false otherwise."]
    #[serde(rename = "origin_fallback")]
    pub origin_fallback: Option<bool>,
    #[doc = "The map of ."]
    pub metrics: Option<serde_json::Value>,
    #[doc = "The human readable speed \"category\" of the id."]
    #[serde(rename = "overall_category")]
    pub overall_category: Option<String>,
    #[doc = "The requested URL, which may differ from the resolved \"id\"."]
    #[serde(rename = "initial_url")]
    pub initial_url: Option<String>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
#[doc = "A light reference to an audit by id, used to group and weight audits in a given category."]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct AuditRefs {
    #[doc = "The category group that the audit belongs to (optional)."]
    pub group: Option<String>,
    #[doc = "The audit ref id."]
    pub id: Option<String>,
    #[doc = "The weight this audit's score has on the overall category score."]
    pub weight: Option<f64>,
}
