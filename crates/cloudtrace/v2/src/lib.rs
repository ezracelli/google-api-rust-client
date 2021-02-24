#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
pub struct QueryParameters {
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "$.xgafv")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "V1 error format."]
    pub xgafv: ::std::option::Option<QueryParametersXgafvEnum>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "access_token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OAuth access token."]
    pub access_token: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ query_parameters_defaults :: alt () }", setter(into))]
    #[serde(rename = "alt")]
    #[serde(default = "query_parameters_defaults :: alt")]
    #[doc = "Data format for response."]
    pub alt: QueryParametersAltEnum,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "callback")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "JSONP"]
    pub callback: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selector specifying which fields to include in a partial response."]
    pub fields: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
    pub key: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "oauth_token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OAuth 2.0 token for the current user."]
    pub oauth_token: ::std::option::Option<::std::string::String>,
    #[builder(
        default = "{ query_parameters_defaults :: pretty_print () }",
        setter(into)
    )]
    #[serde(rename = "prettyPrint")]
    #[serde(default = "query_parameters_defaults :: pretty_print")]
    #[doc = "Returns response with indentations and line breaks."]
    pub pretty_print: ::std::primitive::bool,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "quotaUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
    pub quota_user: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "uploadType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
    pub upload_type: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "upload_protocol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
    pub upload_protocol: ::std::option::Option<::std::string::String>,
}
impl QueryParameters {
    pub fn builder() -> QueryParametersBuilder {
        QueryParametersBuilder::default()
    }
}
mod query_parameters_defaults {
    pub fn alt() -> super::QueryParametersAltEnum {
        serde_json::from_str(&"\"json\"").unwrap()
    }
    pub fn pretty_print() -> ::std::primitive::bool {
        serde_json::from_str(&"true").unwrap()
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "V1 error format."]
pub enum QueryParametersXgafvEnum {
    #[serde(rename = "1")]
    #[doc = "v1 error format"]
    _1,
    #[serde(rename = "2")]
    #[doc = "v2 error format"]
    _2,
}
impl ::std::default::Default for QueryParametersXgafvEnum {
    fn default() -> Self {
        Self::_1
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Data format for response."]
pub enum QueryParametersAltEnum {
    #[serde(rename = "json")]
    #[doc = "Responses with Content-Type of application/json"]
    Json,
    #[serde(rename = "media")]
    #[doc = "Media download with context-dependent Content-Type"]
    Media,
    #[serde(rename = "proto")]
    #[doc = "Responses with Content-Type of application/x-protobuf"]
    Proto,
}
impl ::std::default::Default for QueryParametersAltEnum {
    fn default() -> Self {
        Self::Json
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Text annotation with a set of attributes."]
    pub struct Annotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A set of attributes on the annotation. You can have up to 4 attributes per Annotation."]
        pub attributes: ::std::option::Option<::std::boxed::Box<Attributes>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A user-supplied message describing the event. The maximum length for the description is 256 bytes."]
        pub description: ::std::option::Option<::std::boxed::Box<TruncatableString>>,
    }
    impl Annotation {
        pub fn builder() -> AnnotationBuilder {
            AnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The allowed types for [VALUE] in a `[KEY]:[VALUE]` attribute."]
    pub struct AttributeValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boolValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Boolean value represented by `true` or `false`."]
        pub bool_value: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A 64-bit signed integer."]
        pub int_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A string up to 256 bytes long."]
        pub string_value: ::std::option::Option<::std::boxed::Box<TruncatableString>>,
    }
    impl AttributeValue {
        pub fn builder() -> AttributeValueBuilder {
            AttributeValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A set of attributes, each in the format `[KEY]:[VALUE]`."]
    pub struct Attributes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributeMap")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of attributes. Each attribute's key can be up to 128 bytes long. The value can be a string up to 256 bytes, a signed 64-bit integer, or the Boolean values `true` and `false`. For example: \"/instance_id\": { \"string_value\": { \"value\": \"my-instance\" } } \"/http/request_bytes\": { \"int_value\": 300 } \"abc.com/myattribute\": { \"bool_value\": false }"]
        pub attribute_map: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<AttributeValue>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "droppedAttributesCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of attributes that were discarded. Attributes can be discarded because their keys are too long or because there are too many attributes. If this value is 0 then all attributes are valid."]
        pub dropped_attributes_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl Attributes {
        pub fn builder() -> AttributesBuilder {
            AttributesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for the `BatchWriteSpans` method."]
    pub struct BatchWriteSpansRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spans")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A list of new spans. The span names must not match existing spans, or the results are undefined."]
        pub spans: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Span>>>,
    }
    impl BatchWriteSpansRequest {
        pub fn builder() -> BatchWriteSpansRequestBuilder {
            BatchWriteSpansRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct Empty {}
    impl Empty {
        pub fn builder() -> EmptyBuilder {
            EmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A pointer from the current span to another span in the same trace or in a different trace. For example, this can be used in batching operations, where a single batch handler processes multiple requests from different traces or when the handler receives a request from a different project."]
    pub struct Link {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A set of attributes on the link. You have have up to 32 attributes per link."]
        pub attributes: ::std::option::Option<::std::boxed::Box<Attributes>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spanId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [SPAN_ID] for a span within a trace."]
        pub span_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "traceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [TRACE_ID] for a trace within a project."]
        pub trace_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relationship of the current span relative to the linked span."]
        pub _type: ::std::option::Option<LinkTypeEnum>,
    }
    impl Link {
        pub fn builder() -> LinkBuilder {
            LinkBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The relationship of the current span relative to the linked span."]
    pub enum LinkTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "The relationship of the two spans is unknown."]
        TypeUnspecified,
        #[serde(rename = "CHILD_LINKED_SPAN")]
        #[doc = "The linked span is a child of the current span."]
        ChildLinkedSpan,
        #[serde(rename = "PARENT_LINKED_SPAN")]
        #[doc = "The linked span is a parent of the current span."]
        ParentLinkedSpan,
    }
    impl ::std::default::Default for LinkTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A collection of links, which are references from this span to a span in the same or different trace."]
    pub struct Links {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "droppedLinksCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of dropped links after the maximum size was enforced. If this value is 0, then no links were dropped."]
        pub dropped_links_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A collection of links."]
        pub link: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Link>>>,
    }
    impl Links {
        pub fn builder() -> LinksBuilder {
            LinksBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event describing a message sent/received between Spans."]
    pub struct MessageEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compressedSizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of compressed bytes sent or received. If missing assumed to be the same size as uncompressed."]
        pub compressed_size_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An identifier for the MessageEvent's message that can be used to match SENT and RECEIVED MessageEvents. It is recommended to be unique within a Span."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of MessageEvent. Indicates whether the message was sent or received."]
        pub _type: ::std::option::Option<MessageEventTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uncompressedSizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of uncompressed bytes sent or received."]
        pub uncompressed_size_bytes: ::std::option::Option<::std::string::String>,
    }
    impl MessageEvent {
        pub fn builder() -> MessageEventBuilder {
            MessageEventBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of MessageEvent. Indicates whether the message was sent or received."]
    pub enum MessageEventTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Unknown event type."]
        TypeUnspecified,
        #[serde(rename = "SENT")]
        #[doc = "Indicates a sent message."]
        Sent,
        #[serde(rename = "RECEIVED")]
        #[doc = "Indicates a received message."]
        Received,
    }
    impl ::std::default::Default for MessageEventTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Binary module."]
    pub struct Module {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique identifier for the module, usually a hash of its contents (up to 128 bytes)."]
        pub build_id: ::std::option::Option<::std::boxed::Box<TruncatableString>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "module")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For example: main binary, kernel modules, and dynamic libraries such as libc.so, sharedlib.so (up to 256 bytes)."]
        pub module: ::std::option::Option<::std::boxed::Box<TruncatableString>>,
    }
    impl Module {
        pub fn builder() -> ModuleBuilder {
            ModuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A span represents a single operation within a trace. Spans can be nested to form a trace tree. Often, a trace contains a root span that describes the end-to-end latency, and one or more subspans for its sub-operations. A trace can also contain multiple root spans, or none at all. Spans do not need to be contiguous—there may be gaps or overlaps between spans in a trace."]
    pub struct Span {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A set of attributes on the span. You can have up to 32 attributes per span."]
        pub attributes: ::std::option::Option<::std::boxed::Box<Attributes>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "childSpanCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The number of child spans that were generated while this span was active. If set, allows implementation to detect missing child spans."]
        pub child_span_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A description of the span's operation (up to 128 bytes). Trace displays the description in the Google Cloud Platform Console. For example, the display name can be a qualified method name or a file name and a line number where the operation is called. A best practice is to use the same display name within an application and at the same call point. This makes it easier to correlate spans in different traces."]
        pub display_name: ::std::option::Option<::std::boxed::Box<TruncatableString>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The end time of the span. On the client side, this is the time kept by the local machine where the span execution ends. On the server side, this is the time when the server application handler stops running."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "links")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Links associated with the span. You can have up to 128 links per Span."]
        pub links: ::std::option::Option<::std::boxed::Box<Links>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The resource name of the span in the following format: projects/[PROJECT_ID]/traces/[TRACE_ID]/spans/SPAN_ID is a unique identifier for a trace within a project; it is a 32-character hexadecimal encoding of a 16-byte array. [SPAN_ID] is a unique identifier for a span within a trace; it is a 16-character hexadecimal encoding of an 8-byte array. It should not be zero."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentSpanId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [SPAN_ID] of this span's parent span. If this is a root span, then this field must be empty."]
        pub parent_span_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sameProcessAsParentSpan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Set this parameter to indicate whether this span is in the same process as its parent. If you do not set this parameter, Trace is unable to take advantage of this helpful information."]
        pub same_process_as_parent_span: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spanId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The [SPAN_ID] portion of the span's resource name."]
        pub span_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spanKind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Distinguishes between spans generated in a particular context. For example, two spans with the same name may be distinguished using `CLIENT` (caller) and `SERVER` (callee) to identify an RPC call."]
        pub span_kind: ::std::option::Option<SpanSpanKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stackTrace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stack trace captured at the start of the span."]
        pub stack_trace: ::std::option::Option<::std::boxed::Box<StackTrace>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The start time of the span. On the client side, this is the time kept by the local machine where the span execution starts. On the server side, this is the time when the server's application handler starts running."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The final status for this span."]
        pub status: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeEvents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A set of time events. You can have up to 32 annotations and 128 message events per span."]
        pub time_events: ::std::option::Option<::std::boxed::Box<TimeEvents>>,
    }
    impl Span {
        pub fn builder() -> SpanBuilder {
            SpanBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Distinguishes between spans generated in a particular context. For example, two spans with the same name may be distinguished using `CLIENT` (caller) and `SERVER` (callee) to identify an RPC call."]
    pub enum SpanSpanKindEnum {
        #[serde(rename = "SPAN_KIND_UNSPECIFIED")]
        #[doc = "Unspecified. Do NOT use as default. Implementations MAY assume SpanKind.INTERNAL to be default."]
        SpanKindUnspecified,
        #[serde(rename = "INTERNAL")]
        #[doc = "Indicates that the span is used internally. Default value."]
        Internal,
        #[serde(rename = "SERVER")]
        #[doc = "Indicates that the span covers server-side handling of an RPC or other remote network request."]
        Server,
        #[serde(rename = "CLIENT")]
        #[doc = "Indicates that the span covers the client-side wrapper around an RPC or other remote request."]
        Client,
        #[serde(rename = "PRODUCER")]
        #[doc = "Indicates that the span describes producer sending a message to a broker. Unlike client and server, there is no direct critical path latency relationship between producer and consumer spans (e.g. publishing a message to a pubsub service)."]
        Producer,
        #[serde(rename = "CONSUMER")]
        #[doc = "Indicates that the span describes consumer receiving a message from a broker. Unlike client and server, there is no direct critical path latency relationship between producer and consumer spans (e.g. receiving a message from a pubsub service subscription)."]
        Consumer,
    }
    impl ::std::default::Default for SpanSpanKindEnum {
        fn default() -> Self {
            Self::SpanKindUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a single stack frame in a stack trace."]
    pub struct StackFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The column number where the function call appears, if available. This is important in JavaScript because of its anonymous functions."]
        pub column_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the source file where the function call appears (up to 256 bytes)."]
        pub file_name: ::std::option::Option<::std::boxed::Box<TruncatableString>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "functionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fully-qualified name that uniquely identifies the function or method that is active in this frame (up to 1024 bytes)."]
        pub function_name: ::std::option::Option<::std::boxed::Box<TruncatableString>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The line number in `file_name` where the function call appears."]
        pub line_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "loadModule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The binary module from where the code was loaded."]
        pub load_module: ::std::option::Option<::std::boxed::Box<Module>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalFunctionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An un-mangled function name, if `function_name` is [mangled](http://www.avabodh.com/cxxin/namemangling.html). The name can be fully-qualified (up to 1024 bytes)."]
        pub original_function_name: ::std::option::Option<::std::boxed::Box<TruncatableString>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the deployed source code (up to 128 bytes)."]
        pub source_version: ::std::option::Option<::std::boxed::Box<TruncatableString>>,
    }
    impl StackFrame {
        pub fn builder() -> StackFrameBuilder {
            StackFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A collection of stack frames, which can be truncated."]
    pub struct StackFrames {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "droppedFramesCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of stack frames that were dropped because there were too many stack frames. If this value is 0, then no stack frames were dropped."]
        pub dropped_frames_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frame")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stack frames in this call stack."]
        pub frame: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StackFrame>>>,
    }
    impl StackFrames {
        pub fn builder() -> StackFramesBuilder {
            StackFramesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A call stack appearing in a trace."]
    pub struct StackTrace {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stackFrames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stack frames in this stack trace. A maximum of 128 frames are allowed."]
        pub stack_frames: ::std::option::Option<::std::boxed::Box<StackFrames>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stackTraceHashId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hash ID is used to conserve network bandwidth for duplicate stack traces within a single trace. Often multiple spans will have identical stack traces. The first occurrence of a stack trace should contain both the `stackFrame` content and a value in `stackTraceHashId`. Subsequent spans within the same request can refer to that stack trace by only setting `stackTraceHashId`."]
        pub stack_trace_hash_id: ::std::option::Option<::std::string::String>,
    }
    impl StackTrace {
        pub fn builder() -> StackTraceBuilder {
            StackTraceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub struct Status {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        pub code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "details")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
        pub details: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl Status {
        pub fn builder() -> StatusBuilder {
            StatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A time-stamped annotation or message event in the Span."]
    pub struct TimeEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text annotation with a set of attributes."]
        pub annotation: ::std::option::Option<::std::boxed::Box<Annotation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageEvent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An event describing a message sent/received between Spans."]
        pub message_event: ::std::option::Option<::std::boxed::Box<MessageEvent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "time")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp indicating the time the event occurred."]
        pub time: ::std::option::Option<::std::string::String>,
    }
    impl TimeEvent {
        pub fn builder() -> TimeEventBuilder {
            TimeEventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A collection of `TimeEvent`s. A `TimeEvent` is a time-stamped annotation on the span, consisting of either user-supplied key:value pairs, or details of a message sent/received between Spans."]
    pub struct TimeEvents {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "droppedAnnotationsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of dropped annotations in all the included time events. If the value is 0, then no annotations were dropped."]
        pub dropped_annotations_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "droppedMessageEventsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of dropped message events in all the included time events. If the value is 0, then no message events were dropped."]
        pub dropped_message_events_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeEvent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A collection of `TimeEvent`s."]
        pub time_event: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TimeEvent>>>,
    }
    impl TimeEvents {
        pub fn builder() -> TimeEventsBuilder {
            TimeEventsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a string that might be shortened to a specified length."]
    pub struct TruncatableString {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "truncatedByteCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of bytes removed from the original string. If this value is 0, then the string was not shortened."]
        pub truncated_byte_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The shortened string. For example, if the original string is 500 bytes long and the limit of the string is 128 bytes, then `value` contains the first 128 bytes of the 500-byte string. Truncation always happens on a UTF8 character boundary. If there are multi-byte characters in the string, then the length of the shortened string might be less than the size limit."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl TruncatableString {
        pub fn builder() -> TruncatableStringBuilder {
            TruncatableStringBuilder::default()
        }
    }
}
