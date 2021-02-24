#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "App Engine target. The job will be pushed to a job handler by means of an HTTP request via an http_method such as HTTP POST, HTTP GET, etc. The job is acknowledged by means of an HTTP response code in the range [200 - 299]. Error 503 is considered an App Engine system error instead of an application error. Requests returning error 503 will be retried regardless of retry configuration and not counted against retry counts. Any other response code, or a failure to receive a response before the deadline, constitutes a failed attempt."]
pub struct AppEngineHttpTarget {
    #[serde(rename = "appEngineRouting")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "App Engine Routing setting for the job."]
    pub app_engine_routing: ::std::option::Option<::std::boxed::Box<AppEngineRouting>>,
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Body. HTTP request body. A request body is allowed only if the HTTP method is POST or PUT. It will result in invalid argument error to set a body on a job with an incompatible HttpMethod."]
    pub body: ::std::option::Option<::std::string::String>,
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP request headers. This map contains the header field names and values. Headers can be set when the job is created. Cloud Scheduler sets some headers to default values: * `User-Agent`: By default, this header is `\"AppEngine-Google; (+http://code.google.com/appengine)\"`. This header can be modified, but Cloud Scheduler will append `\"AppEngine-Google; (+http://code.google.com/appengine)\"` to the modified `User-Agent`. * `X-CloudScheduler`: This header will be set to true. If the job has an body, Cloud Scheduler sets the following headers: * `Content-Type`: By default, the `Content-Type` header is set to `\"application/octet-stream\"`. The default can be overridden by explictly setting `Content-Type` to a particular media type when the job is created. For example, `Content-Type` can be set to `\"application/json\"`. * `Content-Length`: This is computed by Cloud Scheduler. This value is output only. It cannot be changed. The headers below are output only. They cannot be set or overridden: * `X-Google-*`: For Google internal use only. * `X-AppEngine-*`: For Google internal use only. In addition, some App Engine headers, which contain job-specific information, are also be sent to the job handler."]
    pub headers: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HTTP method to use for the request. PATCH and OPTIONS are not permitted."]
    pub http_method: ::std::option::Option<AppEngineHttpTargetHttpMethodEnum>,
    #[serde(rename = "relativeUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relative URI. The relative URL must begin with \"/\" and must be a valid HTTP relative URL. It can contain a path, query string arguments, and `#` fragments. If the relative URL is empty, then the root path \"/\" will be used. No spaces are allowed, and the maximum length allowed is 2083 characters."]
    pub relative_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The HTTP method to use for the request. PATCH and OPTIONS are not permitted."]
pub enum AppEngineHttpTargetHttpMethodEnum {
    #[serde(rename = "HTTP_METHOD_UNSPECIFIED")]
    #[doc = "HTTP method unspecified. Defaults to POST."]
    HttpMethodUnspecified,
    #[serde(rename = "POST")]
    #[doc = "HTTP POST"]
    Post,
    #[serde(rename = "GET")]
    #[doc = "HTTP GET"]
    Get,
    #[serde(rename = "HEAD")]
    #[doc = "HTTP HEAD"]
    Head,
    #[serde(rename = "PUT")]
    #[doc = "HTTP PUT"]
    Put,
    #[serde(rename = "DELETE")]
    #[doc = "HTTP DELETE"]
    Delete,
    #[serde(rename = "PATCH")]
    #[doc = "HTTP PATCH"]
    Patch,
    #[serde(rename = "OPTIONS")]
    #[doc = "HTTP OPTIONS"]
    Options,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "App Engine Routing. For more information about services, versions, and instances see [An Overview of App Engine](https://cloud.google.com/appengine/docs/python/an-overview-of-app-engine), [Microservices Architecture on Google App Engine](https://cloud.google.com/appengine/docs/python/microservices-on-app-engine), [App Engine Standard request routing](https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed), and [App Engine Flex request routing](https://cloud.google.com/appengine/docs/flexible/python/how-requests-are-routed)."]
pub struct AppEngineRouting {
    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The host that the job is sent to. For more information about how App Engine requests are routed, see [here](https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed). The host is constructed as: * `host = [application_domain_name]` `| [service] + '.' + [application_domain_name]` `| [version] + '.' + [application_domain_name]` `| [version_dot_service]+ '.' + [application_domain_name]` `| [instance] + '.' + [application_domain_name]` `| [instance_dot_service] + '.' + [application_domain_name]` `| [instance_dot_version] + '.' + [application_domain_name]` `| [instance_dot_version_dot_service] + '.' + [application_domain_name]` * `application_domain_name` = The domain name of the app, for example .appspot.com, which is associated with the job's project ID. * `service =` service * `version =` version * `version_dot_service =` version `+ '.' +` service * `instance =` instance * `instance_dot_service =` instance `+ '.' +` service * `instance_dot_version =` instance `+ '.' +` version * `instance_dot_version_dot_service =` instance `+ '.' +` version `+ '.' +` service If service is empty, then the job will be sent to the service which is the default service when the job is attempted. If version is empty, then the job will be sent to the version which is the default version when the job is attempted. If instance is empty, then the job will be sent to an instance which is available when the job is attempted. If service, version, or instance is invalid, then the job will be sent to the default version of the default service when the job is attempted."]
    pub host: ::std::option::Option<::std::string::String>,
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "App instance. By default, the job is sent to an instance which is available when the job is attempted. Requests can only be sent to a specific instance if [manual scaling is used in App Engine Standard](https://cloud.google.com/appengine/docs/python/an-overview-of-app-engine?hl=en_US#scaling_types_and_instance_classes). App Engine Flex does not support instances. For more information, see [App Engine Standard request routing](https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed) and [App Engine Flex request routing](https://cloud.google.com/appengine/docs/flexible/python/how-requests-are-routed)."]
    pub instance: ::std::option::Option<::std::string::String>,
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "App service. By default, the job is sent to the service which is the default service when the job is attempted."]
    pub service: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "App version. By default, the job is sent to the version which is the default version when the job is attempted."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Http target. The job will be pushed to the job handler by means of an HTTP request via an http_method such as HTTP POST, HTTP GET, etc. The job is acknowledged by means of an HTTP response code in the range [200 - 299]. A failure to receive a response constitutes a failed execution. For a redirected request, the response returned by the redirected request is considered."]
pub struct HttpTarget {
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP request body. A request body is allowed only if the HTTP method is POST, PUT, or PATCH. It is an error to set body on a job with an incompatible HttpMethod."]
    pub body: ::std::option::Option<::std::string::String>,
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user can specify HTTP request headers to send with the job's HTTP request. This map contains the header field names and values. Repeated headers are not supported, but a header value can contain commas. These headers represent a subset of the headers that will accompany the job's HTTP request. Some HTTP request headers will be ignored or replaced. A partial list of headers that will be ignored or replaced is below: - Host: This will be computed by Cloud Scheduler and derived from uri. * `Content-Length`: This will be computed by Cloud Scheduler. * `User-Agent`: This will be set to `\"Google-Cloud-Scheduler\"`. * `X-Google-*`: Google internal use only. * `X-AppEngine-*`: Google internal use only. The total size of headers must be less than 80KB."]
    pub headers: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Which HTTP method to use for the request."]
    pub http_method: ::std::option::Option<HttpTargetHttpMethodEnum>,
    #[serde(rename = "oauthToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If specified, an [OAuth token](https://developers.google.com/identity/protocols/OAuth2) will be generated and attached as an `Authorization` header in the HTTP request. This type of authorization should generally only be used when calling Google APIs hosted on *.googleapis.com."]
    pub oauth_token: ::std::option::Option<::std::boxed::Box<OAuthToken>>,
    #[serde(rename = "oidcToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If specified, an [OIDC](https://developers.google.com/identity/protocols/OpenIDConnect) token will be generated and attached as an `Authorization` header in the HTTP request. This type of authorization can be used for many scenarios, including calling Cloud Run, or endpoints where you intend to validate the token yourself."]
    pub oidc_token: ::std::option::Option<::std::boxed::Box<OidcToken>>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The full URI path that the request will be sent to. This string must begin with either \"http://\" or \"https://\". Some examples of valid values for uri are: `http://acme.com` and `https://acme.com/sales:8080`. Cloud Scheduler will encode some characters for safety and compatibility. The maximum allowed URL length is 2083 characters after encoding."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Which HTTP method to use for the request."]
pub enum HttpTargetHttpMethodEnum {
    #[serde(rename = "HTTP_METHOD_UNSPECIFIED")]
    #[doc = "HTTP method unspecified. Defaults to POST."]
    HttpMethodUnspecified,
    #[serde(rename = "POST")]
    #[doc = "HTTP POST"]
    Post,
    #[serde(rename = "GET")]
    #[doc = "HTTP GET"]
    Get,
    #[serde(rename = "HEAD")]
    #[doc = "HTTP HEAD"]
    Head,
    #[serde(rename = "PUT")]
    #[doc = "HTTP PUT"]
    Put,
    #[serde(rename = "DELETE")]
    #[doc = "HTTP DELETE"]
    Delete,
    #[serde(rename = "PATCH")]
    #[doc = "HTTP PATCH"]
    Patch,
    #[serde(rename = "OPTIONS")]
    #[doc = "HTTP OPTIONS"]
    Options,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for a job. The maximum allowed size for a job is 100KB."]
pub struct Job {
    #[serde(rename = "appEngineHttpTarget")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "App Engine HTTP target."]
    pub app_engine_http_target: ::std::option::Option<::std::boxed::Box<AppEngineHttpTarget>>,
    #[serde(rename = "attemptDeadline")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The deadline for job attempts. If the request handler does not respond by this deadline then the request is cancelled and the attempt is marked as a `DEADLINE_EXCEEDED` failure. The failed attempt can be viewed in execution logs. Cloud Scheduler will retry the job according to the RetryConfig. The allowed duration for this deadline is: * For HTTP targets, between 15 seconds and 30 minutes. * For App Engine HTTP targets, between 15 seconds and 24 hours. * For PubSub targets, this field is ignored."]
    pub attempt_deadline: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optionally caller-specified in CreateJob or UpdateJob. A human-readable description for the job. This string must not contain more than 500 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "httpTarget")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP target."]
    pub http_target: ::std::option::Option<::std::boxed::Box<HttpTarget>>,
    #[serde(rename = "lastAttemptTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the last job attempt started."]
    pub last_attempt_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "legacyAppEngineCron")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. This field is used to manage the legacy App Engine Cron jobs using the Cloud Scheduler API. If the field is set to true, the job will be considered a legacy job. Note that App Engine Cron jobs have fewer features than Cloud Scheduler jobs, e.g., are only limited to App Engine targets."]
    pub legacy_app_engine_cron: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optionally caller-specified in CreateJob, after which it becomes output only. The job name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/jobs/JOB_ID`. * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), or periods (.). For more information, see [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects) * `LOCATION_ID` is the canonical ID for the job's location. The list of available locations can be obtained by calling ListLocations. For more information, see https://cloud.google.com/about/locations/. * `JOB_ID` can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), or underscores (_). The maximum length is 500 characters."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pubsubTarget")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pub/Sub target."]
    pub pubsub_target: ::std::option::Option<::std::boxed::Box<PubsubTarget>>,
    #[serde(rename = "retryConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings that determine the retry behavior."]
    pub retry_config: ::std::option::Option<::std::boxed::Box<RetryConfig>>,
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required, except when used with UpdateJob. Describes the schedule on which the job will be executed. The schedule can be either of the following types: * [Crontab](http://en.wikipedia.org/wiki/Cron#Overview) * English-like [schedule](https://cloud.google.com/scheduler/docs/configuring/cron-job-schedules) As a general rule, execution `n + 1` of a job will not begin until execution `n` has finished. Cloud Scheduler will never allow two simultaneously outstanding executions. For example, this implies that if the `n+1`th execution is scheduled to run at 16:00 but the `n`th execution takes until 16:15, the `n+1`th execution will not start until `16:15`. A scheduled start time will be delayed if the previous execution has not ended when its scheduled time occurs. If retry_count > 0 and a job attempt fails, the job will be tried a total of retry_count times, with exponential backoff, until the next scheduled start time."]
    pub schedule: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scheduleTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The next time the job is scheduled. Note that this may be a retry of a previously failed attempt or the next execution time according to the schedule."]
    pub schedule_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. State of the job."]
    pub state: ::std::option::Option<JobStateEnum>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The response from the target for the last attempted execution."]
    pub status: ::std::option::Option<::std::boxed::Box<Status>>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the time zone to be used in interpreting schedule. The value of this field must be a time zone name from the [tz database](http://en.wikipedia.org/wiki/Tz_database). Note that some time zones include a provision for daylight savings time. The rules for daylight saving time are determined by the chosen tz. For UTC use the string \"utc\". If a time zone is not specified, the default will be in UTC (also known as GMT)."]
    pub time_zone: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userUpdateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The creation time of the job."]
    pub user_update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. State of the job."]
pub enum JobStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Unspecified state."]
    StateUnspecified,
    #[serde(rename = "ENABLED")]
    #[doc = "The job is executing normally."]
    Enabled,
    #[serde(rename = "PAUSED")]
    #[doc = "The job is paused by the user. It will not execute. A user can intentionally pause the job using PauseJobRequest."]
    Paused,
    #[serde(rename = "DISABLED")]
    #[doc = "The job is disabled by the system due to error. The user cannot directly set a job to be disabled."]
    Disabled,
    #[serde(rename = "UPDATE_FAILED")]
    #[doc = "The job state resulting from a failed CloudScheduler.UpdateJob operation. To recover a job from this state, retry CloudScheduler.UpdateJob until a successful response is received."]
    UpdateFailed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for listing jobs using ListJobs."]
pub struct ListJobsResponse {
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of jobs."]
    pub jobs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Job>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve next page of results. Pass this value in the page_token field in the subsequent call to ListJobs to retrieve the next page of results. If this is empty it indicates that there are no more results through which to paginate. The page token is valid for only 2 hours."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Locations.ListLocations."]
pub struct ListLocationsResponse {
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of locations that matches the specified filter in the request."]
    pub locations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Location>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The standard List next-page token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A resource that represents Google Cloud Platform location."]
pub struct Location {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The friendly name for this location, typically a nearby city name. For example, \"Tokyo\"."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cross-service attributes for the location. For example {\"cloud.googleapis.com/region\": \"us-east1\"}"]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The canonical id for this location. For example: `\"us-east1\"`."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Service-specific metadata. For example the available capacity at the given location."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name for the location, which may vary between implementations. For example: `\"projects/example-project/locations/us-east1\"`"]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information needed for generating an [OAuth token](https://developers.google.com/identity/protocols/OAuth2). This type of authorization should generally only be used when calling Google APIs hosted on *.googleapis.com."]
pub struct OAuthToken {
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OAuth scope to be used for generating OAuth access token. If not specified, \"https://www.googleapis.com/auth/cloud-platform\" will be used."]
    pub scope: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceAccountEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Service account email](https://cloud.google.com/iam/docs/service-accounts) to be used for generating OAuth token. The service account must be within the same project as the job. The caller must have iam.serviceAccounts.actAs permission for the service account."]
    pub service_account_email: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information needed for generating an [OpenID Connect token](https://developers.google.com/identity/protocols/OpenIDConnect). This type of authorization can be used for many scenarios, including calling Cloud Run, or endpoints where you intend to validate the token yourself."]
pub struct OidcToken {
    #[serde(rename = "audience")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Audience to be used when generating OIDC token. If not specified, the URI specified in target will be used."]
    pub audience: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceAccountEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Service account email](https://cloud.google.com/iam/docs/service-accounts) to be used for generating OIDC token. The service account must be within the same project as the job. The caller must have iam.serviceAccounts.actAs permission for the service account."]
    pub service_account_email: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for PauseJob."]
pub struct PauseJobRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message that is published by publishers and consumed by subscribers. The message must contain either a non-empty data field or at least one attribute. Note that client libraries represent this object differently depending on the language. See the corresponding [client library documentation](https://cloud.google.com/pubsub/docs/reference/libraries) for more information. See [quotas and limits] (https://cloud.google.com/pubsub/quotas) for more information about message limits."]
pub struct PubsubMessage {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Attributes for this message. If this field is empty, the message must contain non-empty data. This can be used to filter messages on the subscription."]
    pub attributes:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The message data field. If this field is empty, the message must contain at least one attribute."]
    pub data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this message, assigned by the server when the message is published. Guaranteed to be unique within the topic. This value may be read by a subscriber that receives a `PubsubMessage` via a `Pull` call or a push delivery. It must not be populated by the publisher in a `Publish` call."]
    pub message_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderingKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If non-empty, identifies related messages for which publish order should be respected. If a `Subscription` has `enable_message_ordering` set to `true`, messages published with the same non-empty `ordering_key` value will be delivered to subscribers in the order in which they are received by the Pub/Sub system. All `PubsubMessage`s published in a given `PublishRequest` must specify the same `ordering_key` value."]
    pub ordering_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publishTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the message was published, populated by the server when it receives the `Publish` call. It must not be populated by the publisher in a `Publish` call."]
    pub publish_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Pub/Sub target. The job will be delivered by publishing a message to the given Pub/Sub topic."]
pub struct PubsubTarget {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Attributes for PubsubMessage. Pubsub message must contain either non-empty data, or at least one attribute."]
    pub attributes:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The message payload for PubsubMessage. Pubsub message must contain either non-empty data, or at least one attribute."]
    pub data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the Cloud Pub/Sub topic to which messages will be published when a job is delivered. The topic name must be in the same format as required by PubSub's [PublishRequest.name](https://cloud.google.com/pubsub/docs/reference/rpc/google.pubsub.v1#publishrequest), for example `projects/PROJECT_ID/topics/TOPIC_ID`. The topic must be in the same project as the Cloud Scheduler job."]
    pub topic_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for ResumeJob."]
pub struct ResumeJobRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that determine the retry behavior. By default, if a job does not complete successfully (meaning that an acknowledgement is not received from the handler, then it will be retried with exponential backoff according to the settings in RetryConfig."]
pub struct RetryConfig {
    #[serde(rename = "maxBackoffDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum amount of time to wait before retrying a job after it fails. The default value of this field is 1 hour."]
    pub max_backoff_duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxDoublings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time between retries will double `max_doublings` times. A job's retry interval starts at min_backoff_duration, then doubles `max_doublings` times, then increases linearly, and finally retries at intervals of max_backoff_duration up to retry_count times. For example, if min_backoff_duration is 10s, max_backoff_duration is 300s, and `max_doublings` is 3, then the a job will first be retried in 10s. The retry interval will double three times, and then increase linearly by 2^3 * 10s. Finally, the job will retry at intervals of max_backoff_duration until the job has been attempted retry_count times. Thus, the requests will retry at 10s, 20s, 40s, 80s, 160s, 240s, 300s, 300s, .... The default value of this field is 5."]
    pub max_doublings: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "maxRetryDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time limit for retrying a failed job, measured from time when an execution was first attempted. If specified with retry_count, the job will be retried until both limits are reached. The default value for max_retry_duration is zero, which means retry duration is unlimited."]
    pub max_retry_duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minBackoffDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum amount of time to wait before retrying a job after it fails. The default value of this field is 5 seconds."]
    pub min_backoff_duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "retryCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of attempts that the system will make to run a job using the exponential backoff procedure described by max_doublings. The default value of retry_count is zero. If retry_count is zero, a job attempt will *not* be retried if it fails. Instead the Cloud Scheduler system will wait for the next scheduled execution time. If retry_count is set to a non-zero number then Cloud Scheduler will retry failed attempts, using exponential backoff, retry_count times, or until the next scheduled execution time, whichever comes first. Values greater than 5 and negative values are not allowed."]
    pub retry_count: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for forcing a job to run now using RunJob."]
pub struct RunJobRequest {
    #[serde(rename = "legacyAppEngineCron")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is used to manage the legacy App Engine Cron jobs using the Cloud Scheduler API. If the field is set to true, the job in the __cron queue with the corresponding name will be forced to run instead."]
    pub legacy_app_engine_cron: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
pub struct Status {
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
