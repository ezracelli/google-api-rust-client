#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for the `ListTraces` method."]
pub struct ListTracesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If defined, indicates that there are more traces that match the request and that this value should be passed to the next request to continue retrieving additional traces."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "traces")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of trace records as specified by the view parameter."]
    pub traces: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Trace>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A trace describes how long it takes for an application to perform an operation. It consists of a set of spans, each of which represent a single timed event within the operation."]
pub struct Trace {
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Project ID of the Cloud project where the trace data is stored."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "spans")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Collection of spans in the trace."]
    pub spans: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TraceSpan>>>,
    #[serde(rename = "traceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Globally unique identifier for the trace. This identifier is a 128-bit numeric value formatted as a 32-byte hex string. For example, `382d4f4c6b7bb2f4a972559d9085001d`. The numeric value should not be zero."]
    pub trace_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A span represents a single timed event within a trace. Spans can be nested and form a trace tree. Often, a trace contains a root span that describes the end-to-end latency of an operation and, optionally, one or more subspans for its suboperations. Spans do not need to be contiguous. There may be gaps between spans in a trace."]
pub struct TraceSpan {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "End time of the span in nanoseconds from the UNIX epoch."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Distinguishes between spans generated in a particular context. For example, two spans with the same name may be distinguished using `RPC_CLIENT` and `RPC_SERVER` to identify queueing latency associated with the span."]
    pub kind: ::std::option::Option<TraceSpanKindEnum>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Collection of labels associated with the span. Label keys must be less than 128 bytes. Label values must be less than 16 kilobytes (10MB for `/stacktrace` values). Some predefined label keys exist, or you may create your own. When creating your own, we recommend the following formats: * `/category/product/key` for agents of well-known products (e.g. `/db/mongodb/read_size`). * `short_host/path/key` for domain-specific keys (e.g. `foo.com/myproduct/bar`) Predefined labels include: * `/agent` * `/component` * `/error/message` * `/error/name` * `/http/client_city` * `/http/client_country` * `/http/client_protocol` * `/http/client_region` * `/http/host` * `/http/method` * `/http/path` * `/http/redirected_url` * `/http/request/size` * `/http/response/size` * `/http/route` * `/http/status_code` * `/http/url` * `/http/user_agent` * `/pid` * `/stacktrace` * `/tid`"]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the span. Must be less than 128 bytes. The span name is sanitized and displayed in the Trace tool in the Google Cloud Platform Console. The name may be a method name or some other per-call site name. For the same executable and the same call point, a best practice is to use a consistent name, which makes it easier to correlate cross-trace spans."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parentSpanId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. ID of the parent span, if any."]
    pub parent_span_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "spanId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier for the span. Must be a 64-bit integer other than 0 and unique within a trace. For example, `2205310701640571284`."]
    pub span_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Start time of the span in nanoseconds from the UNIX epoch."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Distinguishes between spans generated in a particular context. For example, two spans with the same name may be distinguished using `RPC_CLIENT` and `RPC_SERVER` to identify queueing latency associated with the span."]
pub enum TraceSpanKindEnum {
    #[serde(rename = "SPAN_KIND_UNSPECIFIED")]
    #[doc = "Unspecified."]
    SpanKindUnspecified,
    #[serde(rename = "RPC_SERVER")]
    #[doc = "Indicates that the span covers server-side handling of an RPC or other remote network request."]
    RpcServer,
    #[serde(rename = "RPC_CLIENT")]
    #[doc = "Indicates that the span covers the client-side wrapper around an RPC or other remote request."]
    RpcClient,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of new or updated traces."]
pub struct Traces {
    #[serde(rename = "traces")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of traces."]
    pub traces: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Trace>>>,
}
