#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Encapsulates impression event details."]
pub struct GoogleMapsPlayablelocationsV3Impression {
    #[serde(rename = "gameObjectType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An arbitrary, developer-defined type identifier for each type of game object used in your game. Since players interact with differ types of game objects in different ways, this field allows you to segregate impression data by type for analysis. You should assign a unique `game_object_type` ID to represent a distinct type of game object in your game. For example, 1=monster location, 2=powerup location."]
    pub game_object_type: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "impressionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of impression event."]
    pub impression_type:
        ::std::option::Option<GoogleMapsPlayablelocationsV3ImpressionImpressionTypeEnum>,
    #[serde(rename = "locationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the playable location."]
    pub location_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of impression event."]
pub enum GoogleMapsPlayablelocationsV3ImpressionImpressionTypeEnum {
    #[serde(rename = "IMPRESSION_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified type. Do not use."]
    ImpressionTypeUnspecified,
    #[serde(rename = "PRESENTED")]
    #[doc = "The playable location was presented to a player."]
    Presented,
    #[serde(rename = "INTERACTED")]
    #[doc = "A player interacted with the playable location."]
    Interacted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request for logging impressions."]
pub struct GoogleMapsPlayablelocationsV3LogImpressionsRequest {
    #[serde(rename = "clientInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Information about the client device. For example, device model and operating system."]
    pub client_info: ::std::option::Option<::std::boxed::Box<GoogleMapsUnityClientInfo>>,
    #[serde(rename = "impressions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Impression event details. The maximum number of impression reports that you can log at once is 50."]
    pub impressions: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleMapsPlayablelocationsV3Impression>>,
    >,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A string that uniquely identifies the log impressions request. This allows you to detect duplicate requests. We recommend that you use UUIDs for this value. The value must not exceed 50 characters. You should reuse the `request_id` only when retrying a request in case of failure. In this case, the request must be identical to the one that failed."]
    pub request_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response for the LogImpressions method. This method returns no data upon success."]
pub struct GoogleMapsPlayablelocationsV3LogImpressionsResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request for logging your player's bad location reports."]
pub struct GoogleMapsPlayablelocationsV3LogPlayerReportsRequest {
    #[serde(rename = "clientInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Information about the client device (for example, device model and operating system)."]
    pub client_info: ::std::option::Option<::std::boxed::Box<GoogleMapsUnityClientInfo>>,
    #[serde(rename = "playerReports")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Player reports. The maximum number of player reports that you can log at once is 50."]
    pub player_reports: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleMapsPlayablelocationsV3PlayerReport>>,
    >,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A string that uniquely identifies the log player reports request. This allows you to detect duplicate requests. We recommend that you use UUIDs for this value. The value must not exceed 50 characters. You should reuse the `request_id` only when retrying a request in the case of a failure. In that case, the request must be identical to the one that failed."]
    pub request_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response for the LogPlayerReports method. This method returns no data upon success."]
pub struct GoogleMapsPlayablelocationsV3LogPlayerReportsResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A report submitted by a player about a playable location that is considered inappropriate for use in the game."]
pub struct GoogleMapsPlayablelocationsV3PlayerReport {
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language code (in BCP-47 format) indicating the language of the freeform description provided in `reason_details`. Examples are \"en\", \"en-US\" or \"ja-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the playable location."]
    pub location_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A free-form description detailing why the playable location is considered bad."]
    pub reason_details: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. One or more reasons why this playable location is considered bad."]
    pub reasons: ::std::option::Option<
        ::std::vec::Vec<GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum>,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum {
    #[serde(rename = "BAD_LOCATION_REASON_UNSPECIFIED")]
    #[doc = "Unspecified reason. Do not use."]
    BadLocationReasonUnspecified,
    #[serde(rename = "OTHER")]
    #[doc = "The reason isn't one of the reasons in this enumeration."]
    Other,
    #[serde(rename = "NOT_PEDESTRIAN_ACCESSIBLE")]
    #[doc = "The playable location isn't accessible to pedestrians. For example, if it's in the middle of a highway."]
    NotPedestrianAccessible,
    #[serde(rename = "NOT_OPEN_TO_PUBLIC")]
    #[doc = "The playable location isn't open to the public. For example, a private office building."]
    NotOpenToPublic,
    #[serde(rename = "PERMANENTLY_CLOSED")]
    #[doc = "The playable location is permanently closed. For example, when a business has been shut down."]
    PermanentlyClosed,
    #[serde(rename = "TEMPORARILY_INACCESSIBLE")]
    #[doc = "The playable location is temporarily inaccessible. For example, when a business has closed for renovations."]
    TemporarilyInaccessible,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the area to search for playable locations."]
pub struct GoogleMapsPlayablelocationsV3SampleAreaFilter {
    #[serde(rename = "s2CellId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The S2 cell ID of the area you want. This must be between cell level 11 and 14 (inclusive). S2 cells are 64-bit integers that identify areas on the Earth. They are hierarchical, and can therefore be used for spatial indexing. The S2 geometry library is available in a number of languages: * [C++](https://github.com/google/s2geometry) * [Java](https://github.com/google/s2-geometry-library-java) * [Go](https://github.com/golang/geo) * [Python](https://github.com/google/s2geometry/tree/master/src/python)"]
    pub s2_cell_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Encapsulates a filter criterion for searching for a set of playable locations."]
pub struct GoogleMapsPlayablelocationsV3SampleCriterion {
    #[serde(rename = "fieldsToReturn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies which `PlayableLocation` fields are returned. `name` (which is used for logging impressions), `center_point` and `place_id` (or `plus_code`) are always returned. The following fields are omitted unless you specify them here: * snapped_point * types Note: The more fields you include, the more expensive in terms of data and associated latency your query will be."]
    pub fields_to_return: ::std::option::Option<::std::string::String>,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies filtering options, and specifies what will be included in the result set."]
    pub filter: ::std::option::Option<::std::boxed::Box<GoogleMapsPlayablelocationsV3SampleFilter>>,
    #[serde(rename = "gameObjectType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. An arbitrary, developer-defined identifier of the type of game object that the playable location is used for. This field allows you to specify criteria per game object type when searching for playable locations. You should assign a unique `game_object_type` ID across all `request_criteria` to represent a distinct type of game object. For example, 1=monster location, 2=powerup location. The response contains a map."]
    pub game_object_type: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the filters to use when searching for playable locations."]
pub struct GoogleMapsPlayablelocationsV3SampleFilter {
    #[serde(rename = "includedTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts the set of playable locations to just the [types](/maps/documentation/gaming/tt/types) that you want."]
    pub included_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "maxLocationCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the maximum number of playable locations to return. This value must not be greater than 1000. The default value is 100. Only the top-ranking playable locations are returned."]
    pub max_location_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "spacing")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of options that control the spacing between playable locations. By default the minimum distance between locations is 200m."]
    pub spacing:
        ::std::option::Option<::std::boxed::Box<GoogleMapsPlayablelocationsV3SampleSpacingOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A geographical point suitable for placing game objects in location-based games."]
pub struct GoogleMapsPlayablelocationsV3SamplePlayableLocation {
    #[serde(rename = "centerPoint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The latitude and longitude associated with the center of the playable location. By default, the set of playable locations returned from SamplePlayableLocations use center-point coordinates."]
    pub center_point: ::std::option::Option<::std::boxed::Box<GoogleTypeLatLng>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of this playable location."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A [place ID] (https://developers.google.com/places/place-id)"]
    pub place_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "plusCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A [plus code] (http://openlocationcode.com)"]
    pub plus_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "snappedPoint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The playable location's coordinates, snapped to the sidewalk of the nearest road, if a nearby road exists."]
    pub snapped_point: ::std::option::Option<::std::boxed::Box<GoogleTypeLatLng>>,
    #[serde(rename = "types")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A collection of [Playable Location Types](/maps/documentation/gaming/tt/types) for this playable location. The first type in the collection is the primary type. Type information might not be available for all playable locations."]
    pub types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of PlayableLocation objects that satisfies a single Criterion."]
pub struct GoogleMapsPlayablelocationsV3SamplePlayableLocationList {
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of playable locations for this game object type."]
    pub locations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleMapsPlayablelocationsV3SamplePlayableLocation>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = " Life of a query: - When a game starts in a new location, your game server issues a SamplePlayableLocations request. The request specifies the S2 cell, and contains one or more \"criteria\" for filtering: - Criterion 0: i locations for long-lived bases, or level 0 monsters, or... - Criterion 1: j locations for short-lived bases, or level 1 monsters, ... - Criterion 2: k locations for random objects. - etc (up to 5 criterion may be specified). `PlayableLocationList` will then contain mutually exclusive lists of `PlayableLocation` objects that satisfy each of the criteria. Think of it as a collection of real-world locations that you can then associate with your game state. Note: These points are impermanent in nature. E.g, parks can close, and places can be removed. The response specifies how long you can expect the playable locations to last. Once they expire, you should query the `samplePlayableLocations` API again to get a fresh view of the real world."]
pub struct GoogleMapsPlayablelocationsV3SamplePlayableLocationsRequest {
    #[serde(rename = "areaFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Specifies the area to search within for playable locations."]
    pub area_filter:
        ::std::option::Option<::std::boxed::Box<GoogleMapsPlayablelocationsV3SampleAreaFilter>>,
    #[serde(rename = "criteria")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Specifies one or more (up to 5) criteria for filtering the returned playable locations."]
    pub criteria: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleMapsPlayablelocationsV3SampleCriterion>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = " Response for the SamplePlayableLocations method."]
pub struct GoogleMapsPlayablelocationsV3SamplePlayableLocationsResponse {
    #[serde(rename = "locationsPerGameObjectType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Each PlayableLocation object corresponds to a game_object_type specified in the request."]
    pub locations_per_game_object_type: ::std::option::Option<
        ::std::collections::BTreeMap<
            String,
            ::std::boxed::Box<GoogleMapsPlayablelocationsV3SamplePlayableLocationList>,
        >,
    >,
    #[serde(rename = "ttl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Specifies the \"time-to-live\" for the set of playable locations. You can use this value to determine how long to cache the set of playable locations. After this length of time, your back-end game server should issue a new SamplePlayableLocations request to get a fresh set of playable locations (because for example, they might have been removed, a park might have closed for the day, a business might have closed permanently)."]
    pub ttl: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A set of options that specifies the separation between playable locations."]
pub struct GoogleMapsPlayablelocationsV3SampleSpacingOptions {
    #[serde(rename = "minSpacingMeters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The minimum spacing between any two playable locations, measured in meters. The minimum value is 30. The maximum value is 1000. Inputs will be rounded up to the next 10 meter interval. The default value is 200m. Set this field to remove tight clusters of playable locations. Note: The spacing is a greedy algorithm. It optimizes for selecting the highest ranking locations first, not to maximize the number of locations selected. Consider the following scenario: * Rank: A: 2, B: 1, C: 3. * Distance: A--200m--B--200m--C If spacing=250, it will pick the highest ranked location [B], not [A, C]. Note: Spacing works within the game object type itself, as well as the previous ones. Suppose three game object types, each with the following spacing: * X: 400m, Y: undefined, Z: 200m. 1. Add locations for X, within 400m of each other. 2. Add locations for Y, without any spacing. 3. Finally, add locations for Z within 200m of each other as well X and Y. The distance diagram between those locations end up as: * From->To. * X->X: 400m * Y->X, Y->Y: unspecified. * Z->X, Z->Y, Z->Z: 200m."]
    pub min_spacing_meters: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "pointType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies whether the minimum spacing constraint applies to the center-point or to the snapped point of playable locations. The default value is `CENTER_POINT`. If a snapped point is not available for a playable location, its center-point is used instead. Set this to the point type used in your game."]
    pub point_type:
        ::std::option::Option<GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Specifies whether the minimum spacing constraint applies to the center-point or to the snapped point of playable locations. The default value is `CENTER_POINT`. If a snapped point is not available for a playable location, its center-point is used instead. Set this to the point type used in your game."]
pub enum GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointTypeEnum {
    #[serde(rename = "POINT_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified point type. Do not use this value."]
    PointTypeUnspecified,
    #[serde(rename = "CENTER_POINT")]
    #[doc = "The geographic coordinates correspond to the center of the location."]
    CenterPoint,
    #[serde(rename = "SNAPPED_POINT")]
    #[doc = "The geographic coordinates correspond to the location snapped to the sidewalk of the nearest road (when a nearby road exists)."]
    SnappedPoint,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Client information."]
pub struct GoogleMapsUnityClientInfo {
    #[serde(rename = "apiClient")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API client name and version. For example, the SDK calling the API. The exact format is up to the client."]
    pub api_client: ::std::option::Option<::std::string::String>,
    #[serde(rename = "applicationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Application ID, such as the package name on Android and the bundle identifier on iOS platforms."]
    pub application_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "applicationVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Application version number, such as \"1.2.3\". The exact format is application-dependent."]
    pub application_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deviceModel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device model as reported by the device. The exact format is platform-dependent."]
    pub device_model: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language code (in BCP-47 format) indicating the UI language of the client. Examples are \"en\", \"en-US\" or \"ja-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operatingSystem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Operating system name and version as reported by the OS. For example, \"Mac OS X 10.10.4\". The exact format is platform-dependent."]
    pub operating_system: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operatingSystemBuild")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Build number/version of the operating system. e.g., the contents of android.os.Build.ID in Android, or the contents of sysctl \"kern.osversion\" in iOS."]
    pub operating_system_build: ::std::option::Option<::std::string::String>,
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Platform where the application is running."]
    pub platform: ::std::option::Option<GoogleMapsUnityClientInfoPlatformEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Platform where the application is running."]
pub enum GoogleMapsUnityClientInfoPlatformEnum {
    #[serde(rename = "PLATFORM_UNSPECIFIED")]
    #[doc = "Unspecified or unknown OS."]
    PlatformUnspecified,
    #[serde(rename = "EDITOR")]
    #[doc = "Development environment."]
    Editor,
    #[serde(rename = "MAC_OS")]
    #[doc = "macOS."]
    MacOs,
    #[serde(rename = "WINDOWS")]
    #[doc = "Windows."]
    Windows,
    #[serde(rename = "LINUX")]
    #[doc = "Linux"]
    Linux,
    #[serde(rename = "ANDROID")]
    #[doc = "Android"]
    Android,
    #[serde(rename = "IOS")]
    #[doc = "iOS"]
    Ios,
    #[serde(rename = "WEB_GL")]
    #[doc = "WebGL."]
    WebGl,
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
