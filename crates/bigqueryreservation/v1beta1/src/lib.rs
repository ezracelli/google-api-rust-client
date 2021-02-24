#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Assignment allows a project to submit jobs of a certain type using slots from the specified reservation."]
pub struct Assignment {
    #[serde(rename = "assignee")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource which will use the reservation. E.g. `projects/myproject`, `folders/123`, or `organizations/456`."]
    pub assignee: ::std::option::Option<::std::string::String>,
    #[serde(rename = "jobType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Which type of jobs will use the reservation."]
    pub job_type: ::std::option::Option<AssignmentJobTypeEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Name of the resource. E.g.: `projects/myproject/locations/US/reservations/team1-prod/assignments/123`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. State of the assignment."]
    pub state: ::std::option::Option<AssignmentStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Which type of jobs will use the reservation."]
pub enum AssignmentJobTypeEnum {
    #[serde(rename = "JOB_TYPE_UNSPECIFIED")]
    #[doc = "Invalid type. Requests with this value will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`."]
    JobTypeUnspecified,
    #[serde(rename = "PIPELINE")]
    #[doc = "Pipeline (load/export) jobs from the project will use the reservation."]
    Pipeline,
    #[serde(rename = "QUERY")]
    #[doc = "Query jobs from the project will use the reservation."]
    Query,
    #[serde(rename = "ML_EXTERNAL")]
    #[doc = "BigQuery ML jobs that use services external to BigQuery for model training. These jobs will not utilize idle slots from other reservations."]
    MlExternal,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. State of the assignment."]
pub enum AssignmentStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Invalid state value."]
    StateUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "Queries from assignee will be executed as on-demand, if related assignment is pending."]
    Pending,
    #[serde(rename = "ACTIVE")]
    #[doc = "Assignment is ready."]
    Active,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a BI Reservation."]
pub struct BiReservation {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the singleton BI reservation. Reservation names have the form `projects/{project_id}/locations/{location_id}/bireservation`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size of a reservation, in bytes."]
    pub size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The last update timestamp of a reservation."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Capacity commitment is a way to purchase compute capacity for BigQuery jobs (in the form of slots) with some committed period of usage. Annual commitments renew by default. Commitments can be removed after their commitment end time passes. In order to remove annual commitment, its plan needs to be changed to monthly or flex first. A capacity commitment resource exists as a child resource of the admin project."]
pub struct CapacityCommitment {
    #[serde(rename = "commitmentEndTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The end of the current commitment period. It is applicable only for ACTIVE capacity commitments."]
    pub commitment_end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "commitmentStartTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The start of the current commitment period. It is applicable only for ACTIVE capacity commitments."]
    pub commitment_start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "failureStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. For FAILED commitment plan, provides the reason of failure."]
    pub failure_status: ::std::option::Option<::std::boxed::Box<Status>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the capacity commitment, e.g., `projects/myproject/locations/US/capacityCommitments/123`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "plan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Capacity commitment commitment plan."]
    pub plan: ::std::option::Option<CapacityCommitmentPlanEnum>,
    #[serde(rename = "renewalPlan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The plan this capacity commitment is converted to after commitment_end_time passes. Once the plan is changed, committed period is extended according to commitment plan. Only applicable for ANNUAL commitments."]
    pub renewal_plan: ::std::option::Option<CapacityCommitmentRenewalPlanEnum>,
    #[serde(rename = "slotCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of slots in this commitment."]
    pub slot_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. State of the commitment."]
    pub state: ::std::option::Option<CapacityCommitmentStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Capacity commitment commitment plan."]
pub enum CapacityCommitmentPlanEnum {
    #[serde(rename = "COMMITMENT_PLAN_UNSPECIFIED")]
    #[doc = "Invalid plan value. Requests with this value will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`."]
    CommitmentPlanUnspecified,
    #[serde(rename = "FLEX")]
    #[doc = "Flex commitments have committed period of 1 minute after becoming ACTIVE. After that, they are not in a committed period anymore and can be removed any time."]
    Flex,
    #[serde(rename = "TRIAL")]
    #[doc = "Trial commitments have a committed period of 182 days after becoming ACTIVE. After that, they are converted to a new commitment based on the `renewal_plan`. Default `renewal_plan` for Trial commitment is Flex so that it can be deleted right after committed period ends."]
    Trial,
    #[serde(rename = "MONTHLY")]
    #[doc = "Monthly commitments have a committed period of 30 days after becoming ACTIVE. After that, they are not in a committed period anymore and can be removed any time."]
    Monthly,
    #[serde(rename = "ANNUAL")]
    #[doc = "Annual commitments have a committed period of 365 days after becoming ACTIVE. After that they are converted to a new commitment based on the renewal_plan."]
    Annual,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The plan this capacity commitment is converted to after commitment_end_time passes. Once the plan is changed, committed period is extended according to commitment plan. Only applicable for ANNUAL commitments."]
pub enum CapacityCommitmentRenewalPlanEnum {
    #[serde(rename = "COMMITMENT_PLAN_UNSPECIFIED")]
    #[doc = "Invalid plan value. Requests with this value will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`."]
    CommitmentPlanUnspecified,
    #[serde(rename = "FLEX")]
    #[doc = "Flex commitments have committed period of 1 minute after becoming ACTIVE. After that, they are not in a committed period anymore and can be removed any time."]
    Flex,
    #[serde(rename = "TRIAL")]
    #[doc = "Trial commitments have a committed period of 182 days after becoming ACTIVE. After that, they are converted to a new commitment based on the `renewal_plan`. Default `renewal_plan` for Trial commitment is Flex so that it can be deleted right after committed period ends."]
    Trial,
    #[serde(rename = "MONTHLY")]
    #[doc = "Monthly commitments have a committed period of 30 days after becoming ACTIVE. After that, they are not in a committed period anymore and can be removed any time."]
    Monthly,
    #[serde(rename = "ANNUAL")]
    #[doc = "Annual commitments have a committed period of 365 days after becoming ACTIVE. After that they are converted to a new commitment based on the renewal_plan."]
    Annual,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. State of the commitment."]
pub enum CapacityCommitmentStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Invalid state value."]
    StateUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "Capacity commitment is pending provisioning. Pending capacity commitment does not contribute to the parent's slot_capacity."]
    Pending,
    #[serde(rename = "ACTIVE")]
    #[doc = "Once slots are provisioned, capacity commitment becomes active. slot_count is added to the parent's slot_capacity."]
    Active,
    #[serde(rename = "FAILED")]
    #[doc = "Capacity commitment is failed to be activated by the backend."]
    Failed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The metadata for operation returned from ReservationService.CreateSlotPool."]
pub struct CreateSlotPoolMetadata {
    #[serde(rename = "slotPool")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name of the slot pool that is being created. E.g., projects/myproject/locations/us-central1/reservations/foo/slotPools/123"]
    pub slot_pool: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for ReservationService.ListAssignments."]
pub struct ListAssignmentsResponse {
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of assignments visible to the user."]
    pub assignments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Assignment>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for ReservationService.ListCapacityCommitments."]
pub struct ListCapacityCommitmentsResponse {
    #[serde(rename = "capacityCommitments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of capacity commitments visible to the user."]
    pub capacity_commitments:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CapacityCommitment>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for ReservationService.ListReservations."]
pub struct ListReservationsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reservations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of reservations visible to the user."]
    pub reservations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Reservation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for ReservationService.MergeCapacityCommitments."]
pub struct MergeCapacityCommitmentsRequest {
    #[serde(rename = "capacityCommitmentIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ids of capacity commitments to merge. These capacity commitments must exist under admin project and location specified in the parent. ID is the last portion of capacity commitment name e.g., 'abc' for projects/myproject/locations/US/capacityCommitments/abc"]
    pub capacity_commitment_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for ReservationService.MoveAssignment. **Note**: \"bigquery.reservationAssignments.create\" permission is required on the destination_id. **Note**: \"bigquery.reservationAssignments.create\" and \"bigquery.reservationAssignments.delete\" permission are required on the related assignee."]
pub struct MoveAssignmentRequest {
    #[serde(rename = "destinationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The new reservation ID, e.g.: `projects/myotherproject/locations/US/reservations/team2-prod`"]
    pub destination_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A reservation is a mechanism used to guarantee slots to users."]
pub struct Reservation {
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Creation time of the reservation."]
    pub creation_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ignoreIdleSlots")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If false, any query using this reservation will use idle slots from other reservations within the same admin project. If true, a query using this reservation will execute with the slot capacity specified above at most."]
    pub ignore_idle_slots: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the reservation, e.g., `projects/*/locations/*/reservations/team1-prod`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "slotCapacity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minimum slots available to this reservation. A slot is a unit of computational power in BigQuery, and serves as the unit of parallelism. Queries using this reservation might use more slots during runtime if ignore_idle_slots is set to false. If the new reservation's slot capacity exceed the parent's slot capacity or if total slot capacity of the new reservation and its siblings exceeds the parent's slot capacity, the request will fail with `google.rpc.Code.RESOURCE_EXHAUSTED`."]
    pub slot_capacity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Last update time of the reservation."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for ReservationService.SearchAssignments."]
pub struct SearchAssignmentsResponse {
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of assignments visible to the user."]
    pub assignments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Assignment>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for ReservationService.SplitCapacityCommitment."]
pub struct SplitCapacityCommitmentRequest {
    #[serde(rename = "slotCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of slots in the capacity commitment after the split."]
    pub slot_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for ReservationService.SplitCapacityCommitment."]
pub struct SplitCapacityCommitmentResponse {
    #[serde(rename = "first")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "First capacity commitment, result of a split."]
    pub first: ::std::option::Option<::std::boxed::Box<CapacityCommitment>>,
    #[serde(rename = "second")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Second capacity commitment, result of a split."]
    pub second: ::std::option::Option<::std::boxed::Box<CapacityCommitment>>,
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
