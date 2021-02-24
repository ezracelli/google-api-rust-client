#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the natural speech audio to be processed."]
pub struct GoogleCloudDialogflowCxV3AudioInput {
    #[serde(rename = "audio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The natural language speech audio to be processed. A single request can contain up to 1 minute of speech audio data. The transcribed text cannot contain more than 256 bytes. For non-streaming audio detect intent, both `config` and `audio` must be provided. For streaming audio detect intent, `config` must be provided in the first request and `audio` must be provided in all following requests."]
    pub audio: ::std::option::Option<::std::string::String>,
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Instructs the speech recognizer how to process the speech audio."]
    pub config: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3InputAudioConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata returned for the TestCases.BatchRunTestCases long running operation."]
pub struct GoogleCloudDialogflowCxV3BatchRunTestCasesMetadata {
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The test errors."]
    pub errors: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3TestError>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for TestCases.BatchRunTestCases."]
pub struct GoogleCloudDialogflowCxV3BatchRunTestCasesResponse {
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The test case results. The detailed conversation turns are empty in this response."]
    pub results: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3TestCaseResult>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "One interaction between a human and virtual agent. The human provides some input and the virtual agent provides a response."]
pub struct GoogleCloudDialogflowCxV3ConversationTurn {
    #[serde(rename = "userInput")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user input."]
    pub user_input: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3ConversationTurnUserInput>,
    >,
    #[serde(rename = "virtualAgentOutput")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The virtual agent output."]
    pub virtual_agent_output: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3ConversationTurnVirtualAgentOutput>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The input from the human user."]
pub struct GoogleCloudDialogflowCxV3ConversationTurnUserInput {
    #[serde(rename = "injectedParameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameters that need to be injected into the conversation during intent detection."]
    pub injected_parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supports text input, event input, dtmf input in the test case."]
    pub input: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3QueryInput>>,
    #[serde(rename = "isWebhookEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If webhooks should be allowed to trigger in response to the user utterance. Often if parameters are injected, webhooks should not be enabled."]
    pub is_webhook_enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The output from the virtual agent."]
pub struct GoogleCloudDialogflowCxV3ConversationTurnVirtualAgentOutput {
    #[serde(rename = "currentPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Page on which the utterance was spoken. Only name and displayName will be set."]
    pub current_page: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3Page>>,
    #[serde(rename = "diagnosticInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Input only. The diagnostic info output for the turn."]
    pub diagnostic_info:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "differences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If this is part of a result conversation turn, the list of differences between the original run and the replay for this output, if any."]
    pub differences: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3TestRunDifference>>,
    >,
    #[serde(rename = "sessionParameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The session parameters available to the bot at this point."]
    pub session_parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Response error from the agent in the test result. If set, other output is empty."]
    pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    #[serde(rename = "textResponses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text responses from the agent for the turn."]
    pub text_responses: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessageText>>,
    >,
    #[serde(rename = "triggeredIntent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Intent that triggered the response. Only name and displayName will be set."]
    pub triggered_intent: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3Intent>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for CreateDocument operation."]
pub struct GoogleCloudDialogflowCxV3CreateDocumentOperationMetadata {
    #[serde(rename = "genericMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generic information of the operation."]
    pub generic_metadata: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadata>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata associated with the long running operation for Versions.CreateVersion."]
pub struct GoogleCloudDialogflowCxV3CreateVersionOperationMetadata {
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the created version. Format: `projects//locations//agents//flows//versions/`."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for DeleteDocument operation."]
pub struct GoogleCloudDialogflowCxV3DeleteDocumentOperationMetadata {
    #[serde(rename = "genericMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generic information of the operation."]
    pub generic_metadata: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadata>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the input for dtmf event."]
pub struct GoogleCloudDialogflowCxV3DtmfInput {
    #[serde(rename = "digits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The dtmf digits."]
    pub digits: ::std::option::Option<::std::string::String>,
    #[serde(rename = "finishDigit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The finish digit (if any)."]
    pub finish_digit: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event handler specifies an event that can be handled during a session. When the specified event happens, the following actions are taken in order: * If there is a `trigger_fulfillment` associated with the event, it will be called. * If there is a `target_page` associated with the event, the session will transition into the specified page. * If there is a `target_flow` associated with the event, the session will transition into the specified flow."]
pub struct GoogleCloudDialogflowCxV3EventHandler {
    #[serde(rename = "event")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the event to handle."]
    pub event: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique identifier of this event handler."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetFlow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target flow to transition to. Format: `projects//locations//agents//flows/`."]
    pub target_flow: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target page to transition to. Format: `projects//locations//agents//flows//pages/`."]
    pub target_page: ::std::option::Option<::std::string::String>,
    #[serde(rename = "triggerFulfillment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fulfillment to call when the event occurs. Handling webhook errors with a fulfillment enabled with webhook could cause infinite loop. It is invalid to specify such fulfillment for a handler handling webhooks."]
    pub trigger_fulfillment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3Fulfillment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the event to trigger."]
pub struct GoogleCloudDialogflowCxV3EventInput {
    #[serde(rename = "event")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the event."]
    pub event: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Agents.ExportAgent."]
pub struct GoogleCloudDialogflowCxV3ExportAgentResponse {
    #[serde(rename = "agentContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uncompressed raw byte content for agent."]
    pub agent_content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "agentUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI to a file containing the exported agent. This field is populated only if `agent_uri` is specified in ExportAgentRequest."]
    pub agent_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata returned for the TestCases.ExportTestCases long running operation."]
pub struct GoogleCloudDialogflowCxV3ExportTestCasesMetadata {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for TestCases.ExportTestCases."]
pub struct GoogleCloudDialogflowCxV3ExportTestCasesResponse {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uncompressed raw byte content for test cases."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gcsUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI to a file containing the exported test cases. This field is populated only if `gcs_uri` is specified in ExportTestCasesRequest."]
    pub gcs_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A form is a data model that groups related parameters that can be collected from the user. The process in which the agent prompts the user and collects parameter values from the user is called form filling. A form can be added to a page. When form filling is done, the filled parameters will be written to the session."]
pub struct GoogleCloudDialogflowCxV3Form {
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameters to collect from the user."]
    pub parameters: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3FormParameter>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a form parameter."]
pub struct GoogleCloudDialogflowCxV3FormParameter {
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default value of an optional parameter. If the parameter is required, the default value will be ignored."]
    pub default_value: ::std::option::Option<::serde_json::Value>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The human-readable name of the parameter, unique within the form."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The entity type of the parameter. Format: `projects/-/locations/-/agents/-/entityTypes/` for system entity types (for example, `projects/-/locations/-/agents/-/entityTypes/sys.date`), or `projects//locations//agents//entityTypes/` for developer entity types."]
    pub entity_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fillBehavior")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Defines fill behavior for the parameter."]
    pub fill_behavior: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3FormParameterFillBehavior>,
    >,
    #[serde(rename = "isList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the parameter represents a list of values."]
    pub is_list: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "redact")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the parameter content should be redacted in log. If redaction is enabled, the parameter content will be replaced by parameter name during logging. Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled."]
    pub redact: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "required")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the parameter is required. Optional parameters will not trigger prompts; however, they are filled if the user specifies them. Required parameters must be filled before form filling concludes."]
    pub required: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for how the filling of a parameter should be handled."]
pub struct GoogleCloudDialogflowCxV3FormParameterFillBehavior {
    #[serde(rename = "initialPromptFulfillment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The fulfillment to provide the initial prompt that the agent can present to the user in order to fill the parameter."]
    pub initial_prompt_fulfillment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3Fulfillment>>,
    #[serde(rename = "repromptEventHandlers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The handlers for parameter-level events, used to provide reprompt for the parameter or transition to a different page/flow. The supported events are: * `sys.no-match-`, where N can be from 1 to 6 * `sys.no-match-default` * `sys.no-input-`, where N can be from 1 to 6 * `sys.no-input-default` * `sys.invalid-parameter` `initial_prompt_fulfillment` provides the first prompt for the parameter. If the user's response does not fill the parameter, a no-match/no-input event will be triggered, and the fulfillment associated with the `sys.no-match-1`/`sys.no-input-1` handler (if defined) will be called to provide a prompt. The `sys.no-match-2`/`sys.no-input-2` handler (if defined) will respond to the next no-match/no-input event, and so on. A `sys.no-match-default` or `sys.no-input-default` handler will be used to handle all following no-match/no-input events after all numbered no-match/no-input handlers for the parameter are consumed. A `sys.invalid-parameter` handler can be defined to handle the case where the parameter values have been `invalidated` by webhook. For example, if the user's response fill the parameter, however the parameter was invalidated by webhook, the fulfillment associated with the `sys.invalid-parameter` handler (if defined) will be called to provide a prompt. If the event handler for the corresponding event can't be found on the parameter, `initial_prompt_fulfillment` will be re-prompted."]
    pub reprompt_event_handlers: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3EventHandler>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A fulfillment can do one or more of the following actions at the same time: * Generate rich message responses. * Set parameter values. * Call the webhook. Fulfillments can be called at various stages in the Page or Form lifecycle. For example, when a DetectIntentRequest drives a session to enter a new page, the page's entry fulfillment can add a static response to the QueryResult in the returning DetectIntentResponse, call the webhook (for example, to load user data from a database), or both."]
pub struct GoogleCloudDialogflowCxV3Fulfillment {
    #[serde(rename = "conditionalCases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Conditional cases for this fulfillment."]
    pub conditional_cases: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3FulfillmentConditionalCases>>,
    >,
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of rich message responses to present to the user."]
    pub messages: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessage>>,
    >,
    #[serde(rename = "setParameterActions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set parameter values before executing the webhook."]
    pub set_parameter_actions: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3FulfillmentSetParameterAction>>,
    >,
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tag used by the webhook to identify which fulfillment is being called. This field is required if `webhook` is specified."]
    pub tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "webhook")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The webhook to call. Format: `projects//locations//agents//webhooks/`."]
    pub webhook: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored."]
pub struct GoogleCloudDialogflowCxV3FulfillmentConditionalCases {
    #[serde(rename = "cases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of cascading if-else conditions."]
    pub cases: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCase>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Each case has a Boolean condition. When it is evaluated to be True, the corresponding messages will be selected and evaluated recursively."]
pub struct GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCase {
    #[serde(rename = "caseContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of case content."]
    pub case_content: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCaseCaseContent>,
        >,
    >,
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The condition to activate and select this case. Empty means the condition is always true. The condition is evaluated against form parameters or session parameters. See the [conditions reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition)."]
    pub condition: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The list of messages or conditional cases to activate for this case."]
pub struct GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCaseCaseContent {
    #[serde(rename = "additionalCases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional cases to be evaluated."]
    pub additional_cases: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3FulfillmentConditionalCases>,
    >,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Returned message."]
    pub message: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessage>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Setting a parameter value."]
pub struct GoogleCloudDialogflowCxV3FulfillmentSetParameterAction {
    #[serde(rename = "parameter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display name of the parameter."]
    pub parameter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The new value of the parameter. A null value clears the parameter."]
    pub value: ::std::option::Option<::serde_json::Value>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata in google::longrunning::Operation for Knowledge operations."]
pub struct GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadata {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Output only. The current state of this operation."]
    pub state:
        ::std::option::Option<GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadataStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Output only. The current state of this operation."]
pub enum GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadataStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "State unspecified."]
    StateUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "The operation has been created."]
    Pending,
    #[serde(rename = "RUNNING")]
    #[doc = "The operation is currently running."]
    Running,
    #[serde(rename = "DONE")]
    #[doc = "The operation is done, either cancelled or completed."]
    Done,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for ImportDocuments operation."]
pub struct GoogleCloudDialogflowCxV3ImportDocumentsOperationMetadata {
    #[serde(rename = "genericMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generic information of the operation."]
    pub generic_metadata: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadata>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for Documents.ImportDocuments."]
pub struct GoogleCloudDialogflowCxV3ImportDocumentsResponse {
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Includes details about skipped documents or any other warnings."]
    pub warnings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata returned for the TestCases.ImportTestCases long running operation."]
pub struct GoogleCloudDialogflowCxV3ImportTestCasesMetadata {
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Errors for failed test cases."]
    pub errors: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3TestCaseError>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for TestCases.ImportTestCases."]
pub struct GoogleCloudDialogflowCxV3ImportTestCasesResponse {
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifiers of the new test cases. Format: `projects//locations//agents//testCases/`."]
    pub names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Instructs the speech recognizer on how to process the audio content."]
pub struct GoogleCloudDialogflowCxV3InputAudioConfig {
    #[serde(rename = "audioEncoding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Audio encoding of the audio content to process."]
    pub audio_encoding:
        ::std::option::Option<GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum>,
    #[serde(rename = "enableWordInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If `true`, Dialogflow returns SpeechWordInfo in StreamingRecognitionResult with information about the recognized speech words, e.g. start and end time offsets. If false or unspecified, Speech doesn't return any word-level information."]
    pub enable_word_info: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Which Speech model to select for the given request. Select the model best suited to your domain to get best results. If a model is not explicitly specified, then we auto-select a model based on the parameters in the InputAudioConfig. If enhanced speech model is enabled for the agent and an enhanced version of the specified model for the language does not exist, then the speech is recognized using the standard version of the specified model. Refer to [Cloud Speech API documentation](https://cloud.google.com/speech-to-text/docs/basics#select-model) for more details."]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(rename = "modelVariant")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Which variant of the Speech model to use."]
    pub model_variant:
        ::std::option::Option<GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum>,
    #[serde(rename = "phraseHints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A list of strings containing words and phrases that the speech recognizer should recognize with higher likelihood. See [the Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/basics#phrase-hints) for more details."]
    pub phrase_hints: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "sampleRateHertz")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sample rate (in Hertz) of the audio content sent in the query. Refer to [Cloud Speech API documentation](https://cloud.google.com/speech-to-text/docs/basics) for more details."]
    pub sample_rate_hertz: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "singleUtterance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If `false` (default), recognition does not cease until the client closes the stream. If `true`, the recognizer will detect a single spoken utterance in input audio. Recognition ceases when it detects the audio's voice has stopped or paused. In this case, once a detected intent is received, the client should close the stream and start a new request with a new stream as needed. Note: This setting is relevant only for streaming methods."]
    pub single_utterance: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Audio encoding of the audio content to process."]
pub enum GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum {
    #[serde(rename = "AUDIO_ENCODING_UNSPECIFIED")]
    #[doc = "Not specified."]
    AudioEncodingUnspecified,
    #[serde(rename = "AUDIO_ENCODING_LINEAR_16")]
    #[doc = "Uncompressed 16-bit signed little-endian samples (Linear PCM)."]
    AudioEncodingLinear16,
    #[serde(rename = "AUDIO_ENCODING_FLAC")]
    #[doc = "[`FLAC`](https://xiph.org/flac/documentation.html) (Free Lossless Audio Codec) is the recommended encoding because it is lossless (therefore recognition is not compromised) and requires only about half the bandwidth of `LINEAR16`. `FLAC` stream encoding supports 16-bit and 24-bit samples, however, not all fields in `STREAMINFO` are supported."]
    AudioEncodingFlac,
    #[serde(rename = "AUDIO_ENCODING_MULAW")]
    #[doc = "8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law."]
    AudioEncodingMulaw,
    #[serde(rename = "AUDIO_ENCODING_AMR")]
    #[doc = "Adaptive Multi-Rate Narrowband codec. `sample_rate_hertz` must be 8000."]
    AudioEncodingAmr,
    #[serde(rename = "AUDIO_ENCODING_AMR_WB")]
    #[doc = "Adaptive Multi-Rate Wideband codec. `sample_rate_hertz` must be 16000."]
    AudioEncodingAmrWb,
    #[serde(rename = "AUDIO_ENCODING_OGG_OPUS")]
    #[doc = "Opus encoded audio frames in Ogg container ([OggOpus](https://wiki.xiph.org/OggOpus)). `sample_rate_hertz` must be 16000."]
    AudioEncodingOggOpus,
    #[serde(rename = "AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE")]
    #[doc = "Although the use of lossy encodings is not recommended, if a very low bitrate encoding is required, `OGG_OPUS` is highly preferred over Speex encoding. The [Speex](https://speex.org/) encoding supported by Dialogflow API has a header byte in each block, as in MIME type `audio/x-speex-with-header-byte`. It is a variant of the RTP Speex encoding defined in [RFC 5574](https://tools.ietf.org/html/rfc5574). The stream is a sequence of blocks, one block per RTP packet. Each block starts with a byte containing the length of the block, in bytes, followed by one or more frames of Speex data, padded to an integral number of bytes (octets) as specified in RFC 5574. In other words, each RTP header is replaced with a single byte containing the block length. Only Speex wideband is supported. `sample_rate_hertz` must be 16000."]
    AudioEncodingSpeexWithHeaderByte,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Which variant of the Speech model to use."]
pub enum GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum {
    #[serde(rename = "SPEECH_MODEL_VARIANT_UNSPECIFIED")]
    #[doc = "No model variant specified. In this case Dialogflow defaults to USE_BEST_AVAILABLE."]
    SpeechModelVariantUnspecified,
    #[serde(rename = "USE_BEST_AVAILABLE")]
    #[doc = "Use the best available variant of the Speech model that the caller is eligible for. Please see the [Dialogflow docs](https://cloud.google.com/dialogflow/docs/data-logging) for how to make your project eligible for enhanced models."]
    UseBestAvailable,
    #[serde(rename = "USE_STANDARD")]
    #[doc = "Use standard model variant even if an enhanced model is available. See the [Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models) for details about enhanced models."]
    UseStandard,
    #[serde(rename = "USE_ENHANCED")]
    #[doc = "Use an enhanced model variant: * If an enhanced variant does not exist for the given model and request language, Dialogflow falls back to the standard variant. The [Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models) describes which models have enhanced variants. * If the API caller isn't eligible for enhanced models, Dialogflow returns an error. Please see the [Dialogflow docs](https://cloud.google.com/dialogflow/docs/data-logging) for how to make your project eligible."]
    UseEnhanced,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An intent represents a user's intent to interact with a conversational agent. You can provide information for the Dialogflow API to use to match user input to an intent by adding training phrases (i.e., examples of user input) to your intent."]
pub struct GoogleCloudDialogflowCxV3Intent {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Human readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The human-readable name of the intent, unique within the agent."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isFallback")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation. Adding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event."]
    pub is_fallback: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes. Prefix \"sys.\" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys.head * sys.contextual The above labels do not require value. \"sys.head\" means the intent is a head intent. \"sys.contextual\" means the intent is a contextual intent."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the intent. Required for the Intents.UpdateIntent method. Intents.CreateIntent populates the name automatically. Format: `projects//locations//agents//intents/`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of parameters associated with the intent."]
    pub parameters: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3IntentParameter>>,
    >,
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests."]
    pub priority: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "trainingPhrases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of training phrases the agent is trained on to identify the intent."]
    pub training_phrases: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3IntentTrainingPhrase>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the intent to trigger programmatically rather than as a result of natural language processing."]
pub struct GoogleCloudDialogflowCxV3IntentInput {
    #[serde(rename = "intent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The unique identifier of the intent. Format: `projects//locations//agents//intents/`."]
    pub intent: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an intent parameter."]
pub struct GoogleCloudDialogflowCxV3IntentParameter {
    #[serde(rename = "entityType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The entity type of the parameter. Format: `projects/-/locations/-/agents/-/entityTypes/` for system entity types (for example, `projects/-/locations/-/agents/-/entityTypes/sys.date`), or `projects//locations//agents//entityTypes/` for developer entity types."]
    pub entity_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The unique identifier of the parameter. This field is used by training phrases to annotate their parts."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the parameter represents a list of values."]
    pub is_list: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "redact")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the parameter content should be redacted in log. If redaction is enabled, the parameter content will be replaced by parameter name during logging. Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled."]
    pub redact: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an example that the agent is trained on to identify the intent."]
pub struct GoogleCloudDialogflowCxV3IntentTrainingPhrase {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique identifier of the training phrase."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ordered list of training phrase parts. The parts are concatenated in order to form the training phrase. Note: The API does not automatically annotate training phrases like the Dialogflow Console does. Note: Do not forget to include whitespace at part boundaries, so the training phrase is well formatted when the parts are concatenated. If the training phrase does not need to be annotated with parameters, you just need a single part with only the Part.text field set. If you want to annotate the training phrase, you must create multiple parts, where the fields of each part are populated in one of two ways: - `Part.text` is set to a part of the phrase that has no parameters. - `Part.text` is set to a part of the phrase that you want to annotate, and the `parameter_id` field is set."]
    pub parts: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3IntentTrainingPhrasePart>>,
    >,
    #[serde(rename = "repeatCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates how many times this example was added to the intent."]
    pub repeat_count: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a part of a training phrase."]
pub struct GoogleCloudDialogflowCxV3IntentTrainingPhrasePart {
    #[serde(rename = "parameterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The parameter used to annotate this part of the training phrase. This field is required for annotated parts of the training phrase."]
    pub parameter_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The text for this part."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Dialogflow CX conversation (session) can be described and visualized as a state machine. The states of a CX session are represented by pages. For each flow, you define many pages, where your combined pages can handle a complete conversation on the topics the flow is designed for. At any given moment, exactly one page is the current page, the current page is considered active, and the flow associated with that page is considered active. Every flow has a special start page. When a flow initially becomes active, the start page page becomes the current page. For each conversational turn, the current page will either stay the same or transition to another page. You configure each page to collect information from the end-user that is relevant for the conversational state represented by the page. For more information, see the [Page guide](https://cloud.google.com/dialogflow/cx/docs/concept/page)."]
pub struct GoogleCloudDialogflowCxV3Page {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The human-readable name of the page, unique within the agent."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entryFulfillment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fulfillment to call when the session is entering the page."]
    pub entry_fulfillment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3Fulfillment>>,
    #[serde(rename = "eventHandlers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Handlers associated with the page to handle events such as webhook errors, no match or no input."]
    pub event_handlers: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3EventHandler>>,
    >,
    #[serde(rename = "form")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The form associated with the page, used for collecting parameters relevant to the page."]
    pub form: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3Form>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the page. Required for the Pages.UpdatePage method. Pages.CreatePage populates the name automatically. Format: `projects//locations//agents//flows//pages/`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transitionRouteGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ordered list of `TransitionRouteGroups` associated with the page. Transition route groups must be unique within a page. * If multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes. * If multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence. Format:`projects//locations//agents//flows//transitionRouteGroups/`."]
    pub transition_route_groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "transitionRoutes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of transitions for the transition rules of this page. They route the conversation to another page in the same flow, or another flow. When we are in a certain page, the TransitionRoutes are evalauted in the following order: * TransitionRoutes defined in the page with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in flow with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in the page with only condition specified. * TransitionRoutes defined in the transition route groups with only condition specified."]
    pub transition_routes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3TransitionRoute>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents page information communicated to and from the webhook."]
pub struct GoogleCloudDialogflowCxV3PageInfo {
    #[serde(rename = "currentPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present for WebhookRequest. Ignored for WebhookResponse. The unique identifier of the current page. Format: `projects//locations//agents//flows//pages/`."]
    pub current_page: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional for both WebhookRequest and WebhookResponse. Information about the form."]
    pub form_info:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3PageInfoFormInfo>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents form information."]
pub struct GoogleCloudDialogflowCxV3PageInfoFormInfo {
    #[serde(rename = "parameterInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional for both WebhookRequest and WebhookResponse. The parameters contained in the form. Note that the webhook cannot add or remove any form parameter."]
    pub parameter_info: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3PageInfoFormInfoParameterInfo>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents parameter information."]
pub struct GoogleCloudDialogflowCxV3PageInfoFormInfoParameterInfo {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present for WebhookRequest. Required for WebhookResponse. The human-readable name of the parameter, unique within the form. This field cannot be modified by the webhook."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "justCollected")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional for WebhookRequest. Ignored for WebhookResponse. Indicates if the parameter value was just collected on the last conversation turn."]
    pub just_collected: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "required")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional for both WebhookRequest and WebhookResponse. Indicates whether the parameter is required. Optional parameters will not trigger prompts; however, they are filled if the user specifies them. Required parameters must be filled before form filling concludes."]
    pub required: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present for WebhookRequest. Required for WebhookResponse. The state of the parameter. This field can be set to INVALID by the webhook to invalidate the parameter; other values set by the webhook will be ignored."]
    pub state:
        ::std::option::Option<GoogleCloudDialogflowCxV3PageInfoFormInfoParameterInfoStateEnum>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional for both WebhookRequest and WebhookResponse. The value of the parameter. This field can be set by the webhook to change the parameter value."]
    pub value: ::std::option::Option<::serde_json::Value>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Always present for WebhookRequest. Required for WebhookResponse. The state of the parameter. This field can be set to INVALID by the webhook to invalidate the parameter; other values set by the webhook will be ignored."]
pub enum GoogleCloudDialogflowCxV3PageInfoFormInfoParameterInfoStateEnum {
    #[serde(rename = "PARAMETER_STATE_UNSPECIFIED")]
    #[doc = "Not specified. This value should be never used."]
    ParameterStateUnspecified,
    #[serde(rename = "EMPTY")]
    #[doc = "Indicates that the parameter does not have a value."]
    Empty,
    #[serde(rename = "INVALID")]
    #[doc = "Indicates that the parameter value is invalid. This field can be used by the webhook to invalidate the parameter and ask the server to collect it from the user again."]
    Invalid,
    #[serde(rename = "FILLED")]
    #[doc = "Indicates that the parameter has a value."]
    Filled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the query input. It can contain one of: 1. A conversational query in the form of text. 2. An intent query that specifies which intent to trigger. 3. Natural language speech audio to be processed. 4. An event to be triggered. "]
pub struct GoogleCloudDialogflowCxV3QueryInput {
    #[serde(rename = "audio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The natural language speech audio to be processed."]
    pub audio: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3AudioInput>>,
    #[serde(rename = "dtmf")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The DTMF event to be handled."]
    pub dtmf: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3DtmfInput>>,
    #[serde(rename = "event")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The event to be triggered."]
    pub event: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3EventInput>>,
    #[serde(rename = "intent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The intent to be triggered."]
    pub intent: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3IntentInput>>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The language of the input. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. Note that queries in the same session do not necessarily need to specify the same language."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The natural language text to be processed."]
    pub text: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3TextInput>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for ReloadDocument operation."]
pub struct GoogleCloudDialogflowCxV3ReloadDocumentOperationMetadata {
    #[serde(rename = "genericMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generic information of the operation."]
    pub generic_metadata: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadata>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a response message that can be returned by a conversational agent. Response messages are also used for output audio synthesis. The approach is as follows: * If at least one OutputAudioText response is present, then all OutputAudioText responses are linearly concatenated, and the result is used for output audio synthesis. * If the OutputAudioText responses are a mixture of text and SSML, then the concatenated result is treated as SSML; otherwise, the result is treated as either text or SSML as appropriate. The agent designer should ideally use either text or SSML consistently throughout the bot design. * Otherwise, all Text responses are linearly concatenated, and the result is used for output audio synthesis. This approach allows for more sophisticated user experience scenarios, where the text displayed to the user may differ from what is heard."]
pub struct GoogleCloudDialogflowCxV3ResponseMessage {
    #[serde(rename = "conversationSuccess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that the conversation succeeded."]
    pub conversation_success: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessageConversationSuccess>,
    >,
    #[serde(rename = "endInteraction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A signal that indicates the interaction with the Dialogflow agent has ended. This message is generated by Dialogflow only when the conversation reaches `END_SESSION` or `END_PAGE` page. It is not supposed to be defined by the user. It's guaranteed that there is at most one such message in each response."]
    pub end_interaction: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessageEndInteraction>,
    >,
    #[serde(rename = "liveAgentHandoff")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hands off conversation to a human agent."]
    pub live_agent_handoff: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessageLiveAgentHandoff>,
    >,
    #[serde(rename = "mixedAudio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. An audio response message composed of both the synthesized Dialogflow agent responses and responses defined via play_audio. This message is generated by Dialogflow only and not supposed to be defined by the user."]
    pub mixed_audio: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessageMixedAudio>,
    >,
    #[serde(rename = "outputAudioText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A text or ssml response that is preferentially used for TTS output audio synthesis, as described in the comment on the ResponseMessage message."]
    pub output_audio_text: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessageOutputAudioText>,
    >,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Returns a response containing a custom, platform-specific payload."]
    pub payload: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "playAudio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Signal that the client should play an audio clip hosted at a client-specific URI. Dialogflow uses this to construct mixed_audio. However, Dialogflow itself does not try to read or process the URI in any way."]
    pub play_audio:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessagePlayAudio>>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Returns a text response."]
    pub text:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessageText>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Indicates that the conversation succeeded, i.e., the bot handled the issue that the customer talked to it about. Dialogflow only uses this to determine which conversations should be counted as successful and doesn't process the metadata in this message in any way. Note that Dialogflow also considers conversations that get to the conversation end page as successful even if they don't return ConversationSuccess. You may set this, for example: * In the entry_fulfillment of a Page if entering the page indicates that the conversation succeeded. * In a webhook response when you determine that you handled the customer issue."]
pub struct GoogleCloudDialogflowCxV3ResponseMessageConversationSuccess {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom metadata. Dialogflow doesn't impose any structure on this."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Indicates that interaction with the Dialogflow agent has ended. This message is generated by Dialogflow only and not supposed to be defined by the user."]
pub struct GoogleCloudDialogflowCxV3ResponseMessageEndInteraction {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Indicates that the conversation should be handed off to a live agent. Dialogflow only uses this to determine which conversations were handed off to a human agent for measurement purposes. What else to do with this signal is up to you and your handoff procedures. You may set this, for example: * In the entry_fulfillment of a Page if entering the page indicates something went extremely wrong in the conversation. * In a webhook response when you determine that the customer issue can only be handled by a human."]
pub struct GoogleCloudDialogflowCxV3ResponseMessageLiveAgentHandoff {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom metadata for your handoff procedure. Dialogflow doesn't impose any structure on this."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an audio message that is composed of both segments synthesized from the Dialogflow agent prompts and ones hosted externally at the specified URIs. The external URIs are specified via play_audio. This message is generated by Dialogflow only and not supposed to be defined by the user."]
pub struct GoogleCloudDialogflowCxV3ResponseMessageMixedAudio {
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Segments this audio response is composed of."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessageMixedAudioSegment>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents one segment of audio."]
pub struct GoogleCloudDialogflowCxV3ResponseMessageMixedAudioSegment {
    #[serde(rename = "allowPlaybackInterruption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether the playback of this segment can be interrupted by the end user's speech and the client should then start the next Dialogflow request."]
    pub allow_playback_interruption: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "audio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Raw audio synthesized from the Dialogflow agent's response using the output config specified in the request."]
    pub audio: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Client-specific URI that points to an audio clip accessible to the client. Dialogflow does not impose any validation on it."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A text or ssml response that is preferentially used for TTS output audio synthesis, as described in the comment on the ResponseMessage message."]
pub struct GoogleCloudDialogflowCxV3ResponseMessageOutputAudioText {
    #[serde(rename = "allowPlaybackInterruption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub allow_playback_interruption: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "ssml")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The SSML text to be synthesized. For more information, see [SSML](/speech/text-to-speech/docs/ssml)."]
    pub ssml: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The raw text to be synthesized."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies an audio clip to be played by the client as part of the response."]
pub struct GoogleCloudDialogflowCxV3ResponseMessagePlayAudio {
    #[serde(rename = "allowPlaybackInterruption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub allow_playback_interruption: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "audioUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The text response message."]
pub struct GoogleCloudDialogflowCxV3ResponseMessageText {
    #[serde(rename = "allowPlaybackInterruption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub allow_playback_interruption: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A collection of text responses."]
    pub text: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata returned for the TestCases.RunTestCase long running operation."]
pub struct GoogleCloudDialogflowCxV3RunTestCaseMetadata {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for TestCases.RunTestCase."]
pub struct GoogleCloudDialogflowCxV3RunTestCaseResponse {
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result."]
    pub result: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3TestCaseResult>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents session information communicated to and from the webhook."]
pub struct GoogleCloudDialogflowCxV3SessionInfo {
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional for WebhookRequest. Optional for WebhookResponse. All parameters collected from forms and intents during the session. Parameters can be created, updated, or removed by the webhook. To remove a parameter from the session, the webhook should explicitly set the parameter value to null in WebhookResponse. The map is keyed by parameters' display names."]
    pub parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "session")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present for WebhookRequest. Ignored for WebhookResponse. The unique identifier of the session. This field can be used by the webhook to identify a session. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/` if environment is specified."]
    pub session: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a test case."]
pub struct GoogleCloudDialogflowCxV3TestCase {
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. When the test was created."]
    pub creation_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The human-readable name of the test case, unique within the agent. Limit of 200 characters."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastTestResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The latest test result."]
    pub last_test_result:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3TestCaseResult>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the test case. TestCases.CreateTestCase will populate the name automatically. Otherwise use format: `projects//locations//agents/ /testCases/`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional freeform notes about the test case. Limit of 400 characters."]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tags are short descriptions that users may apply to test cases for organizational and filtering purposes. Each tag should start with \"#\" and has a limit of 30 characters."]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "testCaseConversationTurns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The conversation turns uttered when the test case was created, in chronological order. These include the canonical set of agent utterances that should occur when the agent is working properly."]
    pub test_case_conversation_turns: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3ConversationTurn>>,
    >,
    #[serde(rename = "testConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Config for the test case."]
    pub test_config: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3TestConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Error info for importing a test."]
pub struct GoogleCloudDialogflowCxV3TestCaseError {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status associated with the test case."]
    pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    #[serde(rename = "testCase")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The test case."]
    pub test_case: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3TestCase>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a result from running a test case in an agent environment."]
pub struct GoogleCloudDialogflowCxV3TestCaseResult {
    #[serde(rename = "conversationTurns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The conversation turns uttered during the test case replay in chronological order."]
    pub conversation_turns: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3ConversationTurn>>,
    >,
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Environment where the test was run. If not set, it indicates the draft environment."]
    pub environment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name for the test case result. Format: `projects//locations//agents//testCases/ /results/`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "testResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the test case passed in the agent environment."]
    pub test_result: ::std::option::Option<GoogleCloudDialogflowCxV3TestCaseResultTestResultEnum>,
    #[serde(rename = "testTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time that the test was run."]
    pub test_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Whether the test case passed in the agent environment."]
pub enum GoogleCloudDialogflowCxV3TestCaseResultTestResultEnum {
    #[serde(rename = "TEST_RESULT_UNSPECIFIED")]
    #[doc = "Not specified. Should never be used."]
    TestResultUnspecified,
    #[serde(rename = "PASSED")]
    #[doc = "The test passed."]
    Passed,
    #[serde(rename = "FAILED")]
    #[doc = "The test did not pass."]
    Failed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents configurations for a test case."]
pub struct GoogleCloudDialogflowCxV3TestConfig {
    #[serde(rename = "flow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flow name. If not set, default start flow is assumed. Format: `projects//locations//agents//flows/`."]
    pub flow: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trackingParameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Session parameters to be compared when calculating differences."]
    pub tracking_parameters: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Error info for running a test."]
pub struct GoogleCloudDialogflowCxV3TestError {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status associated with the test."]
    pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    #[serde(rename = "testCase")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The test case resource name."]
    pub test_case: ::std::option::Option<::std::string::String>,
    #[serde(rename = "testTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp when the test was completed."]
    pub test_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The description of differences between original and replayed agent output."]
pub struct GoogleCloudDialogflowCxV3TestRunDifference {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of the diff, showing the actual output vs expected output."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of diff."]
    pub _type: ::std::option::Option<GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of diff."]
pub enum GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum {
    #[serde(rename = "DIFF_TYPE_UNSPECIFIED")]
    #[doc = "Should never be used."]
    DiffTypeUnspecified,
    #[serde(rename = "INTENT")]
    #[doc = "The intent."]
    Intent,
    #[serde(rename = "PAGE")]
    #[doc = "The page."]
    Page,
    #[serde(rename = "PARAMETERS")]
    #[doc = "The parameters."]
    Parameters,
    #[serde(rename = "UTTERANCE")]
    #[doc = "The message utterance."]
    Utterance,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the natural language text to be processed."]
pub struct GoogleCloudDialogflowCxV3TextInput {
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The UTF-8 encoded natural language text to be processed. Text length must not exceed 256 characters."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A transition route specifies a intent that can be matched and/or a data condition that can be evaluated during a session. When a specified transition is matched, the following actions are taken in order: * If there is a `trigger_fulfillment` associated with the transition, it will be called. * If there is a `target_page` associated with the transition, the session will transition into the specified page. * If there is a `target_flow` associated with the transition, the session will transition into the specified flow."]
pub struct GoogleCloudDialogflowCxV3TransitionRoute {
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The condition to evaluate against form parameters or session parameters. See the [conditions reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition). At least one of `intent` or `condition` must be specified. When both `intent` and `condition` are specified, the transition can only happen when both are fulfilled."]
    pub condition: ::std::option::Option<::std::string::String>,
    #[serde(rename = "intent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of an Intent. Format: `projects//locations//agents//intents/`. Indicates that the transition can only happen when the given intent is matched. At least one of `intent` or `condition` must be specified. When both `intent` and `condition` are specified, the transition can only happen when both are fulfilled."]
    pub intent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique identifier of this transition route."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetFlow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target flow to transition to. Format: `projects//locations//agents//flows/`."]
    pub target_flow: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target page to transition to. Format: `projects//locations//agents//flows//pages/`."]
    pub target_page: ::std::option::Option<::std::string::String>,
    #[serde(rename = "triggerFulfillment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fulfillment to call when the condition is satisfied. At least one of `trigger_fulfillment` and `target` must be specified. When both are defined, `trigger_fulfillment` is executed first."]
    pub trigger_fulfillment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3Fulfillment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for UpdateDocument operation."]
pub struct GoogleCloudDialogflowCxV3UpdateDocumentOperationMetadata {
    #[serde(rename = "genericMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generic information of the operation."]
    pub generic_metadata: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadata>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for a webhook call."]
pub struct GoogleCloudDialogflowCxV3WebhookRequest {
    #[serde(rename = "detectIntentResponseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present. The unique identifier of the DetectIntentResponse that will be returned to the API caller."]
    pub detect_intent_response_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fulfillmentInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present. Information about the fulfillment that triggered this webhook call."]
    pub fulfillment_info: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3WebhookRequestFulfillmentInfo>,
    >,
    #[serde(rename = "intentInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the last matched intent."]
    pub intent_info:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3WebhookRequestIntentInfo>>,
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of rich message responses to present to the user. Webhook can choose to append or replace this list in WebhookResponse.fulfillment_response;"]
    pub messages: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessage>>,
    >,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about page status."]
    pub page_info: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3PageInfo>>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom data set in QueryParameters.payload."]
    pub payload: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "sentimentAnalysisResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sentiment analysis result of the current user request. The field is filled when sentiment analysis is configured to be enabled for the request."]
    pub sentiment_analysis_result: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3WebhookRequestSentimentAnalysisResult>,
    >,
    #[serde(rename = "sessionInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about session status."]
    pub session_info:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3SessionInfo>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents fulfillment information communicated to the webhook."]
pub struct GoogleCloudDialogflowCxV3WebhookRequestFulfillmentInfo {
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present. The tag used to identify which fulfillment is being called."]
    pub tag: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents intent information communicated to the webhook."]
pub struct GoogleCloudDialogflowCxV3WebhookRequestIntentInfo {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The confidence of the matched intent. Values range from 0.0 (completely uncertain) to 1.0 (completely certain)."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present. The display name of the last matched intent."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastMatchedIntent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present. The unique identifier of the last matched intent. Format: `projects//locations//agents//intents/`."]
    pub last_matched_intent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameters identified as a result of intent matching. This is a map of the name of the identified parameter to the value of the parameter identified from the user's utterance. All parameters defined in the matched intent that are identified will be surfaced here."]
    pub parameters: ::std::option::Option<
        ::std::collections::BTreeMap<
            String,
            ::std::boxed::Box<
                GoogleCloudDialogflowCxV3WebhookRequestIntentInfoIntentParameterValue,
            >,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a value for an intent parameter."]
pub struct GoogleCloudDialogflowCxV3WebhookRequestIntentInfoIntentParameterValue {
    #[serde(rename = "originalValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present. Original text value extracted from user utterance."]
    pub original_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resolvedValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present. Structured value for the parameter extracted from user utterance."]
    pub resolved_value: ::std::option::Option<::serde_json::Value>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the result of sentiment analysis."]
pub struct GoogleCloudDialogflowCxV3WebhookRequestSentimentAnalysisResult {
    #[serde(rename = "magnitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A non-negative number in the [0, +inf) range, which represents the absolute magnitude of sentiment, regardless of score (positive or negative)."]
    pub magnitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sentiment score between -1.0 (negative sentiment) and 1.0 (positive sentiment)."]
    pub score: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for a webhook call."]
pub struct GoogleCloudDialogflowCxV3WebhookResponse {
    #[serde(rename = "fulfillmentResponse")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fulfillment response to send to the user. This field can be omitted by the webhook if it does not intend to send any response to the user."]
    pub fulfillment_response: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3WebhookResponseFulfillmentResponse>,
    >,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about page status. This field can be omitted by the webhook if it does not intend to modify page status."]
    pub page_info: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3PageInfo>>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value to append directly to QueryResult.webhook_payloads."]
    pub payload: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "sessionInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about session status. This field can be omitted by the webhook if it does not intend to modify session status."]
    pub session_info:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3SessionInfo>>,
    #[serde(rename = "targetFlow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target flow to transition to. Format: `projects//locations//agents//flows/`."]
    pub target_flow: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target page to transition to. Format: `projects//locations//agents//flows//pages/`."]
    pub target_page: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a fulfillment response to the user."]
pub struct GoogleCloudDialogflowCxV3WebhookResponseFulfillmentResponse {
    #[serde(rename = "mergeBehavior")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Merge behavior for `messages`."]
    pub merge_behavior: ::std::option::Option<
        GoogleCloudDialogflowCxV3WebhookResponseFulfillmentResponseMergeBehaviorEnum,
    >,
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of rich message responses to present to the user."]
    pub messages: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessage>>,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Merge behavior for `messages`."]
pub enum GoogleCloudDialogflowCxV3WebhookResponseFulfillmentResponseMergeBehaviorEnum {
    #[serde(rename = "MERGE_BEHAVIOR_UNSPECIFIED")]
    #[doc = "Not specified. `APPEND` will be used."]
    MergeBehaviorUnspecified,
    #[serde(rename = "APPEND")]
    #[doc = "`messages` will be appended to the list of messages waiting to be sent to the user."]
    Append,
    #[serde(rename = "REPLACE")]
    #[doc = "`messages` will replace the list of messages waiting to be sent to the user."]
    Replace,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the natural speech audio to be processed."]
pub struct GoogleCloudDialogflowCxV3beta1AudioInput {
    #[serde(rename = "audio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The natural language speech audio to be processed. A single request can contain up to 1 minute of speech audio data. The transcribed text cannot contain more than 256 bytes. For non-streaming audio detect intent, both `config` and `audio` must be provided. For streaming audio detect intent, `config` must be provided in the first request and `audio` must be provided in all following requests."]
    pub audio: ::std::option::Option<::std::string::String>,
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Instructs the speech recognizer how to process the speech audio."]
    pub config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1InputAudioConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata returned for the TestCases.BatchRunTestCases long running operation."]
pub struct GoogleCloudDialogflowCxV3beta1BatchRunTestCasesMetadata {
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The test errors."]
    pub errors: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestError>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for TestCases.BatchRunTestCases."]
pub struct GoogleCloudDialogflowCxV3beta1BatchRunTestCasesResponse {
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The test case results. The detailed conversation turns are empty in this response."]
    pub results: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestCaseResult>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "One interaction between a human and virtual agent. The human provides some input and the virtual agent provides a response."]
pub struct GoogleCloudDialogflowCxV3beta1ConversationTurn {
    #[serde(rename = "userInput")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user input."]
    pub user_input: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ConversationTurnUserInput>,
    >,
    #[serde(rename = "virtualAgentOutput")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The virtual agent output."]
    pub virtual_agent_output: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ConversationTurnVirtualAgentOutput>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The input from the human user."]
pub struct GoogleCloudDialogflowCxV3beta1ConversationTurnUserInput {
    #[serde(rename = "injectedParameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameters that need to be injected into the conversation during intent detection."]
    pub injected_parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supports text input, event input, dtmf input in the test case."]
    pub input: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1QueryInput>>,
    #[serde(rename = "isWebhookEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If webhooks should be allowed to trigger in response to the user utterance. Often if parameters are injected, webhooks should not be enabled."]
    pub is_webhook_enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The output from the virtual agent."]
pub struct GoogleCloudDialogflowCxV3beta1ConversationTurnVirtualAgentOutput {
    #[serde(rename = "currentPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Page on which the utterance was spoken. Only name and displayName will be set."]
    pub current_page: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Page>>,
    #[serde(rename = "diagnosticInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Input only. The diagnostic info output for the turn."]
    pub diagnostic_info:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "differences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If this is part of a result conversation turn, the list of differences between the original run and the replay for this output, if any."]
    pub differences: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestRunDifference>>,
    >,
    #[serde(rename = "sessionParameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The session parameters available to the bot at this point."]
    pub session_parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Response error from the agent in the test result. If set, other output is empty."]
    pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    #[serde(rename = "textResponses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text responses from the agent for the turn."]
    pub text_responses: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessageText>>,
    >,
    #[serde(rename = "triggeredIntent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Intent that triggered the response. Only name and displayName will be set."]
    pub triggered_intent:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Intent>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for CreateDocument operation."]
pub struct GoogleCloudDialogflowCxV3beta1CreateDocumentOperationMetadata {
    #[serde(rename = "genericMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generic information of the operation."]
    pub generic_metadata: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadata>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata associated with the long running operation for Versions.CreateVersion."]
pub struct GoogleCloudDialogflowCxV3beta1CreateVersionOperationMetadata {
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the created version. Format: `projects//locations//agents//flows//versions/`."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for DeleteDocument operation."]
pub struct GoogleCloudDialogflowCxV3beta1DeleteDocumentOperationMetadata {
    #[serde(rename = "genericMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generic information of the operation."]
    pub generic_metadata: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadata>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the input for dtmf event."]
pub struct GoogleCloudDialogflowCxV3beta1DtmfInput {
    #[serde(rename = "digits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The dtmf digits."]
    pub digits: ::std::option::Option<::std::string::String>,
    #[serde(rename = "finishDigit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The finish digit (if any)."]
    pub finish_digit: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event handler specifies an event that can be handled during a session. When the specified event happens, the following actions are taken in order: * If there is a `trigger_fulfillment` associated with the event, it will be called. * If there is a `target_page` associated with the event, the session will transition into the specified page. * If there is a `target_flow` associated with the event, the session will transition into the specified flow."]
pub struct GoogleCloudDialogflowCxV3beta1EventHandler {
    #[serde(rename = "event")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the event to handle."]
    pub event: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique identifier of this event handler."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetFlow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target flow to transition to. Format: `projects//locations//agents//flows/`."]
    pub target_flow: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target page to transition to. Format: `projects//locations//agents//flows//pages/`."]
    pub target_page: ::std::option::Option<::std::string::String>,
    #[serde(rename = "triggerFulfillment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fulfillment to call when the event occurs. Handling webhook errors with a fulfillment enabled with webhook could cause infinite loop. It is invalid to specify such fulfillment for a handler handling webhooks."]
    pub trigger_fulfillment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Fulfillment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the event to trigger."]
pub struct GoogleCloudDialogflowCxV3beta1EventInput {
    #[serde(rename = "event")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the event."]
    pub event: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Agents.ExportAgent."]
pub struct GoogleCloudDialogflowCxV3beta1ExportAgentResponse {
    #[serde(rename = "agentContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uncompressed raw byte content for agent."]
    pub agent_content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "agentUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI to a file containing the exported agent. This field is populated only if `agent_uri` is specified in ExportAgentRequest."]
    pub agent_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata returned for the TestCases.ExportTestCases long running operation."]
pub struct GoogleCloudDialogflowCxV3beta1ExportTestCasesMetadata {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for TestCases.ExportTestCases."]
pub struct GoogleCloudDialogflowCxV3beta1ExportTestCasesResponse {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uncompressed raw byte content for test cases."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gcsUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI to a file containing the exported test cases. This field is populated only if `gcs_uri` is specified in ExportTestCasesRequest."]
    pub gcs_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A form is a data model that groups related parameters that can be collected from the user. The process in which the agent prompts the user and collects parameter values from the user is called form filling. A form can be added to a page. When form filling is done, the filled parameters will be written to the session."]
pub struct GoogleCloudDialogflowCxV3beta1Form {
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameters to collect from the user."]
    pub parameters: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1FormParameter>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a form parameter."]
pub struct GoogleCloudDialogflowCxV3beta1FormParameter {
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default value of an optional parameter. If the parameter is required, the default value will be ignored."]
    pub default_value: ::std::option::Option<::serde_json::Value>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The human-readable name of the parameter, unique within the form."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The entity type of the parameter. Format: `projects/-/locations/-/agents/-/entityTypes/` for system entity types (for example, `projects/-/locations/-/agents/-/entityTypes/sys.date`), or `projects//locations//agents//entityTypes/` for developer entity types."]
    pub entity_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fillBehavior")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Defines fill behavior for the parameter."]
    pub fill_behavior: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1FormParameterFillBehavior>,
    >,
    #[serde(rename = "isList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the parameter represents a list of values."]
    pub is_list: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "redact")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the parameter content should be redacted in log. If redaction is enabled, the parameter content will be replaced by parameter name during logging. Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled."]
    pub redact: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "required")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the parameter is required. Optional parameters will not trigger prompts; however, they are filled if the user specifies them. Required parameters must be filled before form filling concludes."]
    pub required: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for how the filling of a parameter should be handled."]
pub struct GoogleCloudDialogflowCxV3beta1FormParameterFillBehavior {
    #[serde(rename = "initialPromptFulfillment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The fulfillment to provide the initial prompt that the agent can present to the user in order to fill the parameter."]
    pub initial_prompt_fulfillment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Fulfillment>>,
    #[serde(rename = "repromptEventHandlers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The handlers for parameter-level events, used to provide reprompt for the parameter or transition to a different page/flow. The supported events are: * `sys.no-match-`, where N can be from 1 to 6 * `sys.no-match-default` * `sys.no-input-`, where N can be from 1 to 6 * `sys.no-input-default` * `sys.invalid-parameter` `initial_prompt_fulfillment` provides the first prompt for the parameter. If the user's response does not fill the parameter, a no-match/no-input event will be triggered, and the fulfillment associated with the `sys.no-match-1`/`sys.no-input-1` handler (if defined) will be called to provide a prompt. The `sys.no-match-2`/`sys.no-input-2` handler (if defined) will respond to the next no-match/no-input event, and so on. A `sys.no-match-default` or `sys.no-input-default` handler will be used to handle all following no-match/no-input events after all numbered no-match/no-input handlers for the parameter are consumed. A `sys.invalid-parameter` handler can be defined to handle the case where the parameter values have been `invalidated` by webhook. For example, if the user's response fill the parameter, however the parameter was invalidated by webhook, the fulfillment associated with the `sys.invalid-parameter` handler (if defined) will be called to provide a prompt. If the event handler for the corresponding event can't be found on the parameter, `initial_prompt_fulfillment` will be re-prompted."]
    pub reprompt_event_handlers: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1EventHandler>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A fulfillment can do one or more of the following actions at the same time: * Generate rich message responses. * Set parameter values. * Call the webhook. Fulfillments can be called at various stages in the Page or Form lifecycle. For example, when a DetectIntentRequest drives a session to enter a new page, the page's entry fulfillment can add a static response to the QueryResult in the returning DetectIntentResponse, call the webhook (for example, to load user data from a database), or both."]
pub struct GoogleCloudDialogflowCxV3beta1Fulfillment {
    #[serde(rename = "conditionalCases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Conditional cases for this fulfillment."]
    pub conditional_cases: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCases>,
        >,
    >,
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of rich message responses to present to the user."]
    pub messages: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessage>>,
    >,
    #[serde(rename = "setParameterActions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set parameter values before executing the webhook."]
    pub set_parameter_actions: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1FulfillmentSetParameterAction>,
        >,
    >,
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tag used by the webhook to identify which fulfillment is being called. This field is required if `webhook` is specified."]
    pub tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "webhook")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The webhook to call. Format: `projects//locations//agents//webhooks/`."]
    pub webhook: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored."]
pub struct GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCases {
    #[serde(rename = "cases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of cascading if-else conditions."]
    pub cases: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCasesCase>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Each case has a Boolean condition. When it is evaluated to be True, the corresponding messages will be selected and evaluated recursively."]
pub struct GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCasesCase {
    #[serde(rename = "caseContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of case content."]
    pub case_content: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<
                GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCasesCaseCaseContent,
            >,
        >,
    >,
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The condition to activate and select this case. Empty means the condition is always true. The condition is evaluated against form parameters or session parameters. See the [conditions reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition)."]
    pub condition: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The list of messages or conditional cases to activate for this case."]
pub struct GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCasesCaseCaseContent {
    #[serde(rename = "additionalCases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional cases to be evaluated."]
    pub additional_cases: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCases>,
    >,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Returned message."]
    pub message:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessage>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Setting a parameter value."]
pub struct GoogleCloudDialogflowCxV3beta1FulfillmentSetParameterAction {
    #[serde(rename = "parameter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display name of the parameter."]
    pub parameter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The new value of the parameter. A null value clears the parameter."]
    pub value: ::std::option::Option<::serde_json::Value>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata in google::longrunning::Operation for Knowledge operations."]
pub struct GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadata {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Output only. The current state of this operation."]
    pub state: ::std::option::Option<
        GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadataStateEnum,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Output only. The current state of this operation."]
pub enum GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadataStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "State unspecified."]
    StateUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "The operation has been created."]
    Pending,
    #[serde(rename = "RUNNING")]
    #[doc = "The operation is currently running."]
    Running,
    #[serde(rename = "DONE")]
    #[doc = "The operation is done, either cancelled or completed."]
    Done,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for ImportDocuments operation."]
pub struct GoogleCloudDialogflowCxV3beta1ImportDocumentsOperationMetadata {
    #[serde(rename = "genericMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generic information of the operation."]
    pub generic_metadata: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadata>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for Documents.ImportDocuments."]
pub struct GoogleCloudDialogflowCxV3beta1ImportDocumentsResponse {
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Includes details about skipped documents or any other warnings."]
    pub warnings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata returned for the TestCases.ImportTestCases long running operation."]
pub struct GoogleCloudDialogflowCxV3beta1ImportTestCasesMetadata {
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Errors for failed test cases."]
    pub errors: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestCaseError>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for TestCases.ImportTestCases."]
pub struct GoogleCloudDialogflowCxV3beta1ImportTestCasesResponse {
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifiers of the new test cases. Format: `projects//locations//agents//testCases/`."]
    pub names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Instructs the speech recognizer on how to process the audio content."]
pub struct GoogleCloudDialogflowCxV3beta1InputAudioConfig {
    #[serde(rename = "audioEncoding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Audio encoding of the audio content to process."]
    pub audio_encoding:
        ::std::option::Option<GoogleCloudDialogflowCxV3beta1InputAudioConfigAudioEncodingEnum>,
    #[serde(rename = "enableWordInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If `true`, Dialogflow returns SpeechWordInfo in StreamingRecognitionResult with information about the recognized speech words, e.g. start and end time offsets. If false or unspecified, Speech doesn't return any word-level information."]
    pub enable_word_info: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Which Speech model to select for the given request. Select the model best suited to your domain to get best results. If a model is not explicitly specified, then we auto-select a model based on the parameters in the InputAudioConfig. If enhanced speech model is enabled for the agent and an enhanced version of the specified model for the language does not exist, then the speech is recognized using the standard version of the specified model. Refer to [Cloud Speech API documentation](https://cloud.google.com/speech-to-text/docs/basics#select-model) for more details."]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(rename = "modelVariant")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Which variant of the Speech model to use."]
    pub model_variant:
        ::std::option::Option<GoogleCloudDialogflowCxV3beta1InputAudioConfigModelVariantEnum>,
    #[serde(rename = "phraseHints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A list of strings containing words and phrases that the speech recognizer should recognize with higher likelihood. See [the Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/basics#phrase-hints) for more details."]
    pub phrase_hints: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "sampleRateHertz")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sample rate (in Hertz) of the audio content sent in the query. Refer to [Cloud Speech API documentation](https://cloud.google.com/speech-to-text/docs/basics) for more details."]
    pub sample_rate_hertz: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "singleUtterance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If `false` (default), recognition does not cease until the client closes the stream. If `true`, the recognizer will detect a single spoken utterance in input audio. Recognition ceases when it detects the audio's voice has stopped or paused. In this case, once a detected intent is received, the client should close the stream and start a new request with a new stream as needed. Note: This setting is relevant only for streaming methods."]
    pub single_utterance: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Audio encoding of the audio content to process."]
pub enum GoogleCloudDialogflowCxV3beta1InputAudioConfigAudioEncodingEnum {
    #[serde(rename = "AUDIO_ENCODING_UNSPECIFIED")]
    #[doc = "Not specified."]
    AudioEncodingUnspecified,
    #[serde(rename = "AUDIO_ENCODING_LINEAR_16")]
    #[doc = "Uncompressed 16-bit signed little-endian samples (Linear PCM)."]
    AudioEncodingLinear16,
    #[serde(rename = "AUDIO_ENCODING_FLAC")]
    #[doc = "[`FLAC`](https://xiph.org/flac/documentation.html) (Free Lossless Audio Codec) is the recommended encoding because it is lossless (therefore recognition is not compromised) and requires only about half the bandwidth of `LINEAR16`. `FLAC` stream encoding supports 16-bit and 24-bit samples, however, not all fields in `STREAMINFO` are supported."]
    AudioEncodingFlac,
    #[serde(rename = "AUDIO_ENCODING_MULAW")]
    #[doc = "8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law."]
    AudioEncodingMulaw,
    #[serde(rename = "AUDIO_ENCODING_AMR")]
    #[doc = "Adaptive Multi-Rate Narrowband codec. `sample_rate_hertz` must be 8000."]
    AudioEncodingAmr,
    #[serde(rename = "AUDIO_ENCODING_AMR_WB")]
    #[doc = "Adaptive Multi-Rate Wideband codec. `sample_rate_hertz` must be 16000."]
    AudioEncodingAmrWb,
    #[serde(rename = "AUDIO_ENCODING_OGG_OPUS")]
    #[doc = "Opus encoded audio frames in Ogg container ([OggOpus](https://wiki.xiph.org/OggOpus)). `sample_rate_hertz` must be 16000."]
    AudioEncodingOggOpus,
    #[serde(rename = "AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE")]
    #[doc = "Although the use of lossy encodings is not recommended, if a very low bitrate encoding is required, `OGG_OPUS` is highly preferred over Speex encoding. The [Speex](https://speex.org/) encoding supported by Dialogflow API has a header byte in each block, as in MIME type `audio/x-speex-with-header-byte`. It is a variant of the RTP Speex encoding defined in [RFC 5574](https://tools.ietf.org/html/rfc5574). The stream is a sequence of blocks, one block per RTP packet. Each block starts with a byte containing the length of the block, in bytes, followed by one or more frames of Speex data, padded to an integral number of bytes (octets) as specified in RFC 5574. In other words, each RTP header is replaced with a single byte containing the block length. Only Speex wideband is supported. `sample_rate_hertz` must be 16000."]
    AudioEncodingSpeexWithHeaderByte,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Which variant of the Speech model to use."]
pub enum GoogleCloudDialogflowCxV3beta1InputAudioConfigModelVariantEnum {
    #[serde(rename = "SPEECH_MODEL_VARIANT_UNSPECIFIED")]
    #[doc = "No model variant specified. In this case Dialogflow defaults to USE_BEST_AVAILABLE."]
    SpeechModelVariantUnspecified,
    #[serde(rename = "USE_BEST_AVAILABLE")]
    #[doc = "Use the best available variant of the Speech model that the caller is eligible for. Please see the [Dialogflow docs](https://cloud.google.com/dialogflow/docs/data-logging) for how to make your project eligible for enhanced models."]
    UseBestAvailable,
    #[serde(rename = "USE_STANDARD")]
    #[doc = "Use standard model variant even if an enhanced model is available. See the [Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models) for details about enhanced models."]
    UseStandard,
    #[serde(rename = "USE_ENHANCED")]
    #[doc = "Use an enhanced model variant: * If an enhanced variant does not exist for the given model and request language, Dialogflow falls back to the standard variant. The [Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models) describes which models have enhanced variants. * If the API caller isn't eligible for enhanced models, Dialogflow returns an error. Please see the [Dialogflow docs](https://cloud.google.com/dialogflow/docs/data-logging) for how to make your project eligible."]
    UseEnhanced,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An intent represents a user's intent to interact with a conversational agent. You can provide information for the Dialogflow API to use to match user input to an intent by adding training phrases (i.e., examples of user input) to your intent."]
pub struct GoogleCloudDialogflowCxV3beta1Intent {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Human readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The human-readable name of the intent, unique within the agent."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isFallback")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation. Adding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event."]
    pub is_fallback: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes. Prefix \"sys-\" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys-head * sys-contextual The above labels do not require value. \"sys-head\" means the intent is a head intent. \"sys-contextual\" means the intent is a contextual intent."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the intent. Required for the Intents.UpdateIntent method. Intents.CreateIntent populates the name automatically. Format: `projects//locations//agents//intents/`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of parameters associated with the intent."]
    pub parameters: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1IntentParameter>>,
    >,
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests."]
    pub priority: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "trainingPhrases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of training phrases the agent is trained on to identify the intent."]
    pub training_phrases: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1IntentTrainingPhrase>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the intent to trigger programmatically rather than as a result of natural language processing."]
pub struct GoogleCloudDialogflowCxV3beta1IntentInput {
    #[serde(rename = "intent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The unique identifier of the intent. Format: `projects//locations//agents//intents/`."]
    pub intent: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an intent parameter."]
pub struct GoogleCloudDialogflowCxV3beta1IntentParameter {
    #[serde(rename = "entityType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The entity type of the parameter. Format: `projects/-/locations/-/agents/-/entityTypes/` for system entity types (for example, `projects/-/locations/-/agents/-/entityTypes/sys.date`), or `projects//locations//agents//entityTypes/` for developer entity types."]
    pub entity_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The unique identifier of the parameter. This field is used by training phrases to annotate their parts."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the parameter represents a list of values."]
    pub is_list: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "redact")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the parameter content should be redacted in log. If redaction is enabled, the parameter content will be replaced by parameter name during logging. Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled."]
    pub redact: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an example that the agent is trained on to identify the intent."]
pub struct GoogleCloudDialogflowCxV3beta1IntentTrainingPhrase {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique identifier of the training phrase."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ordered list of training phrase parts. The parts are concatenated in order to form the training phrase. Note: The API does not automatically annotate training phrases like the Dialogflow Console does. Note: Do not forget to include whitespace at part boundaries, so the training phrase is well formatted when the parts are concatenated. If the training phrase does not need to be annotated with parameters, you just need a single part with only the Part.text field set. If you want to annotate the training phrase, you must create multiple parts, where the fields of each part are populated in one of two ways: - `Part.text` is set to a part of the phrase that has no parameters. - `Part.text` is set to a part of the phrase that you want to annotate, and the `parameter_id` field is set."]
    pub parts: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1IntentTrainingPhrasePart>>,
    >,
    #[serde(rename = "repeatCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates how many times this example was added to the intent."]
    pub repeat_count: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a part of a training phrase."]
pub struct GoogleCloudDialogflowCxV3beta1IntentTrainingPhrasePart {
    #[serde(rename = "parameterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The parameter used to annotate this part of the training phrase. This field is required for annotated parts of the training phrase."]
    pub parameter_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The text for this part."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Dialogflow CX conversation (session) can be described and visualized as a state machine. The states of a CX session are represented by pages. For each flow, you define many pages, where your combined pages can handle a complete conversation on the topics the flow is designed for. At any given moment, exactly one page is the current page, the current page is considered active, and the flow associated with that page is considered active. Every flow has a special start page. When a flow initially becomes active, the start page page becomes the current page. For each conversational turn, the current page will either stay the same or transition to another page. You configure each page to collect information from the end-user that is relevant for the conversational state represented by the page. For more information, see the [Page guide](https://cloud.google.com/dialogflow/cx/docs/concept/page)."]
pub struct GoogleCloudDialogflowCxV3beta1Page {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The human-readable name of the page, unique within the agent."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entryFulfillment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fulfillment to call when the session is entering the page."]
    pub entry_fulfillment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Fulfillment>>,
    #[serde(rename = "eventHandlers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Handlers associated with the page to handle events such as webhook errors, no match or no input."]
    pub event_handlers: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1EventHandler>>,
    >,
    #[serde(rename = "form")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The form associated with the page, used for collecting parameters relevant to the page."]
    pub form: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Form>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the page. Required for the Pages.UpdatePage method. Pages.CreatePage populates the name automatically. Format: `projects//locations//agents//flows//pages/`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transitionRouteGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ordered list of `TransitionRouteGroups` associated with the page. Transition route groups must be unique within a page. * If multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes. * If multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence. Format:`projects//locations//agents//flows//transitionRouteGroups/`."]
    pub transition_route_groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "transitionRoutes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of transitions for the transition rules of this page. They route the conversation to another page in the same flow, or another flow. When we are in a certain page, the TransitionRoutes are evalauted in the following order: * TransitionRoutes defined in the page with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in flow with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in the page with only condition specified. * TransitionRoutes defined in the transition route groups with only condition specified."]
    pub transition_routes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TransitionRoute>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents page information communicated to and from the webhook."]
pub struct GoogleCloudDialogflowCxV3beta1PageInfo {
    #[serde(rename = "currentPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present for WebhookRequest. Ignored for WebhookResponse. The unique identifier of the current page. Format: `projects//locations//agents//flows//pages/`."]
    pub current_page: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional for both WebhookRequest and WebhookResponse. Information about the form."]
    pub form_info:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1PageInfoFormInfo>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents form information."]
pub struct GoogleCloudDialogflowCxV3beta1PageInfoFormInfo {
    #[serde(rename = "parameterInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional for both WebhookRequest and WebhookResponse. The parameters contained in the form. Note that the webhook cannot add or remove any form parameter."]
    pub parameter_info: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1PageInfoFormInfoParameterInfo>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents parameter information."]
pub struct GoogleCloudDialogflowCxV3beta1PageInfoFormInfoParameterInfo {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present for WebhookRequest. Required for WebhookResponse. The human-readable name of the parameter, unique within the form. This field cannot be modified by the webhook."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "justCollected")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional for WebhookRequest. Ignored for WebhookResponse. Indicates if the parameter value was just collected on the last conversation turn."]
    pub just_collected: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "required")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional for both WebhookRequest and WebhookResponse. Indicates whether the parameter is required. Optional parameters will not trigger prompts; however, they are filled if the user specifies them. Required parameters must be filled before form filling concludes."]
    pub required: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present for WebhookRequest. Required for WebhookResponse. The state of the parameter. This field can be set to INVALID by the webhook to invalidate the parameter; other values set by the webhook will be ignored."]
    pub state:
        ::std::option::Option<GoogleCloudDialogflowCxV3beta1PageInfoFormInfoParameterInfoStateEnum>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional for both WebhookRequest and WebhookResponse. The value of the parameter. This field can be set by the webhook to change the parameter value."]
    pub value: ::std::option::Option<::serde_json::Value>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Always present for WebhookRequest. Required for WebhookResponse. The state of the parameter. This field can be set to INVALID by the webhook to invalidate the parameter; other values set by the webhook will be ignored."]
pub enum GoogleCloudDialogflowCxV3beta1PageInfoFormInfoParameterInfoStateEnum {
    #[serde(rename = "PARAMETER_STATE_UNSPECIFIED")]
    #[doc = "Not specified. This value should be never used."]
    ParameterStateUnspecified,
    #[serde(rename = "EMPTY")]
    #[doc = "Indicates that the parameter does not have a value."]
    Empty,
    #[serde(rename = "INVALID")]
    #[doc = "Indicates that the parameter value is invalid. This field can be used by the webhook to invalidate the parameter and ask the server to collect it from the user again."]
    Invalid,
    #[serde(rename = "FILLED")]
    #[doc = "Indicates that the parameter has a value."]
    Filled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the query input. It can contain one of: 1. A conversational query in the form of text. 2. An intent query that specifies which intent to trigger. 3. Natural language speech audio to be processed. 4. An event to be triggered. "]
pub struct GoogleCloudDialogflowCxV3beta1QueryInput {
    #[serde(rename = "audio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The natural language speech audio to be processed."]
    pub audio: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1AudioInput>>,
    #[serde(rename = "dtmf")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The DTMF event to be handled."]
    pub dtmf: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1DtmfInput>>,
    #[serde(rename = "event")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The event to be triggered."]
    pub event: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1EventInput>>,
    #[serde(rename = "intent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The intent to be triggered."]
    pub intent: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1IntentInput>>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The language of the input. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. Note that queries in the same session do not necessarily need to specify the same language."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The natural language text to be processed."]
    pub text: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TextInput>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for ReloadDocument operation."]
pub struct GoogleCloudDialogflowCxV3beta1ReloadDocumentOperationMetadata {
    #[serde(rename = "genericMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generic information of the operation."]
    pub generic_metadata: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadata>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a response message that can be returned by a conversational agent. Response messages are also used for output audio synthesis. The approach is as follows: * If at least one OutputAudioText response is present, then all OutputAudioText responses are linearly concatenated, and the result is used for output audio synthesis. * If the OutputAudioText responses are a mixture of text and SSML, then the concatenated result is treated as SSML; otherwise, the result is treated as either text or SSML as appropriate. The agent designer should ideally use either text or SSML consistently throughout the bot design. * Otherwise, all Text responses are linearly concatenated, and the result is used for output audio synthesis. This approach allows for more sophisticated user experience scenarios, where the text displayed to the user may differ from what is heard."]
pub struct GoogleCloudDialogflowCxV3beta1ResponseMessage {
    #[serde(rename = "conversationSuccess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that the conversation succeeded."]
    pub conversation_success: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessageConversationSuccess>,
    >,
    #[serde(rename = "endInteraction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A signal that indicates the interaction with the Dialogflow agent has ended. This message is generated by Dialogflow only when the conversation reaches `END_SESSION` or `END_PAGE` page. It is not supposed to be defined by the user. It's guaranteed that there is at most one such message in each response."]
    pub end_interaction: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessageEndInteraction>,
    >,
    #[serde(rename = "liveAgentHandoff")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hands off conversation to a human agent."]
    pub live_agent_handoff: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessageLiveAgentHandoff>,
    >,
    #[serde(rename = "mixedAudio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. An audio response message composed of both the synthesized Dialogflow agent responses and responses defined via play_audio. This message is generated by Dialogflow only and not supposed to be defined by the user."]
    pub mixed_audio: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessageMixedAudio>,
    >,
    #[serde(rename = "outputAudioText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A text or ssml response that is preferentially used for TTS output audio synthesis, as described in the comment on the ResponseMessage message."]
    pub output_audio_text: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessageOutputAudioText>,
    >,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Returns a response containing a custom, platform-specific payload."]
    pub payload: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "playAudio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Signal that the client should play an audio clip hosted at a client-specific URI. Dialogflow uses this to construct mixed_audio. However, Dialogflow itself does not try to read or process the URI in any way."]
    pub play_audio: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessagePlayAudio>,
    >,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Returns a text response."]
    pub text:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessageText>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Indicates that the conversation succeeded, i.e., the bot handled the issue that the customer talked to it about. Dialogflow only uses this to determine which conversations should be counted as successful and doesn't process the metadata in this message in any way. Note that Dialogflow also considers conversations that get to the conversation end page as successful even if they don't return ConversationSuccess. You may set this, for example: * In the entry_fulfillment of a Page if entering the page indicates that the conversation succeeded. * In a webhook response when you determine that you handled the customer issue."]
pub struct GoogleCloudDialogflowCxV3beta1ResponseMessageConversationSuccess {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom metadata. Dialogflow doesn't impose any structure on this."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Indicates that interaction with the Dialogflow agent has ended. This message is generated by Dialogflow only and not supposed to be defined by the user."]
pub struct GoogleCloudDialogflowCxV3beta1ResponseMessageEndInteraction {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Indicates that the conversation should be handed off to a live agent. Dialogflow only uses this to determine which conversations were handed off to a human agent for measurement purposes. What else to do with this signal is up to you and your handoff procedures. You may set this, for example: * In the entry_fulfillment of a Page if entering the page indicates something went extremely wrong in the conversation. * In a webhook response when you determine that the customer issue can only be handled by a human."]
pub struct GoogleCloudDialogflowCxV3beta1ResponseMessageLiveAgentHandoff {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom metadata for your handoff procedure. Dialogflow doesn't impose any structure on this."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an audio message that is composed of both segments synthesized from the Dialogflow agent prompts and ones hosted externally at the specified URIs. The external URIs are specified via play_audio. This message is generated by Dialogflow only and not supposed to be defined by the user."]
pub struct GoogleCloudDialogflowCxV3beta1ResponseMessageMixedAudio {
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Segments this audio response is composed of."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessageMixedAudioSegment>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents one segment of audio."]
pub struct GoogleCloudDialogflowCxV3beta1ResponseMessageMixedAudioSegment {
    #[serde(rename = "allowPlaybackInterruption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether the playback of this segment can be interrupted by the end user's speech and the client should then start the next Dialogflow request."]
    pub allow_playback_interruption: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "audio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Raw audio synthesized from the Dialogflow agent's response using the output config specified in the request."]
    pub audio: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Client-specific URI that points to an audio clip accessible to the client. Dialogflow does not impose any validation on it."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A text or ssml response that is preferentially used for TTS output audio synthesis, as described in the comment on the ResponseMessage message."]
pub struct GoogleCloudDialogflowCxV3beta1ResponseMessageOutputAudioText {
    #[serde(rename = "allowPlaybackInterruption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub allow_playback_interruption: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "ssml")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The SSML text to be synthesized. For more information, see [SSML](/speech/text-to-speech/docs/ssml)."]
    pub ssml: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The raw text to be synthesized."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies an audio clip to be played by the client as part of the response."]
pub struct GoogleCloudDialogflowCxV3beta1ResponseMessagePlayAudio {
    #[serde(rename = "allowPlaybackInterruption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub allow_playback_interruption: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "audioUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The text response message."]
pub struct GoogleCloudDialogflowCxV3beta1ResponseMessageText {
    #[serde(rename = "allowPlaybackInterruption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub allow_playback_interruption: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A collection of text responses."]
    pub text: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata returned for the TestCases.RunTestCase long running operation."]
pub struct GoogleCloudDialogflowCxV3beta1RunTestCaseMetadata {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for TestCases.RunTestCase."]
pub struct GoogleCloudDialogflowCxV3beta1RunTestCaseResponse {
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result."]
    pub result:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestCaseResult>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents session information communicated to and from the webhook."]
pub struct GoogleCloudDialogflowCxV3beta1SessionInfo {
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional for WebhookRequest. Optional for WebhookResponse. All parameters collected from forms and intents during the session. Parameters can be created, updated, or removed by the webhook. To remove a parameter from the session, the webhook should explicitly set the parameter value to null in WebhookResponse. The map is keyed by parameters' display names."]
    pub parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "session")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present for WebhookRequest. Ignored for WebhookResponse. The unique identifier of the session. This field can be used by the webhook to identify a session. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/` if environment is specified."]
    pub session: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a test case."]
pub struct GoogleCloudDialogflowCxV3beta1TestCase {
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. When the test was created."]
    pub creation_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The human-readable name of the test case, unique within the agent. Limit of 200 characters."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastTestResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The latest test result."]
    pub last_test_result:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestCaseResult>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the test case. TestCases.CreateTestCase will populate the name automatically. Otherwise use format: `projects//locations//agents/ /testCases/`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional freeform notes about the test case. Limit of 400 characters."]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tags are short descriptions that users may apply to test cases for organizational and filtering purposes. Each tag should start with \"#\" and has a limit of 30 characters."]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "testCaseConversationTurns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The conversation turns uttered when the test case was created, in chronological order. These include the canonical set of agent utterances that should occur when the agent is working properly."]
    pub test_case_conversation_turns: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ConversationTurn>>,
    >,
    #[serde(rename = "testConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Config for the test case."]
    pub test_config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Error info for importing a test."]
pub struct GoogleCloudDialogflowCxV3beta1TestCaseError {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status associated with the test case."]
    pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    #[serde(rename = "testCase")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The test case."]
    pub test_case: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestCase>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a result from running a test case in an agent environment."]
pub struct GoogleCloudDialogflowCxV3beta1TestCaseResult {
    #[serde(rename = "conversationTurns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The conversation turns uttered during the test case replay in chronological order."]
    pub conversation_turns: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ConversationTurn>>,
    >,
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Environment where the test was run. If not set, it indicates the draft environment."]
    pub environment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name for the test case result. Format: `projects//locations//agents//testCases/ /results/`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "testResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the test case passed in the agent environment."]
    pub test_result:
        ::std::option::Option<GoogleCloudDialogflowCxV3beta1TestCaseResultTestResultEnum>,
    #[serde(rename = "testTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time that the test was run."]
    pub test_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Whether the test case passed in the agent environment."]
pub enum GoogleCloudDialogflowCxV3beta1TestCaseResultTestResultEnum {
    #[serde(rename = "TEST_RESULT_UNSPECIFIED")]
    #[doc = "Not specified. Should never be used."]
    TestResultUnspecified,
    #[serde(rename = "PASSED")]
    #[doc = "The test passed."]
    Passed,
    #[serde(rename = "FAILED")]
    #[doc = "The test did not pass."]
    Failed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents configurations for a test case."]
pub struct GoogleCloudDialogflowCxV3beta1TestConfig {
    #[serde(rename = "flow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flow name. If not set, default start flow is assumed. Format: `projects//locations//agents//flows/`."]
    pub flow: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trackingParameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Session parameters to be compared when calculating differences."]
    pub tracking_parameters: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Error info for running a test."]
pub struct GoogleCloudDialogflowCxV3beta1TestError {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status associated with the test."]
    pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    #[serde(rename = "testCase")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The test case resource name."]
    pub test_case: ::std::option::Option<::std::string::String>,
    #[serde(rename = "testTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp when the test was completed."]
    pub test_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The description of differences between original and replayed agent output."]
pub struct GoogleCloudDialogflowCxV3beta1TestRunDifference {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of the diff, showing the actual output vs expected output."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of diff."]
    pub _type: ::std::option::Option<GoogleCloudDialogflowCxV3beta1TestRunDifferenceTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of diff."]
pub enum GoogleCloudDialogflowCxV3beta1TestRunDifferenceTypeEnum {
    #[serde(rename = "DIFF_TYPE_UNSPECIFIED")]
    #[doc = "Should never be used."]
    DiffTypeUnspecified,
    #[serde(rename = "INTENT")]
    #[doc = "The intent."]
    Intent,
    #[serde(rename = "PAGE")]
    #[doc = "The page."]
    Page,
    #[serde(rename = "PARAMETERS")]
    #[doc = "The parameters."]
    Parameters,
    #[serde(rename = "UTTERANCE")]
    #[doc = "The message utterance."]
    Utterance,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the natural language text to be processed."]
pub struct GoogleCloudDialogflowCxV3beta1TextInput {
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The UTF-8 encoded natural language text to be processed. Text length must not exceed 256 characters."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A transition route specifies a intent that can be matched and/or a data condition that can be evaluated during a session. When a specified transition is matched, the following actions are taken in order: * If there is a `trigger_fulfillment` associated with the transition, it will be called. * If there is a `target_page` associated with the transition, the session will transition into the specified page. * If there is a `target_flow` associated with the transition, the session will transition into the specified flow."]
pub struct GoogleCloudDialogflowCxV3beta1TransitionRoute {
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The condition to evaluate against form parameters or session parameters. See the [conditions reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition). At least one of `intent` or `condition` must be specified. When both `intent` and `condition` are specified, the transition can only happen when both are fulfilled."]
    pub condition: ::std::option::Option<::std::string::String>,
    #[serde(rename = "intent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of an Intent. Format: `projects//locations//agents//intents/`. Indicates that the transition can only happen when the given intent is matched. At least one of `intent` or `condition` must be specified. When both `intent` and `condition` are specified, the transition can only happen when both are fulfilled."]
    pub intent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique identifier of this transition route."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetFlow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target flow to transition to. Format: `projects//locations//agents//flows/`."]
    pub target_flow: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target page to transition to. Format: `projects//locations//agents//flows//pages/`."]
    pub target_page: ::std::option::Option<::std::string::String>,
    #[serde(rename = "triggerFulfillment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fulfillment to call when the condition is satisfied. At least one of `trigger_fulfillment` and `target` must be specified. When both are defined, `trigger_fulfillment` is executed first."]
    pub trigger_fulfillment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Fulfillment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for UpdateDocument operation."]
pub struct GoogleCloudDialogflowCxV3beta1UpdateDocumentOperationMetadata {
    #[serde(rename = "genericMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generic information of the operation."]
    pub generic_metadata: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadata>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for a webhook call."]
pub struct GoogleCloudDialogflowCxV3beta1WebhookRequest {
    #[serde(rename = "detectIntentResponseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present. The unique identifier of the DetectIntentResponse that will be returned to the API caller."]
    pub detect_intent_response_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fulfillmentInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present. Information about the fulfillment that triggered this webhook call."]
    pub fulfillment_info: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1WebhookRequestFulfillmentInfo>,
    >,
    #[serde(rename = "intentInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the last matched intent."]
    pub intent_info: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1WebhookRequestIntentInfo>,
    >,
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of rich message responses to present to the user. Webhook can choose to append or replace this list in WebhookResponse.fulfillment_response;"]
    pub messages: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessage>>,
    >,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about page status."]
    pub page_info: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1PageInfo>>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom data set in QueryParameters.payload."]
    pub payload: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "sentimentAnalysisResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sentiment analysis result of the current user request. The field is filled when sentiment analysis is configured to be enabled for the request."]
    pub sentiment_analysis_result: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1WebhookRequestSentimentAnalysisResult>,
    >,
    #[serde(rename = "sessionInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about session status."]
    pub session_info:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1SessionInfo>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents fulfillment information communicated to the webhook."]
pub struct GoogleCloudDialogflowCxV3beta1WebhookRequestFulfillmentInfo {
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present. The tag used to identify which fulfillment is being called."]
    pub tag: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents intent information communicated to the webhook."]
pub struct GoogleCloudDialogflowCxV3beta1WebhookRequestIntentInfo {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The confidence of the matched intent. Values range from 0.0 (completely uncertain) to 1.0 (completely certain)."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present. The display name of the last matched intent."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastMatchedIntent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present. The unique identifier of the last matched intent. Format: `projects//locations//agents//intents/`."]
    pub last_matched_intent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameters identified as a result of intent matching. This is a map of the name of the identified parameter to the value of the parameter identified from the user's utterance. All parameters defined in the matched intent that are identified will be surfaced here."]
    pub parameters: ::std::option::Option<
        ::std::collections::BTreeMap<
            String,
            ::std::boxed::Box<
                GoogleCloudDialogflowCxV3beta1WebhookRequestIntentInfoIntentParameterValue,
            >,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a value for an intent parameter."]
pub struct GoogleCloudDialogflowCxV3beta1WebhookRequestIntentInfoIntentParameterValue {
    #[serde(rename = "originalValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present. Original text value extracted from user utterance."]
    pub original_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resolvedValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always present. Structured value for the parameter extracted from user utterance."]
    pub resolved_value: ::std::option::Option<::serde_json::Value>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the result of sentiment analysis."]
pub struct GoogleCloudDialogflowCxV3beta1WebhookRequestSentimentAnalysisResult {
    #[serde(rename = "magnitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A non-negative number in the [0, +inf) range, which represents the absolute magnitude of sentiment, regardless of score (positive or negative)."]
    pub magnitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sentiment score between -1.0 (negative sentiment) and 1.0 (positive sentiment)."]
    pub score: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for a webhook call."]
pub struct GoogleCloudDialogflowCxV3beta1WebhookResponse {
    #[serde(rename = "fulfillmentResponse")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fulfillment response to send to the user. This field can be omitted by the webhook if it does not intend to send any response to the user."]
    pub fulfillment_response: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1WebhookResponseFulfillmentResponse>,
    >,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about page status. This field can be omitted by the webhook if it does not intend to modify page status."]
    pub page_info: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1PageInfo>>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value to append directly to QueryResult.webhook_payloads."]
    pub payload: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "sessionInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about session status. This field can be omitted by the webhook if it does not intend to modify session status."]
    pub session_info:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1SessionInfo>>,
    #[serde(rename = "targetFlow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target flow to transition to. Format: `projects//locations//agents//flows/`."]
    pub target_flow: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target page to transition to. Format: `projects//locations//agents//flows//pages/`."]
    pub target_page: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a fulfillment response to the user."]
pub struct GoogleCloudDialogflowCxV3beta1WebhookResponseFulfillmentResponse {
    #[serde(rename = "mergeBehavior")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Merge behavior for `messages`."]
    pub merge_behavior: ::std::option::Option<
        GoogleCloudDialogflowCxV3beta1WebhookResponseFulfillmentResponseMergeBehaviorEnum,
    >,
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of rich message responses to present to the user."]
    pub messages: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessage>>,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Merge behavior for `messages`."]
pub enum GoogleCloudDialogflowCxV3beta1WebhookResponseFulfillmentResponseMergeBehaviorEnum {
    #[serde(rename = "MERGE_BEHAVIOR_UNSPECIFIED")]
    #[doc = "Not specified. `APPEND` will be used."]
    MergeBehaviorUnspecified,
    #[serde(rename = "APPEND")]
    #[doc = "`messages` will be appended to the list of messages waiting to be sent to the user."]
    Append,
    #[serde(rename = "REPLACE")]
    #[doc = "`messages` will replace the list of messages waiting to be sent to the user."]
    Replace,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a part of a message possibly annotated with an entity. The part can be an entity or purely a part of the message between two entities or message start/end."]
pub struct GoogleCloudDialogflowV2AnnotatedMessagePart {
    #[serde(rename = "entityType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The [Dialogflow system entity type](https://cloud.google.com/dialogflow/docs/reference/system-entities) of this message part. If this is empty, Dialogflow could not annotate the phrase part with a system entity."]
    pub entity_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formattedValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The [Dialogflow system entity formatted value ](https://cloud.google.com/dialogflow/docs/reference/system-entities) of this message part. For example for a system entity of type `@sys.unit-currency`, this may contain: { \"amount\": 5, \"currency\": \"USD\" } "]
    pub formatted_value: ::std::option::Option<::serde_json::Value>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A part of a message possibly annotated with an entity."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for EntityTypes.BatchUpdateEntityTypes."]
pub struct GoogleCloudDialogflowV2BatchUpdateEntityTypesResponse {
    #[serde(rename = "entityTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of updated or created entity types."]
    pub entity_types: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2EntityType>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Intents.BatchUpdateIntents."]
pub struct GoogleCloudDialogflowV2BatchUpdateIntentsResponse {
    #[serde(rename = "intents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of updated or created intents."]
    pub intents:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2Intent>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Dialogflow contexts are similar to natural language context. If a person says to you \"they are orange\", you need context in order to understand what \"they\" is referring to. Similarly, for Dialogflow to handle an end-user expression like that, it needs to be provided with context in order to correctly match an intent. Using contexts, you can control the flow of a conversation. You can configure contexts for an intent by setting input and output contexts, which are identified by string names. When an intent is matched, any configured output contexts for that intent become active. While any contexts are active, Dialogflow is more likely to match intents that are configured with input contexts that correspond to the currently active contexts. For more information about context, see the [Contexts guide](https://cloud.google.com/dialogflow/docs/contexts-overview)."]
pub struct GoogleCloudDialogflowV2Context {
    #[serde(rename = "lifespanCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The number of conversational query requests after which the context expires. The default is `0`. If set to `0`, the context expires immediately. Contexts expire automatically after 20 minutes if there are no matching queries."]
    pub lifespan_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The unique identifier of the context. Format: `projects//agent/sessions//contexts/`, or `projects//agent/environments//users//sessions//contexts/`. The `Context ID` is always converted to lowercase, may only contain characters in a-zA-Z0-9_-% and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of parameters associated with this context. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value"]
    pub parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a notification sent to Pub/Sub subscribers for conversation lifecycle events."]
pub struct GoogleCloudDialogflowV2ConversationEvent {
    #[serde(rename = "conversation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the conversation this notification refers to. Format: `projects//conversations/`."]
    pub conversation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errorStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "More detailed information about an error. Only set for type UNRECOVERABLE_ERROR_IN_PHONE_CALL."]
    pub error_status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    #[serde(rename = "newMessagePayload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Payload of NEW_MESSAGE event."]
    pub new_message_payload:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2Message>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the event that this notification refers to."]
    pub _type: ::std::option::Option<GoogleCloudDialogflowV2ConversationEventTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the event that this notification refers to."]
pub enum GoogleCloudDialogflowV2ConversationEventTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Type not set."]
    TypeUnspecified,
    #[serde(rename = "CONVERSATION_STARTED")]
    #[doc = "A new conversation has been opened. This is fired when a telephone call is answered, or a conversation is created via the API."]
    ConversationStarted,
    #[serde(rename = "CONVERSATION_FINISHED")]
    #[doc = "An existing conversation has closed. This is fired when a telephone call is terminated, or a conversation is closed via the API."]
    ConversationFinished,
    #[serde(rename = "HUMAN_INTERVENTION_NEEDED")]
    #[doc = "An existing conversation has received notification from Dialogflow that human intervention is required."]
    HumanInterventionNeeded,
    #[serde(rename = "NEW_MESSAGE")]
    #[doc = "An existing conversation has received a new message, either from API or telephony. It is configured in ConversationProfile.new_message_event_notification_config"]
    NewMessage,
    #[serde(rename = "UNRECOVERABLE_ERROR")]
    #[doc = "Unrecoverable error during a telephone call. In general non-recoverable errors only occur if something was misconfigured in the ConversationProfile corresponding to the call. After a non-recoverable error, Dialogflow may stop responding. We don't fire this event: * in an API call because we can directly return the error, or, * when we can recover from an error."]
    UnrecoverableError,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Each intent parameter has a type, called the entity type, which dictates exactly how data from an end-user expression is extracted. Dialogflow provides predefined system entities that can match many common types of data. For example, there are system entities for matching dates, times, colors, email addresses, and so on. You can also create your own custom entities for matching custom data. For example, you could define a vegetable entity that can match the types of vegetables available for purchase with a grocery store agent. For more information, see the [Entity guide](https://cloud.google.com/dialogflow/docs/entities-overview)."]
pub struct GoogleCloudDialogflowV2EntityType {
    #[serde(rename = "autoExpansionMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether the entity type can be automatically expanded."]
    pub auto_expansion_mode:
        ::std::option::Option<GoogleCloudDialogflowV2EntityTypeAutoExpansionModeEnum>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the entity type."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enableFuzzyExtraction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Enables fuzzy entity extraction during classification."]
    pub enable_fuzzy_extraction: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "entities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of entity entries associated with the entity type."]
    pub entities: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2EntityTypeEntity>>,
    >,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Indicates the kind of entity type."]
    pub kind: ::std::option::Option<GoogleCloudDialogflowV2EntityTypeKindEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType and EntityTypes.BatchUpdateEntityTypes methods. Format: `projects//agent/entityTypes/`."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Indicates whether the entity type can be automatically expanded."]
pub enum GoogleCloudDialogflowV2EntityTypeAutoExpansionModeEnum {
    #[serde(rename = "AUTO_EXPANSION_MODE_UNSPECIFIED")]
    #[doc = "Auto expansion disabled for the entity."]
    AutoExpansionModeUnspecified,
    #[serde(rename = "AUTO_EXPANSION_MODE_DEFAULT")]
    #[doc = "Allows an agent to recognize values that have not been explicitly listed in the entity."]
    AutoExpansionModeDefault,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Indicates the kind of entity type."]
pub enum GoogleCloudDialogflowV2EntityTypeKindEnum {
    #[serde(rename = "KIND_UNSPECIFIED")]
    #[doc = "Not specified. This value should be never used."]
    KindUnspecified,
    #[serde(rename = "KIND_MAP")]
    #[doc = "Map entity types allow mapping of a group of synonyms to a reference value."]
    KindMap,
    #[serde(rename = "KIND_LIST")]
    #[doc = "List entity types contain a set of entries that do not map to reference values. However, list entity types can contain references to other entity types (with or without aliases)."]
    KindList,
    #[serde(rename = "KIND_REGEXP")]
    #[doc = "Regexp entity types allow to specify regular expressions in entries values."]
    KindRegexp,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An **entity entry** for an associated entity type."]
pub struct GoogleCloudDialogflowV2EntityTypeEntity {
    #[serde(rename = "synonyms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A collection of value synonyms. For example, if the entity type is *vegetable*, and `value` is *scallions*, a synonym could be *green onions*. For `KIND_LIST` entity types: * This collection must contain exactly one synonym equal to `value`."]
    pub synonyms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The primary value associated with this entity entry. For example, if the entity type is *vegetable*, the value could be *scallions*. For `KIND_MAP` entity types: * A reference value to be used in place of synonyms. For `KIND_LIST` entity types: * A string that can contain references to other entity types (with or without aliases)."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Events allow for matching intents by event name instead of the natural language input. For instance, input `` can trigger a personalized welcome response. The parameter `name` may be used by the agent in the response: `\"Hello #welcome_event.name! What can I do for you today?\"`."]
pub struct GoogleCloudDialogflowV2EventInput {
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The language of this query. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. Note that queries in the same session do not necessarily need to specify the same language."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The unique identifier of the event."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of parameters associated with the event. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value"]
    pub parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Agents.ExportAgent."]
pub struct GoogleCloudDialogflowV2ExportAgentResponse {
    #[serde(rename = "agentContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Zip compressed raw byte content for agent."]
    pub agent_content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "agentUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI to a file containing the exported agent. This field is populated only if `agent_uri` is specified in `ExportAgentRequest`."]
    pub agent_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An intent categorizes an end-user's intention for one conversation turn. For each agent, you define many intents, where your combined intents can handle a complete conversation. When an end-user writes or says something, referred to as an end-user expression or end-user input, Dialogflow matches the end-user input to the best intent in your agent. Matching an intent is also known as intent classification. For more information, see the [intent guide](https://cloud.google.com/dialogflow/docs/intents-overview)."]
pub struct GoogleCloudDialogflowV2Intent {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The name of the action associated with the intent. Note: The action name must not contain whitespaces."]
    pub action: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultResponsePlatforms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform)."]
    pub default_response_platforms: ::std::option::Option<
        ::std::vec::Vec<GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum>,
    >,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of this intent."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of event names that trigger the intent. If the collection of input contexts is not empty, all of the contexts must be present in the active user session for an event to trigger this intent. Event names are limited to 150 characters."]
    pub events: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "followupIntentInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Read-only. Information about all followup intents that have this intent as a direct or indirect parent. We populate this field only in the output."]
    pub followup_intent_info: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentFollowupIntentInfo>>,
    >,
    #[serde(rename = "inputContextNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The list of context names required for this intent to be triggered. Format: `projects//agent/sessions/-/contexts/`."]
    pub input_context_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "isFallback")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether this is a fallback intent."]
    pub is_fallback: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of rich messages corresponding to the `Response` field in the Dialogflow console."]
    pub messages: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessage>>,
    >,
    #[serde(rename = "mlDisabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether Machine Learning is disabled for the intent. Note: If `ml_disabled` setting is set to true, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off."]
    pub ml_disabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The unique identifier of this intent. Required for Intents.UpdateIntent and Intents.BatchUpdateIntents methods. Format: `projects//agent/intents/`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outputContexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of contexts that are activated when the intent is matched. Context messages in this collection should not set the parameters field. Setting the `lifespan_count` to 0 will reset the context when the intent is matched. Format: `projects//agent/sessions/-/contexts/`."]
    pub output_contexts:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2Context>>>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of parameters associated with the intent."]
    pub parameters: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentParameter>>,
    >,
    #[serde(rename = "parentFollowupIntentName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Read-only after creation. The unique identifier of the parent intent in the chain of followup intents. You can set this field when creating an intent, for example with CreateIntent or BatchUpdateIntents, in order to make this intent a followup intent. It identifies the parent followup intent. Format: `projects//agent/intents/`."]
    pub parent_followup_intent_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests."]
    pub priority: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "resetContexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether to delete all contexts in the current session when this intent is matched."]
    pub reset_contexts: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "rootFollowupIntentName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Read-only. The unique identifier of the root intent in the chain of followup intents. It identifies the correct followup intents chain for this intent. We populate this field only in the output. Format: `projects//agent/intents/`."]
    pub root_followup_intent_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trainingPhrases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of examples that the agent is trained on."]
    pub training_phrases: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentTrainingPhrase>>,
    >,
    #[serde(rename = "webhookState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether webhooks are enabled for the intent."]
    pub webhook_state: ::std::option::Option<GoogleCloudDialogflowV2IntentWebhookStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum {
    #[serde(rename = "PLATFORM_UNSPECIFIED")]
    #[doc = "Default platform."]
    PlatformUnspecified,
    #[serde(rename = "FACEBOOK")]
    #[doc = "Facebook."]
    Facebook,
    #[serde(rename = "SLACK")]
    #[doc = "Slack."]
    Slack,
    #[serde(rename = "TELEGRAM")]
    #[doc = "Telegram."]
    Telegram,
    #[serde(rename = "KIK")]
    #[doc = "Kik."]
    Kik,
    #[serde(rename = "SKYPE")]
    #[doc = "Skype."]
    Skype,
    #[serde(rename = "LINE")]
    #[doc = "Line."]
    Line,
    #[serde(rename = "VIBER")]
    #[doc = "Viber."]
    Viber,
    #[serde(rename = "ACTIONS_ON_GOOGLE")]
    #[doc = "Google Assistant See [Dialogflow webhook format](https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json)"]
    ActionsOnGoogle,
    #[serde(rename = "GOOGLE_HANGOUTS")]
    #[doc = "Google Hangouts."]
    GoogleHangouts,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Indicates whether webhooks are enabled for the intent."]
pub enum GoogleCloudDialogflowV2IntentWebhookStateEnum {
    #[serde(rename = "WEBHOOK_STATE_UNSPECIFIED")]
    #[doc = "Webhook is disabled in the agent and in the intent."]
    WebhookStateUnspecified,
    #[serde(rename = "WEBHOOK_STATE_ENABLED")]
    #[doc = "Webhook is enabled in the agent and in the intent."]
    WebhookStateEnabled,
    #[serde(rename = "WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING")]
    #[doc = "Webhook is enabled in the agent and in the intent. Also, each slot filling prompt is forwarded to the webhook."]
    WebhookStateEnabledForSlotFilling,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a single followup intent in the chain."]
pub struct GoogleCloudDialogflowV2IntentFollowupIntentInfo {
    #[serde(rename = "followupIntentName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the followup intent. Format: `projects//agent/intents/`."]
    pub followup_intent_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parentFollowupIntentName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the followup intent's parent. Format: `projects//agent/intents/`."]
    pub parent_followup_intent_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A rich response message. Corresponds to the intent `Response` field in the Dialogflow console. For more information, see [Rich response messages](https://cloud.google.com/dialogflow/docs/intents-rich-messages)."]
pub struct GoogleCloudDialogflowV2IntentMessage {
    #[serde(rename = "basicCard")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The basic card response for Actions on Google."]
    pub basic_card:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageBasicCard>>,
    #[serde(rename = "browseCarouselCard")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Browse carousel card for Actions on Google."]
    pub browse_carousel_card: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageBrowseCarouselCard>,
    >,
    #[serde(rename = "card")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The card response."]
    pub card: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageCard>>,
    #[serde(rename = "carouselSelect")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The carousel card response for Actions on Google."]
    pub carousel_select: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageCarouselSelect>,
    >,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The image response."]
    pub image: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageImage>>,
    #[serde(rename = "linkOutSuggestion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The link out suggestion chip for Actions on Google."]
    pub link_out_suggestion: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageLinkOutSuggestion>,
    >,
    #[serde(rename = "listSelect")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list card response for Actions on Google."]
    pub list_select:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageListSelect>>,
    #[serde(rename = "mediaContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The media content card for Actions on Google."]
    pub media_content:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageMediaContent>>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A custom platform-specific response."]
    pub payload: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The platform that this message is intended for."]
    pub platform: ::std::option::Option<GoogleCloudDialogflowV2IntentMessagePlatformEnum>,
    #[serde(rename = "quickReplies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quick replies response."]
    pub quick_replies:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageQuickReplies>>,
    #[serde(rename = "simpleResponses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The voice and text-only responses for Actions on Google."]
    pub simple_responses: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageSimpleResponses>,
    >,
    #[serde(rename = "suggestions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggestion chips for Actions on Google."]
    pub suggestions:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageSuggestions>>,
    #[serde(rename = "tableCard")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Table card for Actions on Google."]
    pub table_card:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageTableCard>>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text response."]
    pub text: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageText>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. The platform that this message is intended for."]
pub enum GoogleCloudDialogflowV2IntentMessagePlatformEnum {
    #[serde(rename = "PLATFORM_UNSPECIFIED")]
    #[doc = "Default platform."]
    PlatformUnspecified,
    #[serde(rename = "FACEBOOK")]
    #[doc = "Facebook."]
    Facebook,
    #[serde(rename = "SLACK")]
    #[doc = "Slack."]
    Slack,
    #[serde(rename = "TELEGRAM")]
    #[doc = "Telegram."]
    Telegram,
    #[serde(rename = "KIK")]
    #[doc = "Kik."]
    Kik,
    #[serde(rename = "SKYPE")]
    #[doc = "Skype."]
    Skype,
    #[serde(rename = "LINE")]
    #[doc = "Line."]
    Line,
    #[serde(rename = "VIBER")]
    #[doc = "Viber."]
    Viber,
    #[serde(rename = "ACTIONS_ON_GOOGLE")]
    #[doc = "Google Assistant See [Dialogflow webhook format](https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json)"]
    ActionsOnGoogle,
    #[serde(rename = "GOOGLE_HANGOUTS")]
    #[doc = "Google Hangouts."]
    GoogleHangouts,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The basic card message. Useful for displaying information."]
pub struct GoogleCloudDialogflowV2IntentMessageBasicCard {
    #[serde(rename = "buttons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of card buttons."]
    pub buttons: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageBasicCardButton>>,
    >,
    #[serde(rename = "formattedText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required, unless image is present. The body text of the card."]
    pub formatted_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The image for the card."]
    pub image: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageImage>>,
    #[serde(rename = "subtitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The subtitle of the card."]
    pub subtitle: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The title of the card."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The button object that appears at the bottom of a card."]
pub struct GoogleCloudDialogflowV2IntentMessageBasicCardButton {
    #[serde(rename = "openUriAction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Action to take when a user taps on the button."]
    pub open_uri_action: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageBasicCardButtonOpenUriAction>,
    >,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The title of the button."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Opens the given URI."]
pub struct GoogleCloudDialogflowV2IntentMessageBasicCardButtonOpenUriAction {
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The HTTP or HTTPS scheme URI."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Browse Carousel Card for Actions on Google. https://developers.google.com/actions/assistant/responses#browsing_carousel"]
pub struct GoogleCloudDialogflowV2IntentMessageBrowseCarouselCard {
    #[serde(rename = "imageDisplayOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Settings for displaying the image. Applies to every image in items."]
    pub image_display_options: ::std::option::Option<
        GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum,
    >,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. List of items in the Browse Carousel Card. Minimum of two items, maximum of ten."]
    pub items: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<
                GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItem,
            >,
        >,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Settings for displaying the image. Applies to every image in items."]
pub enum GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum {
    #[serde(rename = "IMAGE_DISPLAY_OPTIONS_UNSPECIFIED")]
    #[doc = "Fill the gaps between the image and the image container with gray bars."]
    ImageDisplayOptionsUnspecified,
    #[serde(rename = "GRAY")]
    #[doc = "Fill the gaps between the image and the image container with gray bars."]
    Gray,
    #[serde(rename = "WHITE")]
    #[doc = "Fill the gaps between the image and the image container with white bars."]
    White,
    #[serde(rename = "CROPPED")]
    #[doc = "Image is scaled such that the image width and height match or exceed the container dimensions. This may crop the top and bottom of the image if the scaled image height is greater than the container height, or crop the left and right of the image if the scaled image width is greater than the container width. This is similar to \"Zoom Mode\" on a widescreen TV when playing a 4:3 video."]
    Cropped,
    #[serde(rename = "BLURRED_BACKGROUND")]
    #[doc = "Pad the gaps between image and image frame with a blurred copy of the same image."]
    BlurredBackground,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Browsing carousel tile"]
pub struct GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItem { # [serde (rename = "description")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Optional. Description of the carousel item. Maximum of four lines of text."] pub description : :: std :: option :: Option < :: std :: string :: String > , # [serde (rename = "footer")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Optional. Text that appears at the bottom of the Browse Carousel Card. Maximum of one line of text."] pub footer : :: std :: option :: Option < :: std :: string :: String > , # [serde (rename = "image")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Optional. Hero image for the carousel item."] pub image : :: std :: option :: Option < :: std :: boxed :: Box < GoogleCloudDialogflowV2IntentMessageImage > > , # [serde (rename = "openUriAction")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Required. Action to present to the user."] pub open_uri_action : :: std :: option :: Option < :: std :: boxed :: Box < GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlAction > > , # [serde (rename = "title")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Required. Title of the carousel item. Maximum of two lines of text."] pub title : :: std :: option :: Option < :: std :: string :: String > }
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Actions on Google action to open a given url."]
pub struct GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlAction { # [serde (rename = "url")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Required. URL"] pub url : :: std :: option :: Option < :: std :: string :: String > , # [serde (rename = "urlTypeHint")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Optional. Specifies the type of viewer that is used when opening the URL. Defaults to opening via web browser."] pub url_type_hint : :: std :: option :: Option < GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum > }
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Specifies the type of viewer that is used when opening the URL. Defaults to opening via web browser."]
pub enum GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum
{
    #[serde(rename = "URL_TYPE_HINT_UNSPECIFIED")]
    #[doc = "Unspecified"]
    UrlTypeHintUnspecified,
    #[serde(rename = "AMP_ACTION")]
    #[doc = "Url would be an amp action"]
    AmpAction,
    #[serde(rename = "AMP_CONTENT")]
    #[doc = "URL that points directly to AMP content, or to a canonical URL which refers to AMP content via ."]
    AmpContent,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The card response message."]
pub struct GoogleCloudDialogflowV2IntentMessageCard {
    #[serde(rename = "buttons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of card buttons."]
    pub buttons: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageCardButton>>,
    >,
    #[serde(rename = "imageUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The public URI to an image file for the card."]
    pub image_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subtitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The subtitle of the card."]
    pub subtitle: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The title of the card."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about a button."]
pub struct GoogleCloudDialogflowV2IntentMessageCardButton {
    #[serde(rename = "postback")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The text to send back to the Dialogflow API or a URI to open."]
    pub postback: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The text to show on the button."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The card for presenting a carousel of options to select from."]
pub struct GoogleCloudDialogflowV2IntentMessageCarouselSelect {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Carousel items."]
    pub items: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageCarouselSelectItem>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An item in the carousel."]
pub struct GoogleCloudDialogflowV2IntentMessageCarouselSelectItem {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The body text of the card."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The image to display."]
    pub image: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageImage>>,
    #[serde(rename = "info")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Additional info about the option item."]
    pub info: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageSelectItemInfo>,
    >,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Title of the carousel item."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Column properties for TableCard."]
pub struct GoogleCloudDialogflowV2IntentMessageColumnProperties {
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Column heading."]
    pub header: ::std::option::Option<::std::string::String>,
    #[serde(rename = "horizontalAlignment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Defines text alignment for all cells in this column."]
    pub horizontal_alignment: ::std::option::Option<
        GoogleCloudDialogflowV2IntentMessageColumnPropertiesHorizontalAlignmentEnum,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Defines text alignment for all cells in this column."]
pub enum GoogleCloudDialogflowV2IntentMessageColumnPropertiesHorizontalAlignmentEnum {
    #[serde(rename = "HORIZONTAL_ALIGNMENT_UNSPECIFIED")]
    #[doc = "Text is aligned to the leading edge of the column."]
    HorizontalAlignmentUnspecified,
    #[serde(rename = "LEADING")]
    #[doc = "Text is aligned to the leading edge of the column."]
    Leading,
    #[serde(rename = "CENTER")]
    #[doc = "Text is centered in the column."]
    Center,
    #[serde(rename = "TRAILING")]
    #[doc = "Text is aligned to the trailing edge of the column."]
    Trailing,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The image response message."]
pub struct GoogleCloudDialogflowV2IntentMessageImage {
    #[serde(rename = "accessibilityText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A text description of the image to be used for accessibility, e.g., screen readers."]
    pub accessibility_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imageUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The public URI to an image file."]
    pub image_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The suggestion chip message that allows the user to jump out to the app or website associated with this agent."]
pub struct GoogleCloudDialogflowV2IntentMessageLinkOutSuggestion {
    #[serde(rename = "destinationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the app or site this chip is linking to."]
    pub destination_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The URI of the app or site to open when the user taps the suggestion chip."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The card for presenting a list of options to select from."]
pub struct GoogleCloudDialogflowV2IntentMessageListSelect {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. List items."]
    pub items: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageListSelectItem>>,
    >,
    #[serde(rename = "subtitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Subtitle of the list."]
    pub subtitle: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The overall title of the list."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An item in the list."]
pub struct GoogleCloudDialogflowV2IntentMessageListSelectItem {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The main text describing the item."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The image to display."]
    pub image: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageImage>>,
    #[serde(rename = "info")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Additional information about this option."]
    pub info: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageSelectItemInfo>,
    >,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The title of the list item."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The media content card for Actions on Google."]
pub struct GoogleCloudDialogflowV2IntentMessageMediaContent {
    #[serde(rename = "mediaObjects")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. List of media objects."]
    pub media_objects: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageMediaContentResponseMediaObject>,
        >,
    >,
    #[serde(rename = "mediaType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. What type of media is the content (ie \"audio\")."]
    pub media_type:
        ::std::option::Option<GoogleCloudDialogflowV2IntentMessageMediaContentMediaTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. What type of media is the content (ie \"audio\")."]
pub enum GoogleCloudDialogflowV2IntentMessageMediaContentMediaTypeEnum {
    #[serde(rename = "RESPONSE_MEDIA_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    ResponseMediaTypeUnspecified,
    #[serde(rename = "AUDIO")]
    #[doc = "Response media type is audio."]
    Audio,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response media object for media content card."]
pub struct GoogleCloudDialogflowV2IntentMessageMediaContentResponseMediaObject {
    #[serde(rename = "contentUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Url where the media is stored."]
    pub content_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Description of media card."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "icon")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Icon to display above media content."]
    pub icon: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageImage>>,
    #[serde(rename = "largeImage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Image to display above media content."]
    pub large_image:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageImage>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Name of media card."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The quick replies response message."]
pub struct GoogleCloudDialogflowV2IntentMessageQuickReplies {
    #[serde(rename = "quickReplies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of quick replies."]
    pub quick_replies: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The title of the collection of quick replies."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional info about the select item for when it is triggered in a dialog."]
pub struct GoogleCloudDialogflowV2IntentMessageSelectItemInfo {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A unique key that will be sent back to the agent if this response is given."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "synonyms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A list of synonyms that can also be used to trigger this item in dialog."]
    pub synonyms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The simple response message containing speech or text."]
pub struct GoogleCloudDialogflowV2IntentMessageSimpleResponse {
    #[serde(rename = "displayText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The text to display."]
    pub display_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ssml")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "One of text_to_speech or ssml must be provided. Structured spoken response to the user in the SSML format. Mutually exclusive with text_to_speech."]
    pub ssml: ::std::option::Option<::std::string::String>,
    #[serde(rename = "textToSpeech")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "One of text_to_speech or ssml must be provided. The plain text of the speech output. Mutually exclusive with ssml."]
    pub text_to_speech: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The collection of simple response candidates. This message in `QueryResult.fulfillment_messages` and `WebhookResponse.fulfillment_messages` should contain only one `SimpleResponse`."]
pub struct GoogleCloudDialogflowV2IntentMessageSimpleResponses {
    #[serde(rename = "simpleResponses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The list of simple responses."]
    pub simple_responses: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageSimpleResponse>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The suggestion chip message that the user can tap to quickly post a reply to the conversation."]
pub struct GoogleCloudDialogflowV2IntentMessageSuggestion {
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The text shown the in the suggestion chip."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The collection of suggestions."]
pub struct GoogleCloudDialogflowV2IntentMessageSuggestions {
    #[serde(rename = "suggestions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The list of suggested replies."]
    pub suggestions: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageSuggestion>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Table card for Actions on Google."]
pub struct GoogleCloudDialogflowV2IntentMessageTableCard {
    #[serde(rename = "buttons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. List of buttons for the card."]
    pub buttons: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageBasicCardButton>>,
    >,
    #[serde(rename = "columnProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Display properties for the columns in this table."]
    pub column_properties: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageColumnProperties>>,
    >,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Image which should be displayed on the card."]
    pub image: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageImage>>,
    #[serde(rename = "rows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Rows in this table of data."]
    pub rows: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageTableCardRow>>,
    >,
    #[serde(rename = "subtitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Subtitle to the title."]
    pub subtitle: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Title of the card."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cell of TableCardRow."]
pub struct GoogleCloudDialogflowV2IntentMessageTableCardCell {
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Text in this cell."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Row of TableCard."]
pub struct GoogleCloudDialogflowV2IntentMessageTableCardRow {
    #[serde(rename = "cells")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. List of cells that make up this row."]
    pub cells: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageTableCardCell>>,
    >,
    #[serde(rename = "dividerAfter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Whether to add a visual divider after this row."]
    pub divider_after: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The text response message."]
pub struct GoogleCloudDialogflowV2IntentMessageText {
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of the agent's responses."]
    pub text: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents intent parameters."]
pub struct GoogleCloudDialogflowV2IntentParameter {
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The default value to use when the `value` yields an empty result. Default values can be extracted from contexts by using the following syntax: `#context_name.parameter_name`."]
    pub default_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the parameter."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityTypeDisplayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The name of the entity type, prefixed with `@`, that describes values of the parameter. If the parameter is required, this must be provided."]
    pub entity_type_display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether the parameter represents a list of values."]
    pub is_list: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "mandatory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether the parameter is required. That is, whether the intent cannot be completed without collecting the parameter value."]
    pub mandatory: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of this parameter."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "prompts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of prompts that the agent can present to the user in order to collect a value for the parameter."]
    pub prompts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The definition of the parameter value. It can be: - a constant string, - a parameter value defined as `$parameter_name`, - an original parameter value defined as `$parameter_name.original`, - a parameter value from some context defined as `#context_name.parameter_name`."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an example that the agent is trained on."]
pub struct GoogleCloudDialogflowV2IntentTrainingPhrase {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique identifier of this training phrase."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ordered list of training phrase parts. The parts are concatenated in order to form the training phrase. Note: The API does not automatically annotate training phrases like the Dialogflow Console does. Note: Do not forget to include whitespace at part boundaries, so the training phrase is well formatted when the parts are concatenated. If the training phrase does not need to be annotated with parameters, you just need a single part with only the Part.text field set. If you want to annotate the training phrase, you must create multiple parts, where the fields of each part are populated in one of two ways: - `Part.text` is set to a part of the phrase that has no parameters. - `Part.text` is set to a part of the phrase that you want to annotate, and the `entity_type`, `alias`, and `user_defined` fields are all set."]
    pub parts: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentTrainingPhrasePart>>,
    >,
    #[serde(rename = "timesAddedCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates how many times this example was added to the intent. Each time a developer adds an existing sample by editing an intent or training, this counter is increased."]
    pub times_added_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of the training phrase."]
    pub _type: ::std::option::Option<GoogleCloudDialogflowV2IntentTrainingPhraseTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of the training phrase."]
pub enum GoogleCloudDialogflowV2IntentTrainingPhraseTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Not specified. This value should never be used."]
    TypeUnspecified,
    #[serde(rename = "EXAMPLE")]
    #[doc = "Examples do not contain @-prefixed entity type names, but example parts can be annotated with entity types."]
    Example,
    #[serde(rename = "TEMPLATE")]
    #[doc = "Templates are not annotated with entity types, but they can contain @-prefixed entity type names as substrings. Template mode has been deprecated. Example mode is the only supported way to create new training phrases. If you have existing training phrases that you've created in template mode, those will continue to work."]
    Template,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a part of a training phrase."]
pub struct GoogleCloudDialogflowV2IntentTrainingPhrasePart {
    #[serde(rename = "alias")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The parameter name for the value extracted from the annotated part of the example. This field is required for annotated parts of the training phrase."]
    pub alias: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The entity type name prefixed with `@`. This field is required for annotated parts of the training phrase."]
    pub entity_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The text for this part."]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userDefined")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether the text was manually annotated. This field is set to true when the Dialogflow Console is used to manually annotate the part. When creating an annotated part with the API, you must set this to true."]
    pub user_defined: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a message posted into a conversation."]
pub struct GoogleCloudDialogflowV2Message {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The message content."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the message was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The message language. This should be a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: \"en-US\"."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "messageAnnotation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The annotation for the message."]
    pub message_annotation:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2MessageAnnotation>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the message. Format: `projects//locations//conversations//messages/`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "participant")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The participant that sends this message."]
    pub participant: ::std::option::Option<::std::string::String>,
    #[serde(rename = "participantRole")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The role of the participant."]
    pub participant_role: ::std::option::Option<GoogleCloudDialogflowV2MessageParticipantRoleEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The role of the participant."]
pub enum GoogleCloudDialogflowV2MessageParticipantRoleEnum {
    #[serde(rename = "ROLE_UNSPECIFIED")]
    #[doc = "Participant role not set."]
    RoleUnspecified,
    #[serde(rename = "HUMAN_AGENT")]
    #[doc = "Participant is a human agent."]
    HumanAgent,
    #[serde(rename = "AUTOMATED_AGENT")]
    #[doc = "Participant is an automated agent, such as a Dialogflow agent."]
    AutomatedAgent,
    #[serde(rename = "END_USER")]
    #[doc = "Participant is an end user that has called or chatted with Dialogflow services."]
    EndUser,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the result of annotation for the message."]
pub struct GoogleCloudDialogflowV2MessageAnnotation {
    #[serde(rename = "containEntities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the text message contains entities."]
    pub contain_entities: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "parts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of annotated message parts ordered by their position in the message. You can recover the annotated message by concatenating [AnnotatedMessagePart.text]."]
    pub parts: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2AnnotatedMessagePart>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the contents of the original request that was passed to the `[Streaming]DetectIntent` call."]
pub struct GoogleCloudDialogflowV2OriginalDetectIntentRequest {
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. This field is set to the value of the `QueryParameters.payload` field passed in the request. Some integrations that query a Dialogflow agent may provide additional information in the payload. In particular, for the Dialogflow Phone Gateway integration, this field has the form: { \"telephony\": { \"caller_id\": \"+18558363987\" } } Note: The caller ID field (`caller_id`) will be redacted for Trial Edition agents and populated with the caller ID in [E.164 format](https://en.wikipedia.org/wiki/E.164) for Essentials Edition agents."]
    pub payload: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source of this request, e.g., `google`, `facebook`, `slack`. It is set by Dialogflow-owned servers."]
    pub source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The version of the protocol used for this request. This field is AoG-specific."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the result of conversational query or event processing."]
pub struct GoogleCloudDialogflowV2QueryResult {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The action name from the matched intent."]
    pub action: ::std::option::Option<::std::string::String>,
    #[serde(rename = "allRequiredParamsPresent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is set to: - `false` if the matched intent has required parameters and not all of the required parameter values have been collected. - `true` if all required parameter values have been collected, or if the matched intent doesn't contain any required parameters."]
    pub all_required_params_present: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "diagnosticInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Free-form diagnostic information for the associated detect intent request. The fields of this data can change without notice, so you should not write code that depends on its structure. The data may contain: - webhook call latency - webhook errors"]
    pub diagnostic_info:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "fulfillmentMessages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of rich messages to present to the user."]
    pub fulfillment_messages: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessage>>,
    >,
    #[serde(rename = "fulfillmentText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text to be pronounced to the user or shown on the screen. Note: This is a legacy field, `fulfillment_messages` should be preferred."]
    pub fulfillment_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "intent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The intent that matched the conversational query. Some, not all fields are filled in this message, including but not limited to: `name`, `display_name`, `end_interaction` and `is_fallback`."]
    pub intent: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2Intent>>,
    #[serde(rename = "intentDetectionConfidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The intent detection confidence. Values range from 0.0 (completely uncertain) to 1.0 (completely certain). This value is for informational purpose only and is only used to help match the best intent within the classification threshold. This value may change for the same end-user expression at any time due to a model retraining or change in implementation. If there are `multiple knowledge_answers` messages, this value is set to the greatest `knowledgeAnswers.match_confidence` value in the list."]
    pub intent_detection_confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language that was triggered during intent detection. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outputContexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of output contexts. If applicable, `output_contexts.parameters` contains entries with name `.original` containing the original parameter values before the query."]
    pub output_contexts:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2Context>>>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of extracted parameters. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value"]
    pub parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "queryText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The original conversational query text: - If natural language text was provided as input, `query_text` contains a copy of the input. - If natural language speech audio was provided as input, `query_text` contains the speech recognition result. If speech recognizer produced multiple alternatives, a particular one is picked. - If automatic spell correction is enabled, `query_text` will contain the corrected user input."]
    pub query_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sentimentAnalysisResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sentiment analysis result, which depends on the `sentiment_analysis_request_config` specified in the request."]
    pub sentiment_analysis_result:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2SentimentAnalysisResult>>,
    #[serde(rename = "speechRecognitionConfidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Speech recognition confidence between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. The default of 0.0 is a sentinel value indicating that confidence was not set. This field is not guaranteed to be accurate or set. In particular this field isn't set for StreamingDetectIntent since the streaming endpoint has separate confidence estimates per portion of the audio in StreamingRecognitionResult."]
    pub speech_recognition_confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "webhookPayload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the query was fulfilled by a webhook call, this field is set to the value of the `payload` field returned in the webhook response."]
    pub webhook_payload:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "webhookSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the query was fulfilled by a webhook call, this field is set to the value of the `source` field returned in the webhook response."]
    pub webhook_source: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The sentiment, such as positive/negative feeling or association, for a unit of analysis, such as the query text."]
pub struct GoogleCloudDialogflowV2Sentiment {
    #[serde(rename = "magnitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A non-negative number in the [0, +inf) range, which represents the absolute magnitude of sentiment, regardless of score (positive or negative)."]
    pub magnitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sentiment score between -1.0 (negative sentiment) and 1.0 (positive sentiment)."]
    pub score: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The result of sentiment analysis. Sentiment analysis inspects user input and identifies the prevailing subjective opinion, especially to determine a user's attitude as positive, negative, or neutral. For Participants.DetectIntent, it needs to be configured in DetectIntentRequest.query_params. For Participants.StreamingDetectIntent, it needs to be configured in StreamingDetectIntentRequest.query_params. And for Participants.AnalyzeContent and Participants.StreamingAnalyzeContent, it needs to be configured in ConversationProfile.human_agent_assistant_config"]
pub struct GoogleCloudDialogflowV2SentimentAnalysisResult {
    #[serde(rename = "queryTextSentiment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sentiment analysis result for `query_text`."]
    pub query_text_sentiment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2Sentiment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A session represents a conversation between a Dialogflow agent and an end-user. You can create special entities, called session entities, during a session. Session entities can extend or replace custom entity types and only exist during the session that they were created for. All session data, including session entities, is stored by Dialogflow for 20 minutes. For more information, see the [session entity guide](https://cloud.google.com/dialogflow/docs/entities-session)."]
pub struct GoogleCloudDialogflowV2SessionEntityType {
    #[serde(rename = "entities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The collection of entities associated with this session entity type."]
    pub entities: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2EntityTypeEntity>>,
    >,
    #[serde(rename = "entityOverrideMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Indicates whether the additional data should override or supplement the custom entity type definition."]
    pub entity_override_mode:
        ::std::option::Option<GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideModeEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The unique identifier of this session entity type. Format: `projects//agent/sessions//entityTypes/`, or `projects//agent/environments//users//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. `` must be the display name of an existing entity type in the same agent that will be overridden or supplemented."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Indicates whether the additional data should override or supplement the custom entity type definition."]
pub enum GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideModeEnum {
    #[serde(rename = "ENTITY_OVERRIDE_MODE_UNSPECIFIED")]
    #[doc = "Not specified. This value should be never used."]
    EntityOverrideModeUnspecified,
    #[serde(rename = "ENTITY_OVERRIDE_MODE_OVERRIDE")]
    #[doc = "The collection of session entities overrides the collection of entities in the corresponding custom entity type."]
    EntityOverrideModeOverride,
    #[serde(rename = "ENTITY_OVERRIDE_MODE_SUPPLEMENT")]
    #[doc = "The collection of session entities extends the collection of entities in the corresponding custom entity type. Note: Even in this override mode calls to `ListSessionEntityTypes`, `GetSessionEntityType`, `CreateSessionEntityType` and `UpdateSessionEntityType` only return the additional entities added in this session entity type. If you want to get the supplemented list, please call EntityTypes.GetEntityType on the custom entity type and merge."]
    EntityOverrideModeSupplement,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for a webhook call."]
pub struct GoogleCloudDialogflowV2WebhookRequest {
    #[serde(rename = "originalDetectIntentRequest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The contents of the original request that was passed to `[Streaming]DetectIntent` call."]
    pub original_detect_intent_request: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2OriginalDetectIntentRequest>,
    >,
    #[serde(rename = "queryResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the conversational query or event processing. Contains the same value as `[Streaming]DetectIntentResponse.query_result`."]
    pub query_result: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2QueryResult>>,
    #[serde(rename = "responseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the response. Contains the same value as `[Streaming]DetectIntentResponse.response_id`."]
    pub response_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "session")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of detectIntent request session. Can be used to identify end-user inside webhook implementation. Format: `projects//agent/sessions/`, or `projects//agent/environments//users//sessions/`."]
    pub session: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for a webhook call. This response is validated by the Dialogflow server. If validation fails, an error will be returned in the QueryResult.diagnostic_info field. Setting JSON fields to an empty value with the wrong type is a common error. To avoid this error: - Use `\"\"` for empty strings - Use `{}` or `null` for empty objects - Use `[]` or `null` for empty arrays For more information, see the [Protocol Buffers Language Guide](https://developers.google.com/protocol-buffers/docs/proto3#json)."]
pub struct GoogleCloudDialogflowV2WebhookResponse {
    #[serde(rename = "followupEventInput")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Invokes the supplied events. When this field is set, Dialogflow ignores the `fulfillment_text`, `fulfillment_messages`, and `payload` fields."]
    pub followup_event_input:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2EventInput>>,
    #[serde(rename = "fulfillmentMessages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The rich response messages intended for the end-user. When provided, Dialogflow uses this field to populate QueryResult.fulfillment_messages sent to the integration or API caller."]
    pub fulfillment_messages: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessage>>,
    >,
    #[serde(rename = "fulfillmentText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The text response message intended for the end-user. It is recommended to use `fulfillment_messages.text.text[0]` instead. When provided, Dialogflow uses this field to populate QueryResult.fulfillment_text sent to the integration or API caller."]
    pub fulfillment_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outputContexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of output contexts that will overwrite currently active contexts for the session and reset their lifespans. When provided, Dialogflow uses this field to populate QueryResult.output_contexts sent to the integration or API caller."]
    pub output_contexts:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2Context>>>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. This field can be used to pass custom data from your webhook to the integration or API caller. Arbitrary JSON objects are supported. When provided, Dialogflow uses this field to populate QueryResult.webhook_payload sent to the integration or API caller. This field is also used by the [Google Assistant integration](https://cloud.google.com/dialogflow/docs/integrations/aog) for rich response messages. See the format definition at [Google Assistant Dialogflow webhook format](https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json)"]
    pub payload: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "sessionEntityTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Additional session entity types to replace or extend developer entity types with. The entity synonyms apply to all languages and persist for the session. Setting this data from a webhook overwrites the session entity types that have been set using `detectIntent`, `streamingDetectIntent` or SessionEntityType management methods."]
    pub session_entity_types: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2SessionEntityType>>,
    >,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A custom field used to identify the webhook source. Arbitrary strings are supported. When provided, Dialogflow uses this field to populate QueryResult.webhook_source sent to the integration or API caller."]
    pub source: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Dialogflow agent is a virtual agent that handles conversations with your end-users. It is a natural language understanding module that understands the nuances of human language. Dialogflow translates end-user text or audio during a conversation to structured data that your apps and services can understand. You design and build a Dialogflow agent to handle the types of conversations required for your system. For more information about agents, see the [Agent guide](https://cloud.google.com/dialogflow/docs/agents-overview)."]
pub struct GoogleCloudDialogflowV2beta1Agent {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version."]
    pub api_version: ::std::option::Option<GoogleCloudDialogflowV2beta1AgentApiVersionEnum>,
    #[serde(rename = "avatarUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration."]
    pub avatar_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "classificationThreshold")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. To filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold. If the returned score value is less than the threshold value, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used."]
    pub classification_threshold: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "defaultLanguageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the `Update` method."]
    pub default_language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of this agent."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enableLogging")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Determines whether this agent should log conversation queries."]
    pub enable_logging: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "matchMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Determines how intents are detected from user queries."]
    pub match_mode: ::std::option::Option<GoogleCloudDialogflowV2beta1AgentMatchModeEnum>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The project of this agent. Format: `projects/` or `projects//locations/`"]
    pub parent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "supportedLanguageCodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The list of all languages supported by this agent (except for the `default_language_code`)."]
    pub supported_language_codes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "tier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The agent tier. If not specified, TIER_STANDARD is assumed."]
    pub tier: ::std::option::Option<GoogleCloudDialogflowV2beta1AgentTierEnum>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris."]
    pub time_zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version."]
pub enum GoogleCloudDialogflowV2beta1AgentApiVersionEnum {
    #[serde(rename = "API_VERSION_UNSPECIFIED")]
    #[doc = "Not specified."]
    ApiVersionUnspecified,
    #[serde(rename = "API_VERSION_V1")]
    #[doc = "Legacy V1 API."]
    ApiVersionV1,
    #[serde(rename = "API_VERSION_V2")]
    #[doc = "V2 API."]
    ApiVersionV2,
    #[serde(rename = "API_VERSION_V2_BETA_1")]
    #[doc = "V2beta1 API."]
    ApiVersionV2Beta1,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Determines how intents are detected from user queries."]
pub enum GoogleCloudDialogflowV2beta1AgentMatchModeEnum {
    #[serde(rename = "MATCH_MODE_UNSPECIFIED")]
    #[doc = "Not specified."]
    MatchModeUnspecified,
    #[serde(rename = "MATCH_MODE_HYBRID")]
    #[doc = "Best for agents with a small number of examples in intents and/or wide use of templates syntax and composite entities."]
    MatchModeHybrid,
    #[serde(rename = "MATCH_MODE_ML_ONLY")]
    #[doc = "Can be used for agents with a large number of examples in intents, especially the ones using @sys.any or very large custom entities."]
    MatchModeMlOnly,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. The agent tier. If not specified, TIER_STANDARD is assumed."]
pub enum GoogleCloudDialogflowV2beta1AgentTierEnum {
    #[serde(rename = "TIER_UNSPECIFIED")]
    #[doc = "Not specified. This value should never be used."]
    TierUnspecified,
    #[serde(rename = "TIER_STANDARD")]
    #[doc = "Standard tier."]
    TierStandard,
    #[serde(rename = "TIER_ENTERPRISE")]
    #[doc = "Enterprise tier (Essentials)."]
    TierEnterprise,
    #[serde(rename = "TIER_ENTERPRISE_PLUS")]
    #[doc = "Enterprise tier (Plus)."]
    TierEnterprisePlus,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for EntityTypes.BatchCreateEntities."]
pub struct GoogleCloudDialogflowV2beta1BatchCreateEntitiesRequest {
    #[serde(rename = "entities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The entities to create."]
    pub entities: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1EntityTypeEntity>>,
    >,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The language used to access language-specific data. If not specified, the agent's default language is used. For more information, see [Multilingual intent and entity data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity)."]
    pub language_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for EntityTypes.BatchDeleteEntities."]
pub struct GoogleCloudDialogflowV2beta1BatchDeleteEntitiesRequest {
    #[serde(rename = "entityValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The reference `values` of the entities to delete. Note that these are not fully-qualified names, i.e. they don't start with `projects/`."]
    pub entity_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The language used to access language-specific data. If not specified, the agent's default language is used. For more information, see [Multilingual intent and entity data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity)."]
    pub language_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for EntityTypes.BatchDeleteEntityTypes."]
pub struct GoogleCloudDialogflowV2beta1BatchDeleteEntityTypesRequest {
    #[serde(rename = "entityTypeNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The names entity types to delete. All names must point to the same agent as `parent`."]
    pub entity_type_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for Intents.BatchDeleteIntents."]
pub struct GoogleCloudDialogflowV2beta1BatchDeleteIntentsRequest {
    #[serde(rename = "intents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The collection of intents to delete. Only intent `name` must be filled in."]
    pub intents: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1Intent>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for EntityTypes.BatchUpdateEntities."]
pub struct GoogleCloudDialogflowV2beta1BatchUpdateEntitiesRequest {
    #[serde(rename = "entities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The entities to update or create."]
    pub entities: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1EntityTypeEntity>>,
    >,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The language used to access language-specific data. If not specified, the agent's default language is used. For more information, see [Multilingual intent and entity data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity)."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The mask to control which fields get updated."]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for EntityTypes.BatchUpdateEntityTypes."]
pub struct GoogleCloudDialogflowV2beta1BatchUpdateEntityTypesRequest {
    #[serde(rename = "entityTypeBatchInline")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of entity types to update or create."]
    pub entity_type_batch_inline:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1EntityTypeBatch>>,
    #[serde(rename = "entityTypeBatchUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI to a Google Cloud Storage file containing entity types to update or create. The file format can either be a serialized proto (of EntityBatch type) or a JSON object. Note: The URI must start with \"gs://\"."]
    pub entity_type_batch_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The language used to access language-specific data. If not specified, the agent's default language is used. For more information, see [Multilingual intent and entity data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity)."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The mask to control which fields get updated."]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for EntityTypes.BatchUpdateEntityTypes."]
pub struct GoogleCloudDialogflowV2beta1BatchUpdateEntityTypesResponse {
    #[serde(rename = "entityTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of updated or created entity types."]
    pub entity_types: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1EntityType>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for Intents.BatchUpdateIntents."]
pub struct GoogleCloudDialogflowV2beta1BatchUpdateIntentsRequest {
    #[serde(rename = "intentBatchInline")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of intents to update or create."]
    pub intent_batch_inline:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentBatch>>,
    #[serde(rename = "intentBatchUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI to a Google Cloud Storage file containing intents to update or create. The file format can either be a serialized proto (of IntentBatch type) or JSON object. Note: The URI must start with \"gs://\"."]
    pub intent_batch_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "intentView")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The resource view to apply to the returned intent."]
    pub intent_view:
        ::std::option::Option<GoogleCloudDialogflowV2beta1BatchUpdateIntentsRequestIntentViewEnum>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The language used to access language-specific data. If not specified, the agent's default language is used. For more information, see [Multilingual intent and entity data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity)."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The mask to control which fields get updated."]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. The resource view to apply to the returned intent."]
pub enum GoogleCloudDialogflowV2beta1BatchUpdateIntentsRequestIntentViewEnum {
    #[serde(rename = "INTENT_VIEW_UNSPECIFIED")]
    #[doc = "Training phrases field is not populated in the response."]
    IntentViewUnspecified,
    #[serde(rename = "INTENT_VIEW_FULL")]
    #[doc = "All fields are populated."]
    IntentViewFull,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Intents.BatchUpdateIntents."]
pub struct GoogleCloudDialogflowV2beta1BatchUpdateIntentsResponse {
    #[serde(rename = "intents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of updated or created intents."]
    pub intents: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1Intent>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Dialogflow contexts are similar to natural language context. If a person says to you \"they are orange\", you need context in order to understand what \"they\" is referring to. Similarly, for Dialogflow to handle an end-user expression like that, it needs to be provided with context in order to correctly match an intent. Using contexts, you can control the flow of a conversation. You can configure contexts for an intent by setting input and output contexts, which are identified by string names. When an intent is matched, any configured output contexts for that intent become active. While any contexts are active, Dialogflow is more likely to match intents that are configured with input contexts that correspond to the currently active contexts. For more information about context, see the [Contexts guide](https://cloud.google.com/dialogflow/docs/contexts-overview)."]
pub struct GoogleCloudDialogflowV2beta1Context {
    #[serde(rename = "lifespanCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The number of conversational query requests after which the context expires. The default is `0`. If set to `0`, the context expires immediately. Contexts expire automatically after 20 minutes if there are no matching queries."]
    pub lifespan_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The unique identifier of the context. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, The `Context ID` is always converted to lowercase, may only contain characters in a-zA-Z0-9_-% and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of parameters associated with this context. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value"]
    pub parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request to detect user's intent."]
pub struct GoogleCloudDialogflowV2beta1DetectIntentRequest {
    #[serde(rename = "inputAudio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The natural language speech audio to be processed. This field should be populated iff `query_input` is set to an input audio config. A single request can contain up to 1 minute of speech audio data."]
    pub input_audio: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outputAudioConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Instructs the speech synthesizer how to generate the output audio. If this field is not set and agent-level speech synthesizer is not configured, no output audio is generated."]
    pub output_audio_config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1OutputAudioConfig>>,
    #[serde(rename = "outputAudioConfigMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mask for output_audio_config indicating which settings in this request-level config should override speech synthesizer settings defined at agent-level. If unspecified or empty, output_audio_config replaces the agent-level config in its entirety."]
    pub output_audio_config_mask: ::std::option::Option<::std::string::String>,
    #[serde(rename = "queryInput")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The input specification. It can be set to: 1. an audio config which instructs the speech recognizer how to process the speech audio, 2. a conversational query in the form of text, or 3. an event that specifies which intent to trigger."]
    pub query_input:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1QueryInput>>,
    #[serde(rename = "queryParams")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The parameters of this query."]
    pub query_params:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1QueryParameters>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The message returned from the DetectIntent method."]
pub struct GoogleCloudDialogflowV2beta1DetectIntentResponse {
    #[serde(rename = "alternativeQueryResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If Knowledge Connectors are enabled, there could be more than one result returned for a given query or event, and this field will contain all results except for the top one, which is captured in query_result. The alternative results are ordered by decreasing `QueryResult.intent_detection_confidence`. If Knowledge Connectors are disabled, this field will be empty until multiple responses for regular intents are supported, at which point those additional results will be surfaced here."]
    pub alternative_query_results: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1QueryResult>>,
    >,
    #[serde(rename = "outputAudio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The audio data bytes encoded as specified in the request. Note: The output audio is generated based on the values of default platform text responses found in the `query_result.fulfillment_messages` field. If multiple default text responses exist, they will be concatenated when generating audio. If no default platform text responses exist, the generated audio content will be empty. In some scenarios, multiple output audio fields may be present in the response structure. In these cases, only the top-most-level audio output has content."]
    pub output_audio: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outputAudioConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The config used by the speech synthesizer to generate the output audio."]
    pub output_audio_config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1OutputAudioConfig>>,
    #[serde(rename = "queryResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The selected results of the conversational query or event processing. See `alternative_query_results` for additional potential results."]
    pub query_result:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1QueryResult>>,
    #[serde(rename = "responseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the response. It can be used to locate a response in the training example set or for reporting issues."]
    pub response_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "webhookStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the status of the webhook request."]
    pub webhook_status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A knowledge document to be used by a KnowledgeBase. For more information, see the [knowledge base guide](https://cloud.google.com/dialogflow/docs/how/knowledge-bases). Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`."]
pub struct GoogleCloudDialogflowV2beta1Document {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The raw content of the document. This field is only permitted for EXTRACTIVE_QA and FAQ knowledge types. Note: This field is in the process of being deprecated, please use raw_content instead."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI where the file content is located. For documents stored in Google Cloud Storage, these URIs must have the form `gs:///`. NOTE: External URLs must correspond to public webpages, i.e., they must be indexed by Google Search. In particular, URLs for showing documents in Google Cloud Storage (i.e. the URL in your browser) are not supported. Instead use the `gs://` format URI described above."]
    pub content_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The display name of the document. The name must be 1024 bytes or less; otherwise, the creation request fails."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enableAutoReload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If true, we try to automatically reload the document every day (at a time picked by the system). If false or unspecified, we don't try to automatically reload the document. Currently you can only enable automatic reload for documents sourced from a public url, see `source` field for the source types. Reload status can be tracked in `latest_reload_status`. If a reload fails, we will keep the document unchanged. If a reload fails with internal errors, the system will try to reload the document on the next day. If a reload fails with non-retriable errors (e.g. PERMISION_DENIED), the system will not try to reload the document anymore. You need to manually reload the document successfully by calling `ReloadDocument` and clear the errors."]
    pub enable_auto_reload: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "knowledgeTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The knowledge type of document content."]
    pub knowledge_types: ::std::option::Option<
        ::std::vec::Vec<GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum>,
    >,
    #[serde(rename = "latestReloadStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time and status of the latest reload. This reload may have been triggered automatically or manually and may not have succeeded."]
    pub latest_reload_status:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1DocumentReloadStatus>>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The MIME type of this document."]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The document resource name. The name must be empty when creating a document. Format: `projects//locations//knowledgeBases//documents/`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rawContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The raw content of the document. This field is only permitted for EXTRACTIVE_QA and FAQ knowledge types."]
    pub raw_content: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum {
    #[serde(rename = "KNOWLEDGE_TYPE_UNSPECIFIED")]
    #[doc = "The type is unspecified or arbitrary."]
    KnowledgeTypeUnspecified,
    #[serde(rename = "FAQ")]
    #[doc = "The document content contains question and answer pairs as either HTML or CSV. Typical FAQ HTML formats are parsed accurately, but unusual formats may fail to be parsed. CSV must have questions in the first column and answers in the second, with no header. Because of this explicit format, they are always parsed accurately."]
    Faq,
    #[serde(rename = "EXTRACTIVE_QA")]
    #[doc = "Documents for which unstructured text is extracted and used for question answering."]
    ExtractiveQa,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The status of a reload attempt."]
pub struct GoogleCloudDialogflowV2beta1DocumentReloadStatus {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The status of a reload attempt or the initial load."]
    pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time of a reload attempt. This reload may have been triggered automatically or manually and may not have succeeded."]
    pub time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Each intent parameter has a type, called the entity type, which dictates exactly how data from an end-user expression is extracted. Dialogflow provides predefined system entities that can match many common types of data. For example, there are system entities for matching dates, times, colors, email addresses, and so on. You can also create your own custom entities for matching custom data. For example, you could define a vegetable entity that can match the types of vegetables available for purchase with a grocery store agent. For more information, see the [Entity guide](https://cloud.google.com/dialogflow/docs/entities-overview)."]
pub struct GoogleCloudDialogflowV2beta1EntityType {
    #[serde(rename = "autoExpansionMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether the entity type can be automatically expanded."]
    pub auto_expansion_mode:
        ::std::option::Option<GoogleCloudDialogflowV2beta1EntityTypeAutoExpansionModeEnum>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the entity type."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enableFuzzyExtraction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Enables fuzzy entity extraction during classification."]
    pub enable_fuzzy_extraction: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "entities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of entity entries associated with the entity type."]
    pub entities: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1EntityTypeEntity>>,
    >,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Indicates the kind of entity type."]
    pub kind: ::std::option::Option<GoogleCloudDialogflowV2beta1EntityTypeKindEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType and EntityTypes.BatchUpdateEntityTypes methods. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/`"]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Indicates whether the entity type can be automatically expanded."]
pub enum GoogleCloudDialogflowV2beta1EntityTypeAutoExpansionModeEnum {
    #[serde(rename = "AUTO_EXPANSION_MODE_UNSPECIFIED")]
    #[doc = "Auto expansion disabled for the entity."]
    AutoExpansionModeUnspecified,
    #[serde(rename = "AUTO_EXPANSION_MODE_DEFAULT")]
    #[doc = "Allows an agent to recognize values that have not been explicitly listed in the entity."]
    AutoExpansionModeDefault,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Indicates the kind of entity type."]
pub enum GoogleCloudDialogflowV2beta1EntityTypeKindEnum {
    #[serde(rename = "KIND_UNSPECIFIED")]
    #[doc = "Not specified. This value should be never used."]
    KindUnspecified,
    #[serde(rename = "KIND_MAP")]
    #[doc = "Map entity types allow mapping of a group of synonyms to a reference value."]
    KindMap,
    #[serde(rename = "KIND_LIST")]
    #[doc = "List entity types contain a set of entries that do not map to reference values. However, list entity types can contain references to other entity types (with or without aliases)."]
    KindList,
    #[serde(rename = "KIND_REGEXP")]
    #[doc = "Regexp entity types allow to specify regular expressions in entries values."]
    KindRegexp,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This message is a wrapper around a collection of entity types."]
pub struct GoogleCloudDialogflowV2beta1EntityTypeBatch {
    #[serde(rename = "entityTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A collection of entity types."]
    pub entity_types: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1EntityType>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An **entity entry** for an associated entity type."]
pub struct GoogleCloudDialogflowV2beta1EntityTypeEntity {
    #[serde(rename = "synonyms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A collection of value synonyms. For example, if the entity type is *vegetable*, and `value` is *scallions*, a synonym could be *green onions*. For `KIND_LIST` entity types: * This collection must contain exactly one synonym equal to `value`."]
    pub synonyms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The primary value associated with this entity entry. For example, if the entity type is *vegetable*, the value could be *scallions*. For `KIND_MAP` entity types: * A reference value to be used in place of synonyms. For `KIND_LIST` entity types: * A string that can contain references to other entity types (with or without aliases)."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "You can create multiple versions of your agent and publish them to separate environments. When you edit an agent, you are editing the draft agent. At any point, you can save the draft agent as an agent version, which is an immutable snapshot of your agent. When you save the draft agent, it is published to the default environment. When you create agent versions, you can publish them to custom environments. You can create a variety of custom environments for: - testing - development - production - etc. For more information, see the [versions and environments guide](https://cloud.google.com/dialogflow/docs/agents-versions)."]
pub struct GoogleCloudDialogflowV2beta1Environment {
    #[serde(rename = "agentVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The agent version loaded into this environment. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/`"]
    pub agent_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The developer-provided description for this environment. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique identifier of this agent environment. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The state of this environment. This field is read-only, i.e., it cannot be set by create and update methods."]
    pub state: ::std::option::Option<GoogleCloudDialogflowV2beta1EnvironmentStateEnum>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The last update time of this environment. This field is read-only, i.e., it cannot be set by create and update methods."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The state of this environment. This field is read-only, i.e., it cannot be set by create and update methods."]
pub enum GoogleCloudDialogflowV2beta1EnvironmentStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Not specified. This value is not used."]
    StateUnspecified,
    #[serde(rename = "STOPPED")]
    #[doc = "Stopped."]
    Stopped,
    #[serde(rename = "LOADING")]
    #[doc = "Loading."]
    Loading,
    #[serde(rename = "RUNNING")]
    #[doc = "Running."]
    Running,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Events allow for matching intents by event name instead of the natural language input. For instance, input `` can trigger a personalized welcome response. The parameter `name` may be used by the agent in the response: `\"Hello #welcome_event.name! What can I do for you today?\"`."]
pub struct GoogleCloudDialogflowV2beta1EventInput {
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The language of this query. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. Note that queries in the same session do not necessarily need to specify the same language."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The unique identifier of the event."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of parameters associated with the event. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value"]
    pub parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for Agents.ExportAgent."]
pub struct GoogleCloudDialogflowV2beta1ExportAgentRequest {
    #[serde(rename = "agentUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to export the agent to. The format of this URI must be `gs:///`. If left unspecified, the serialized agent is returned inline."]
    pub agent_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Agents.ExportAgent."]
pub struct GoogleCloudDialogflowV2beta1ExportAgentResponse {
    #[serde(rename = "agentContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Zip compressed raw byte content for agent."]
    pub agent_content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "agentUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI to a file containing the exported agent. This field is populated only if `agent_uri` is specified in `ExportAgentRequest`."]
    pub agent_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "By default, your agent responds to a matched intent with a static response. As an alternative, you can provide a more dynamic response by using fulfillment. When you enable fulfillment for an intent, Dialogflow responds to that intent by calling a service that you define. For example, if an end-user wants to schedule a haircut on Friday, your service can check your database and respond to the end-user with availability information for Friday. For more information, see the [fulfillment guide](https://cloud.google.com/dialogflow/docs/fulfillment-overview)."]
pub struct GoogleCloudDialogflowV2beta1Fulfillment {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human-readable name of the fulfillment, unique within the agent."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether fulfillment is enabled."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The field defines whether the fulfillment is enabled for certain features."]
    pub features: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1FulfillmentFeature>>,
    >,
    #[serde(rename = "genericWebService")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for a generic web service."]
    pub generic_web_service: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1FulfillmentGenericWebService>,
    >,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The unique identifier of the fulfillment. Supported formats: - `projects//agent/fulfillment` - `projects//locations//agent/fulfillment`"]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Whether fulfillment is enabled for the specific feature."]
pub struct GoogleCloudDialogflowV2beta1FulfillmentFeature {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the feature that enabled for fulfillment."]
    pub _type: ::std::option::Option<GoogleCloudDialogflowV2beta1FulfillmentFeatureTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the feature that enabled for fulfillment."]
pub enum GoogleCloudDialogflowV2beta1FulfillmentFeatureTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Feature type not specified."]
    TypeUnspecified,
    #[serde(rename = "SMALLTALK")]
    #[doc = "Fulfillment is enabled for SmallTalk."]
    Smalltalk,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents configuration for a generic web service. Dialogflow supports two mechanisms for authentications: - Basic authentication with username and password. - Authentication with additional authentication headers. More information could be found at: https://cloud.google.com/dialogflow/docs/fulfillment-configure."]
pub struct GoogleCloudDialogflowV2beta1FulfillmentGenericWebService {
    #[serde(rename = "isCloudFunction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if generic web service is created through Cloud Functions integration. Defaults to false."]
    pub is_cloud_function: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The password for HTTP Basic authentication."]
    pub password: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestHeaders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HTTP request headers to send together with fulfillment requests."]
    pub request_headers:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The fulfillment URI for receiving POST requests. It must use https protocol."]
    pub uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user name for HTTP Basic authentication."]
    pub username: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Google Cloud Storage location for single input."]
pub struct GoogleCloudDialogflowV2beta1GcsSource {
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The Google Cloud Storage URIs for the inputs. A URI is of the form: gs://bucket/object-prefix-or-name Whether a prefix or name is used depends on the use case."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for Agents.ImportAgent."]
pub struct GoogleCloudDialogflowV2beta1ImportAgentRequest {
    #[serde(rename = "agentContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Zip compressed raw byte content for agent."]
    pub agent_content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "agentUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI to a Google Cloud Storage file containing the agent to import. Note: The URI must start with \"gs://\"."]
    pub agent_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Instructs the speech recognizer on how to process the audio content."]
pub struct GoogleCloudDialogflowV2beta1InputAudioConfig {
    #[serde(rename = "audioEncoding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Audio encoding of the audio content to process."]
    pub audio_encoding:
        ::std::option::Option<GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum>,
    #[serde(rename = "enableWordInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If `true`, Dialogflow returns SpeechWordInfo in StreamingRecognitionResult with information about the recognized speech words, e.g. start and end time offsets. If false or unspecified, Speech doesn't return any word-level information."]
    pub enable_word_info: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The language of the supplied audio. Dialogflow does not do translations. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. Note that queries in the same session do not necessarily need to specify the same language."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Which Speech model to select for the given request. Select the model best suited to your domain to get best results. If a model is not explicitly specified, then we auto-select a model based on the parameters in the InputAudioConfig. If enhanced speech model is enabled for the agent and an enhanced version of the specified model for the language does not exist, then the speech is recognized using the standard version of the specified model. Refer to [Cloud Speech API documentation](https://cloud.google.com/speech-to-text/docs/basics#select-model) for more details."]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(rename = "modelVariant")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Which variant of the Speech model to use."]
    pub model_variant:
        ::std::option::Option<GoogleCloudDialogflowV2beta1InputAudioConfigModelVariantEnum>,
    #[serde(rename = "phraseHints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of strings containing words and phrases that the speech recognizer should recognize with higher likelihood. See [the Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/basics#phrase-hints) for more details. This field is deprecated. Please use [speech_contexts]() instead. If you specify both [phrase_hints]() and [speech_contexts](), Dialogflow will treat the [phrase_hints]() as a single additional [SpeechContext]()."]
    pub phrase_hints: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "sampleRateHertz")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Sample rate (in Hertz) of the audio content sent in the query. Refer to [Cloud Speech API documentation](https://cloud.google.com/speech-to-text/docs/basics) for more details."]
    pub sample_rate_hertz: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "singleUtterance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If `false` (default), recognition does not cease until the client closes the stream. If `true`, the recognizer will detect a single spoken utterance in input audio. Recognition ceases when it detects the audio's voice has stopped or paused. In this case, once a detected intent is received, the client should close the stream and start a new request with a new stream as needed. Note: This setting is relevant only for streaming methods. Note: When specified, InputAudioConfig.single_utterance takes precedence over StreamingDetectIntentRequest.single_utterance."]
    pub single_utterance: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "speechContexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Context information to assist speech recognition. See [the Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/basics#phrase-hints) for more details."]
    pub speech_contexts: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1SpeechContext>>,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Audio encoding of the audio content to process."]
pub enum GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum {
    #[serde(rename = "AUDIO_ENCODING_UNSPECIFIED")]
    #[doc = "Not specified."]
    AudioEncodingUnspecified,
    #[serde(rename = "AUDIO_ENCODING_LINEAR_16")]
    #[doc = "Uncompressed 16-bit signed little-endian samples (Linear PCM)."]
    AudioEncodingLinear16,
    #[serde(rename = "AUDIO_ENCODING_FLAC")]
    #[doc = "[`FLAC`](https://xiph.org/flac/documentation.html) (Free Lossless Audio Codec) is the recommended encoding because it is lossless (therefore recognition is not compromised) and requires only about half the bandwidth of `LINEAR16`. `FLAC` stream encoding supports 16-bit and 24-bit samples, however, not all fields in `STREAMINFO` are supported."]
    AudioEncodingFlac,
    #[serde(rename = "AUDIO_ENCODING_MULAW")]
    #[doc = "8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law."]
    AudioEncodingMulaw,
    #[serde(rename = "AUDIO_ENCODING_AMR")]
    #[doc = "Adaptive Multi-Rate Narrowband codec. `sample_rate_hertz` must be 8000."]
    AudioEncodingAmr,
    #[serde(rename = "AUDIO_ENCODING_AMR_WB")]
    #[doc = "Adaptive Multi-Rate Wideband codec. `sample_rate_hertz` must be 16000."]
    AudioEncodingAmrWb,
    #[serde(rename = "AUDIO_ENCODING_OGG_OPUS")]
    #[doc = "Opus encoded audio frames in Ogg container ([OggOpus](https://wiki.xiph.org/OggOpus)). `sample_rate_hertz` must be 16000."]
    AudioEncodingOggOpus,
    #[serde(rename = "AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE")]
    #[doc = "Although the use of lossy encodings is not recommended, if a very low bitrate encoding is required, `OGG_OPUS` is highly preferred over Speex encoding. The [Speex](https://speex.org/) encoding supported by Dialogflow API has a header byte in each block, as in MIME type `audio/x-speex-with-header-byte`. It is a variant of the RTP Speex encoding defined in [RFC 5574](https://tools.ietf.org/html/rfc5574). The stream is a sequence of blocks, one block per RTP packet. Each block starts with a byte containing the length of the block, in bytes, followed by one or more frames of Speex data, padded to an integral number of bytes (octets) as specified in RFC 5574. In other words, each RTP header is replaced with a single byte containing the block length. Only Speex wideband is supported. `sample_rate_hertz` must be 16000."]
    AudioEncodingSpeexWithHeaderByte,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Which variant of the Speech model to use."]
pub enum GoogleCloudDialogflowV2beta1InputAudioConfigModelVariantEnum {
    #[serde(rename = "SPEECH_MODEL_VARIANT_UNSPECIFIED")]
    #[doc = "No model variant specified. In this case Dialogflow defaults to USE_BEST_AVAILABLE."]
    SpeechModelVariantUnspecified,
    #[serde(rename = "USE_BEST_AVAILABLE")]
    #[doc = "Use the best available variant of the Speech model that the caller is eligible for. Please see the [Dialogflow docs](https://cloud.google.com/dialogflow/docs/data-logging) for how to make your project eligible for enhanced models."]
    UseBestAvailable,
    #[serde(rename = "USE_STANDARD")]
    #[doc = "Use standard model variant even if an enhanced model is available. See the [Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models) for details about enhanced models."]
    UseStandard,
    #[serde(rename = "USE_ENHANCED")]
    #[doc = "Use an enhanced model variant: * If an enhanced variant does not exist for the given model and request language, Dialogflow falls back to the standard variant. The [Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models) describes which models have enhanced variants. * If the API caller isn't eligible for enhanced models, Dialogflow returns an error. Please see the [Dialogflow docs](https://cloud.google.com/dialogflow/docs/data-logging) for how to make your project eligible."]
    UseEnhanced,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An intent categorizes an end-user's intention for one conversation turn. For each agent, you define many intents, where your combined intents can handle a complete conversation. When an end-user writes or says something, referred to as an end-user expression or end-user input, Dialogflow matches the end-user input to the best intent in your agent. Matching an intent is also known as intent classification. For more information, see the [intent guide](https://cloud.google.com/dialogflow/docs/intents-overview)."]
pub struct GoogleCloudDialogflowV2beta1Intent {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The name of the action associated with the intent. Note: The action name must not contain whitespaces."]
    pub action: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultResponsePlatforms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform)."]
    pub default_response_platforms: ::std::option::Option<
        ::std::vec::Vec<GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum>,
    >,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of this intent."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endInteraction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates that this intent ends an interaction. Some integrations (e.g., Actions on Google or Dialogflow phone gateway) use this information to close interaction with an end user. Default is false."]
    pub end_interaction: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of event names that trigger the intent. If the collection of input contexts is not empty, all of the contexts must be present in the active user session for an event to trigger this intent. Event names are limited to 150 characters."]
    pub events: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "followupIntentInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Information about all followup intents that have this intent as a direct or indirect parent. We populate this field only in the output."]
    pub followup_intent_info: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentFollowupIntentInfo>>,
    >,
    #[serde(rename = "inputContextNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The list of context names required for this intent to be triggered. Formats: - `projects//agent/sessions/-/contexts/` - `projects//locations//agent/sessions/-/contexts/`"]
    pub input_context_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "isFallback")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether this is a fallback intent."]
    pub is_fallback: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of rich messages corresponding to the `Response` field in the Dialogflow console."]
    pub messages: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessage>>,
    >,
    #[serde(rename = "mlDisabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether Machine Learning is disabled for the intent. Note: If `ml_disabled` setting is set to true, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off."]
    pub ml_disabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "mlEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether Machine Learning is enabled for the intent. Note: If `ml_enabled` setting is set to false, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off. DEPRECATED! Please use `ml_disabled` field instead. NOTE: If both `ml_enabled` and `ml_disabled` are either not set or false, then the default value is determined as follows: - Before April 15th, 2018 the default is: ml_enabled = false / ml_disabled = true. - After April 15th, 2018 the default is: ml_enabled = true / ml_disabled = false."]
    pub ml_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The unique identifier of this intent. Required for Intents.UpdateIntent and Intents.BatchUpdateIntents methods. Supported formats: - `projects//agent/intents/` - `projects//locations//agent/intents/`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outputContexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of contexts that are activated when the intent is matched. Context messages in this collection should not set the parameters field. Setting the `lifespan_count` to 0 will reset the context when the intent is matched. Format: `projects//agent/sessions/-/contexts/`."]
    pub output_contexts: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1Context>>,
    >,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of parameters associated with the intent."]
    pub parameters: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentParameter>>,
    >,
    #[serde(rename = "parentFollowupIntentName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The unique identifier of the parent intent in the chain of followup intents. You can set this field when creating an intent, for example with CreateIntent or BatchUpdateIntents, in order to make this intent a followup intent. It identifies the parent followup intent. Format: `projects//agent/intents/`."]
    pub parent_followup_intent_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests."]
    pub priority: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "resetContexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether to delete all contexts in the current session when this intent is matched."]
    pub reset_contexts: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "rootFollowupIntentName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique identifier of the root intent in the chain of followup intents. It identifies the correct followup intents chain for this intent. Format: `projects//agent/intents/`."]
    pub root_followup_intent_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trainingPhrases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of examples that the agent is trained on."]
    pub training_phrases: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentTrainingPhrase>>,
    >,
    #[serde(rename = "webhookState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether webhooks are enabled for the intent."]
    pub webhook_state: ::std::option::Option<GoogleCloudDialogflowV2beta1IntentWebhookStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum {
    #[serde(rename = "PLATFORM_UNSPECIFIED")]
    #[doc = "Not specified."]
    PlatformUnspecified,
    #[serde(rename = "FACEBOOK")]
    #[doc = "Facebook."]
    Facebook,
    #[serde(rename = "SLACK")]
    #[doc = "Slack."]
    Slack,
    #[serde(rename = "TELEGRAM")]
    #[doc = "Telegram."]
    Telegram,
    #[serde(rename = "KIK")]
    #[doc = "Kik."]
    Kik,
    #[serde(rename = "SKYPE")]
    #[doc = "Skype."]
    Skype,
    #[serde(rename = "LINE")]
    #[doc = "Line."]
    Line,
    #[serde(rename = "VIBER")]
    #[doc = "Viber."]
    Viber,
    #[serde(rename = "ACTIONS_ON_GOOGLE")]
    #[doc = "Google Assistant See [Dialogflow webhook format](https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json)"]
    ActionsOnGoogle,
    #[serde(rename = "TELEPHONY")]
    #[doc = "Telephony Gateway."]
    Telephony,
    #[serde(rename = "GOOGLE_HANGOUTS")]
    #[doc = "Google Hangouts."]
    GoogleHangouts,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Indicates whether webhooks are enabled for the intent."]
pub enum GoogleCloudDialogflowV2beta1IntentWebhookStateEnum {
    #[serde(rename = "WEBHOOK_STATE_UNSPECIFIED")]
    #[doc = "Webhook is disabled in the agent and in the intent."]
    WebhookStateUnspecified,
    #[serde(rename = "WEBHOOK_STATE_ENABLED")]
    #[doc = "Webhook is enabled in the agent and in the intent."]
    WebhookStateEnabled,
    #[serde(rename = "WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING")]
    #[doc = "Webhook is enabled in the agent and in the intent. Also, each slot filling prompt is forwarded to the webhook."]
    WebhookStateEnabledForSlotFilling,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This message is a wrapper around a collection of intents."]
pub struct GoogleCloudDialogflowV2beta1IntentBatch {
    #[serde(rename = "intents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A collection of intents."]
    pub intents: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1Intent>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a single followup intent in the chain."]
pub struct GoogleCloudDialogflowV2beta1IntentFollowupIntentInfo {
    #[serde(rename = "followupIntentName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the followup intent. Format: `projects//agent/intents/`."]
    pub followup_intent_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parentFollowupIntentName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the followup intent's parent. Format: `projects//agent/intents/`."]
    pub parent_followup_intent_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Corresponds to the `Response` field in the Dialogflow console."]
pub struct GoogleCloudDialogflowV2beta1IntentMessage {
    #[serde(rename = "basicCard")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Displays a basic card for Actions on Google."]
    pub basic_card: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageBasicCard>,
    >,
    #[serde(rename = "browseCarouselCard")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Browse carousel card for Actions on Google."]
    pub browse_carousel_card: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCard>,
    >,
    #[serde(rename = "card")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Displays a card."]
    pub card:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageCard>>,
    #[serde(rename = "carouselSelect")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Displays a carousel card for Actions on Google."]
    pub carousel_select: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageCarouselSelect>,
    >,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Displays an image."]
    pub image:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageImage>>,
    #[serde(rename = "linkOutSuggestion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Displays a link out suggestion chip for Actions on Google."]
    pub link_out_suggestion: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageLinkOutSuggestion>,
    >,
    #[serde(rename = "listSelect")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Displays a list card for Actions on Google."]
    pub list_select: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageListSelect>,
    >,
    #[serde(rename = "mediaContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The media content card for Actions on Google."]
    pub media_content: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageMediaContent>,
    >,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A custom platform-specific response."]
    pub payload: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The platform that this message is intended for."]
    pub platform: ::std::option::Option<GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum>,
    #[serde(rename = "quickReplies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Displays quick replies."]
    pub quick_replies: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageQuickReplies>,
    >,
    #[serde(rename = "rbmCarouselRichCard")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rich Business Messaging (RBM) carousel rich card response."]
    pub rbm_carousel_rich_card: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCard>,
    >,
    #[serde(rename = "rbmStandaloneRichCard")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Standalone Rich Business Messaging (RBM) rich card response."]
    pub rbm_standalone_rich_card: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCard>,
    >,
    #[serde(rename = "rbmText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rich Business Messaging (RBM) text response. RBM allows businesses to send enriched and branded versions of SMS. See https://jibe.google.com/business-messaging."]
    pub rbm_text:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmText>>,
    #[serde(rename = "simpleResponses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Returns a voice or text-only response for Actions on Google."]
    pub simple_responses: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageSimpleResponses>,
    >,
    #[serde(rename = "suggestions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Displays suggestion chips for Actions on Google."]
    pub suggestions: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageSuggestions>,
    >,
    #[serde(rename = "tableCard")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Table card for Actions on Google."]
    pub table_card: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageTableCard>,
    >,
    #[serde(rename = "telephonyPlayAudio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Plays audio from a file in Telephony Gateway."]
    pub telephony_play_audio: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageTelephonyPlayAudio>,
    >,
    #[serde(rename = "telephonySynthesizeSpeech")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Synthesizes speech in Telephony Gateway."]
    pub telephony_synthesize_speech: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageTelephonySynthesizeSpeech>,
    >,
    #[serde(rename = "telephonyTransferCall")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Transfers the call in Telephony Gateway."]
    pub telephony_transfer_call: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageTelephonyTransferCall>,
    >,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Returns a text response."]
    pub text:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageText>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. The platform that this message is intended for."]
pub enum GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum {
    #[serde(rename = "PLATFORM_UNSPECIFIED")]
    #[doc = "Not specified."]
    PlatformUnspecified,
    #[serde(rename = "FACEBOOK")]
    #[doc = "Facebook."]
    Facebook,
    #[serde(rename = "SLACK")]
    #[doc = "Slack."]
    Slack,
    #[serde(rename = "TELEGRAM")]
    #[doc = "Telegram."]
    Telegram,
    #[serde(rename = "KIK")]
    #[doc = "Kik."]
    Kik,
    #[serde(rename = "SKYPE")]
    #[doc = "Skype."]
    Skype,
    #[serde(rename = "LINE")]
    #[doc = "Line."]
    Line,
    #[serde(rename = "VIBER")]
    #[doc = "Viber."]
    Viber,
    #[serde(rename = "ACTIONS_ON_GOOGLE")]
    #[doc = "Google Assistant See [Dialogflow webhook format](https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json)"]
    ActionsOnGoogle,
    #[serde(rename = "TELEPHONY")]
    #[doc = "Telephony Gateway."]
    Telephony,
    #[serde(rename = "GOOGLE_HANGOUTS")]
    #[doc = "Google Hangouts."]
    GoogleHangouts,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The basic card message. Useful for displaying information."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageBasicCard {
    #[serde(rename = "buttons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of card buttons."]
    pub buttons: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageBasicCardButton>,
        >,
    >,
    #[serde(rename = "formattedText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required, unless image is present. The body text of the card."]
    pub formatted_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The image for the card."]
    pub image:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageImage>>,
    #[serde(rename = "subtitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The subtitle of the card."]
    pub subtitle: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The title of the card."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The button object that appears at the bottom of a card."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageBasicCardButton {
    #[serde(rename = "openUriAction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Action to take when a user taps on the button."]
    pub open_uri_action: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageBasicCardButtonOpenUriAction>,
    >,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The title of the button."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Opens the given URI."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageBasicCardButtonOpenUriAction {
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The HTTP or HTTPS scheme URI."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Browse Carousel Card for Actions on Google. https://developers.google.com/actions/assistant/responses#browsing_carousel"]
pub struct GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCard {
    #[serde(rename = "imageDisplayOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Settings for displaying the image. Applies to every image in items."]
    pub image_display_options: ::std::option::Option<
        GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum,
    >,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. List of items in the Browse Carousel Card. Minimum of two items, maximum of ten."]
    pub items: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<
                GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItem,
            >,
        >,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Settings for displaying the image. Applies to every image in items."]
pub enum GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum {
    #[serde(rename = "IMAGE_DISPLAY_OPTIONS_UNSPECIFIED")]
    #[doc = "Fill the gaps between the image and the image container with gray bars."]
    ImageDisplayOptionsUnspecified,
    #[serde(rename = "GRAY")]
    #[doc = "Fill the gaps between the image and the image container with gray bars."]
    Gray,
    #[serde(rename = "WHITE")]
    #[doc = "Fill the gaps between the image and the image container with white bars."]
    White,
    #[serde(rename = "CROPPED")]
    #[doc = "Image is scaled such that the image width and height match or exceed the container dimensions. This may crop the top and bottom of the image if the scaled image height is greater than the container height, or crop the left and right of the image if the scaled image width is greater than the container width. This is similar to \"Zoom Mode\" on a widescreen TV when playing a 4:3 video."]
    Cropped,
    #[serde(rename = "BLURRED_BACKGROUND")]
    #[doc = "Pad the gaps between image and image frame with a blurred copy of the same image."]
    BlurredBackground,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Browsing carousel tile"]
pub struct GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItem { # [serde (rename = "description")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Optional. Description of the carousel item. Maximum of four lines of text."] pub description : :: std :: option :: Option < :: std :: string :: String > , # [serde (rename = "footer")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Optional. Text that appears at the bottom of the Browse Carousel Card. Maximum of one line of text."] pub footer : :: std :: option :: Option < :: std :: string :: String > , # [serde (rename = "image")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Optional. Hero image for the carousel item."] pub image : :: std :: option :: Option < :: std :: boxed :: Box < GoogleCloudDialogflowV2beta1IntentMessageImage > > , # [serde (rename = "openUriAction")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Required. Action to present to the user."] pub open_uri_action : :: std :: option :: Option < :: std :: boxed :: Box < GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlAction > > , # [serde (rename = "title")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Required. Title of the carousel item. Maximum of two lines of text."] pub title : :: std :: option :: Option < :: std :: string :: String > }
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Actions on Google action to open a given url."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlAction { # [serde (rename = "url")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Required. URL"] pub url : :: std :: option :: Option < :: std :: string :: String > , # [serde (rename = "urlTypeHint")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Optional. Specifies the type of viewer that is used when opening the URL. Defaults to opening via web browser."] pub url_type_hint : :: std :: option :: Option < GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum > }
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Specifies the type of viewer that is used when opening the URL. Defaults to opening via web browser."]
pub enum GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum
{
    #[serde(rename = "URL_TYPE_HINT_UNSPECIFIED")]
    #[doc = "Unspecified"]
    UrlTypeHintUnspecified,
    #[serde(rename = "AMP_ACTION")]
    #[doc = "Url would be an amp action"]
    AmpAction,
    #[serde(rename = "AMP_CONTENT")]
    #[doc = "URL that points directly to AMP content, or to a canonical URL which refers to AMP content via ."]
    AmpContent,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The card response message."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageCard {
    #[serde(rename = "buttons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of card buttons."]
    pub buttons: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageCardButton>>,
    >,
    #[serde(rename = "imageUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The public URI to an image file for the card."]
    pub image_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subtitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The subtitle of the card."]
    pub subtitle: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The title of the card."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Optional. Contains information about a button."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageCardButton {
    #[serde(rename = "postback")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The text to send back to the Dialogflow API or a URI to open."]
    pub postback: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The text to show on the button."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The card for presenting a carousel of options to select from."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageCarouselSelect {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Carousel items."]
    pub items: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageCarouselSelectItem>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An item in the carousel."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageCarouselSelectItem {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The body text of the card."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The image to display."]
    pub image:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageImage>>,
    #[serde(rename = "info")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Additional info about the option item."]
    pub info: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageSelectItemInfo>,
    >,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Title of the carousel item."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Column properties for TableCard."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageColumnProperties {
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Column heading."]
    pub header: ::std::option::Option<::std::string::String>,
    #[serde(rename = "horizontalAlignment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Defines text alignment for all cells in this column."]
    pub horizontal_alignment: ::std::option::Option<
        GoogleCloudDialogflowV2beta1IntentMessageColumnPropertiesHorizontalAlignmentEnum,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Defines text alignment for all cells in this column."]
pub enum GoogleCloudDialogflowV2beta1IntentMessageColumnPropertiesHorizontalAlignmentEnum {
    #[serde(rename = "HORIZONTAL_ALIGNMENT_UNSPECIFIED")]
    #[doc = "Text is aligned to the leading edge of the column."]
    HorizontalAlignmentUnspecified,
    #[serde(rename = "LEADING")]
    #[doc = "Text is aligned to the leading edge of the column."]
    Leading,
    #[serde(rename = "CENTER")]
    #[doc = "Text is centered in the column."]
    Center,
    #[serde(rename = "TRAILING")]
    #[doc = "Text is aligned to the trailing edge of the column."]
    Trailing,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The image response message."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageImage {
    #[serde(rename = "accessibilityText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A text description of the image to be used for accessibility, e.g., screen readers. Required if image_uri is set for CarouselSelect."]
    pub accessibility_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imageUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The public URI to an image file."]
    pub image_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The suggestion chip message that allows the user to jump out to the app or website associated with this agent."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageLinkOutSuggestion {
    #[serde(rename = "destinationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the app or site this chip is linking to."]
    pub destination_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The URI of the app or site to open when the user taps the suggestion chip."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The card for presenting a list of options to select from."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageListSelect {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. List items."]
    pub items: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageListSelectItem>>,
    >,
    #[serde(rename = "subtitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Subtitle of the list."]
    pub subtitle: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The overall title of the list."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An item in the list."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageListSelectItem {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The main text describing the item."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The image to display."]
    pub image:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageImage>>,
    #[serde(rename = "info")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Additional information about this option."]
    pub info: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageSelectItemInfo>,
    >,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The title of the list item."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The media content card for Actions on Google."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageMediaContent {
    #[serde(rename = "mediaObjects")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. List of media objects."]
    pub media_objects: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<
                GoogleCloudDialogflowV2beta1IntentMessageMediaContentResponseMediaObject,
            >,
        >,
    >,
    #[serde(rename = "mediaType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. What type of media is the content (ie \"audio\")."]
    pub media_type:
        ::std::option::Option<GoogleCloudDialogflowV2beta1IntentMessageMediaContentMediaTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. What type of media is the content (ie \"audio\")."]
pub enum GoogleCloudDialogflowV2beta1IntentMessageMediaContentMediaTypeEnum {
    #[serde(rename = "RESPONSE_MEDIA_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    ResponseMediaTypeUnspecified,
    #[serde(rename = "AUDIO")]
    #[doc = "Response media type is audio."]
    Audio,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response media object for media content card."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageMediaContentResponseMediaObject {
    #[serde(rename = "contentUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Url where the media is stored."]
    pub content_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Description of media card."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "icon")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Icon to display above media content."]
    pub icon:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageImage>>,
    #[serde(rename = "largeImage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Image to display above media content."]
    pub large_image:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageImage>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Name of media card."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The quick replies response message."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageQuickReplies {
    #[serde(rename = "quickReplies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of quick replies."]
    pub quick_replies: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The title of the collection of quick replies."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Rich Business Messaging (RBM) Card content"]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmCardContent {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Description of the card (at most 2000 bytes). At least one of the title, description or media must be set."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "media")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. However at least one of the title, description or media must be set. Media (image, GIF or a video) to include in the card."]
    pub media: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMedia>,
    >,
    #[serde(rename = "suggestions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. List of suggestions to include in the card."]
    pub suggestions: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestion>>,
    >,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Title of the card (at most 200 bytes). At least one of the title, description or media must be set."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Rich Business Messaging (RBM) Media displayed in Cards The following media-types are currently supported: Image Types * image/jpeg * image/jpg' * image/gif * image/png Video Types * video/h263 * video/m4v * video/mp4 * video/mpeg * video/mpeg4 * video/webm"]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMedia {
    #[serde(rename = "fileUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Publicly reachable URI of the file. The RBM platform determines the MIME type of the file from the content-type field in the HTTP headers when the platform fetches the file. The content-type field must be present and accurate in the HTTP response from the URL."]
    pub file_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for cards with vertical orientation. The height of the media within a rich card with a vertical layout. For a standalone card with horizontal layout, height is not customizable, and this field is ignored."]
    pub height: ::std::option::Option<
        GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum,
    >,
    #[serde(rename = "thumbnailUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Publicly reachable URI of the thumbnail.If you don't provide a thumbnail URI, the RBM platform displays a blank placeholder thumbnail until the user's device downloads the file. Depending on the user's setting, the file may not download automatically and may require the user to tap a download button."]
    pub thumbnail_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required for cards with vertical orientation. The height of the media within a rich card with a vertical layout. For a standalone card with horizontal layout, height is not customizable, and this field is ignored."]
pub enum GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum {
    #[serde(rename = "HEIGHT_UNSPECIFIED")]
    #[doc = "Not specified."]
    HeightUnspecified,
    #[serde(rename = "SHORT")]
    #[doc = "112 DP."]
    Short,
    #[serde(rename = "MEDIUM")]
    #[doc = "168 DP."]
    Medium,
    #[serde(rename = "TALL")]
    #[doc = "264 DP. Not available for rich card carousels when the card width is set to small."]
    Tall,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Carousel Rich Business Messaging (RBM) rich card. Rich cards allow you to respond to users with more vivid content, e.g. with media and suggestions. If you want to show a single card with more control over the layout, please use RbmStandaloneCard instead."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCard {
    #[serde(rename = "cardContents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The cards in the carousel. A carousel must have at least 2 cards and at most 10."]
    pub card_contents: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmCardContent>>,
    >,
    #[serde(rename = "cardWidth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The width of the cards in the carousel."]
    pub card_width: ::std::option::Option<
        GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardCardWidthEnum,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The width of the cards in the carousel."]
pub enum GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardCardWidthEnum {
    #[serde(rename = "CARD_WIDTH_UNSPECIFIED")]
    #[doc = "Not specified."]
    CardWidthUnspecified,
    #[serde(rename = "SMALL")]
    #[doc = "120 DP. Note that tall media cannot be used."]
    Small,
    #[serde(rename = "MEDIUM")]
    #[doc = "232 DP."]
    Medium,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Standalone Rich Business Messaging (RBM) rich card. Rich cards allow you to respond to users with more vivid content, e.g. with media and suggestions. You can group multiple rich cards into one using RbmCarouselCard but carousel cards will give you less control over the card layout."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCard {
    #[serde(rename = "cardContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Card content."]
    pub card_content: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmCardContent>,
    >,
    #[serde(rename = "cardOrientation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Orientation of the card."]
    pub card_orientation: ::std::option::Option<
        GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardCardOrientationEnum,
    >,
    #[serde(rename = "thumbnailImageAlignment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required if orientation is horizontal. Image preview alignment for standalone cards with horizontal layout."]
    pub thumbnail_image_alignment: ::std::option::Option<
        GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardThumbnailImageAlignmentEnum,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Orientation of the card."]
pub enum GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardCardOrientationEnum {
    #[serde(rename = "CARD_ORIENTATION_UNSPECIFIED")]
    #[doc = "Not specified."]
    CardOrientationUnspecified,
    #[serde(rename = "HORIZONTAL")]
    #[doc = "Horizontal layout."]
    Horizontal,
    #[serde(rename = "VERTICAL")]
    #[doc = "Vertical layout."]
    Vertical,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required if orientation is horizontal. Image preview alignment for standalone cards with horizontal layout."]
pub enum GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardThumbnailImageAlignmentEnum {
    #[serde(rename = "THUMBNAIL_IMAGE_ALIGNMENT_UNSPECIFIED")]
    #[doc = "Not specified."]
    ThumbnailImageAlignmentUnspecified,
    #[serde(rename = "LEFT")]
    #[doc = "Thumbnail preview is left-aligned."]
    Left,
    #[serde(rename = "RIGHT")]
    #[doc = "Thumbnail preview is right-aligned."]
    Right,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Rich Business Messaging (RBM) suggested client-side action that the user can choose from the card."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedAction { # [serde (rename = "dial")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Suggested client side action: Dial a phone number"] pub dial : :: std :: option :: Option < :: std :: boxed :: Box < GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionDial > > , # [serde (rename = "openUrl")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Suggested client side action: Open a URI on device"] pub open_url : :: std :: option :: Option < :: std :: boxed :: Box < GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionOpenUri > > , # [serde (rename = "postbackData")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Opaque payload that the Dialogflow receives in a user event when the user taps the suggested action. This data will be also forwarded to webhook to allow performing custom business logic."] pub postback_data : :: std :: option :: Option < :: std :: string :: String > , # [serde (rename = "shareLocation")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Suggested client side action: Share user location"] pub share_location : :: std :: option :: Option < :: std :: boxed :: Box < GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionShareLocation > > , # [serde (rename = "text")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Text to display alongside the action."] pub text : :: std :: option :: Option < :: std :: string :: String > }
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Opens the user's default dialer app with the specified phone number but does not dial automatically."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionDial {
    #[serde(rename = "phoneNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The phone number to fill in the default dialer app. This field should be in [E.164](https://en.wikipedia.org/wiki/E.164) format. An example of a correctly formatted phone number: +15556767888."]
    pub phone_number: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Opens the user's default web browser app to the specified uri If the user has an app installed that is registered as the default handler for the URL, then this app will be opened instead, and its icon will be used in the suggested action UI."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionOpenUri {
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The uri to open on the user device"]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Opens the device's location chooser so the user can pick a location to send back to the agent."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionShareLocation
{}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Rich Business Messaging (RBM) suggested reply that the user can click instead of typing in their own response."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedReply {
    #[serde(rename = "postbackData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Opaque payload that the Dialogflow receives in a user event when the user taps the suggested reply. This data will be also forwarded to webhook to allow performing custom business logic."]
    pub postback_data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Suggested reply text."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Rich Business Messaging (RBM) suggestion. Suggestions allow user to easily select/click a predefined response or perform an action (like opening a web uri)."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestion {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Predefined client side actions that user can choose"]
    pub action: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedAction>,
    >,
    #[serde(rename = "reply")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Predefined replies for user to select instead of typing"]
    pub reply: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedReply>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Rich Business Messaging (RBM) text response with suggestions."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmText {
    #[serde(rename = "rbmSuggestion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. One or more suggestions to show to the user."]
    pub rbm_suggestion: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestion>>,
    >,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Text sent and displayed to the user."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional info about the select item for when it is triggered in a dialog."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageSelectItemInfo {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A unique key that will be sent back to the agent if this response is given."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "synonyms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A list of synonyms that can also be used to trigger this item in dialog."]
    pub synonyms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The simple response message containing speech or text."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageSimpleResponse {
    #[serde(rename = "displayText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The text to display."]
    pub display_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ssml")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "One of text_to_speech or ssml must be provided. Structured spoken response to the user in the SSML format. Mutually exclusive with text_to_speech."]
    pub ssml: ::std::option::Option<::std::string::String>,
    #[serde(rename = "textToSpeech")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "One of text_to_speech or ssml must be provided. The plain text of the speech output. Mutually exclusive with ssml."]
    pub text_to_speech: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The collection of simple response candidates. This message in `QueryResult.fulfillment_messages` and `WebhookResponse.fulfillment_messages` should contain only one `SimpleResponse`."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageSimpleResponses {
    #[serde(rename = "simpleResponses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The list of simple responses."]
    pub simple_responses: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageSimpleResponse>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The suggestion chip message that the user can tap to quickly post a reply to the conversation."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageSuggestion {
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The text shown the in the suggestion chip."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The collection of suggestions."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageSuggestions {
    #[serde(rename = "suggestions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The list of suggested replies."]
    pub suggestions: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageSuggestion>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Table card for Actions on Google."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageTableCard {
    #[serde(rename = "buttons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. List of buttons for the card."]
    pub buttons: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageBasicCardButton>,
        >,
    >,
    #[serde(rename = "columnProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Display properties for the columns in this table."]
    pub column_properties: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageColumnProperties>,
        >,
    >,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Image which should be displayed on the card."]
    pub image:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageImage>>,
    #[serde(rename = "rows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Rows in this table of data."]
    pub rows: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageTableCardRow>>,
    >,
    #[serde(rename = "subtitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Subtitle to the title."]
    pub subtitle: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Title of the card."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cell of TableCardRow."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageTableCardCell {
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Text in this cell."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Row of TableCard."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageTableCardRow {
    #[serde(rename = "cells")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. List of cells that make up this row."]
    pub cells: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageTableCardCell>>,
    >,
    #[serde(rename = "dividerAfter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Whether to add a visual divider after this row."]
    pub divider_after: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Plays audio from a file in Telephony Gateway."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageTelephonyPlayAudio {
    #[serde(rename = "audioUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. URI to a Google Cloud Storage object containing the audio to play, e.g., \"gs://bucket/object\". The object must contain a single channel (mono) of linear PCM audio (2 bytes / sample) at 8kHz. This object must be readable by the `service-@gcp-sa-dialogflow.iam.gserviceaccount.com` service account where is the number of the Telephony Gateway project (usually the same as the Dialogflow agent project). If the Google Cloud Storage bucket is in the Telephony Gateway project, this permission is added by default when enabling the Dialogflow V2 API. For audio from other sources, consider using the `TelephonySynthesizeSpeech` message with SSML."]
    pub audio_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Synthesizes speech and plays back the synthesized audio to the caller in Telephony Gateway. Telephony Gateway takes the synthesizer settings from `DetectIntentResponse.output_audio_config` which can either be set at request-level or can come from the agent-level synthesizer config."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageTelephonySynthesizeSpeech {
    #[serde(rename = "ssml")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The SSML to be synthesized. For more information, see [SSML](https://developers.google.com/actions/reference/ssml)."]
    pub ssml: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The raw text to be synthesized."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Transfers the call in Telephony Gateway."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageTelephonyTransferCall {
    #[serde(rename = "phoneNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The phone number to transfer the call to in [E.164 format](https://en.wikipedia.org/wiki/E.164). We currently only allow transferring to US numbers (+1xxxyyyzzzz)."]
    pub phone_number: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The text response message."]
pub struct GoogleCloudDialogflowV2beta1IntentMessageText {
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of the agent's responses."]
    pub text: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents intent parameters."]
pub struct GoogleCloudDialogflowV2beta1IntentParameter {
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The default value to use when the `value` yields an empty result. Default values can be extracted from contexts by using the following syntax: `#context_name.parameter_name`."]
    pub default_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the parameter."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityTypeDisplayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The name of the entity type, prefixed with `@`, that describes values of the parameter. If the parameter is required, this must be provided."]
    pub entity_type_display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether the parameter represents a list of values."]
    pub is_list: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "mandatory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether the parameter is required. That is, whether the intent cannot be completed without collecting the parameter value."]
    pub mandatory: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of this parameter."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "prompts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of prompts that the agent can present to the user in order to collect a value for the parameter."]
    pub prompts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The definition of the parameter value. It can be: - a constant string, - a parameter value defined as `$parameter_name`, - an original parameter value defined as `$parameter_name.original`, - a parameter value from some context defined as `#context_name.parameter_name`."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an example that the agent is trained on."]
pub struct GoogleCloudDialogflowV2beta1IntentTrainingPhrase {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique identifier of this training phrase."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ordered list of training phrase parts. The parts are concatenated in order to form the training phrase. Note: The API does not automatically annotate training phrases like the Dialogflow Console does. Note: Do not forget to include whitespace at part boundaries, so the training phrase is well formatted when the parts are concatenated. If the training phrase does not need to be annotated with parameters, you just need a single part with only the Part.text field set. If you want to annotate the training phrase, you must create multiple parts, where the fields of each part are populated in one of two ways: - `Part.text` is set to a part of the phrase that has no parameters. - `Part.text` is set to a part of the phrase that you want to annotate, and the `entity_type`, `alias`, and `user_defined` fields are all set."]
    pub parts: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentTrainingPhrasePart>>,
    >,
    #[serde(rename = "timesAddedCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates how many times this example was added to the intent. Each time a developer adds an existing sample by editing an intent or training, this counter is increased."]
    pub times_added_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of the training phrase."]
    pub _type: ::std::option::Option<GoogleCloudDialogflowV2beta1IntentTrainingPhraseTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of the training phrase."]
pub enum GoogleCloudDialogflowV2beta1IntentTrainingPhraseTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Not specified. This value should never be used."]
    TypeUnspecified,
    #[serde(rename = "EXAMPLE")]
    #[doc = "Examples do not contain @-prefixed entity type names, but example parts can be annotated with entity types."]
    Example,
    #[serde(rename = "TEMPLATE")]
    #[doc = "Templates are not annotated with entity types, but they can contain @-prefixed entity type names as substrings. Template mode has been deprecated. Example mode is the only supported way to create new training phrases. If you have existing training phrases that you've created in template mode, those will continue to work."]
    Template,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a part of a training phrase."]
pub struct GoogleCloudDialogflowV2beta1IntentTrainingPhrasePart {
    #[serde(rename = "alias")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The parameter name for the value extracted from the annotated part of the example. This field is required for annotated parts of the training phrase."]
    pub alias: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The entity type name prefixed with `@`. This field is required for annotated parts of the training phrase."]
    pub entity_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The text for this part."]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userDefined")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether the text was manually annotated. This field is set to true when the Dialogflow Console is used to manually annotate the part. When creating an annotated part with the API, you must set this to true."]
    pub user_defined: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the result of querying a Knowledge base."]
pub struct GoogleCloudDialogflowV2beta1KnowledgeAnswers {
    #[serde(rename = "answers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of answers from Knowledge Connector."]
    pub answers: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswer>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An answer from Knowledge Connector."]
pub struct GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswer {
    #[serde(rename = "answer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The piece of text from the `source` knowledge base document that answers this conversational query."]
    pub answer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "faqQuestion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The corresponding FAQ question if the answer was extracted from a FAQ Document, empty otherwise."]
    pub faq_question: ::std::option::Option<::std::string::String>,
    #[serde(rename = "matchConfidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The system's confidence score that this Knowledge answer is a good match for this conversational query. The range is from 0.0 (completely uncertain) to 1.0 (completely certain). Note: The confidence score is likely to vary somewhat (possibly even for identical requests), as the underlying model is under constant improvement. It may be deprecated in the future. We recommend using `match_confidence_level` which should be generally more stable."]
    pub match_confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "matchConfidenceLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The system's confidence level that this knowledge answer is a good match for this conversational query. NOTE: The confidence level for a given `` pair may change without notice, as it depends on models that are constantly being improved. However, it will change less frequently than the confidence score below, and should be preferred for referencing the quality of an answer."]
    pub match_confidence_level: ::std::option::Option<
        GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum,
    >,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates which Knowledge Document this answer was extracted from. Format: `projects//knowledgeBases//documents/`."]
    pub source: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The system's confidence level that this knowledge answer is a good match for this conversational query. NOTE: The confidence level for a given `` pair may change without notice, as it depends on models that are constantly being improved. However, it will change less frequently than the confidence score below, and should be preferred for referencing the quality of an answer."]
pub enum GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum {
    #[serde(rename = "MATCH_CONFIDENCE_LEVEL_UNSPECIFIED")]
    #[doc = "Not specified."]
    MatchConfidenceLevelUnspecified,
    #[serde(rename = "LOW")]
    #[doc = "Indicates that the confidence is low."]
    Low,
    #[serde(rename = "MEDIUM")]
    #[doc = "Indicates our confidence is medium."]
    Medium,
    #[serde(rename = "HIGH")]
    #[doc = "Indicates our confidence is high."]
    High,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A knowledge base represents a collection of knowledge documents that you provide to Dialogflow. Your knowledge documents contain information that may be useful during conversations with end-users. Some Dialogflow features use knowledge bases when looking for a response to an end-user input. For more information, see the [knowledge base guide](https://cloud.google.com/dialogflow/docs/how/knowledge-bases). Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`."]
pub struct GoogleCloudDialogflowV2beta1KnowledgeBase {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The display name of the knowledge base. The name must be 1024 bytes or less; otherwise, the creation request fails."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language which represents the KnowledgeBase. When the KnowledgeBase is created/updated, this is populated for all non en-us languages. If not populated, the default language en-us applies."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The knowledge base resource name. The name must be empty when creating a knowledge base. Format: `projects//locations//knowledgeBases/`."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata in google::longrunning::Operation for Knowledge operations."]
pub struct GoogleCloudDialogflowV2beta1KnowledgeOperationMetadata {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Output only. The current state of this operation."]
    pub state:
        ::std::option::Option<GoogleCloudDialogflowV2beta1KnowledgeOperationMetadataStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Output only. The current state of this operation."]
pub enum GoogleCloudDialogflowV2beta1KnowledgeOperationMetadataStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "State unspecified."]
    StateUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "The operation has been created."]
    Pending,
    #[serde(rename = "RUNNING")]
    #[doc = "The operation is currently running."]
    Running,
    #[serde(rename = "DONE")]
    #[doc = "The operation is done, either cancelled or completed."]
    Done,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Contexts.ListContexts."]
pub struct GoogleCloudDialogflowV2beta1ListContextsResponse {
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of contexts. There will be a maximum number of items returned based on the page_size field in the request."]
    pub contexts: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1Context>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for Documents.ListDocuments."]
pub struct GoogleCloudDialogflowV2beta1ListDocumentsResponse {
    #[serde(rename = "documents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of documents."]
    pub documents: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1Document>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for EntityTypes.ListEntityTypes."]
pub struct GoogleCloudDialogflowV2beta1ListEntityTypesResponse {
    #[serde(rename = "entityTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of agent entity types. There will be a maximum number of items returned based on the page_size field in the request."]
    pub entity_types: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1EntityType>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Environments.ListEnvironments."]
pub struct GoogleCloudDialogflowV2beta1ListEnvironmentsResponse {
    #[serde(rename = "environments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of agent environments. There will be a maximum number of items returned based on the page_size field in the request."]
    pub environments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1Environment>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Intents.ListIntents."]
pub struct GoogleCloudDialogflowV2beta1ListIntentsResponse {
    #[serde(rename = "intents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of agent intents. There will be a maximum number of items returned based on the page_size field in the request."]
    pub intents: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1Intent>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for KnowledgeBases.ListKnowledgeBases."]
pub struct GoogleCloudDialogflowV2beta1ListKnowledgeBasesResponse {
    #[serde(rename = "knowledgeBases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of knowledge bases."]
    pub knowledge_bases: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1KnowledgeBase>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for SessionEntityTypes.ListSessionEntityTypes."]
pub struct GoogleCloudDialogflowV2beta1ListSessionEntityTypesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sessionEntityTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of session entity types. There will be a maximum number of items returned based on the page_size field in the request."]
    pub session_entity_types: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1SessionEntityType>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the contents of the original request that was passed to the `[Streaming]DetectIntent` call."]
pub struct GoogleCloudDialogflowV2beta1OriginalDetectIntentRequest {
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. This field is set to the value of the `QueryParameters.payload` field passed in the request. Some integrations that query a Dialogflow agent may provide additional information in the payload. In particular, for the Dialogflow Phone Gateway integration, this field has the form: { \"telephony\": { \"caller_id\": \"+18558363987\" } } Note: The caller ID field (`caller_id`) will be redacted for Trial Edition agents and populated with the caller ID in [E.164 format](https://en.wikipedia.org/wiki/E.164) for Essentials Edition agents."]
    pub payload: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source of this request, e.g., `google`, `facebook`, `slack`. It is set by Dialogflow-owned servers."]
    pub source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The version of the protocol used for this request. This field is AoG-specific."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Instructs the speech synthesizer how to generate the output audio content. If this audio config is supplied in a request, it overrides all existing text-to-speech settings applied to the agent."]
pub struct GoogleCloudDialogflowV2beta1OutputAudioConfig {
    #[serde(rename = "audioEncoding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Audio encoding of the synthesized audio content."]
    pub audio_encoding:
        ::std::option::Option<GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum>,
    #[serde(rename = "sampleRateHertz")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The synthesis sample rate (in hertz) for this audio. If not provided, then the synthesizer will use the default sample rate based on the audio encoding. If this is different from the voice's natural sample rate, then the synthesizer will honor this request by converting to the desired sample rate (which might result in worse audio quality)."]
    pub sample_rate_hertz: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "synthesizeSpeechConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration of how speech should be synthesized."]
    pub synthesize_speech_config: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1SynthesizeSpeechConfig>,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Audio encoding of the synthesized audio content."]
pub enum GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum {
    #[serde(rename = "OUTPUT_AUDIO_ENCODING_UNSPECIFIED")]
    #[doc = "Not specified."]
    OutputAudioEncodingUnspecified,
    #[serde(rename = "OUTPUT_AUDIO_ENCODING_LINEAR_16")]
    #[doc = "Uncompressed 16-bit signed little-endian samples (Linear PCM). Audio content returned as LINEAR16 also contains a WAV header."]
    OutputAudioEncodingLinear16,
    #[serde(rename = "OUTPUT_AUDIO_ENCODING_MP3")]
    #[doc = "MP3 audio at 32kbps."]
    OutputAudioEncodingMp3,
    #[serde(rename = "OUTPUT_AUDIO_ENCODING_OGG_OPUS")]
    #[doc = "Opus encoded audio wrapped in an ogg container. The result will be a file which can be played natively on Android, and in browsers (at least Chrome and Firefox). The quality of the encoding is considerably higher than MP3 while using approximately the same bitrate."]
    OutputAudioEncodingOggOpus,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the query input. It can contain either: 1. An audio config which instructs the speech recognizer how to process the speech audio. 2. A conversational query in the form of text. 3. An event that specifies which intent to trigger."]
pub struct GoogleCloudDialogflowV2beta1QueryInput {
    #[serde(rename = "audioConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Instructs the speech recognizer how to process the speech audio."]
    pub audio_config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1InputAudioConfig>>,
    #[serde(rename = "event")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The event to be processed."]
    pub event: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1EventInput>>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The natural language text to be processed."]
    pub text: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1TextInput>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the parameters of the conversational query."]
pub struct GoogleCloudDialogflowV2beta1QueryParameters {
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of contexts to be activated before this query is executed."]
    pub contexts: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1Context>>,
    >,
    #[serde(rename = "geoLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The geo location of this conversational query."]
    pub geo_location: ::std::option::Option<::std::boxed::Box<GoogleTypeLatLng>>,
    #[serde(rename = "knowledgeBaseNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "KnowledgeBases to get alternative results from. If not set, the KnowledgeBases enabled in the agent (through UI) will be used. Format: `projects//knowledgeBases/`."]
    pub knowledge_base_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field can be used to pass custom data to your webhook. Arbitrary JSON objects are supported. If supplied, the value is used to populate the `WebhookRequest.original_detect_intent_request.payload` field sent to your webhook."]
    pub payload: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "resetContexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies whether to delete all contexts in the current session before the new ones are activated."]
    pub reset_contexts: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "sentimentAnalysisRequestConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configures the type of sentiment analysis to perform. If not provided, sentiment analysis is not performed. Note: Sentiment Analysis is only currently available for Essentials Edition agents."]
    pub sentiment_analysis_request_config: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1SentimentAnalysisRequestConfig>,
    >,
    #[serde(rename = "sessionEntityTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional session entity types to replace or extend developer entity types with. The entity synonyms apply to all languages and persist for the session of this query."]
    pub session_entity_types: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1SessionEntityType>>,
    >,
    #[serde(rename = "subAgents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For mega agent query, directly specify which sub agents to query. If any specified sub agent is not linked to the mega agent, an error will be returned. If empty, Dialogflow will decide which sub agents to query. If specified for a non-mega-agent query, will be silently ignored."]
    pub sub_agents: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1SubAgent>>,
    >,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time zone of this conversational query from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. If not provided, the time zone specified in agent settings is used."]
    pub time_zone: ::std::option::Option<::std::string::String>,
    #[serde(rename = "webhookHeaders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field can be used to pass HTTP headers for a webhook call. These headers will be sent to webhook along with the headers that have been configured through Dialogflow web console. The headers defined within this field will overwrite the headers configured through Dialogflow console if there is a conflict. Header names are case-insensitive. Google's specified headers are not allowed. Including: \"Host\", \"Content-Length\", \"Connection\", \"From\", \"User-Agent\", \"Accept-Encoding\", \"If-Modified-Since\", \"If-None-Match\", \"X-Forwarded-For\", etc."]
    pub webhook_headers:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the result of conversational query or event processing."]
pub struct GoogleCloudDialogflowV2beta1QueryResult {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The action name from the matched intent."]
    pub action: ::std::option::Option<::std::string::String>,
    #[serde(rename = "allRequiredParamsPresent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is set to: - `false` if the matched intent has required parameters and not all of the required parameter values have been collected. - `true` if all required parameter values have been collected, or if the matched intent doesn't contain any required parameters."]
    pub all_required_params_present: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "diagnosticInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Free-form diagnostic information for the associated detect intent request. The fields of this data can change without notice, so you should not write code that depends on its structure. The data may contain: - webhook call latency - webhook errors"]
    pub diagnostic_info:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "fulfillmentMessages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of rich messages to present to the user."]
    pub fulfillment_messages: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessage>>,
    >,
    #[serde(rename = "fulfillmentText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text to be pronounced to the user or shown on the screen. Note: This is a legacy field, `fulfillment_messages` should be preferred."]
    pub fulfillment_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "intent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The intent that matched the conversational query. Some, not all fields are filled in this message, including but not limited to: `name`, `display_name`, `end_interaction` and `is_fallback`."]
    pub intent: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1Intent>>,
    #[serde(rename = "intentDetectionConfidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The intent detection confidence. Values range from 0.0 (completely uncertain) to 1.0 (completely certain). This value is for informational purpose only and is only used to help match the best intent within the classification threshold. This value may change for the same end-user expression at any time due to a model retraining or change in implementation. If there are `multiple knowledge_answers` messages, this value is set to the greatest `knowledgeAnswers.match_confidence` value in the list."]
    pub intent_detection_confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "knowledgeAnswers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result from Knowledge Connector (if any), ordered by decreasing `KnowledgeAnswers.match_confidence`."]
    pub knowledge_answers:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1KnowledgeAnswers>>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language that was triggered during intent detection. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outputContexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of output contexts. If applicable, `output_contexts.parameters` contains entries with name `.original` containing the original parameter values before the query."]
    pub output_contexts: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1Context>>,
    >,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection of extracted parameters. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value"]
    pub parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "queryText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The original conversational query text: - If natural language text was provided as input, `query_text` contains a copy of the input. - If natural language speech audio was provided as input, `query_text` contains the speech recognition result. If speech recognizer produced multiple alternatives, a particular one is picked. - If automatic spell correction is enabled, `query_text` will contain the corrected user input."]
    pub query_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sentimentAnalysisResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sentiment analysis result, which depends on the `sentiment_analysis_request_config` specified in the request."]
    pub sentiment_analysis_result: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1SentimentAnalysisResult>,
    >,
    #[serde(rename = "speechRecognitionConfidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Speech recognition confidence between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. The default of 0.0 is a sentinel value indicating that confidence was not set. This field is not guaranteed to be accurate or set. In particular this field isn't set for StreamingDetectIntent since the streaming endpoint has separate confidence estimates per portion of the audio in StreamingRecognitionResult."]
    pub speech_recognition_confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "webhookPayload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the query was fulfilled by a webhook call, this field is set to the value of the `payload` field returned in the webhook response."]
    pub webhook_payload:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "webhookSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the query was fulfilled by a webhook call, this field is set to the value of the `source` field returned in the webhook response."]
    pub webhook_source: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for Documents.ReloadDocument."]
pub struct GoogleCloudDialogflowV2beta1ReloadDocumentRequest {
    #[serde(rename = "gcsSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The path for a Cloud Storage source file for reloading document content. If not provided, the Document's existing source will be reloaded."]
    pub gcs_source: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1GcsSource>>,
    #[serde(rename = "importGcsCustomMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to import custom metadata from Google Cloud Storage. Only valid when the document source is Google Cloud Storage URI."]
    pub import_gcs_custom_metadata: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for Agents.RestoreAgent."]
pub struct GoogleCloudDialogflowV2beta1RestoreAgentRequest {
    #[serde(rename = "agentContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Zip compressed raw byte content for agent."]
    pub agent_content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "agentUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI to a Google Cloud Storage file containing the agent to restore. Note: The URI must start with \"gs://\"."]
    pub agent_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Agents.SearchAgents."]
pub struct GoogleCloudDialogflowV2beta1SearchAgentsResponse {
    #[serde(rename = "agents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of agents. There will be a maximum number of items returned based on the page_size field in the request."]
    pub agents: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1Agent>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The sentiment, such as positive/negative feeling or association, for a unit of analysis, such as the query text."]
pub struct GoogleCloudDialogflowV2beta1Sentiment {
    #[serde(rename = "magnitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A non-negative number in the [0, +inf) range, which represents the absolute magnitude of sentiment, regardless of score (positive or negative)."]
    pub magnitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sentiment score between -1.0 (negative sentiment) and 1.0 (positive sentiment)."]
    pub score: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configures the types of sentiment analysis to perform."]
pub struct GoogleCloudDialogflowV2beta1SentimentAnalysisRequestConfig {
    #[serde(rename = "analyzeQueryTextSentiment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Instructs the service to perform sentiment analysis on `query_text`. If not provided, sentiment analysis is not performed on `query_text`."]
    pub analyze_query_text_sentiment: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The result of sentiment analysis. Sentiment analysis inspects user input and identifies the prevailing subjective opinion, especially to determine a user's attitude as positive, negative, or neutral. For Participants.DetectIntent, it needs to be configured in DetectIntentRequest.query_params. For Participants.StreamingDetectIntent, it needs to be configured in StreamingDetectIntentRequest.query_params. And for Participants.AnalyzeContent and Participants.StreamingAnalyzeContent, it needs to be configured in ConversationProfile.human_agent_assistant_config"]
pub struct GoogleCloudDialogflowV2beta1SentimentAnalysisResult {
    #[serde(rename = "queryTextSentiment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sentiment analysis result for `query_text`."]
    pub query_text_sentiment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1Sentiment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A session represents a conversation between a Dialogflow agent and an end-user. You can create special entities, called session entities, during a session. Session entities can extend or replace custom entity types and only exist during the session that they were created for. All session data, including session entities, is stored by Dialogflow for 20 minutes. For more information, see the [session entity guide](https://cloud.google.com/dialogflow/docs/entities-session)."]
pub struct GoogleCloudDialogflowV2beta1SessionEntityType {
    #[serde(rename = "entities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The collection of entities associated with this session entity type."]
    pub entities: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1EntityTypeEntity>>,
    >,
    #[serde(rename = "entityOverrideMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Indicates whether the additional data should override or supplement the custom entity type definition."]
    pub entity_override_mode:
        ::std::option::Option<GoogleCloudDialogflowV2beta1SessionEntityTypeEntityOverrideModeEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The unique identifier of this session entity type. Supported formats: - `projects//agent/sessions//entityTypes/` - `projects//locations//agent/sessions//entityTypes/` - `projects//agent/environments//users//sessions//entityTypes/` - `projects//locations//agent/environments/ /users//sessions//entityTypes/` If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. `` must be the display name of an existing entity type in the same agent that will be overridden or supplemented."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Indicates whether the additional data should override or supplement the custom entity type definition."]
pub enum GoogleCloudDialogflowV2beta1SessionEntityTypeEntityOverrideModeEnum {
    #[serde(rename = "ENTITY_OVERRIDE_MODE_UNSPECIFIED")]
    #[doc = "Not specified. This value should be never used."]
    EntityOverrideModeUnspecified,
    #[serde(rename = "ENTITY_OVERRIDE_MODE_OVERRIDE")]
    #[doc = "The collection of session entities overrides the collection of entities in the corresponding custom entity type."]
    EntityOverrideModeOverride,
    #[serde(rename = "ENTITY_OVERRIDE_MODE_SUPPLEMENT")]
    #[doc = "The collection of session entities extends the collection of entities in the corresponding custom entity type. Note: Even in this override mode calls to `ListSessionEntityTypes`, `GetSessionEntityType`, `CreateSessionEntityType` and `UpdateSessionEntityType` only return the additional entities added in this session entity type. If you want to get the supplemented list, please call EntityTypes.GetEntityType on the custom entity type and merge."]
    EntityOverrideModeSupplement,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Hints for the speech recognizer to help with recognition in a specific conversation state."]
pub struct GoogleCloudDialogflowV2beta1SpeechContext {
    #[serde(rename = "boost")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Boost for this context compared to other contexts: * If the boost is positive, Dialogflow will increase the probability that the phrases in this context are recognized over similar sounding phrases. * If the boost is unspecified or non-positive, Dialogflow will not apply any boost. Dialogflow recommends that you use boosts in the range (0, 20] and that you find a value that fits your use case with binary search."]
    pub boost: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "phrases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A list of strings containing words and phrases that the speech recognizer should recognize with higher likelihood. This list can be used to: * improve accuracy for words and phrases you expect the user to say, e.g. typical commands for your Dialogflow agent * add additional words to the speech recognizer vocabulary * ... See the [Cloud Speech documentation](https://cloud.google.com/speech-to-text/quotas) for usage limits."]
    pub phrases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains basic configuration for a sub-agent."]
pub struct GoogleCloudDialogflowV2beta1SubAgent {
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The unique identifier (`environment name` in dialogflow console) of this sub-agent environment. Assumes draft environment if `environment` is not set."]
    pub environment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The project of this agent. Format: `projects/` or `projects//locations/`."]
    pub project: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration of how speech should be synthesized."]
pub struct GoogleCloudDialogflowV2beta1SynthesizeSpeechConfig {
    #[serde(rename = "effectsProfileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. An identifier which selects 'audio effects' profiles that are applied on (post synthesized) text to speech. Effects are applied on top of each other in the order they are given."]
    pub effects_profile_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "pitch")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Speaking pitch, in the range [-20.0, 20.0]. 20 means increase 20 semitones from the original pitch. -20 means decrease 20 semitones from the original pitch."]
    pub pitch: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "speakingRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Speaking rate/speed, in the range [0.25, 4.0]. 1.0 is the normal native speed supported by the specific voice. 2.0 is twice as fast, and 0.5 is half as fast. If unset(0.0), defaults to the native 1.0 speed. Any other values < 0.25 or > 4.0 will return an error."]
    pub speaking_rate: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "voice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The desired voice of the synthesized audio."]
    pub voice:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1VoiceSelectionParams>>,
    #[serde(rename = "volumeGainDb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Volume gain (in dB) of the normal native volume supported by the specific voice, in the range [-96.0, 16.0]. If unset, or set to a value of 0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB) will play at approximately half the amplitude of the normal native signal amplitude. A value of +6.0 (dB) will play at approximately twice the amplitude of the normal native signal amplitude. We strongly recommend not to exceed +10 (dB) as there's usually no effective increase in loudness for any value greater than that."]
    pub volume_gain_db: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the natural language text to be processed."]
pub struct GoogleCloudDialogflowV2beta1TextInput {
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The language of this conversational query. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. Note that queries in the same session do not necessarily need to specify the same language."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The UTF-8 encoded natural language text to be processed. Text length must not exceed 256 characters."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for Agents.TrainAgent."]
pub struct GoogleCloudDialogflowV2beta1TrainAgentRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a single validation error."]
pub struct GoogleCloudDialogflowV2beta1ValidationError {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The names of the entries that the error is associated with. Format: - \"projects//agent\", if the error is associated with the entire agent. - \"projects//agent/intents/\", if the error is associated with certain intents. - \"projects//agent/intents//trainingPhrases/\", if the error is associated with certain intent training phrases. - \"projects//agent/intents//parameters/\", if the error is associated with certain intent parameters. - \"projects//agent/entities/\", if the error is associated with certain entities."]
    pub entries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The detailed error messsage."]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The severity of the error."]
    pub severity: ::std::option::Option<GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The severity of the error."]
pub enum GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum {
    #[serde(rename = "SEVERITY_UNSPECIFIED")]
    #[doc = "Not specified. This value should never be used."]
    SeverityUnspecified,
    #[serde(rename = "INFO")]
    #[doc = "The agent doesn't follow Dialogflow best practices."]
    Info,
    #[serde(rename = "WARNING")]
    #[doc = "The agent may not behave as expected."]
    Warning,
    #[serde(rename = "ERROR")]
    #[doc = "The agent may experience partial failures."]
    Error,
    #[serde(rename = "CRITICAL")]
    #[doc = "The agent may completely fail."]
    Critical,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the output of agent validation."]
pub struct GoogleCloudDialogflowV2beta1ValidationResult {
    #[serde(rename = "validationErrors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains all validation errors."]
    pub validation_errors: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1ValidationError>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Description of which voice to use for speech synthesis."]
pub struct GoogleCloudDialogflowV2beta1VoiceSelectionParams {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The name of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and ssml_gender. For the list of available voices, please refer to [Supported voices and languages](https://cloud.google.com/text-to-speech/docs/voices)."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ssmlGender")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The preferred gender of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and name. Note that this is only a preference, not requirement. If a voice of the appropriate gender is not available, the synthesizer should substitute a voice with a different gender rather than failing the request."]
    pub ssml_gender:
        ::std::option::Option<GoogleCloudDialogflowV2beta1VoiceSelectionParamsSsmlGenderEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. The preferred gender of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and name. Note that this is only a preference, not requirement. If a voice of the appropriate gender is not available, the synthesizer should substitute a voice with a different gender rather than failing the request."]
pub enum GoogleCloudDialogflowV2beta1VoiceSelectionParamsSsmlGenderEnum {
    #[serde(rename = "SSML_VOICE_GENDER_UNSPECIFIED")]
    #[doc = "An unspecified gender, which means that the client doesn't care which gender the selected voice will have."]
    SsmlVoiceGenderUnspecified,
    #[serde(rename = "SSML_VOICE_GENDER_MALE")]
    #[doc = "A male voice."]
    SsmlVoiceGenderMale,
    #[serde(rename = "SSML_VOICE_GENDER_FEMALE")]
    #[doc = "A female voice."]
    SsmlVoiceGenderFemale,
    #[serde(rename = "SSML_VOICE_GENDER_NEUTRAL")]
    #[doc = "A gender-neutral voice."]
    SsmlVoiceGenderNeutral,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for a webhook call."]
pub struct GoogleCloudDialogflowV2beta1WebhookRequest {
    #[serde(rename = "alternativeQueryResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Alternative query results from KnowledgeService."]
    pub alternative_query_results: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1QueryResult>>,
    >,
    #[serde(rename = "originalDetectIntentRequest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The contents of the original request that was passed to `[Streaming]DetectIntent` call."]
    pub original_detect_intent_request: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV2beta1OriginalDetectIntentRequest>,
    >,
    #[serde(rename = "queryResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the conversational query or event processing. Contains the same value as `[Streaming]DetectIntentResponse.query_result`."]
    pub query_result:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1QueryResult>>,
    #[serde(rename = "responseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the response. Contains the same value as `[Streaming]DetectIntentResponse.response_id`."]
    pub response_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "session")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of detectIntent request session. Can be used to identify end-user inside webhook implementation. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`,"]
    pub session: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for a webhook call. This response is validated by the Dialogflow server. If validation fails, an error will be returned in the QueryResult.diagnostic_info field. Setting JSON fields to an empty value with the wrong type is a common error. To avoid this error: - Use `\"\"` for empty strings - Use `{}` or `null` for empty objects - Use `[]` or `null` for empty arrays For more information, see the [Protocol Buffers Language Guide](https://developers.google.com/protocol-buffers/docs/proto3#json)."]
pub struct GoogleCloudDialogflowV2beta1WebhookResponse {
    #[serde(rename = "endInteraction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates that this intent ends an interaction. Some integrations (e.g., Actions on Google or Dialogflow phone gateway) use this information to close interaction with an end user. Default is false."]
    pub end_interaction: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "followupEventInput")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Invokes the supplied events. When this field is set, Dialogflow ignores the `fulfillment_text`, `fulfillment_messages`, and `payload` fields."]
    pub followup_event_input:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1EventInput>>,
    #[serde(rename = "fulfillmentMessages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The rich response messages intended for the end-user. When provided, Dialogflow uses this field to populate QueryResult.fulfillment_messages sent to the integration or API caller."]
    pub fulfillment_messages: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessage>>,
    >,
    #[serde(rename = "fulfillmentText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The text response message intended for the end-user. It is recommended to use `fulfillment_messages.text.text[0]` instead. When provided, Dialogflow uses this field to populate QueryResult.fulfillment_text sent to the integration or API caller."]
    pub fulfillment_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outputContexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The collection of output contexts that will overwrite currently active contexts for the session and reset their lifespans. When provided, Dialogflow uses this field to populate QueryResult.output_contexts sent to the integration or API caller."]
    pub output_contexts: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1Context>>,
    >,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. This field can be used to pass custom data from your webhook to the integration or API caller. Arbitrary JSON objects are supported. When provided, Dialogflow uses this field to populate QueryResult.webhook_payload sent to the integration or API caller. This field is also used by the [Google Assistant integration](https://cloud.google.com/dialogflow/docs/integrations/aog) for rich response messages. See the format definition at [Google Assistant Dialogflow webhook format](https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json)"]
    pub payload: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "sessionEntityTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Additional session entity types to replace or extend developer entity types with. The entity synonyms apply to all languages and persist for the session. Setting this data from a webhook overwrites the session entity types that have been set using `detectIntent`, `streamingDetectIntent` or SessionEntityType management methods."]
    pub session_entity_types: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1SessionEntityType>>,
    >,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A custom field used to identify the webhook source. Arbitrary strings are supported. When provided, Dialogflow uses this field to populate QueryResult.webhook_source sent to the integration or API caller."]
    pub source: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for CreateDocument operation."]
pub struct GoogleCloudDialogflowV3alpha1CreateDocumentOperationMetadata {
    #[serde(rename = "genericMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generic information of the operation."]
    pub generic_metadata: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadata>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for DeleteDocument operation."]
pub struct GoogleCloudDialogflowV3alpha1DeleteDocumentOperationMetadata {
    #[serde(rename = "genericMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generic information of the operation."]
    pub generic_metadata: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadata>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata in google::longrunning::Operation for Knowledge operations."]
pub struct GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadata {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Output only. The current state of this operation."]
    pub state: ::std::option::Option<
        GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadataStateEnum,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Output only. The current state of this operation."]
pub enum GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadataStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "State unspecified."]
    StateUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "The operation has been created."]
    Pending,
    #[serde(rename = "RUNNING")]
    #[doc = "The operation is currently running."]
    Running,
    #[serde(rename = "DONE")]
    #[doc = "The operation is done, either cancelled or completed."]
    Done,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for ImportDocuments operation."]
pub struct GoogleCloudDialogflowV3alpha1ImportDocumentsOperationMetadata {
    #[serde(rename = "genericMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generic information of the operation."]
    pub generic_metadata: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadata>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for Documents.ImportDocuments."]
pub struct GoogleCloudDialogflowV3alpha1ImportDocumentsResponse {
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Includes details about skipped documents or any other warnings."]
    pub warnings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for ReloadDocument operation."]
pub struct GoogleCloudDialogflowV3alpha1ReloadDocumentOperationMetadata {
    #[serde(rename = "genericMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generic information of the operation."]
    pub generic_metadata: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadata>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for UpdateDocument operation."]
pub struct GoogleCloudDialogflowV3alpha1UpdateDocumentOperationMetadata {
    #[serde(rename = "genericMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generic information of the operation."]
    pub generic_metadata: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadata>,
    >,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An object that represents a latitude/longitude pair. This is expressed as a pair of doubles to represent degrees latitude and degrees longitude. Unless specified otherwise, this must conform to the WGS84 standard. Values must be within normalized ranges."]
pub struct GoogleTypeLatLng {
    #[serde(rename = "latitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The latitude in degrees. It must be in the range [-90.0, +90.0]."]
    pub latitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "longitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The longitude in degrees. It must be in the range [-180.0, +180.0]."]
    pub longitude: ::std::option::Option<::std::primitive::f64>,
}
