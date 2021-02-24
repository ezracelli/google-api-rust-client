#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message that represents an arbitrary HTTP body. It should only be used for payload formats that can't be represented as JSON, such as raw binary or an HTML page. This message can be used both in streaming and non-streaming API methods in the request as well as the response. It can be used as a top-level request field, which is convenient if one wants to extract parameters from either the URL or HTTP template into the request fields and also want access to the raw HTTP body. Example: message GetResourceRequest { // A unique request id. string request_id = 1; // The raw HTTP body is bound to this field. google.api.HttpBody http_body = 2; } service ResourceService { rpc GetResource(GetResourceRequest) returns (google.api.HttpBody); rpc UpdateResource(google.api.HttpBody) returns (google.protobuf.Empty); } Example with streaming methods: service CaldavService { rpc GetCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); rpc UpdateCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); } Use of this type only changes how the request and response bodies are handled, all other features will continue to work unchanged."]
pub struct GoogleApiHttpBody {
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HTTP Content-Type header value specifying the content type of the body."]
    pub content_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HTTP request/response body as raw binary."]
    pub data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "extensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Application specific response metadata. Must be set in the first response for streaming APIs."]
    pub extensions: ::std::option::Option<
        ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudMlV1AutomatedStoppingConfigDecayCurveAutomatedStoppingConfig {
    #[serde(rename = "useElapsedTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, measurement.elapsed_time is used as the x-axis of each Trials Decay Curve. Otherwise, Measurement.steps will be used as the x-axis."]
    pub use_elapsed_time: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The median automated stopping rule stops a pending trial if the trial's best objective_value is strictly below the median 'performance' of all completed trials reported up to the trial's last measurement. Currently, 'performance' refers to the running average of the objective values reported by the trial in each measurement."]
pub struct GoogleCloudMlV1AutomatedStoppingConfigMedianAutomatedStoppingConfig {
    #[serde(rename = "useElapsedTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, the median automated stopping rule applies to measurement.use_elapsed_time, which means the elapsed_time field of the current trial's latest measurement is used to compute the median objective value for each completed trial."]
    pub use_elapsed_time: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An observed value of a metric."]
pub struct GoogleCloudMlV1HyperparameterOutputHyperparameterMetric {
    #[serde(rename = "objectiveValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The objective value at this training step."]
    pub objective_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "trainingStep")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The global training step for this metric."]
    pub training_step: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message representing a metric in the measurement."]
pub struct GoogleCloudMlV1MeasurementMetric {
    #[serde(rename = "metric")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Metric name."]
    pub metric: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The value for this metric."]
    pub value: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudMlV1StudyConfigParameterSpecCategoricalValueSpec {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Must be specified if type is `CATEGORICAL`. The list of possible categories."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudMlV1StudyConfigParameterSpecDiscreteValueSpec {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Must be specified if type is `DISCRETE`. A list of feasible points. The list should be in strictly increasing order. For instance, this parameter might have possible settings of 1.5, 2.5, and 4.0. This list should not contain more than 1,000 values."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudMlV1StudyConfigParameterSpecDoubleValueSpec {
    #[serde(rename = "maxValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Must be specified if type is `DOUBLE`. Maximum value of the parameter."]
    pub max_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "minValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Must be specified if type is `DOUBLE`. Minimum value of the parameter."]
    pub min_value: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudMlV1StudyConfigParameterSpecIntegerValueSpec {
    #[serde(rename = "maxValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Must be specified if type is `INTEGER`. Maximum value of the parameter."]
    pub max_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Must be specified if type is `INTEGER`. Minimum value of the parameter."]
    pub min_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the spec to match categorical values from parent parameter."]
pub struct GoogleCloudMlV1StudyConfigParameterSpecMatchingParentCategoricalValueSpec {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Matches values of the parent parameter with type 'CATEGORICAL'. All values must exist in `categorical_value_spec` of parent parameter."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the spec to match discrete values from parent parameter."]
pub struct GoogleCloudMlV1StudyConfigParameterSpecMatchingParentDiscreteValueSpec {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Matches values of the parent parameter with type 'DISCRETE'. All values must exist in `discrete_value_spec` of parent parameter."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the spec to match integer values from parent parameter."]
pub struct GoogleCloudMlV1StudyConfigParameterSpecMatchingParentIntValueSpec {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Matches values of the parent parameter with type 'INTEGER'. All values must lie in `integer_value_spec` of parent parameter."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a metric to optimize."]
pub struct GoogleCloudMlV1StudyConfigMetricSpec {
    #[serde(rename = "goal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The optimization goal of the metric."]
    pub goal: ::std::option::Option<GoogleCloudMlV1StudyConfigMetricSpecGoalEnum>,
    #[serde(rename = "metric")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the metric."]
    pub metric: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The optimization goal of the metric."]
pub enum GoogleCloudMlV1StudyConfigMetricSpecGoalEnum {
    #[serde(rename = "GOAL_TYPE_UNSPECIFIED")]
    #[doc = "Goal Type will default to maximize."]
    GoalTypeUnspecified,
    #[serde(rename = "MAXIMIZE")]
    #[doc = "Maximize the goal metric."]
    Maximize,
    #[serde(rename = "MINIMIZE")]
    #[doc = "Minimize the goal metric."]
    Minimize,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a single parameter to optimize."]
pub struct GoogleCloudMlV1StudyConfigParameterSpec {
    #[serde(rename = "categoricalValueSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value spec for a 'CATEGORICAL' parameter."]
    pub categorical_value_spec: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudMlV1StudyConfigParameterSpecCategoricalValueSpec>,
    >,
    #[serde(rename = "childParameterSpecs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A child node is active if the parameter's value matches the child node's matching_parent_values. If two items in child_parameter_specs have the same name, they must have disjoint matching_parent_values."]
    pub child_parameter_specs: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1StudyConfigParameterSpec>>,
    >,
    #[serde(rename = "discreteValueSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value spec for a 'DISCRETE' parameter."]
    pub discrete_value_spec: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudMlV1StudyConfigParameterSpecDiscreteValueSpec>,
    >,
    #[serde(rename = "doubleValueSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value spec for a 'DOUBLE' parameter."]
    pub double_value_spec: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudMlV1StudyConfigParameterSpecDoubleValueSpec>,
    >,
    #[serde(rename = "integerValueSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value spec for an 'INTEGER' parameter."]
    pub integer_value_spec: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudMlV1StudyConfigParameterSpecIntegerValueSpec>,
    >,
    #[serde(rename = "parameter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The parameter name must be unique amongst all ParameterSpecs."]
    pub parameter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parentCategoricalValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parent_categorical_values: ::std::option::Option<
        ::std::boxed::Box<
            GoogleCloudMlV1StudyConfigParameterSpecMatchingParentCategoricalValueSpec,
        >,
    >,
    #[serde(rename = "parentDiscreteValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parent_discrete_values: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudMlV1StudyConfigParameterSpecMatchingParentDiscreteValueSpec>,
    >,
    #[serde(rename = "parentIntValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parent_int_values: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudMlV1StudyConfigParameterSpecMatchingParentIntValueSpec>,
    >,
    #[serde(rename = "scaleType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How the parameter should be scaled. Leave unset for categorical parameters."]
    pub scale_type: ::std::option::Option<GoogleCloudMlV1StudyConfigParameterSpecScaleTypeEnum>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of the parameter."]
    pub _type: ::std::option::Option<GoogleCloudMlV1StudyConfigParameterSpecTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "How the parameter should be scaled. Leave unset for categorical parameters."]
pub enum GoogleCloudMlV1StudyConfigParameterSpecScaleTypeEnum {
    #[serde(rename = "SCALE_TYPE_UNSPECIFIED")]
    #[doc = "By default, no scaling is applied."]
    ScaleTypeUnspecified,
    #[serde(rename = "UNIT_LINEAR_SCALE")]
    #[doc = "Scales the feasible space to (0, 1) linearly."]
    UnitLinearScale,
    #[serde(rename = "UNIT_LOG_SCALE")]
    #[doc = "Scales the feasible space logarithmically to (0, 1). The entire feasible space must be strictly positive."]
    UnitLogScale,
    #[serde(rename = "UNIT_REVERSE_LOG_SCALE")]
    #[doc = "Scales the feasible space \"reverse\" logarithmically to (0, 1). The result is that values close to the top of the feasible space are spread out more than points near the bottom. The entire feasible space must be strictly positive."]
    UnitReverseLogScale,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of the parameter."]
pub enum GoogleCloudMlV1StudyConfigParameterSpecTypeEnum {
    #[serde(rename = "PARAMETER_TYPE_UNSPECIFIED")]
    #[doc = "You must specify a valid type. Using this unspecified type will result in an error."]
    ParameterTypeUnspecified,
    #[serde(rename = "DOUBLE")]
    #[doc = "Type for real-valued parameters."]
    Double,
    #[serde(rename = "INTEGER")]
    #[doc = "Type for integral parameters."]
    Integer,
    #[serde(rename = "CATEGORICAL")]
    #[doc = "The parameter is categorical, with a value chosen from the categories field."]
    Categorical,
    #[serde(rename = "DISCRETE")]
    #[doc = "The parameter is real valued, with a fixed set of feasible points. If `type==DISCRETE`, feasible_points must be provided, and {`min_value`, `max_value`} will be ignored."]
    Discrete,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message representing a parameter to be tuned. Contains the name of the parameter and the suggested value to use for this trial."]
pub struct GoogleCloudMlV1TrialParameter {
    #[serde(rename = "floatValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Must be set if ParameterType is DOUBLE or DISCRETE."]
    pub float_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "intValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Must be set if ParameterType is INTEGER"]
    pub int_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the parameter."]
    pub parameter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Must be set if ParameterTypeis CATEGORICAL"]
    pub string_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a hardware accelerator request config. Note that the AcceleratorConfig can be used in both Jobs and Versions. Learn more about [accelerators for training](/ml-engine/docs/using-gpus) and [accelerators for online prediction](/ml-engine/docs/machine-types-online-prediction#gpus)."]
pub struct GoogleCloudMlV1AcceleratorConfig {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of accelerators to attach to each machine running the job."]
    pub count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of accelerator to use."]
    pub _type: ::std::option::Option<GoogleCloudMlV1AcceleratorConfigTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of accelerator to use."]
pub enum GoogleCloudMlV1AcceleratorConfigTypeEnum {
    #[serde(rename = "ACCELERATOR_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified accelerator type. Default to no GPU."]
    AcceleratorTypeUnspecified,
    #[serde(rename = "NVIDIA_TESLA_K80")]
    #[doc = "Nvidia Tesla K80 GPU."]
    NvidiaTeslaK80,
    #[serde(rename = "NVIDIA_TESLA_P100")]
    #[doc = "Nvidia Tesla P100 GPU."]
    NvidiaTeslaP100,
    #[serde(rename = "NVIDIA_TESLA_V100")]
    #[doc = "Nvidia V100 GPU."]
    NvidiaTeslaV100,
    #[serde(rename = "NVIDIA_TESLA_P4")]
    #[doc = "Nvidia Tesla P4 GPU."]
    NvidiaTeslaP4,
    #[serde(rename = "NVIDIA_TESLA_T4")]
    #[doc = "Nvidia T4 GPU."]
    NvidiaTeslaT4,
    #[serde(rename = "NVIDIA_TESLA_A100")]
    #[doc = "Nvidia A100 GPU."]
    NvidiaTeslaA100,
    #[serde(rename = "TPU_V2")]
    #[doc = "TPU v2."]
    TpuV2,
    #[serde(rename = "TPU_V3")]
    #[doc = "TPU v3."]
    TpuV3,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for the AddTrialMeasurement service method."]
pub struct GoogleCloudMlV1AddTrialMeasurementRequest {
    #[serde(rename = "measurement")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The measurement to be added to a trial."]
    pub measurement: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1Measurement>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for automatically scaling a model."]
pub struct GoogleCloudMlV1AutoScaling {
    #[serde(rename = "maxNodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number of nodes to scale this model under load. The actual value will depend on resource quota and availability."]
    pub max_nodes: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "MetricSpec contains the specifications to use to calculate the desired nodes count."]
    pub metrics:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1MetricSpec>>>,
    #[serde(rename = "minNodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The minimum number of nodes to allocate for this model. These nodes are always up, starting from the time the model is deployed. Therefore, the cost of operating this model will be at least `rate` * `min_nodes` * number of hours since last billing cycle, where `rate` is the cost per node-hour as documented in the [pricing guide](/ml-engine/docs/pricing), even if no predictions are performed. There is additional cost for each prediction performed. Unlike manual scaling, if the load gets too heavy for the nodes that are up, the service will automatically add nodes to handle the increased load as well as scale back as traffic drops, always maintaining at least `min_nodes`. You will be charged for the time in which additional nodes are used. If `min_nodes` is not specified and AutoScaling is used with a [legacy (MLS1) machine type](/ml-engine/docs/machine-types-online-prediction), `min_nodes` defaults to 0, in which case, when traffic to a model stops (and after a cool-down period), nodes will be shut down and no charges will be incurred until traffic to the model resumes. If `min_nodes` is not specified and AutoScaling is used with a [Compute Engine (N1) machine type](/ml-engine/docs/machine-types-online-prediction), `min_nodes` defaults to 1. `min_nodes` must be at least 1 for use with a Compute Engine machine type. You can set `min_nodes` when creating the model version, and you can also update `min_nodes` for an existing version: update_body.json: { 'autoScaling': { 'minNodes': 5 } } HTTP request: PATCH https://ml.googleapis.com/v1/{name=projects/*/models/*/versions/*}?update_mask=autoScaling.minNodes -d @./update_body.json "]
    pub min_nodes: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for Automated Early Stopping of Trials. If no implementation_config is set, automated early stopping will not be run."]
pub struct GoogleCloudMlV1AutomatedStoppingConfig {
    #[serde(rename = "decayCurveStoppingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub decay_curve_stopping_config: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudMlV1AutomatedStoppingConfigDecayCurveAutomatedStoppingConfig>,
    >,
    #[serde(rename = "medianAutomatedStoppingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub median_automated_stopping_config: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudMlV1AutomatedStoppingConfigMedianAutomatedStoppingConfig>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents output related to a built-in algorithm Job."]
pub struct GoogleCloudMlV1BuiltInAlgorithmOutput {
    #[serde(rename = "framework")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Framework on which the built-in algorithm was trained."]
    pub framework: ::std::option::Option<::std::string::String>,
    #[serde(rename = "modelPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Cloud Storage path to the `model/` directory where the training job saves the trained model. Only set for successful jobs that don't use hyperparameter tuning."]
    pub model_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pythonVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Python version on which the built-in algorithm was trained."]
    pub python_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "runtimeVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "AI Platform runtime version on which the built-in algorithm was trained."]
    pub runtime_version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for the CancelJob method."]
pub struct GoogleCloudMlV1CancelJobRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudMlV1Capability {
    #[serde(rename = "availableAccelerators")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Available accelerators for the capability."]
    pub available_accelerators:
        ::std::option::Option<::std::vec::Vec<GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub _type: ::std::option::Option<GoogleCloudMlV1CapabilityTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum {
    #[serde(rename = "ACCELERATOR_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified accelerator type. Default to no GPU."]
    AcceleratorTypeUnspecified,
    #[serde(rename = "NVIDIA_TESLA_K80")]
    #[doc = "Nvidia Tesla K80 GPU."]
    NvidiaTeslaK80,
    #[serde(rename = "NVIDIA_TESLA_P100")]
    #[doc = "Nvidia Tesla P100 GPU."]
    NvidiaTeslaP100,
    #[serde(rename = "NVIDIA_TESLA_V100")]
    #[doc = "Nvidia V100 GPU."]
    NvidiaTeslaV100,
    #[serde(rename = "NVIDIA_TESLA_P4")]
    #[doc = "Nvidia Tesla P4 GPU."]
    NvidiaTeslaP4,
    #[serde(rename = "NVIDIA_TESLA_T4")]
    #[doc = "Nvidia T4 GPU."]
    NvidiaTeslaT4,
    #[serde(rename = "NVIDIA_TESLA_A100")]
    #[doc = "Nvidia A100 GPU."]
    NvidiaTeslaA100,
    #[serde(rename = "TPU_V2")]
    #[doc = "TPU v2."]
    TpuV2,
    #[serde(rename = "TPU_V3")]
    #[doc = "TPU v3."]
    TpuV3,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GoogleCloudMlV1CapabilityTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = ""]
    TypeUnspecified,
    #[serde(rename = "TRAINING")]
    #[doc = ""]
    Training,
    #[serde(rename = "BATCH_PREDICTION")]
    #[doc = ""]
    BatchPrediction,
    #[serde(rename = "ONLINE_PREDICTION")]
    #[doc = ""]
    OnlinePrediction,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This message will be placed in the metadata field of a google.longrunning.Operation associated with a CheckTrialEarlyStoppingState request."]
pub struct GoogleCloudMlV1CheckTrialEarlyStoppingStateMetatdata {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the operation was submitted."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "study")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the study that the trial belongs to."]
    pub study: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trial")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The trial name."]
    pub trial: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for the CheckTrialEarlyStoppingState service method."]
pub struct GoogleCloudMlV1CheckTrialEarlyStoppingStateRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The message will be placed in the response field of a completed google.longrunning.Operation associated with a CheckTrialEarlyStoppingState request."]
pub struct GoogleCloudMlV1CheckTrialEarlyStoppingStateResponse {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which operation processing completed."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shouldStop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if the Trial should stop."]
    pub should_stop: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the operation was started."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for the CompleteTrial service method."]
pub struct GoogleCloudMlV1CompleteTrialRequest {
    #[serde(rename = "finalMeasurement")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If provided, it will be used as the completed trial's final_measurement; Otherwise, the service will auto-select a previously reported measurement as the final-measurement"]
    pub final_measurement: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1Measurement>>,
    #[serde(rename = "infeasibleReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A human readable reason why the trial was infeasible. This should only be provided if `trial_infeasible` is true."]
    pub infeasible_reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trialInfeasible")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. True if the trial cannot be run with the given Parameter, and final_measurement will be ignored."]
    pub trial_infeasible: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudMlV1Config {
    #[serde(rename = "tpuServiceAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The service account Cloud ML uses to run on TPU node."]
    pub tpu_service_account: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a network port in a single container. This message is a subset of the [Kubernetes ContainerPort v1 core specification](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.18/#containerport-v1-core)."]
pub struct GoogleCloudMlV1ContainerPort {
    #[serde(rename = "containerPort")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of the port to expose on the container. This must be a valid port number: 0 < PORT_NUMBER < 65536."]
    pub container_port: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specification of a custom container for serving predictions. This message is a subset of the [Kubernetes Container v1 core specification](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.18/#container-v1-core)."]
pub struct GoogleCloudMlV1ContainerSpec {
    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. Specifies arguments for the command that runs when the container starts. This overrides the container's [`CMD`](https://docs.docker.com/engine/reference/builder/#cmd). Specify this field as an array of executable and arguments, similar to a Docker `CMD`'s \"default parameters\" form. If you don't specify this field but do specify the command field, then the command from the `command` field runs without any additional arguments. See the [Kubernetes documentation about how the `command` and `args` fields interact with a container's `ENTRYPOINT` and `CMD`](https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#notes). If you don't specify this field and don't specify the `commmand` field, then the container's [`ENTRYPOINT`](https://docs.docker.com/engine/reference/builder/#cmd) and `CMD` determine what runs based on their default behavior. See the [Docker documentation about how `CMD` and `ENTRYPOINT` interact](https://docs.docker.com/engine/reference/builder/#understand-how-cmd-and-entrypoint-interact). In this field, you can reference [environment variables set by AI Platform Prediction](/ai-platform/prediction/docs/custom-container-requirements#aip-variables) and environment variables set in the env field. You cannot reference environment variables set in the Docker image. In order for environment variables to be expanded, reference them by using the following syntax: $( VARIABLE_NAME) Note that this differs from Bash variable expansion, which does not use parentheses. If a variable cannot be resolved, the reference in the input string is used unchanged. To avoid variable expansion, you can escape this syntax with `$$`; for example: $$(VARIABLE_NAME) This field corresponds to the `args` field of the [Kubernetes Containers v1 core API](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.18/#container-v1-core)."]
    pub args: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. Specifies the command that runs when the container starts. This overrides the container's [`ENTRYPOINT`](https://docs.docker.com/engine/reference/builder/#entrypoint). Specify this field as an array of executable and arguments, similar to a Docker `ENTRYPOINT`'s \"exec\" form, not its \"shell\" form. If you do not specify this field, then the container's `ENTRYPOINT` runs, in conjunction with the args field or the container's [`CMD`](https://docs.docker.com/engine/reference/builder/#cmd), if either exists. If this field is not specified and the container does not have an `ENTRYPOINT`, then refer to the [Docker documentation about how `CMD` and `ENTRYPOINT` interact](https://docs.docker.com/engine/reference/builder/#understand-how-cmd-and-entrypoint-interact). If you specify this field, then you can also specify the `args` field to provide additional arguments for this command. However, if you specify this field, then the container's `CMD` is ignored. See the [Kubernetes documentation about how the `command` and `args` fields interact with a container's `ENTRYPOINT` and `CMD`](https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#notes). In this field, you can reference [environment variables set by AI Platform Prediction](/ai-platform/prediction/docs/custom-container-requirements#aip-variables) and environment variables set in the env field. You cannot reference environment variables set in the Docker image. In order for environment variables to be expanded, reference them by using the following syntax: $( VARIABLE_NAME) Note that this differs from Bash variable expansion, which does not use parentheses. If a variable cannot be resolved, the reference in the input string is used unchanged. To avoid variable expansion, you can escape this syntax with `$$`; for example: $$(VARIABLE_NAME) This field corresponds to the `command` field of the [Kubernetes Containers v1 core API](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.18/#container-v1-core)."]
    pub command: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "env")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. List of environment variables to set in the container. After the container starts running, code running in the container can read these environment variables. Additionally, the command and args fields can reference these variables. Later entries in this list can also reference earlier entries. For example, the following example sets the variable `VAR_2` to have the value `foo bar`: ```json [ { \"name\": \"VAR_1\", \"value\": \"foo\" }, { \"name\": \"VAR_2\", \"value\": \"$(VAR_1) bar\" } ] ``` If you switch the order of the variables in the example, then the expansion does not occur. This field corresponds to the `env` field of the [Kubernetes Containers v1 core API](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.18/#container-v1-core)."]
    pub env: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1EnvVar>>>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of the Docker image to be used as the custom container for serving predictions. This URI must identify [an image in Artifact Registry](/artifact-registry/docs/overview) and begin with the hostname `{REGION}-docker.pkg.dev`, where `{REGION}` is replaced by the region that matches AI Platform Prediction [regional endpoint](/ai-platform/prediction/docs/regional-endpoints) that you are using. For example, if you are using the `us-central1-ml.googleapis.com` endpoint, then this URI must begin with `us-central1-docker.pkg.dev`. To use a custom container, the [AI Platform Google-managed service account](/ai-platform/prediction/docs/custom-service-account#default) must have permission to pull (read) the Docker image at this URI. The AI Platform Google-managed service account has the following format: `service-{PROJECT_NUMBER}@cloud-ml.google.com.iam.gserviceaccount.com` {PROJECT_NUMBER} is replaced by your Google Cloud project number. By default, this service account has necessary permissions to pull an Artifact Registry image in the same Google Cloud project where you are using AI Platform Prediction. In this case, no configuration is necessary. If you want to use an image from a different Google Cloud project, learn how to [grant the Artifact Registry Reader (roles/artifactregistry.reader) role for a repository](/artifact-registry/docs/access-control#grant-repo) to your projet's AI Platform Google-managed service account. To learn about the requirements for the Docker image itself, read [Custom container requirements](/ai-platform/prediction/docs/custom-container-requirements)."]
    pub image: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ports")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. List of ports to expose from the container. AI Platform Prediction sends any prediction requests that it receives to the first port on this list. AI Platform Prediction also sends [liveness and health checks](/ai-platform/prediction/docs/custom-container-requirements#health) to this port. If you do not specify this field, it defaults to following value: ```json [ { \"containerPort\": 8080 } ] ``` AI Platform Prediction does not use ports other than the first one listed. This field corresponds to the `ports` field of the [Kubernetes Containers v1 core API](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.18/#container-v1-core)."]
    pub ports:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1ContainerPort>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the config of disk options."]
pub struct GoogleCloudMlV1DiskConfig {
    #[serde(rename = "bootDiskSizeGb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size in GB of the boot disk (default is 100GB)."]
    pub boot_disk_size_gb: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "bootDiskType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the boot disk (default is \"pd-ssd\"). Valid values: \"pd-ssd\" (Persistent Disk Solid State Drive) or \"pd-standard\" (Persistent Disk Hard Disk Drive)."]
    pub boot_disk_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a custom encryption key configuration that can be applied to a resource."]
pub struct GoogleCloudMlV1EncryptionConfig {
    #[serde(rename = "kmsKeyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Cloud KMS resource identifier of the customer-managed encryption key used to protect a resource, such as a training job. It has the following format: `projects/{PROJECT_ID}/locations/{REGION}/keyRings/{KEY_RING_NAME}/cryptoKeys/{KEY_NAME}`"]
    pub kms_key_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an environment variable to be made available in a container. This message is a subset of the [Kubernetes EnvVar v1 core specification](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.18/#envvar-v1-core)."]
pub struct GoogleCloudMlV1EnvVar {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the environment variable. Must be a [valid C identifier](https://github.com/kubernetes/kubernetes/blob/v1.18.8/staging/src/k8s.io/apimachinery/pkg/util/validation/validation.go#L258) and must not begin with the prefix `AIP_`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value of the environment variable. Defaults to an empty string. In this field, you can reference [environment variables set by AI Platform Prediction](/ai-platform/prediction/docs/custom-container-requirements#aip-variables) and environment variables set earlier in the same env field as where this message occurs. You cannot reference environment variables set in the Docker image. In order for environment variables to be expanded, reference them by using the following syntax: $(VARIABLE_NAME) Note that this differs from Bash variable expansion, which does not use parentheses. If a variable cannot be resolved, the reference in the input string is used unchanged. To avoid variable expansion, you can escape this syntax with `$$`; for example: $$(VARIABLE_NAME)"]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for explanations to be issued against a trained model."]
pub struct GoogleCloudMlV1ExplainRequest {
    #[serde(rename = "httpBody")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The explanation request body."]
    pub http_body: ::std::option::Option<::std::boxed::Box<GoogleApiHttpBody>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message holding configuration options for explaining model predictions. There are three feature attribution methods supported for TensorFlow models: integrated gradients, sampled Shapley, and XRAI. [Learn more about feature attributions.](/ai-platform/prediction/docs/ai-explanations/overview)"]
pub struct GoogleCloudMlV1ExplanationConfig {
    #[serde(rename = "integratedGradientsAttribution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Attributes credit by computing the Aumann-Shapley value taking advantage of the model's fully differentiable structure. Refer to this paper for more details: https://arxiv.org/abs/1703.01365"]
    pub integrated_gradients_attribution:
        ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1IntegratedGradientsAttribution>>,
    #[serde(rename = "sampledShapleyAttribution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An attribution method that approximates Shapley values for features that contribute to the label being predicted. A sampling strategy is used to approximate the value rather than considering all subsets of features."]
    pub sampled_shapley_attribution:
        ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1SampledShapleyAttribution>>,
    #[serde(rename = "xraiAttribution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Attributes credit by computing the XRAI taking advantage of the model's fully differentiable structure. Refer to this paper for more details: https://arxiv.org/abs/1906.02825 Currently only implemented for models with natural image inputs."]
    pub xrai_attribution: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1XraiAttribution>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Returns service account information associated with a project."]
pub struct GoogleCloudMlV1GetConfigResponse {
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub config: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1Config>>,
    #[serde(rename = "serviceAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The service account Cloud ML uses to access resources in the project."]
    pub service_account: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceAccountProject")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The project number for `service_account`."]
    pub service_account_project: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the result of a single hyperparameter tuning trial from a training job. The TrainingOutput object that is returned on successful completion of a training job with hyperparameter tuning includes a list of HyperparameterOutput objects, one for each successful trial."]
pub struct GoogleCloudMlV1HyperparameterOutput {
    #[serde(rename = "allMetrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All recorded object metrics for this trial. This field is not currently populated."]
    pub all_metrics: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1HyperparameterOutputHyperparameterMetric>>,
    >,
    #[serde(rename = "builtInAlgorithmOutput")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details related to built-in algorithms jobs. Only set for trials of built-in algorithms jobs that have succeeded."]
    pub built_in_algorithm_output:
        ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1BuiltInAlgorithmOutput>>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. End time for the trial."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "finalMetric")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The final objective metric seen for this trial."]
    pub final_metric: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudMlV1HyperparameterOutputHyperparameterMetric>,
    >,
    #[serde(rename = "hyperparameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The hyperparameters given to this trial."]
    pub hyperparameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "isTrialStoppedEarly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if the trial is stopped early."]
    pub is_trial_stopped_early: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Start time for the trial."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The detailed state of the trial."]
    pub state: ::std::option::Option<GoogleCloudMlV1HyperparameterOutputStateEnum>,
    #[serde(rename = "trialId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The trial id for these results."]
    pub trial_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The detailed state of the trial."]
pub enum GoogleCloudMlV1HyperparameterOutputStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The job state is unspecified."]
    StateUnspecified,
    #[serde(rename = "QUEUED")]
    #[doc = "The job has been just created and processing has not yet begun."]
    Queued,
    #[serde(rename = "PREPARING")]
    #[doc = "The service is preparing to run the job."]
    Preparing,
    #[serde(rename = "RUNNING")]
    #[doc = "The job is in progress."]
    Running,
    #[serde(rename = "SUCCEEDED")]
    #[doc = "The job completed successfully."]
    Succeeded,
    #[serde(rename = "FAILED")]
    #[doc = "The job failed. `error_message` should contain the details of the failure."]
    Failed,
    #[serde(rename = "CANCELLING")]
    #[doc = "The job is being cancelled. `error_message` should describe the reason for the cancellation."]
    Cancelling,
    #[serde(rename = "CANCELLED")]
    #[doc = "The job has been cancelled. `error_message` should describe the reason for the cancellation."]
    Cancelled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a set of hyperparameters to optimize."]
pub struct GoogleCloudMlV1HyperparameterSpec {
    #[serde(rename = "algorithm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The search algorithm specified for the hyperparameter tuning job. Uses the default AI Platform hyperparameter tuning algorithm if unspecified."]
    pub algorithm: ::std::option::Option<GoogleCloudMlV1HyperparameterSpecAlgorithmEnum>,
    #[serde(rename = "enableTrialEarlyStopping")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates if the hyperparameter tuning job enables auto trial early stopping."]
    pub enable_trial_early_stopping: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "goal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of goal to use for tuning. Available types are `MAXIMIZE` and `MINIMIZE`. Defaults to `MAXIMIZE`."]
    pub goal: ::std::option::Option<GoogleCloudMlV1HyperparameterSpecGoalEnum>,
    #[serde(rename = "hyperparameterMetricTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The TensorFlow summary tag name to use for optimizing trials. For current versions of TensorFlow, this tag name should exactly match what is shown in TensorBoard, including all scopes. For versions of TensorFlow prior to 0.12, this should be only the tag passed to tf.Summary. By default, \"training/hptuning/metric\" will be used."]
    pub hyperparameter_metric_tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxFailedTrials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The number of failed trials that need to be seen before failing the hyperparameter tuning job. You can specify this field to override the default failing criteria for AI Platform hyperparameter tuning jobs. Defaults to zero, which means the service decides when a hyperparameter job should fail."]
    pub max_failed_trials: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "maxParallelTrials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The number of training trials to run concurrently. You can reduce the time it takes to perform hyperparameter tuning by adding trials in parallel. However, each trail only benefits from the information gained in completed trials. That means that a trial does not get access to the results of trials running at the same time, which could reduce the quality of the overall optimization. Each trial will use the same scale tier and machine types. Defaults to one."]
    pub max_parallel_trials: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "maxTrials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. How many training trials should be attempted to optimize the specified hyperparameters. Defaults to one."]
    pub max_trials: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "params")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The set of parameters to tune."]
    pub params:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1ParameterSpec>>>,
    #[serde(rename = "resumePreviousJobId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The prior hyperparameter tuning job id that users hope to continue with. The job id will be used to find the corresponding vizier study guid and resume the study."]
    pub resume_previous_job_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. The search algorithm specified for the hyperparameter tuning job. Uses the default AI Platform hyperparameter tuning algorithm if unspecified."]
pub enum GoogleCloudMlV1HyperparameterSpecAlgorithmEnum {
    #[serde(rename = "ALGORITHM_UNSPECIFIED")]
    #[doc = "The default algorithm used by the hyperparameter tuning service. This is a Bayesian optimization algorithm."]
    AlgorithmUnspecified,
    #[serde(rename = "GRID_SEARCH")]
    #[doc = "Simple grid search within the feasible space. To use grid search, all parameters must be `INTEGER`, `CATEGORICAL`, or `DISCRETE`."]
    GridSearch,
    #[serde(rename = "RANDOM_SEARCH")]
    #[doc = "Simple random search within the feasible space."]
    RandomSearch,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of goal to use for tuning. Available types are `MAXIMIZE` and `MINIMIZE`. Defaults to `MAXIMIZE`."]
pub enum GoogleCloudMlV1HyperparameterSpecGoalEnum {
    #[serde(rename = "GOAL_TYPE_UNSPECIFIED")]
    #[doc = "Goal Type will default to maximize."]
    GoalTypeUnspecified,
    #[serde(rename = "MAXIMIZE")]
    #[doc = "Maximize the goal metric."]
    Maximize,
    #[serde(rename = "MINIMIZE")]
    #[doc = "Minimize the goal metric."]
    Minimize,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Attributes credit by computing the Aumann-Shapley value taking advantage of the model's fully differentiable structure. Refer to this paper for more details: https://arxiv.org/abs/1703.01365"]
pub struct GoogleCloudMlV1IntegratedGradientsAttribution {
    #[serde(rename = "numIntegralSteps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of steps for approximating the path integral. A good value to start is 50 and gradually increase until the sum to diff property is met within the desired error range."]
    pub num_integral_steps: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a training or prediction job."]
pub struct GoogleCloudMlV1Job {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. When the job was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. When the job processing was completed."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The details of a failure or a cancellation."]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a job from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform job updates in order to avoid race conditions: An `etag` is returned in the response to `GetJob`, and systems are expected to put that etag in the request to `UpdateJob` to ensure that their change will be applied to the same version of the job."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The user-specified id of the job."]
    pub job_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. One or more labels that you can add, to organize your jobs. Each label is a key-value pair, where both the key and the value are arbitrary strings that you supply. For more information, see the documentation on using labels."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "predictionInput")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input parameters to create a prediction job."]
    pub prediction_input: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1PredictionInput>>,
    #[serde(rename = "predictionOutput")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current prediction job result."]
    pub prediction_output:
        ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1PredictionOutput>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. When the job processing was started."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The detailed state of a job."]
    pub state: ::std::option::Option<GoogleCloudMlV1JobStateEnum>,
    #[serde(rename = "trainingInput")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input parameters to create a training job."]
    pub training_input: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1TrainingInput>>,
    #[serde(rename = "trainingOutput")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current training job result."]
    pub training_output: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1TrainingOutput>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The detailed state of a job."]
pub enum GoogleCloudMlV1JobStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The job state is unspecified."]
    StateUnspecified,
    #[serde(rename = "QUEUED")]
    #[doc = "The job has been just created and processing has not yet begun."]
    Queued,
    #[serde(rename = "PREPARING")]
    #[doc = "The service is preparing to run the job."]
    Preparing,
    #[serde(rename = "RUNNING")]
    #[doc = "The job is in progress."]
    Running,
    #[serde(rename = "SUCCEEDED")]
    #[doc = "The job completed successfully."]
    Succeeded,
    #[serde(rename = "FAILED")]
    #[doc = "The job failed. `error_message` should contain the details of the failure."]
    Failed,
    #[serde(rename = "CANCELLING")]
    #[doc = "The job is being cancelled. `error_message` should describe the reason for the cancellation."]
    Cancelling,
    #[serde(rename = "CANCELLED")]
    #[doc = "The job has been cancelled. `error_message` should describe the reason for the cancellation."]
    Cancelled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the ListJobs method."]
pub struct GoogleCloudMlV1ListJobsResponse {
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of jobs."]
    pub jobs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1Job>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Pass this token as the `page_token` field of the request for a subsequent call."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudMlV1ListLocationsResponse {
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations where at least one type of CMLE capability is available."]
    pub locations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1Location>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Pass this token as the `page_token` field of the request for a subsequent call."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the ListModels method."]
pub struct GoogleCloudMlV1ListModelsResponse {
    #[serde(rename = "models")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of models."]
    pub models: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1Model>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Pass this token as the `page_token` field of the request for a subsequent call."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for the ListTrials service method."]
pub struct GoogleCloudMlV1ListOptimalTrialsRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for the ListOptimalTrials method."]
pub struct GoogleCloudMlV1ListOptimalTrialsResponse {
    #[serde(rename = "trials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pareto-optimal trials for multiple objective study or the optimal trial for single objective study. The definition of pareto-optimal can be checked in wiki page. https://en.wikipedia.org/wiki/Pareto_efficiency"]
    pub trials: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1Trial>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudMlV1ListStudiesResponse {
    #[serde(rename = "studies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The studies associated with the project."]
    pub studies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1Study>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for the ListTrials method."]
pub struct GoogleCloudMlV1ListTrialsResponse {
    #[serde(rename = "trials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The trials associated with the study."]
    pub trials: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1Trial>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the ListVersions method."]
pub struct GoogleCloudMlV1ListVersionsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Pass this token as the `page_token` field of the request for a subsequent call."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "versions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of versions."]
    pub versions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1Version>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudMlV1Location {
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Capabilities available in the location."]
    pub capabilities:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1Capability>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for manually scaling a model."]
pub struct GoogleCloudMlV1ManualScaling {
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of nodes to allocate for this model. These nodes are always up, starting from the time the model is deployed, so the cost of operating this model will be proportional to `nodes` * number of hours since last billing cycle plus the cost for each prediction performed."]
    pub nodes: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message representing a measurement."]
pub struct GoogleCloudMlV1Measurement {
    #[serde(rename = "elapsedTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time that the trial has been running at the point of this measurement."]
    pub elapsed_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Provides a list of metrics that act as inputs into the objective function."]
    pub metrics:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1MeasurementMetric>>>,
    #[serde(rename = "stepCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of steps a machine learning model has been trained for. Must be non-negative."]
    pub step_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "MetricSpec contains the specifications to use to calculate the desired nodes count when autoscaling is enabled."]
pub struct GoogleCloudMlV1MetricSpec {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "metric name."]
    pub name: ::std::option::Option<GoogleCloudMlV1MetricSpecNameEnum>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target specifies the target value for the given metric; once real metric deviates from the threshold by a certain percentage, the node count changes."]
    pub target: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "metric name."]
pub enum GoogleCloudMlV1MetricSpecNameEnum {
    #[serde(rename = "METRIC_NAME_UNSPECIFIED")]
    #[doc = "Unspecified MetricName."]
    MetricNameUnspecified,
    #[serde(rename = "CPU_USAGE")]
    #[doc = "CPU usage."]
    CpuUsage,
    #[serde(rename = "GPU_DUTY_CYCLE")]
    #[doc = "GPU duty cycle."]
    GpuDutyCycle,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a machine learning solution. A model can have multiple versions, each of which is a deployed, trained model ready to receive prediction requests. The model itself is just a container."]
pub struct GoogleCloudMlV1Model {
    #[serde(rename = "defaultVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The default version of the model. This version will be used to handle prediction requests that do not specify a version. You can change the default version by calling projects.models.versions.setDefault."]
    pub default_version: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1Version>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The description specified for the model when it was created."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a model from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform model updates in order to avoid race conditions: An `etag` is returned in the response to `GetModel`, and systems are expected to put that etag in the request to `UpdateModel` to ensure that their change will be applied to the model as intended."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. One or more labels that you can add, to organize your models. Each label is a key-value pair, where both the key and the value are arbitrary strings that you supply. For more information, see the documentation on using labels."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name specified for the model when it was created. The model name must be unique within the project it is created in."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "onlinePredictionConsoleLogging")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If true, online prediction nodes send `stderr` and `stdout` streams to Cloud Logging. These can be more verbose than the standard access logs (see `onlinePredictionLogging`) and can incur higher cost. However, they are helpful for debugging. Note that [logs may incur a cost](/stackdriver/pricing), especially if your project receives prediction requests at a high QPS. Estimate your costs before enabling this option. Default is false."]
    pub online_prediction_console_logging: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "onlinePredictionLogging")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If true, online prediction access logs are sent to Cloud Logging. These logs are like standard server access logs, containing information like timestamp and latency for each request. Note that [logs may incur a cost](/stackdriver/pricing), especially if your project receives prediction requests at a high queries per second rate (QPS). Estimate your costs before enabling this option. Default is false."]
    pub online_prediction_logging: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The list of regions where the model is going to be deployed. Only one region per model is supported. Defaults to 'us-central1' if nothing is set. See the available regions for AI Platform services. Note: * No matter where a model is deployed, it can always be accessed by users from anywhere, both for online and batch prediction. * The region for a batch prediction job is set by the region field when submitting the batch prediction job and does not take its value from this field."]
    pub regions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the metadata of the long-running operation."]
pub struct GoogleCloudMlV1OperationMetadata {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time the operation was submitted."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time operation processing completed."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isCancellationRequested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether a request to cancel this operation has been made."]
    pub is_cancellation_requested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user labels, inherited from the model or the model version being operated on."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "modelName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains the name of the model associated with the operation."]
    pub model_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The operation type."]
    pub operation_type: ::std::option::Option<GoogleCloudMlV1OperationMetadataOperationTypeEnum>,
    #[serde(rename = "projectNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains the project number associated with the operation."]
    pub project_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time operation processing started."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains the version associated with the operation."]
    pub version: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1Version>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The operation type."]
pub enum GoogleCloudMlV1OperationMetadataOperationTypeEnum {
    #[serde(rename = "OPERATION_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified operation type."]
    OperationTypeUnspecified,
    #[serde(rename = "CREATE_VERSION")]
    #[doc = "An operation to create a new version."]
    CreateVersion,
    #[serde(rename = "DELETE_VERSION")]
    #[doc = "An operation to delete an existing version."]
    DeleteVersion,
    #[serde(rename = "DELETE_MODEL")]
    #[doc = "An operation to delete an existing model."]
    DeleteModel,
    #[serde(rename = "UPDATE_MODEL")]
    #[doc = "An operation to update an existing model."]
    UpdateModel,
    #[serde(rename = "UPDATE_VERSION")]
    #[doc = "An operation to update an existing version."]
    UpdateVersion,
    #[serde(rename = "UPDATE_CONFIG")]
    #[doc = "An operation to update project configuration."]
    UpdateConfig,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a single hyperparameter to optimize."]
pub struct GoogleCloudMlV1ParameterSpec {
    #[serde(rename = "categoricalValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required if type is `CATEGORICAL`. The list of possible categories."]
    pub categorical_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "discreteValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required if type is `DISCRETE`. A list of feasible points. The list should be in strictly increasing order. For instance, this parameter might have possible settings of 1.5, 2.5, and 4.0. This list should not contain more than 1,000 values."]
    pub discrete_values: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
    #[serde(rename = "maxValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required if type is `DOUBLE` or `INTEGER`. This field should be unset if type is `CATEGORICAL`. This value should be integers if type is `INTEGER`."]
    pub max_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "minValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required if type is `DOUBLE` or `INTEGER`. This field should be unset if type is `CATEGORICAL`. This value should be integers if type is INTEGER."]
    pub min_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "parameterName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The parameter name must be unique amongst all ParameterConfigs in a HyperparameterSpec message. E.g., \"learning_rate\"."]
    pub parameter_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scaleType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. How the parameter should be scaled to the hypercube. Leave unset for categorical parameters. Some kind of scaling is strongly recommended for real or integral parameters (e.g., `UNIT_LINEAR_SCALE`)."]
    pub scale_type: ::std::option::Option<GoogleCloudMlV1ParameterSpecScaleTypeEnum>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of the parameter."]
    pub _type: ::std::option::Option<GoogleCloudMlV1ParameterSpecTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. How the parameter should be scaled to the hypercube. Leave unset for categorical parameters. Some kind of scaling is strongly recommended for real or integral parameters (e.g., `UNIT_LINEAR_SCALE`)."]
pub enum GoogleCloudMlV1ParameterSpecScaleTypeEnum {
    #[serde(rename = "NONE")]
    #[doc = "By default, no scaling is applied."]
    None,
    #[serde(rename = "UNIT_LINEAR_SCALE")]
    #[doc = "Scales the feasible space to (0, 1) linearly."]
    UnitLinearScale,
    #[serde(rename = "UNIT_LOG_SCALE")]
    #[doc = "Scales the feasible space logarithmically to (0, 1). The entire feasible space must be strictly positive."]
    UnitLogScale,
    #[serde(rename = "UNIT_REVERSE_LOG_SCALE")]
    #[doc = "Scales the feasible space \"reverse\" logarithmically to (0, 1). The result is that values close to the top of the feasible space are spread out more than points near the bottom. The entire feasible space must be strictly positive."]
    UnitReverseLogScale,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of the parameter."]
pub enum GoogleCloudMlV1ParameterSpecTypeEnum {
    #[serde(rename = "PARAMETER_TYPE_UNSPECIFIED")]
    #[doc = "You must specify a valid type. Using this unspecified type will result in an error."]
    ParameterTypeUnspecified,
    #[serde(rename = "DOUBLE")]
    #[doc = "Type for real-valued parameters."]
    Double,
    #[serde(rename = "INTEGER")]
    #[doc = "Type for integral parameters."]
    Integer,
    #[serde(rename = "CATEGORICAL")]
    #[doc = "The parameter is categorical, with a value chosen from the categories field."]
    Categorical,
    #[serde(rename = "DISCRETE")]
    #[doc = "The parameter is real valued, with a fixed set of feasible points. If `type==DISCRETE`, feasible_points must be provided, and {`min_value`, `max_value`} will be ignored."]
    Discrete,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for predictions to be issued against a trained model."]
pub struct GoogleCloudMlV1PredictRequest {
    #[serde(rename = "httpBody")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = " Required. The prediction request body. Refer to the [request body details section](#request-body-details) for more information on how to structure your request."]
    pub http_body: ::std::option::Option<::std::boxed::Box<GoogleApiHttpBody>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents input parameters for a prediction job."]
pub struct GoogleCloudMlV1PredictionInput {
    #[serde(rename = "batchSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Number of records per batch, defaults to 64. The service will buffer batch_size number of records in memory before invoking one Tensorflow prediction call internally. So take the record size and memory available into consideration when setting this parameter."]
    pub batch_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dataFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The format of the input data files."]
    pub data_format: ::std::option::Option<GoogleCloudMlV1PredictionInputDataFormatEnum>,
    #[serde(rename = "inputPaths")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The Cloud Storage location of the input data files. May contain wildcards."]
    pub input_paths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "maxWorkerCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The maximum number of workers to be used for parallel processing. Defaults to 10 if not specified."]
    pub max_worker_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "modelName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Use this field if you want to use the default version for the specified model. The string must use the following format: `\"projects/YOUR_PROJECT/models/YOUR_MODEL\"`"]
    pub model_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outputDataFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Format of the output data files, defaults to JSON."]
    pub output_data_format:
        ::std::option::Option<GoogleCloudMlV1PredictionInputOutputDataFormatEnum>,
    #[serde(rename = "outputPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The output Google Cloud Storage location."]
    pub output_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The Google Compute Engine region to run the prediction job in. See the available regions for AI Platform services."]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "runtimeVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The AI Platform runtime version to use for this batch prediction. If not set, AI Platform will pick the runtime version used during the CreateVersion request for this model version, or choose the latest stable version when model version information is not available such as when the model is specified by uri."]
    pub runtime_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "signatureName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The name of the signature defined in the SavedModel to use for this job. Please refer to [SavedModel](https://tensorflow.github.io/serving/serving_basic.html) for information about how to use signatures. Defaults to [DEFAULT_SERVING_SIGNATURE_DEF_KEY](https://www.tensorflow.org/api_docs/python/tf/saved_model/signature_constants) , which is \"serving_default\"."]
    pub signature_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Use this field if you want to specify a Google Cloud Storage path for the model to use."]
    pub uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "versionName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Use this field if you want to specify a version of the model to use. The string is formatted the same way as `model_version`, with the addition of the version information: `\"projects/YOUR_PROJECT/models/YOUR_MODEL/versions/YOUR_VERSION\"`"]
    pub version_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The format of the input data files."]
pub enum GoogleCloudMlV1PredictionInputDataFormatEnum {
    #[serde(rename = "DATA_FORMAT_UNSPECIFIED")]
    #[doc = "Unspecified format."]
    DataFormatUnspecified,
    #[serde(rename = "JSON")]
    #[doc = "Each line of the file is a JSON dictionary representing one record."]
    Json,
    #[serde(rename = "TEXT")]
    #[doc = "Deprecated. Use JSON instead."]
    Text,
    #[serde(rename = "TF_RECORD")]
    #[doc = "The source file is a TFRecord file. Currently available only for input data."]
    TfRecord,
    #[serde(rename = "TF_RECORD_GZIP")]
    #[doc = "The source file is a GZIP-compressed TFRecord file. Currently available only for input data."]
    TfRecordGzip,
    #[serde(rename = "CSV")]
    #[doc = "Values are comma-separated rows, with keys in a separate file. Currently available only for output data."]
    Csv,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Format of the output data files, defaults to JSON."]
pub enum GoogleCloudMlV1PredictionInputOutputDataFormatEnum {
    #[serde(rename = "DATA_FORMAT_UNSPECIFIED")]
    #[doc = "Unspecified format."]
    DataFormatUnspecified,
    #[serde(rename = "JSON")]
    #[doc = "Each line of the file is a JSON dictionary representing one record."]
    Json,
    #[serde(rename = "TEXT")]
    #[doc = "Deprecated. Use JSON instead."]
    Text,
    #[serde(rename = "TF_RECORD")]
    #[doc = "The source file is a TFRecord file. Currently available only for input data."]
    TfRecord,
    #[serde(rename = "TF_RECORD_GZIP")]
    #[doc = "The source file is a GZIP-compressed TFRecord file. Currently available only for input data."]
    TfRecordGzip,
    #[serde(rename = "CSV")]
    #[doc = "Values are comma-separated rows, with keys in a separate file. Currently available only for output data."]
    Csv,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents results of a prediction job."]
pub struct GoogleCloudMlV1PredictionOutput {
    #[serde(rename = "errorCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of data instances which resulted in errors."]
    pub error_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nodeHours")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Node hours used by the batch prediction job."]
    pub node_hours: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "outputPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The output Google Cloud Storage location provided at the job creation time."]
    pub output_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "predictionCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of generated predictions."]
    pub prediction_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the configuration for a replica in a cluster."]
pub struct GoogleCloudMlV1ReplicaConfig {
    #[serde(rename = "acceleratorConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents the type and number of accelerators used by the replica. [Learn about restrictions on accelerator configurations for training.](/ai-platform/training/docs/using-gpus#compute-engine-machine-types-with-gpu)"]
    pub accelerator_config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1AcceleratorConfig>>,
    #[serde(rename = "containerArgs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Arguments to the entrypoint command. The following rules apply for container_command and container_args: - If you do not supply command or args: The defaults defined in the Docker image are used. - If you supply a command but no args: The default EntryPoint and the default Cmd defined in the Docker image are ignored. Your command is run without any arguments. - If you supply only args: The default Entrypoint defined in the Docker image is run with the args that you supplied. - If you supply a command and args: The default Entrypoint and the default Cmd defined in the Docker image are ignored. Your command is run with your args. It cannot be set if custom container image is not provided. Note that this field and [TrainingInput.args] are mutually exclusive, i.e., both cannot be set at the same time."]
    pub container_args: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "containerCommand")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The command with which the replica's custom container is run. If provided, it will override default ENTRYPOINT of the docker image. If not provided, the docker image's ENTRYPOINT is used. It cannot be set if custom container image is not provided. Note that this field and [TrainingInput.args] are mutually exclusive, i.e., both cannot be set at the same time."]
    pub container_command: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "diskConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents the configuration of disk options."]
    pub disk_config: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1DiskConfig>>,
    #[serde(rename = "imageUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Docker image to run on the replica. This image must be in Container Registry. Learn more about [configuring custom containers](/ai-platform/training/docs/distributed-training-containers)."]
    pub image_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tpuTfVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The AI Platform runtime version that includes a TensorFlow version matching the one used in the custom container. This field is required if the replica is a TPU worker that uses a custom container. Otherwise, do not specify this field. This must be a [runtime version that currently supports training with TPUs](/ml-engine/docs/tensorflow/runtime-version-list#tpu-support). Note that the version of TensorFlow included in a runtime version may differ from the numbering of the runtime version itself, because it may have a different [patch version](https://www.tensorflow.org/guide/version_compat#semantic_versioning_20). In this field, you must specify the runtime version (TensorFlow minor version). For example, if your custom container runs TensorFlow `1.x.y`, specify `1.x`."]
    pub tpu_tf_version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for logging request-response pairs to a BigQuery table. Online prediction requests to a model version and the responses to these requests are converted to raw strings and saved to the specified BigQuery table. Logging is constrained by [BigQuery quotas and limits](/bigquery/quotas). If your project exceeds BigQuery quotas or limits, AI Platform Prediction does not log request-response pairs, but it continues to serve predictions. If you are using [continuous evaluation](/ml-engine/docs/continuous-evaluation/), you do not need to specify this configuration manually. Setting up continuous evaluation automatically enables logging of request-response pairs."]
pub struct GoogleCloudMlV1RequestLoggingConfig {
    #[serde(rename = "bigqueryTableName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Fully qualified BigQuery table name in the following format: \" project_id.dataset_name.table_name\" The specified table must already exist, and the \"Cloud ML Service Agent\" for your project must have permission to write to it. The table must have the following [schema](/bigquery/docs/schemas): Field nameType Mode model STRING REQUIRED model_version STRING REQUIRED time TIMESTAMP REQUIRED raw_data STRING REQUIRED raw_prediction STRING NULLABLE groundtruth STRING NULLABLE "]
    pub bigquery_table_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "samplingPercentage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Percentage of requests to be logged, expressed as a fraction from 0 to 1. For example, if you want to log 10% of requests, enter `0.1`. The sampling window is the lifetime of the model version. Defaults to 0."]
    pub sampling_percentage: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies HTTP paths served by a custom container. AI Platform Prediction sends requests to these paths on the container; the custom container must run an HTTP server that responds to these requests with appropriate responses. Read [Custom container requirements](/ai-platform/prediction/docs/custom-container-requirements) for details on how to create your container image to meet these requirements."]
pub struct GoogleCloudMlV1RouteMap {
    #[serde(rename = "health")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP path on the container to send health checkss to. AI Platform Prediction intermittently sends GET requests to this path on the container's IP address and port to check that the container is healthy. Read more about [health checks](/ai-platform/prediction/docs/custom-container-requirements#checks). For example, if you set this field to `/bar`, then AI Platform Prediction intermittently sends a GET request to the `/bar` path on the port of your container specified by the first value of Version.container.ports. If you don't specify this field, it defaults to the following value: /v1/models/ MODEL/versions/VERSION The placeholders in this value are replaced as follows: * MODEL: The name of the parent Model. This does not include the \"projects/PROJECT_ID/models/\" prefix that the API returns in output; it is the bare model name, as provided to projects.models.create. * VERSION: The name of the model version. This does not include the \"projects/PROJECT_ID /models/MODEL/versions/\" prefix that the API returns in output; it is the bare version name, as provided to projects.models.versions.create."]
    pub health: ::std::option::Option<::std::string::String>,
    #[serde(rename = "predict")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP path on the container to send prediction requests to. AI Platform Prediction forwards requests sent using projects.predict to this path on the container's IP address and port. AI Platform Prediction then returns the container's response in the API response. For example, if you set this field to `/foo`, then when AI Platform Prediction receives a prediction request, it forwards the request body in a POST request to the `/foo` path on the port of your container specified by the first value of Version.container.ports. If you don't specify this field, it defaults to the following value: /v1/models/MODEL/versions/VERSION:predict The placeholders in this value are replaced as follows: * MODEL: The name of the parent Model. This does not include the \"projects/PROJECT_ID/models/\" prefix that the API returns in output; it is the bare model name, as provided to projects.models.create. * VERSION: The name of the model version. This does not include the \"projects/PROJECT_ID/models/MODEL/versions/\" prefix that the API returns in output; it is the bare version name, as provided to projects.models.versions.create."]
    pub predict: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An attribution method that approximates Shapley values for features that contribute to the label being predicted. A sampling strategy is used to approximate the value rather than considering all subsets of features."]
pub struct GoogleCloudMlV1SampledShapleyAttribution {
    #[serde(rename = "numPaths")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of feature permutations to consider when approximating the Shapley values."]
    pub num_paths: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "All parameters related to scheduling of training jobs."]
pub struct GoogleCloudMlV1Scheduling {
    #[serde(rename = "maxRunningTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The maximum job running time, expressed in seconds. The field can contain up to nine fractional digits, terminated by `s`. If not specified, this field defaults to `604800s` (seven days). If the training job is still running after this duration, AI Platform Training cancels it. The duration is measured from when the job enters the `RUNNING` state; therefore it does not overlap with the duration limited by Scheduling.max_wait_time. For example, if you want to ensure your job runs for no more than 2 hours, set this field to `7200s` (2 hours * 60 minutes / hour * 60 seconds / minute). If you submit your training job using the `gcloud` tool, you can [specify this field in a `config.yaml` file](/ai-platform/training/docs/training-jobs#formatting_your_configuration_parameters). For example: ```yaml trainingInput: scheduling: maxRunningTime: 7200s ```"]
    pub max_running_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxWaitTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The maximum job wait time, expressed in seconds. The field can contain up to nine fractional digits, terminated by `s`. If not specified, there is no limit to the wait time. The minimum for this field is `1800s` (30 minutes). If the training job has not entered the `RUNNING` state after this duration, AI Platform Training cancels it. After the job begins running, it can no longer be cancelled due to the maximum wait time. Therefore the duration limited by this field does not overlap with the duration limited by Scheduling.max_running_time. For example, if the job temporarily stops running and retries due to a [VM restart](/ai-platform/training/docs/overview#restarts), this cannot lead to a maximum wait time cancellation. However, independently of this constraint, AI Platform Training might stop a job if there are too many retries due to exhausted resources in a region. The following example describes how you might use this field: To cancel your job if it doesn't start running within 1 hour, set this field to `3600s` (1 hour * 60 minutes / hour * 60 seconds / minute). If the job is still in the `QUEUED` or `PREPARING` state after an hour of waiting, AI Platform Training cancels the job. If you submit your training job using the `gcloud` tool, you can [specify this field in a `config.yaml` file](/ai-platform/training/docs/training-jobs#formatting_your_configuration_parameters). For example: ```yaml trainingInput: scheduling: maxWaitTime: 3600s ```"]
    pub max_wait_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for the SetDefaultVersion request."]
pub struct GoogleCloudMlV1SetDefaultVersionRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudMlV1StopTrialRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message representing a Study."]
pub struct GoogleCloudMlV1Study {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time at which the study was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inactiveReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A human readable reason why the Study is inactive. This should be empty if a study is ACTIVE or COMPLETED."]
    pub inactive_reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of a study."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The detailed state of a study."]
    pub state: ::std::option::Option<GoogleCloudMlV1StudyStateEnum>,
    #[serde(rename = "studyConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Configuration of the study."]
    pub study_config: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1StudyConfig>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The detailed state of a study."]
pub enum GoogleCloudMlV1StudyStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The study state is unspecified."]
    StateUnspecified,
    #[serde(rename = "ACTIVE")]
    #[doc = "The study is active."]
    Active,
    #[serde(rename = "INACTIVE")]
    #[doc = "The study is stopped due to an internal error."]
    Inactive,
    #[serde(rename = "COMPLETED")]
    #[doc = "The study is done when the service exhausts the parameter search space or max_trial_count is reached."]
    Completed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents configuration of a study."]
pub struct GoogleCloudMlV1StudyConfig {
    #[serde(rename = "algorithm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The search algorithm specified for the study."]
    pub algorithm: ::std::option::Option<GoogleCloudMlV1StudyConfigAlgorithmEnum>,
    #[serde(rename = "automatedStoppingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for automated stopping of unpromising Trials."]
    pub automated_stopping_config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1AutomatedStoppingConfig>>,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metric specs for the study."]
    pub metrics: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1StudyConfigMetricSpec>>,
    >,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The set of parameters to tune."]
    pub parameters: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1StudyConfigParameterSpec>>,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The search algorithm specified for the study."]
pub enum GoogleCloudMlV1StudyConfigAlgorithmEnum {
    #[serde(rename = "ALGORITHM_UNSPECIFIED")]
    #[doc = "The default algorithm used by the Cloud AI Platform Vizier service."]
    AlgorithmUnspecified,
    #[serde(rename = "GAUSSIAN_PROCESS_BANDIT")]
    #[doc = "Gaussian Process Bandit."]
    GaussianProcessBandit,
    #[serde(rename = "GRID_SEARCH")]
    #[doc = "Simple grid search within the feasible space. To use grid search, all parameters must be `INTEGER`, `CATEGORICAL`, or `DISCRETE`."]
    GridSearch,
    #[serde(rename = "RANDOM_SEARCH")]
    #[doc = "Simple random search within the feasible space."]
    RandomSearch,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata field of a google.longrunning.Operation associated with a SuggestTrialsRequest."]
pub struct GoogleCloudMlV1SuggestTrialsMetadata {
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of the client that is requesting the suggestion."]
    pub client_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time operation was submitted."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "study")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the study that the trial belongs to."]
    pub study: ::std::option::Option<::std::string::String>,
    #[serde(rename = "suggestionCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of suggestions requested."]
    pub suggestion_count: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for the SuggestTrial service method."]
pub struct GoogleCloudMlV1SuggestTrialsRequest {
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The identifier of the client that is requesting the suggestion. If multiple SuggestTrialsRequests have the same `client_id`, the service will return the identical suggested trial if the trial is pending, and provide a new trial if the last suggested trial was completed."]
    pub client_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "suggestionCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The number of suggestions requested."]
    pub suggestion_count: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This message will be placed in the response field of a completed google.longrunning.Operation associated with a SuggestTrials request."]
pub struct GoogleCloudMlV1SuggestTrialsResponse {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which operation processing completed."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the operation was started."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "studyState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of the study."]
    pub study_state: ::std::option::Option<GoogleCloudMlV1SuggestTrialsResponseStudyStateEnum>,
    #[serde(rename = "trials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of trials."]
    pub trials: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1Trial>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of the study."]
pub enum GoogleCloudMlV1SuggestTrialsResponseStudyStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The study state is unspecified."]
    StateUnspecified,
    #[serde(rename = "ACTIVE")]
    #[doc = "The study is active."]
    Active,
    #[serde(rename = "INACTIVE")]
    #[doc = "The study is stopped due to an internal error."]
    Inactive,
    #[serde(rename = "COMPLETED")]
    #[doc = "The study is done when the service exhausts the parameter search space or max_trial_count is reached."]
    Completed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents input parameters for a training job. When using the gcloud command to submit your training job, you can specify the input parameters as command-line arguments and/or in a YAML configuration file referenced from the --config command-line argument. For details, see the guide to [submitting a training job](/ai-platform/training/docs/training-jobs)."]
pub struct GoogleCloudMlV1TrainingInput {
    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Command-line arguments passed to the training application when it starts. If your job uses a custom container, then the arguments are passed to the container's `ENTRYPOINT` command."]
    pub args: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "encryptionConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Options for using customer-managed encryption keys (CMEK) to protect resources created by a training job, instead of using Google's default encryption. If this is set, then all resources created by the training job will be encrypted with the customer-managed encryption key that you specify. [Learn how and when to use CMEK with AI Platform Training](/ai-platform/training/docs/cmek)."]
    pub encryption_config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1EncryptionConfig>>,
    #[serde(rename = "evaluatorConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The configuration for evaluators. You should only set `evaluatorConfig.acceleratorConfig` if `evaluatorType` is set to a Compute Engine machine type. [Learn about restrictions on accelerator configurations for training.](/ai-platform/training/docs/using-gpus#compute-engine-machine-types-with-gpu) Set `evaluatorConfig.imageUri` only if you build a custom image for your evaluator. If `evaluatorConfig.imageUri` has not been set, AI Platform uses the value of `masterConfig.imageUri`. Learn more about [configuring custom containers](/ai-platform/training/docs/distributed-training-containers)."]
    pub evaluator_config: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1ReplicaConfig>>,
    #[serde(rename = "evaluatorCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The number of evaluator replicas to use for the training job. Each replica in the cluster will be of the type specified in `evaluator_type`. This value can only be used when `scale_tier` is set to `CUSTOM`. If you set this value, you must also set `evaluator_type`. The default value is zero."]
    pub evaluator_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "evaluatorType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies the type of virtual machine to use for your training job's evaluator nodes. The supported values are the same as those described in the entry for `masterType`. This value must be consistent with the category of machine type that `masterType` uses. In other words, both must be Compute Engine machine types or both must be legacy machine types. This value must be present when `scaleTier` is set to `CUSTOM` and `evaluatorCount` is greater than zero."]
    pub evaluator_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hyperparameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The set of Hyperparameters to tune."]
    pub hyperparameters:
        ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1HyperparameterSpec>>,
    #[serde(rename = "jobDir")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A Google Cloud Storage path in which to store training outputs and other data needed for training. This path is passed to your TensorFlow program as the '--job-dir' command-line argument. The benefit of specifying this field is that Cloud ML validates the path for use in training."]
    pub job_dir: ::std::option::Option<::std::string::String>,
    #[serde(rename = "masterConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The configuration for your master worker. You should only set `masterConfig.acceleratorConfig` if `masterType` is set to a Compute Engine machine type. Learn about [restrictions on accelerator configurations for training.](/ai-platform/training/docs/using-gpus#compute-engine-machine-types-with-gpu) Set `masterConfig.imageUri` only if you build a custom image. Only one of `masterConfig.imageUri` and `runtimeVersion` should be set. Learn more about [configuring custom containers](/ai-platform/training/docs/distributed-training-containers)."]
    pub master_config: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1ReplicaConfig>>,
    #[serde(rename = "masterType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies the type of virtual machine to use for your training job's master worker. You must specify this field when `scaleTier` is set to `CUSTOM`. You can use certain Compute Engine machine types directly in this field. See the [list of compatible Compute Engine machine types](/ai-platform/training/docs/machine-types#compute-engine-machine-types). Alternatively, you can use the certain legacy machine types in this field. See the [list of legacy machine types](/ai-platform/training/docs/machine-types#legacy-machine-types). Finally, if you want to use a TPU for training, specify `cloud_tpu` in this field. Learn more about the [special configuration options for training with TPUs](/ai-platform/training/docs/using-tpus#configuring_a_custom_tpu_machine)."]
    pub master_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "network")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The full name of the [Compute Engine network](/vpc/docs/vpc) to which the Job is peered. For example, `projects/12345/global/networks/myVPC`. The format of this field is `projects/{project}/global/networks/{network}`, where {project} is a project number (like `12345`) and {network} is network name. Private services access must already be configured for the network. If left unspecified, the Job is not peered with any network. [Learn about using VPC Network Peering.](/ai-platform/training/docs/vpc-peering)."]
    pub network: ::std::option::Option<::std::string::String>,
    #[serde(rename = "packageUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The Google Cloud Storage location of the packages with the training program and any additional dependencies. The maximum number of package URIs is 100."]
    pub package_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "parameterServerConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The configuration for parameter servers. You should only set `parameterServerConfig.acceleratorConfig` if `parameterServerType` is set to a Compute Engine machine type. [Learn about restrictions on accelerator configurations for training.](/ai-platform/training/docs/using-gpus#compute-engine-machine-types-with-gpu) Set `parameterServerConfig.imageUri` only if you build a custom image for your parameter server. If `parameterServerConfig.imageUri` has not been set, AI Platform uses the value of `masterConfig.imageUri`. Learn more about [configuring custom containers](/ai-platform/training/docs/distributed-training-containers)."]
    pub parameter_server_config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1ReplicaConfig>>,
    #[serde(rename = "parameterServerCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The number of parameter server replicas to use for the training job. Each replica in the cluster will be of the type specified in `parameter_server_type`. This value can only be used when `scale_tier` is set to `CUSTOM`. If you set this value, you must also set `parameter_server_type`. The default value is zero."]
    pub parameter_server_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameterServerType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies the type of virtual machine to use for your training job's parameter server. The supported values are the same as those described in the entry for `master_type`. This value must be consistent with the category of machine type that `masterType` uses. In other words, both must be Compute Engine machine types or both must be legacy machine types. This value must be present when `scaleTier` is set to `CUSTOM` and `parameter_server_count` is greater than zero."]
    pub parameter_server_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pythonModule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The Python module name to run after installing the packages."]
    pub python_module: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pythonVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The version of Python used in training. You must either specify this field or specify `masterConfig.imageUri`. The following Python versions are available: * Python '3.7' is available when `runtime_version` is set to '1.15' or later. * Python '3.5' is available when `runtime_version` is set to a version from '1.4' to '1.14'. * Python '2.7' is available when `runtime_version` is set to '1.15' or earlier. Read more about the Python versions available for [each runtime version](/ml-engine/docs/runtime-version-list)."]
    pub python_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The region to run the training job in. See the [available regions](/ai-platform/training/docs/regions) for AI Platform Training."]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "runtimeVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The AI Platform runtime version to use for training. You must either specify this field or specify `masterConfig.imageUri`. For more information, see the [runtime version list](/ai-platform/training/docs/runtime-version-list) and learn [how to manage runtime versions](/ai-platform/training/docs/versioning)."]
    pub runtime_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scaleTier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Specifies the machine types, the number of replicas for workers and parameter servers."]
    pub scale_tier: ::std::option::Option<GoogleCloudMlV1TrainingInputScaleTierEnum>,
    #[serde(rename = "scheduling")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Scheduling options for a training job."]
    pub scheduling: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1Scheduling>>,
    #[serde(rename = "serviceAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The email address of a service account to use when running the training appplication. You must have the `iam.serviceAccounts.actAs` permission for the specified service account. In addition, the AI Platform Training Google-managed service account must have the `roles/iam.serviceAccountAdmin` role for the specified service account. [Learn more about configuring a service account.](/ai-platform/training/docs/custom-service-account) If not specified, the AI Platform Training Google-managed service account is used by default."]
    pub service_account: ::std::option::Option<::std::string::String>,
    #[serde(rename = "useChiefInTfConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Use `chief` instead of `master` in the `TF_CONFIG` environment variable when training with a custom container. Defaults to `false`. [Learn more about this field.](/ai-platform/training/docs/distributed-training-details#chief-versus-master) This field has no effect for training jobs that don't use a custom container."]
    pub use_chief_in_tf_config: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "workerConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The configuration for workers. You should only set `workerConfig.acceleratorConfig` if `workerType` is set to a Compute Engine machine type. [Learn about restrictions on accelerator configurations for training.](/ai-platform/training/docs/using-gpus#compute-engine-machine-types-with-gpu) Set `workerConfig.imageUri` only if you build a custom image for your worker. If `workerConfig.imageUri` has not been set, AI Platform uses the value of `masterConfig.imageUri`. Learn more about [configuring custom containers](/ai-platform/training/docs/distributed-training-containers)."]
    pub worker_config: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1ReplicaConfig>>,
    #[serde(rename = "workerCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The number of worker replicas to use for the training job. Each replica in the cluster will be of the type specified in `worker_type`. This value can only be used when `scale_tier` is set to `CUSTOM`. If you set this value, you must also set `worker_type`. The default value is zero."]
    pub worker_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "workerType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies the type of virtual machine to use for your training job's worker nodes. The supported values are the same as those described in the entry for `masterType`. This value must be consistent with the category of machine type that `masterType` uses. In other words, both must be Compute Engine machine types or both must be legacy machine types. If you use `cloud_tpu` for this value, see special instructions for [configuring a custom TPU machine](/ml-engine/docs/tensorflow/using-tpus#configuring_a_custom_tpu_machine). This value must be present when `scaleTier` is set to `CUSTOM` and `workerCount` is greater than zero."]
    pub worker_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Specifies the machine types, the number of replicas for workers and parameter servers."]
pub enum GoogleCloudMlV1TrainingInputScaleTierEnum {
    #[serde(rename = "BASIC")]
    #[doc = "A single worker instance. This tier is suitable for learning how to use Cloud ML, and for experimenting with new models using small datasets."]
    Basic,
    #[serde(rename = "STANDARD_1")]
    #[doc = "Many workers and a few parameter servers."]
    Standard1,
    #[serde(rename = "PREMIUM_1")]
    #[doc = "A large number of workers with many parameter servers."]
    Premium1,
    #[serde(rename = "BASIC_GPU")]
    #[doc = "A single worker instance [with a GPU](/ai-platform/training/docs/using-gpus)."]
    BasicGpu,
    #[serde(rename = "BASIC_TPU")]
    #[doc = "A single worker instance with a [Cloud TPU](/ml-engine/docs/tensorflow/using-tpus)."]
    BasicTpu,
    #[serde(rename = "CUSTOM")]
    #[doc = "The CUSTOM tier is not a set tier, but rather enables you to use your own cluster specification. When you use this tier, set values to configure your processing cluster according to these guidelines: * You _must_ set `TrainingInput.masterType` to specify the type of machine to use for your master node. This is the only required setting. * You _may_ set `TrainingInput.workerCount` to specify the number of workers to use. If you specify one or more workers, you _must_ also set `TrainingInput.workerType` to specify the type of machine to use for your worker nodes. * You _may_ set `TrainingInput.parameterServerCount` to specify the number of parameter servers to use. If you specify one or more parameter servers, you _must_ also set `TrainingInput.parameterServerType` to specify the type of machine to use for your parameter servers. Note that all of your workers must use the same machine type, which can be different from your parameter server type and master type. Your parameter servers must likewise use the same machine type, which can be different from your worker type and master type."]
    Custom,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents results of a training job. Output only."]
pub struct GoogleCloudMlV1TrainingOutput {
    #[serde(rename = "builtInAlgorithmOutput")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details related to built-in algorithms jobs. Only set for built-in algorithms jobs."]
    pub built_in_algorithm_output:
        ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1BuiltInAlgorithmOutput>>,
    #[serde(rename = "completedTrialCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of hyperparameter tuning trials that completed successfully. Only set for hyperparameter tuning jobs."]
    pub completed_trial_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "consumedMLUnits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of ML units consumed by the job."]
    pub consumed_ml_units: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "hyperparameterMetricTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The TensorFlow summary tag name used for optimizing hyperparameter tuning trials. See [`HyperparameterSpec.hyperparameterMetricTag`](#HyperparameterSpec.FIELDS.hyperparameter_metric_tag) for more information. Only set for hyperparameter tuning jobs."]
    pub hyperparameter_metric_tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isBuiltInAlgorithmJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this job is a built-in Algorithm job."]
    pub is_built_in_algorithm_job: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isHyperparameterTuningJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this job is a hyperparameter tuning job."]
    pub is_hyperparameter_tuning_job: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "trials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Results for individual Hyperparameter trials. Only set for hyperparameter tuning jobs."]
    pub trials: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1HyperparameterOutput>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message representing a trial."]
pub struct GoogleCloudMlV1Trial {
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The identifier of the client that originally requested this trial."]
    pub client_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time at which the trial's status changed to COMPLETED."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "finalMeasurement")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The final measurement containing the objective value."]
    pub final_measurement: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1Measurement>>,
    #[serde(rename = "infeasibleReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A human readable string describing why the trial is infeasible. This should only be set if trial_infeasible is true."]
    pub infeasible_reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "measurements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of measurements that are strictly lexicographically ordered by their induced tuples (steps, elapsed_time). These are used for early stopping computations."]
    pub measurements:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1Measurement>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Name of the trial assigned by the service."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The parameters of the trial."]
    pub parameters:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudMlV1TrialParameter>>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time at which the trial was started."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The detailed state of a trial."]
    pub state: ::std::option::Option<GoogleCloudMlV1TrialStateEnum>,
    #[serde(rename = "trialInfeasible")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If true, the parameters in this trial are not attempted again."]
    pub trial_infeasible: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The detailed state of a trial."]
pub enum GoogleCloudMlV1TrialStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The trial state is unspecified."]
    StateUnspecified,
    #[serde(rename = "REQUESTED")]
    #[doc = "Indicates that a specific trial has been requested, but it has not yet been suggested by the service."]
    Requested,
    #[serde(rename = "ACTIVE")]
    #[doc = "Indicates that the trial has been suggested."]
    Active,
    #[serde(rename = "COMPLETED")]
    #[doc = "Indicates that the trial is done, and either has a final_measurement set, or is marked as trial_infeasible."]
    Completed,
    #[serde(rename = "STOPPING")]
    #[doc = "Indicates that the trial should stop according to the service."]
    Stopping,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a version of the model. Each version is a trained model deployed in the cloud, ready to handle prediction requests. A model can have multiple versions. You can get information about all of the versions of a given model by calling projects.models.versions.list."]
pub struct GoogleCloudMlV1Version {
    #[serde(rename = "acceleratorConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Accelerator config for using GPUs for online prediction (beta). Only specify this field if you have specified a Compute Engine (N1) machine type in the `machineType` field. Learn more about [using GPUs for online prediction](/ml-engine/docs/machine-types-online-prediction#gpus)."]
    pub accelerator_config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1AcceleratorConfig>>,
    #[serde(rename = "autoScaling")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Automatically scale the number of nodes used to serve the model in response to increases and decreases in traffic. Care should be taken to ramp up traffic according to the model's ability to scale or you will start seeing increases in latency and 429 response codes."]
    pub auto_scaling: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1AutoScaling>>,
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies a custom container to use for serving predictions. If you specify this field, then `machineType` is required. If you specify this field, then `deploymentUri` is optional. If you specify this field, then you must not specify `runtimeVersion`, `packageUris`, `framework`, `pythonVersion`, or `predictionClass`."]
    pub container: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1ContainerSpec>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the version was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deploymentUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Cloud Storage URI of a directory containing trained model artifacts to be used to create the model version. See the [guide to deploying models](/ai-platform/prediction/docs/deploying-models) for more information. The total number of files under this directory must not exceed 1000. During projects.models.versions.create, AI Platform Prediction copies all files from the specified directory to a location managed by the service. From then on, AI Platform Prediction uses these copies of the model artifacts to serve predictions, not the original files in Cloud Storage, so this location is useful only as a historical record. If you specify container, then this field is optional. Otherwise, it is required. Learn [how to use this field with a custom container](/ai-platform/prediction/docs/custom-container-requirements#artifacts)."]
    pub deployment_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The description specified for the version when it was created."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The details of a failure or a cancellation."]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a model from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform model updates in order to avoid race conditions: An `etag` is returned in the response to `GetVersion`, and systems are expected to put that etag in the request to `UpdateVersion` to ensure that their change will be applied to the model as intended."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "explanationConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Configures explainability features on the model's version. Some explanation features require additional metadata to be loaded as part of the model payload."]
    pub explanation_config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1ExplanationConfig>>,
    #[serde(rename = "framework")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The machine learning framework AI Platform uses to train this version of the model. Valid values are `TENSORFLOW`, `SCIKIT_LEARN`, `XGBOOST`. If you do not specify a framework, AI Platform will analyze files in the deployment_uri to determine a framework. If you choose `SCIKIT_LEARN` or `XGBOOST`, you must also set the runtime version of the model to 1.4 or greater. Do **not** specify a framework if you're deploying a [custom prediction routine](/ai-platform/prediction/docs/custom-prediction-routines) or if you're using a [custom container](/ai-platform/prediction/docs/use-custom-container)."]
    pub framework: ::std::option::Option<GoogleCloudMlV1VersionFrameworkEnum>,
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If true, this version will be used to handle prediction requests that do not specify a version. You can change the default version by calling projects.methods.versions.setDefault."]
    pub is_default: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. One or more labels that you can add, to organize your model versions. Each label is a key-value pair, where both the key and the value are arbitrary strings that you supply. For more information, see the documentation on using labels."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "lastMigrationModelId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The [AI Platform (Unified) `Model`](https://cloud.google.com/ai-platform-unified/docs/reference/rest/v1beta1/projects.locations.models) ID for the last [model migration](https://cloud.google.com/ai-platform-unified/docs/start/migrating-to-ai-platform-unified)."]
    pub last_migration_model_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastMigrationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The last time this version was successfully [migrated to AI Platform (Unified)](https://cloud.google.com/ai-platform-unified/docs/start/migrating-to-ai-platform-unified)."]
    pub last_migration_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastUseTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the version was last used for prediction."]
    pub last_use_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "machineType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The type of machine on which to serve the model. Currently only applies to online prediction service. To learn about valid values for this field, read [Choosing a machine type for online prediction](/ai-platform/prediction/docs/machine-types-online-prediction). If this field is not specified and you are using a [regional endpoint](/ai-platform/prediction/docs/regional-endpoints), then the machine type defaults to `n1-standard-2`. If this field is not specified and you are using the global endpoint (`ml.googleapis.com`), then the machine type defaults to `mls1-c1-m2`."]
    pub machine_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "manualScaling")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Manually select the number of nodes to use for serving the model. You should generally use `auto_scaling` with an appropriate `min_nodes` instead, but this option is available if you want more predictable billing. Beware that latency and error rates will increase if the traffic exceeds that capability of the system to serve it based on the selected number of nodes."]
    pub manual_scaling: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1ManualScaling>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name specified for the version when it was created. The version name must be unique within the model it is created in."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "packageUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Cloud Storage paths (`gs://…`) of packages for [custom prediction routines](/ml-engine/docs/tensorflow/custom-prediction-routines) or [scikit-learn pipelines with custom code](/ml-engine/docs/scikit/exporting-for-prediction#custom-pipeline-code). For a custom prediction routine, one of these packages must contain your Predictor class (see [`predictionClass`](#Version.FIELDS.prediction_class)). Additionally, include any dependencies used by your Predictor or scikit-learn pipeline uses that are not already included in your selected [runtime version](/ml-engine/docs/tensorflow/runtime-version-list). If you specify this field, you must also set [`runtimeVersion`](#Version.FIELDS.runtime_version) to 1.4 or greater."]
    pub package_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "predictionClass")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The fully qualified name (module_name.class_name) of a class that implements the Predictor interface described in this reference field. The module containing this class should be included in a package provided to the [`packageUris` field](#Version.FIELDS.package_uris). Specify this field if and only if you are deploying a [custom prediction routine (beta)](/ml-engine/docs/tensorflow/custom-prediction-routines). If you specify this field, you must set [`runtimeVersion`](#Version.FIELDS.runtime_version) to 1.4 or greater and you must set `machineType` to a [legacy (MLS1) machine type](/ml-engine/docs/machine-types-online-prediction). The following code sample provides the Predictor interface: class Predictor(object): \"\"\"Interface for constructing custom predictors.\"\"\" def predict(self, instances, **kwargs): \"\"\"Performs custom prediction. Instances are the decoded values from the request. They have already been deserialized from JSON. Args: instances: A list of prediction input instances. **kwargs: A dictionary of keyword args provided as additional fields on the predict request body. Returns: A list of outputs containing the prediction results. This list must be JSON serializable. \"\"\" raise NotImplementedError() @classmethod def from_path(cls, model_dir): \"\"\"Creates an instance of Predictor using the given path. Loading of the predictor should be done in this method. Args: model_dir: The local directory that contains the exported model file along with any additional files uploaded when creating the version resource. Returns: An instance implementing this Predictor class. \"\"\" raise NotImplementedError() Learn more about [the Predictor interface and custom prediction routines](/ml-engine/docs/tensorflow/custom-prediction-routines)."]
    pub prediction_class: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pythonVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The version of Python used in prediction. The following Python versions are available: * Python '3.7' is available when `runtime_version` is set to '1.15' or later. * Python '3.5' is available when `runtime_version` is set to a version from '1.4' to '1.14'. * Python '2.7' is available when `runtime_version` is set to '1.15' or earlier. Read more about the Python versions available for [each runtime version](/ml-engine/docs/runtime-version-list)."]
    pub python_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestLoggingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. *Only* specify this field in a projects.models.versions.patch request. Specifying it in a projects.models.versions.create request has no effect. Configures the request-response pair logging on predictions from this Version."]
    pub request_logging_config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1RequestLoggingConfig>>,
    #[serde(rename = "routes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies paths on a custom container's HTTP server where AI Platform Prediction sends certain requests. If you specify this field, then you must also specify the `container` field. If you specify the `container` field and do not specify this field, it defaults to the following: ```json { \"predict\": \"/v1/models/MODEL/versions/VERSION:predict\", \"health\": \"/v1/models/MODEL/versions/VERSION\" } ``` See RouteMap for more details about these default values."]
    pub routes: ::std::option::Option<::std::boxed::Box<GoogleCloudMlV1RouteMap>>,
    #[serde(rename = "runtimeVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The AI Platform runtime version to use for this deployment. For more information, see the [runtime version list](/ml-engine/docs/runtime-version-list) and [how to manage runtime versions](/ml-engine/docs/versioning)."]
    pub runtime_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies the service account for resource access control. If you specify this field, then you must also specify either the `containerSpec` or the `predictionClass` field. Learn more about [using a custom service account](/ai-platform/prediction/docs/custom-service-account)."]
    pub service_account: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The state of a version."]
    pub state: ::std::option::Option<GoogleCloudMlV1VersionStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. The machine learning framework AI Platform uses to train this version of the model. Valid values are `TENSORFLOW`, `SCIKIT_LEARN`, `XGBOOST`. If you do not specify a framework, AI Platform will analyze files in the deployment_uri to determine a framework. If you choose `SCIKIT_LEARN` or `XGBOOST`, you must also set the runtime version of the model to 1.4 or greater. Do **not** specify a framework if you're deploying a [custom prediction routine](/ai-platform/prediction/docs/custom-prediction-routines) or if you're using a [custom container](/ai-platform/prediction/docs/use-custom-container)."]
pub enum GoogleCloudMlV1VersionFrameworkEnum {
    #[serde(rename = "FRAMEWORK_UNSPECIFIED")]
    #[doc = "Unspecified framework. Assigns a value based on the file suffix."]
    FrameworkUnspecified,
    #[serde(rename = "TENSORFLOW")]
    #[doc = "Tensorflow framework."]
    Tensorflow,
    #[serde(rename = "SCIKIT_LEARN")]
    #[doc = "Scikit-learn framework."]
    ScikitLearn,
    #[serde(rename = "XGBOOST")]
    #[doc = "XGBoost framework."]
    Xgboost,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The state of a version."]
pub enum GoogleCloudMlV1VersionStateEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "The version state is unspecified."]
    Unknown,
    #[serde(rename = "READY")]
    #[doc = "The version is ready for prediction."]
    Ready,
    #[serde(rename = "CREATING")]
    #[doc = "The version is being created. New UpdateVersion and DeleteVersion requests will fail if a version is in the CREATING state."]
    Creating,
    #[serde(rename = "FAILED")]
    #[doc = "The version failed to be created, possibly cancelled. `error_message` should contain the details of the failure."]
    Failed,
    #[serde(rename = "DELETING")]
    #[doc = "The version is being deleted. New UpdateVersion and DeleteVersion requests will fail if a version is in the DELETING state."]
    Deleting,
    #[serde(rename = "UPDATING")]
    #[doc = "The version is being updated. New UpdateVersion and DeleteVersion requests will fail if a version is in the UPDATING state."]
    Updating,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Attributes credit by computing the XRAI taking advantage of the model's fully differentiable structure. Refer to this paper for more details: https://arxiv.org/abs/1906.02825 Currently only implemented for models with natural image inputs."]
pub struct GoogleCloudMlV1XraiAttribution {
    #[serde(rename = "numIntegralSteps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of steps for approximating the path integral. A good value to start is 50 and gradually increase until the sum to diff property is met within the desired error range."]
    pub num_integral_steps: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { \"audit_configs\": [ { \"service\": \"allServices\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" }, { \"log_type\": \"ADMIN_READ\" } ] }, { \"service\": \"sampleservice.googleapis.com\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\" }, { \"log_type\": \"DATA_WRITE\", \"exempted_members\": [ \"user:aliya@example.com\" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging."]
pub struct GoogleIamV1AuditConfig {
    #[serde(rename = "auditLogConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for logging of each type of permission."]
    pub audit_log_configs:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleIamV1AuditLogConfig>>>,
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services."]
    pub service: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Provides the configuration for logging a type of permissions. Example: { \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging."]
pub struct GoogleIamV1AuditLogConfig {
    #[serde(rename = "exemptedMembers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members."]
    pub exempted_members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "logType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The log type that this config enables."]
    pub log_type: ::std::option::Option<GoogleIamV1AuditLogConfigLogTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The log type that this config enables."]
pub enum GoogleIamV1AuditLogConfigLogTypeEnum {
    #[serde(rename = "LOG_TYPE_UNSPECIFIED")]
    #[doc = "Default case. Should never be this."]
    LogTypeUnspecified,
    #[serde(rename = "ADMIN_READ")]
    #[doc = "Admin reads. Example: CloudIAM getIamPolicy"]
    AdminRead,
    #[serde(rename = "DATA_WRITE")]
    #[doc = "Data writes. Example: CloudSQL Users create"]
    DataWrite,
    #[serde(rename = "DATA_READ")]
    #[doc = "Data reads. Example: CloudSQL Users list"]
    DataRead,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Associates `members` with a `role`."]
pub struct GoogleIamV1Binding {
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the members in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub condition: ::std::option::Option<::std::boxed::Box<GoogleTypeExpr>>,
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the identities requesting access for a Cloud Platform resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. "]
    pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Role that is assigned to `members`. For example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
    pub role: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/)."]
pub struct GoogleIamV1Policy {
    #[serde(rename = "auditConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies cloud audit logging configuration for this policy."]
    pub audit_configs:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleIamV1AuditConfig>>>,
    #[serde(rename = "bindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member."]
    pub bindings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleIamV1Binding>>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `SetIamPolicy` method."]
pub struct GoogleIamV1SetIamPolicyRequest {
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them."]
    pub policy: ::std::option::Option<::std::boxed::Box<GoogleIamV1Policy>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: \"bindings, etag\"`"]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `TestIamPermissions` method."]
pub struct GoogleIamV1TestIamPermissionsRequest {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of permissions to check for the `resource`. Permissions with wildcards (such as '*' or 'storage.*') are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions)."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for `TestIamPermissions` method."]
pub struct GoogleIamV1TestIamPermissionsResponse {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is allowed."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
#[doc = "Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() < 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\" expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description: \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information."]
pub struct GoogleTypeExpr {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expression")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub title: ::std::option::Option<::std::string::String>,
}
