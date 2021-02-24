#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Applications resources represent applications installed on the domain that support transferring ownership of user data."]
pub struct Application {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Etag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The application's ID."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "application_defaults :: kind")]
    #[doc = "Identifies the resource as a DataTransfer Application Resource."]
    pub kind: ::std::string::String,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The application's name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transferParams")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of all possible transfer parameters for this application. These parameters can be used to select the data of the user in this application to be transferred."]
    pub transfer_params:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApplicationTransferParam>>>,
}
mod application_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("admin#datatransfer#ApplicationResource")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Template to map fields of ApplicationDataTransfer resource."]
pub struct ApplicationDataTransfer {
    #[serde(rename = "applicationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The application's ID."]
    pub application_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "applicationTransferParams")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transfer parameters for the application. These parameters are used to select the data which will get transferred in context of this application."]
    pub application_transfer_params:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApplicationTransferParam>>>,
    #[serde(rename = "applicationTransferStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Current status of transfer for this application. (Read-only)"]
    pub application_transfer_status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Template for application transfer parameters."]
pub struct ApplicationTransferParam {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the transfer parameter. eg: 'PRIVACY_LEVEL'"]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of the corresponding transfer parameter. eg: 'PRIVATE' or 'SHARED'"]
    pub value: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Template for a collection of Applications."]
pub struct ApplicationsListResponse {
    #[serde(rename = "applications")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of applications that support data transfer and are also installed for the customer."]
    pub applications: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Application>>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "applications_list_response_defaults :: kind")]
    #[doc = "Identifies the resource as a collection of Applications."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token which will be used to specify next page in list API."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod applications_list_response_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("admin#datatransfer#applicationsList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Transfer resource represents the transfer of the ownership of user data between users."]
pub struct DataTransfer {
    #[serde(rename = "applicationDataTransfers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of per application data transfer resources. It contains data transfer details of the applications associated with this transfer resource. Note that this list is also used to specify the applications for which data transfer has to be done at the time of the transfer resource creation."]
    pub application_data_transfers:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApplicationDataTransfer>>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transfer's ID (Read-only)."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "data_transfer_defaults :: kind")]
    #[doc = "Identifies the resource as a DataTransfer request."]
    pub kind: ::std::string::String,
    #[serde(rename = "newOwnerUserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the user to whom the data is being transferred."]
    pub new_owner_user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "oldOwnerUserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the user whose data is being transferred."]
    pub old_owner_user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "overallTransferStatusCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Overall transfer status (Read-only)."]
    pub overall_transfer_status_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the data transfer was requested (Read-only)."]
    pub request_time: ::std::option::Option<::std::string::String>,
}
mod data_transfer_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("admin#datatransfer#DataTransfer")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Template for a collection of DataTransfer resources."]
pub struct DataTransfersListResponse {
    #[serde(rename = "dataTransfers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of data transfer requests."]
    pub data_transfers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataTransfer>>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "data_transfers_list_response_defaults :: kind")]
    #[doc = "Identifies the resource as a collection of data transfer requests."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token which will be used to specify next page in list API."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod data_transfers_list_response_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("admin#datatransfer#dataTransfersList")
    }
}
