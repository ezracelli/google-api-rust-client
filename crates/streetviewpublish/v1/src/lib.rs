#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to delete multiple Photos."]
pub struct BatchDeletePhotosRequest {
    #[serde(rename = "photoIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. IDs of the Photos. HTTP GET requests require the following syntax for the URL query parameter: `photoIds=&photoIds=&...`."]
    pub photo_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response to batch delete of one or more Photos."]
pub struct BatchDeletePhotosResponse {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status for the operation to delete a single Photo in the batch request."]
    pub status: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Status>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response to batch get of Photos."]
pub struct BatchGetPhotosResponse {
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of results for each individual Photo requested, in the same order as the requests in BatchGetPhotos."]
    pub results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PhotoResponse>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to update the metadata of photos. Updating the pixels of photos is not supported."]
pub struct BatchUpdatePhotosRequest {
    #[serde(rename = "updatePhotoRequests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. List of UpdatePhotoRequests."]
    pub update_photo_requests:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UpdatePhotoRequest>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response to batch update of metadata of one or more Photos."]
pub struct BatchUpdatePhotosResponse {
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of results for each individual Photo updated, in the same order as the request."]
    pub results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PhotoResponse>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A connection is the link from a source photo to a destination photo."]
pub struct Connection {
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The destination of the connection from the containing photo to another photo."]
    pub target: ::std::option::Option<::std::boxed::Box<PhotoId>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An object that represents a latitude/longitude pair. This is expressed as a pair of doubles to represent degrees latitude and degrees longitude. Unless specified otherwise, this must conform to the WGS84 standard. Values must be within normalized ranges."]
pub struct LatLng {
    #[serde(rename = "latitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The latitude in degrees. It must be in the range [-90.0, +90.0]."]
    pub latitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "longitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The longitude in degrees. It must be in the range [-180.0, +180.0]."]
    pub longitude: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Level information containing level number and its corresponding name."]
pub struct Level {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A name assigned to this Level, restricted to 3 characters. Consider how the elevator buttons would be labeled for this level if there was an elevator."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "number")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Floor number, used for ordering. 0 indicates the ground level, 1 indicates the first level above ground level, -1 indicates the first level under ground level. Non-integer values are OK."]
    pub number: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response to list all photos that belong to a user."]
pub struct ListPhotosResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "photos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of photos. The pageSize field in the request determines the number of items returned."]
    pub photos: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Photo>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This resource represents a long-running operation that is the result of a network API call."]
pub struct Operation {
    #[serde(rename = "done")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
    pub done: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error result of the operation in case of failure or cancellation."]
    pub error: ::std::option::Option<::std::boxed::Box<Status>>,
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
#[doc = "Photo is used to store 360 photos along with photo metadata."]
pub struct Photo {
    #[serde(rename = "captureTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Absolute time when the photo was captured. When the photo has no exif timestamp, this is used to set a timestamp in the photo metadata."]
    pub capture_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "connections")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Connections to other photos. A connection represents the link from this photo to another photo."]
    pub connections: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Connection>>>,
    #[serde(rename = "downloadUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The download URL for the photo bytes. This field is set only when GetPhotoRequest.view is set to PhotoView.INCLUDE_DOWNLOAD_URL."]
    pub download_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mapsPublishStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Status in Google Maps, whether this photo was published or rejected. Not currently populated."]
    pub maps_publish_status: ::std::option::Option<PhotoMapsPublishStatusEnum>,
    #[serde(rename = "photoId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required when updating a photo. Output only when creating a photo. Identifier for the photo, which is unique among all photos in Google."]
    pub photo_id: ::std::option::Option<::std::boxed::Box<PhotoId>>,
    #[serde(rename = "places")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Places where this photo belongs."]
    pub places: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Place>>>,
    #[serde(rename = "pose")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pose of the photo."]
    pub pose: ::std::option::Option<::std::boxed::Box<Pose>>,
    #[serde(rename = "shareLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The share link for the photo."]
    pub share_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thumbnailUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The thumbnail URL for showing a preview of the given photo."]
    pub thumbnail_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transferStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Status of rights transfer on this photo."]
    pub transfer_status: ::std::option::Option<PhotoTransferStatusEnum>,
    #[serde(rename = "uploadReference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required when creating a photo. Input only. The resource URL where the photo bytes are uploaded to."]
    pub upload_reference: ::std::option::Option<::std::boxed::Box<UploadRef>>,
    #[serde(rename = "viewCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. View count of the photo."]
    pub view_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Status in Google Maps, whether this photo was published or rejected. Not currently populated."]
pub enum PhotoMapsPublishStatusEnum {
    #[serde(rename = "UNSPECIFIED_MAPS_PUBLISH_STATUS")]
    #[doc = "The status of the photo is unknown."]
    UnspecifiedMapsPublishStatus,
    #[serde(rename = "PUBLISHED")]
    #[doc = "The photo is published to the public through Google Maps."]
    Published,
    #[serde(rename = "REJECTED_UNKNOWN")]
    #[doc = "The photo has been rejected for an unknown reason."]
    RejectedUnknown,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Status of rights transfer on this photo."]
pub enum PhotoTransferStatusEnum {
    #[serde(rename = "TRANSFER_STATUS_UNKNOWN")]
    #[doc = "The status of this transfer is unspecified."]
    TransferStatusUnknown,
    #[serde(rename = "NEVER_TRANSFERRED")]
    #[doc = "This photo has never been in a transfer."]
    NeverTransferred,
    #[serde(rename = "PENDING")]
    #[doc = "This photo transfer has been initiated, but the receiver has not yet responded."]
    Pending,
    #[serde(rename = "COMPLETED")]
    #[doc = "The photo transfer has been completed, and this photo has been transferred to the recipient."]
    Completed,
    #[serde(rename = "REJECTED")]
    #[doc = "The recipient rejected this photo transfer."]
    Rejected,
    #[serde(rename = "EXPIRED")]
    #[doc = "The photo transfer expired before the recipient took any action."]
    Expired,
    #[serde(rename = "CANCELLED")]
    #[doc = "The sender cancelled this photo transfer."]
    Cancelled,
    #[serde(rename = "RECEIVED_VIA_TRANSFER")]
    #[doc = "The recipient owns this photo due to a rights transfer."]
    ReceivedViaTransfer,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Identifier for a Photo."]
pub struct PhotoId {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A unique identifier for a photo."]
    pub id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response payload for a single Photo in batch operations including BatchGetPhotos and BatchUpdatePhotos."]
pub struct PhotoResponse {
    #[serde(rename = "photo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Photo resource, if the request was successful."]
    pub photo: ::std::option::Option<::std::boxed::Box<Photo>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status for the operation to get or update a single photo in the batch request."]
    pub status: ::std::option::Option<::std::boxed::Box<Status>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Place metadata for an entity."]
pub struct Place {
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output-only. The language_code that the name is localized with. This should be the language_code specified in the request, but may be a fallback."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output-only. The name of the place, localized to the language_code."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Place identifier, as described in https://developers.google.com/places/place-id."]
    pub place_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Raw pose measurement for an entity."]
pub struct Pose {
    #[serde(rename = "accuracyMeters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The estimated horizontal accuracy of this pose in meters with 68% confidence (one standard deviation). For example, on Android, this value is available from this method: https://developer.android.com/reference/android/location/Location#getAccuracy(). Other platforms have different methods of obtaining similar accuracy estimations."]
    pub accuracy_meters: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "altitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Altitude of the pose in meters above WGS84 ellipsoid. NaN indicates an unmeasured quantity."]
    pub altitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "heading")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Compass heading, measured at the center of the photo in degrees clockwise from North. Value must be >=0 and <360. NaN indicates an unmeasured quantity."]
    pub heading: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "latLngPair")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Latitude and longitude pair of the pose, as explained here: https://cloud.google.com/datastore/docs/reference/rest/Shared.Types/LatLng When creating a Photo, if the latitude and longitude pair are not provided, the geolocation from the exif header is used. A latitude and longitude pair not provided in the photo or exif header causes the photo process to fail."]
    pub lat_lng_pair: ::std::option::Option<::std::boxed::Box<LatLng>>,
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Level (the floor in a building) used to configure vertical navigation."]
    pub level: ::std::option::Option<::std::boxed::Box<Level>>,
    #[serde(rename = "pitch")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pitch, measured at the center of the photo in degrees. Value must be >=-90 and <= 90. A value of -90 means looking directly down, and a value of 90 means looking directly up. NaN indicates an unmeasured quantity."]
    pub pitch: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "roll")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Roll, measured in degrees. Value must be >= 0 and <360. A value of 0 means level with the horizon. NaN indicates an unmeasured quantity."]
    pub roll: ::std::option::Option<::std::primitive::f64>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to update the metadata of a Photo. Updating the pixels of a photo is not supported."]
pub struct UpdatePhotoRequest {
    #[serde(rename = "photo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Photo object containing the new metadata."]
    pub photo: ::std::option::Option<::std::boxed::Box<Photo>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Mask that identifies fields on the photo metadata to update. If not present, the old Photo metadata is entirely replaced with the new Photo metadata in this request. The update fails if invalid fields are specified. Multiple fields can be specified in a comma-delimited list. The following fields are valid: * `pose.heading` * `pose.latLngPair` * `pose.pitch` * `pose.roll` * `pose.level` * `pose.altitude` * `connections` * `places` *Note:* When updateMask contains repeated fields, the entire set of repeated values get replaced with the new contents. For example, if updateMask contains `connections` and `UpdatePhotoRequest.photo.connections` is empty, all connections are removed."]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Upload reference for media files."]
pub struct UploadRef {
    #[serde(rename = "uploadUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An upload reference should be unique for each user. It follows the form: \"https://streetviewpublish.googleapis.com/media/user/{account_id}/photo/{upload_reference}\""]
    pub upload_url: ::std::option::Option<::std::string::String>,
}
