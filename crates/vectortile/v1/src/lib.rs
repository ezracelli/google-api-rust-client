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
        serde_json::from_str(&"json").unwrap()
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
pub mod resources {
    pub mod featuretiles {
        pub mod methods {
            pub mod get {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "alwaysIncludeBuildingFootprints")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag indicating whether the returned tile will always contain 2.5D footprints for structures. If enabled_modeled_volumes is set, this will mean that structures will have both their 3D models and 2.5D footprints returned."]
                    pub always_include_building_footprints:
                        ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientInfo.apiClient")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "API client name and version. For example, the SDK calling the API. The exact format is up to the client."]
                    pub client_info_api_client: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientInfo.applicationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Application ID, such as the package name on Android and the bundle identifier on iOS platforms."]
                    pub client_info_application_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientInfo.applicationVersion")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Application version number, such as \"1.2.3\". The exact format is application-dependent."]
                    pub client_info_application_version:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientInfo.deviceModel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Device model as reported by the device. The exact format is platform-dependent."]
                    pub client_info_device_model: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientInfo.operatingSystem")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Operating system name and version as reported by the OS. For example, \"Mac OS X 10.10.4\". The exact format is platform-dependent."]
                    pub client_info_operating_system: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientInfo.platform")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Platform where the application is running."]
                    pub client_info_platform:
                        ::std::option::Option<QueryParametersClientInfoPlatformEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientInfo.userId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. A client-generated user ID. The ID should be generated and persisted during the first user session or whenever a pre-existing ID is not found. The exact format is up to the client. This must be non-empty in a GetFeatureTileRequest (whether via the header or GetFeatureTileRequest.client_info)."]
                    pub client_info_user_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientTileVersionId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional version id identifying the tile that is already in the client's cache. This field should be populated with the most recent version_id value returned by the API for the requested tile. If the version id is empty the server always returns a newly rendered tile. If it is provided the server checks if the tile contents would be identical to one that's already on the client, and if so, returns a stripped-down response tile with STATUS_OK_DATA_UNCHANGED instead."]
                    pub client_tile_version_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "enableDetailedHighwayTypes")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag indicating whether detailed highway types should be returned. If this is set, the CONTROLLED_ACCESS_HIGHWAY type may be returned. If not, then these highways will have the generic HIGHWAY type. This exists for backwards compatibility reasons."]
                    pub enable_detailed_highway_types:
                        ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "enableFeatureNames")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag indicating whether human-readable names should be returned for features. If this is set, the display_name field on the feature will be filled out."]
                    pub enable_feature_names: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "enableModeledVolumes")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag indicating whether 3D building models should be enabled. If this is set structures will be returned as 3D modeled volumes rather than 2.5D extruded areas where possible."]
                    pub enable_modeled_volumes: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "enablePoliticalFeatures")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag indicating whether political features should be returned."]
                    pub enable_political_features: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "enablePrivateRoads")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag indicating whether the returned tile will contain road features that are marked private. Private roads are indicated by the Feature.segment_info.road_info.is_private field."]
                    pub enable_private_roads: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "enableUnclippedBuildings")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag indicating whether unclipped buildings should be returned. If this is set, building render ops will extend beyond the tile boundary. Buildings will only be returned on the tile that contains their centroid."]
                    pub enable_unclipped_buildings: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "languageCode")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. The BCP-47 language code corresponding to the language in which the name was requested, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
                    pub language_code: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "regionCode")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. The Unicode country/region code (CLDR) of the location from which the request is coming from, such as \"US\" and \"419\". For more information, see http://www.unicode.org/reports/tr35/#unicode_region_subtag."]
                    pub region_code: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Platform where the application is running."]
                pub enum QueryParametersClientInfoPlatformEnum {
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
                impl ::std::default::Default for QueryParametersClientInfoPlatformEnum {
                    fn default() -> Self {
                        Self::PlatformUnspecified
                    }
                }
            }
        }
    }
    pub mod terraintiles {
        pub mod methods {
            pub mod get {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "altitudePrecisionCentimeters")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The precision of terrain altitudes in centimeters. Possible values: between 1 (cm level precision) and 1,000,000 (10-kilometer level precision)."]
                    pub altitude_precision_centimeters:
                        ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientInfo.apiClient")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "API client name and version. For example, the SDK calling the API. The exact format is up to the client."]
                    pub client_info_api_client: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientInfo.applicationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Application ID, such as the package name on Android and the bundle identifier on iOS platforms."]
                    pub client_info_application_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientInfo.applicationVersion")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Application version number, such as \"1.2.3\". The exact format is application-dependent."]
                    pub client_info_application_version:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientInfo.deviceModel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Device model as reported by the device. The exact format is platform-dependent."]
                    pub client_info_device_model: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientInfo.operatingSystem")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Operating system name and version as reported by the OS. For example, \"Mac OS X 10.10.4\". The exact format is platform-dependent."]
                    pub client_info_operating_system: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientInfo.platform")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Platform where the application is running."]
                    pub client_info_platform:
                        ::std::option::Option<QueryParametersClientInfoPlatformEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientInfo.userId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. A client-generated user ID. The ID should be generated and persisted during the first user session or whenever a pre-existing ID is not found. The exact format is up to the client. This must be non-empty in a GetFeatureTileRequest (whether via the header or GetFeatureTileRequest.client_info)."]
                    pub client_info_user_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxElevationResolutionCells")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum allowed resolution for the returned elevation heightmap. Possible values: between 1 and 1024 (and not less than min_elevation_resolution_cells). Over-sized heightmaps will be non-uniformly down-sampled such that each edge is no longer than this value. Non-uniformity is chosen to maximise the amount of preserved data. For example: Original resolution: 100px (width) * 30px (height) max_elevation_resolution: 30 New resolution: 30px (width) * 30px (height)"]
                    pub max_elevation_resolution_cells:
                        ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "minElevationResolutionCells")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The minimum allowed resolution for the returned elevation heightmap. Possible values: between 0 and 1024 (and not more than max_elevation_resolution_cells). Zero is supported for backward compatibility. Under-sized heightmaps will be non-uniformly up-sampled such that each edge is no shorter than this value. Non-uniformity is chosen to maximise the amount of preserved data. For example: Original resolution: 30px (width) * 10px (height) min_elevation_resolution: 30 New resolution: 30px (width) * 30px (height)"]
                    pub min_elevation_resolution_cells:
                        ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "terrainFormats")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Terrain formats that the client understands."]
                    pub terrain_formats: ::std::option::Option<QueryParametersTerrainFormatsEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Platform where the application is running."]
                pub enum QueryParametersClientInfoPlatformEnum {
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
                impl ::std::default::Default for QueryParametersClientInfoPlatformEnum {
                    fn default() -> Self {
                        Self::PlatformUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Terrain formats that the client understands."]
                pub enum QueryParametersTerrainFormatsEnum {
                    #[serde(rename = "TERRAIN_FORMAT_UNKNOWN")]
                    #[doc = "An unknown or unspecified terrain format."]
                    TerrainFormatUnknown,
                    #[serde(rename = "FIRST_DERIVATIVE")]
                    #[doc = "Terrain elevation data encoded as a FirstDerivativeElevationGrid. ."]
                    FirstDerivative,
                    #[serde(rename = "SECOND_DERIVATIVE")]
                    #[doc = "Terrain elevation data encoded as a SecondDerivativeElevationGrid."]
                    SecondDerivative,
                }
                impl ::std::default::Default for QueryParametersTerrainFormatsEnum {
                    fn default() -> Self {
                        Self::TerrainFormatUnknown
                    }
                }
            }
        }
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an area. Used to represent regions such as water, parks, etc. Next ID: 10"]
    pub struct Area {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basemapZOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The z-order of this geometry when rendered on a flat basemap. Geometry with a lower z-order should be rendered beneath geometry with a higher z-order. This z-ordering does not imply anything about the altitude of the area relative to the ground, but it can be used to prevent z-fighting. Unlike Area.z_order this can be used to compare with Line.basemap_z_order, and in fact may yield more accurate rendering (where a line may be rendered beneath an area)."]
        pub basemap_z_order: ::std::option::Option<::std::boxed::Box<BasemapZOrder>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasExternalEdges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the polygon is not entirely internal to the feature that it belongs to: that is, some of the edges are bordering another feature."]
        pub has_external_edges: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internalEdges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When has_external_edges is true, the polygon has some edges that border another feature. This field indicates the internal edges that do not border another feature. Each value is an index into the vertices array, and denotes the start vertex of the internal edge (the next vertex in the boundary loop is the end of the edge). If the selected vertex is the last vertex in the boundary loop, then the edge between that vertex and the starting vertex of the loop is internal. This field may be used for styling. For example, building parapets could be placed only on the external edges of a building polygon, or water could be lighter colored near the external edges of a body of water. If has_external_edges is false, all edges are internal and this field will be empty."]
        pub internal_edges: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "loopBreaks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies the boundary loops of the polygon. Only set for INDEXED_TRIANGLE polygons. Each value is an index into the vertices array indicating the beginning of a loop. For instance, values of [2, 5] would indicate loop_data contained 3 loops with indices 0-1, 2-4, and 5-end. This may be used in conjunction with the internal_edges field for styling polygon boundaries. Note that an edge may be on a polygon boundary but still internal to the feature. For example, a feature split across multiple tiles will have an internal polygon boundary edge along the edge of the tile."]
        pub loop_breaks: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triangleIndices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the polygon encoding is of type INDEXED_TRIANGLES, this contains the indices of the triangle vertices in the vertex_offsets field. There are 3 vertex indices per triangle."]
        pub triangle_indices: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The polygon encoding type used for this area."]
        pub _type: ::std::option::Option<AreaTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertexOffsets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The vertices present in the polygon defining the area."]
        pub vertex_offsets: ::std::option::Option<::std::boxed::Box<Vertex2DList>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The z-ordering of this area. Areas with a lower z-order should be rendered beneath areas with a higher z-order. This z-ordering does not imply anything about the altitude of the line relative to the ground, but it can be used to prevent z-fighting during rendering on the client. This z-ordering can only be used to compare areas, and cannot be compared with the z_order field in the Line message. The z-order may be negative or zero. Prefer Area.basemap_z_order."]
        pub z_order: ::std::option::Option<::std::primitive::i64>,
    }
    impl Area {
        pub fn builder() -> AreaBuilder {
            AreaBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The polygon encoding type used for this area."]
    pub enum AreaTypeEnum {
        #[serde(rename = "TRIANGLE_FAN")]
        #[doc = "The first vertex in vertex_offset is the center of a triangle fan. The other vertices are arranged around this vertex in a fan shape. The following diagram showes a triangle fan polygon with the vertices labelled with their indices in the vertex_offset list. Triangle fan polygons always have a single boundary loop. Vertices may be in either a clockwise or counterclockwise order. (1) / \\ / \\ / \\ (0)-----(2) / \\ / / \\ / / \\ / (4)-----(3)"]
        TriangleFan,
        #[serde(rename = "INDEXED_TRIANGLES")]
        #[doc = "The polygon is a set of triangles with three vertex indices per triangle. The vertex indices can be found in the triangle_indices field. Indexed triangle polygons also contain information about boundary loops. These identify the loops at the boundary of the polygon and may be used in conjunction with the internal_edges field for styling. Boundary loops may represent either a hole or a disconnected component of the polygon. The following diagram shows an indexed triangle polygon with two boundary loops. (0) (4) / \\ / \\ / \\ / \\ (1)----(2) (3)----(5)"]
        IndexedTriangles,
        #[serde(rename = "TRIANGLE_STRIP")]
        #[doc = "A strip of triangles, where each triangle uses the last edge of the previous triangle. Vertices may be in either a clockwise or counterclockwise order. Only polygons without the has_external_edges flag set will use triangle strips. (0) / \\ / \\ / \\ (2)-----(1) / \\ / / \\ / / \\ / (4)-----(3)"]
        TriangleStrip,
    }
    impl ::std::default::Default for AreaTypeEnum {
        fn default() -> Self {
            Self::TriangleFan
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata necessary to determine the ordering of a particular basemap element relative to others. To render the basemap correctly, sort by z-plane, then z-grade, then z-within-grade."]
    pub struct BasemapZOrder {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zGrade")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The second most significant component of the ordering of a component to be rendered onto the basemap."]
        pub z_grade: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zPlane")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The most significant component of the ordering of a component to be rendered onto the basemap."]
        pub z_plane: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zWithinGrade")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The least significant component of the ordering of a component to be rendered onto the basemap."]
        pub z_within_grade: ::std::option::Option<::std::primitive::i64>,
    }
    impl BasemapZOrder {
        pub fn builder() -> BasemapZOrderBuilder {
            BasemapZOrderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a height-extruded area: a 3D prism with a constant X-Y plane cross section. Used to represent extruded buildings. A single building may consist of several extruded areas. The min_z and max_z fields are scaled to the size of the tile. An extruded area with a max_z value of 4096 has the same height as the width of the tile that it is on."]
    pub struct ExtrudedArea {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "area")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The area representing the footprint of the extruded area."]
        pub area: ::std::option::Option<::std::boxed::Box<Area>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxZ")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The z-value in local tile coordinates where the extruded area ends."]
        pub max_z: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minZ")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The z-value in local tile coordinates where the extruded area begins. This is non-zero for extruded areas that begin off the ground. For example, a building with a skybridge may have an extruded area component with a non-zero min_z."]
        pub min_z: ::std::option::Option<::std::primitive::i64>,
    }
    impl ExtrudedArea {
        pub fn builder() -> ExtrudedAreaBuilder {
            ExtrudedAreaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A feature representing a single geographic entity."]
    pub struct Feature {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localized name of this feature. Currently only returned for roads."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "geometry")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The geometry of this feature, representing the space that it occupies in the world."]
        pub geometry: ::std::option::Option<::std::boxed::Box<Geometry>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "placeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Place ID of this feature, suitable for use in Places API details requests."]
        pub place_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relations to other features."]
        pub relations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Relation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata for features with the SEGMENT FeatureType."]
        pub segment_info: ::std::option::Option<::std::boxed::Box<SegmentInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of this feature."]
        pub _type: ::std::option::Option<FeatureTypeEnum>,
    }
    impl Feature {
        pub fn builder() -> FeatureBuilder {
            FeatureBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of this feature."]
    pub enum FeatureTypeEnum {
        #[serde(rename = "FEATURE_TYPE_UNSPECIFIED")]
        #[doc = "Unknown feature type."]
        FeatureTypeUnspecified,
        #[serde(rename = "STRUCTURE")]
        #[doc = "Structures such as buildings and bridges."]
        Structure,
        #[serde(rename = "BAR")]
        #[doc = "A business serving alcoholic drinks to be consumed onsite."]
        Bar,
        #[serde(rename = "BANK")]
        #[doc = "A financial institution that offers services to the general public."]
        Bank,
        #[serde(rename = "LODGING")]
        #[doc = "A place that provides any type of lodging for travelers."]
        Lodging,
        #[serde(rename = "CAFE")]
        #[doc = "A business that sells coffee, tea, and sometimes small meals."]
        Cafe,
        #[serde(rename = "RESTAURANT")]
        #[doc = "A business that prepares meals on-site for service to customers."]
        Restaurant,
        #[serde(rename = "EVENT_VENUE")]
        #[doc = "A venue for private and public events."]
        EventVenue,
        #[serde(rename = "TOURIST_DESTINATION")]
        #[doc = "Place of interest to tourists, typically for natural or cultural value."]
        TouristDestination,
        #[serde(rename = "SHOPPING")]
        #[doc = "A structure containing a business or businesses that sell goods."]
        Shopping,
        #[serde(rename = "SCHOOL")]
        #[doc = "Institution where young people receive general (not vocation or professional) education."]
        School,
        #[serde(rename = "SEGMENT")]
        #[doc = "Segments such as roads and train lines."]
        Segment,
        #[serde(rename = "ROAD")]
        #[doc = "A way leading from one place to another intended for use by vehicles."]
        Road,
        #[serde(rename = "LOCAL_ROAD")]
        #[doc = "A small city street, typically for travel in a residential neighborhood."]
        LocalRoad,
        #[serde(rename = "ARTERIAL_ROAD")]
        #[doc = "Major through road that's expected to carry large volumes of traffic."]
        ArterialRoad,
        #[serde(rename = "HIGHWAY")]
        #[doc = "A major road including freeways and state highways."]
        Highway,
        #[serde(rename = "CONTROLLED_ACCESS_HIGHWAY")]
        #[doc = "A highway with grade-separated crossings that is accessed exclusively by ramps. These are usually called \"freeways\" or \"motorways\". The enable_detailed_highway_types request flag must be set in order for this type to be returned."]
        ControlledAccessHighway,
        #[serde(rename = "FOOTPATH")]
        #[doc = "A path that's primarily intended for use by pedestrians and/or cyclists."]
        Footpath,
        #[serde(rename = "RAIL")]
        #[doc = "Tracks intended for use by trains."]
        Rail,
        #[serde(rename = "FERRY")]
        #[doc = "Services which are part of the road network but are not roads."]
        Ferry,
        #[serde(rename = "REGION")]
        #[doc = "Non-water areas such as parks and forest."]
        Region,
        #[serde(rename = "PARK")]
        #[doc = "Outdoor areas such as parks and botanical gardens."]
        Park,
        #[serde(rename = "BEACH")]
        #[doc = "A pebbly or sandy shore along the edge of a sea or lake."]
        Beach,
        #[serde(rename = "FOREST")]
        #[doc = "Area of land covered by trees."]
        Forest,
        #[serde(rename = "POLITICAL")]
        #[doc = "Political entities, such as provinces and districts."]
        Political,
        #[serde(rename = "ADMINISTRATIVE_AREA1")]
        #[doc = "Top-level divisions within a country, such as prefectures or states."]
        AdministrativeArea1,
        #[serde(rename = "LOCALITY")]
        #[doc = "Cities, towns, and other municipalities."]
        Locality,
        #[serde(rename = "SUBLOCALITY")]
        #[doc = "Divisions within a locality like a borough or ward."]
        Sublocality,
        #[serde(rename = "WATER")]
        #[doc = "Water features such as rivers and lakes."]
        Water,
    }
    impl ::std::default::Default for FeatureTypeEnum {
        fn default() -> Self {
            Self::FeatureTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A tile containing information about the map features located in the region it covers."]
    pub struct FeatureTile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "coordinates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The global tile coordinates that uniquely identify this tile."]
        pub coordinates: ::std::option::Option<::std::boxed::Box<TileCoordinates>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "features")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Features present on this map tile."]
        pub features: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Feature>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of the tile. The tile resource name is prefixed by its collection ID `tiles/` followed by the resource ID, which encodes the tile's global x and y coordinates and zoom level as `@,,z`. For example, `tiles/@1,2,3z`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "providers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data providers for the data contained in this tile."]
        pub providers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProviderInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tile response status code to support tile caching."]
        pub status: ::std::option::Option<FeatureTileStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An opaque value, usually less than 30 characters, that contains version info about this tile and the data that was used to generate it. The client should store this value in its tile cache and pass it back to the API in the client_tile_version_id field of subsequent tile requests in order to enable the API to detect when the new tile would be the same as the one the client already has in its cache. Also see STATUS_OK_DATA_UNCHANGED."]
        pub version_id: ::std::option::Option<::std::string::String>,
    }
    impl FeatureTile {
        pub fn builder() -> FeatureTileBuilder {
            FeatureTileBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Tile response status code to support tile caching."]
    pub enum FeatureTileStatusEnum {
        #[serde(rename = "STATUS_OK")]
        #[doc = "Everything worked out OK. The cache-control header determines how long this Tile response may be cached by the client. See also version_id and STATUS_OK_DATA_UNCHANGED."]
        StatusOk,
        #[serde(rename = "STATUS_OK_DATA_UNCHANGED")]
        #[doc = "Indicates that the request was processed successfully and that the tile data that would have been returned are identical to the data already in the client's cache, as specified by the value of client_tile_version_id contained in GetFeatureTileRequest. In particular, the tile's features and providers will not be populated when the tile data is identical. However, the cache-control header and version_id can still change even when the tile contents itself does not, so clients should always use the most recent values returned by the API."]
        StatusOkDataUnchanged,
    }
    impl ::std::default::Default for FeatureTileStatusEnum {
        fn default() -> Self {
            Self::StatusOk
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A packed representation of a 2D grid of uniformly spaced points containing elevation data. Each point within the grid represents the altitude in meters above average sea level at that location within the tile. Elevations provided are (generally) relative to the EGM96 geoid, however some areas will be relative to NAVD88. EGM96 and NAVD88 are off by no more than 2 meters. The grid is oriented north-west to south-east, as illustrated: rows[0].a[0] rows[0].a[m] +-----------------+ | | | N | | ^ | | | | | W <-----> E | | | | | v | | S | | | +-----------------+ rows[n].a[0] rows[n].a[m] Rather than storing the altitudes directly, we store the diffs between them as integers at some requested level of precision to take advantage of integer packing. The actual altitude values a[] can be reconstructed using the scale and each row's first_altitude and altitude_diff fields."]
    pub struct FirstDerivativeElevationGrid {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "altitudeMultiplier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A multiplier applied to the altitude fields below to extract the actual altitudes in meters from the elevation grid."]
        pub altitude_multiplier: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rows of points containing altitude data making up the elevation grid. Each row is the same length. Rows are ordered from north to south. E.g: rows[0] is the north-most row, and rows[n] is the south-most row."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
    }
    impl FirstDerivativeElevationGrid {
        pub fn builder() -> FirstDerivativeElevationGridBuilder {
            FirstDerivativeElevationGridBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the geometry of a feature, that is, the shape that it has on the map. The local tile coordinate system has the origin at the north-west (upper-left) corner of the tile, and is scaled to 4096 units across each edge. The height (Z) axis has the same scale factor: an extruded area with a max_z value of 4096 has the same height as the width of the tile that it is on. There is no clipping boundary, so it is possible that some coordinates will lie outside the tile boundaries."]
    pub struct Geometry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "areas")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The areas present in this geometry."]
        pub areas: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Area>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "extrudedAreas")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The extruded areas present in this geometry. Not populated if modeled_volumes are included in this geometry unless always_include_building_footprints is set in GetFeatureTileRequest, in which case the client should decide which (extruded areas or modeled volumes) should be used (they should not be rendered together)."]
        pub extruded_areas: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ExtrudedArea>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lines")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The lines present in this geometry."]
        pub lines: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Line>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modeledVolumes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The modeled volumes present in this geometry. Not populated unless enable_modeled_volumes has been set in GetFeatureTileRequest."]
        pub modeled_volumes:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ModeledVolume>>>,
    }
    impl Geometry {
        pub fn builder() -> GeometryBuilder {
            GeometryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a 2D polyline. Used to represent segments such as roads, train tracks, etc."]
    pub struct Line {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basemapZOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The z-order of this geometry when rendered on a flat basemap. Geometry with a lower z-order should be rendered beneath geometry with a higher z-order. This z-ordering does not imply anything about the altitude of the area relative to the ground, but it can be used to prevent z-fighting. Unlike Line.z_order this can be used to compare with Area.basemap_z_order, and in fact may yield more accurate rendering (where a line may be rendered beneath an area)."]
        pub basemap_z_order: ::std::option::Option<::std::boxed::Box<BasemapZOrder>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertexOffsets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The vertices present in the polyline."]
        pub vertex_offsets: ::std::option::Option<::std::boxed::Box<Vertex2DList>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The z-order of the line. Lines with a lower z-order should be rendered beneath lines with a higher z-order. This z-ordering does not imply anything about the altitude of the area relative to the ground, but it can be used to prevent z-fighting during rendering on the client. In general, larger and more important road features will have a higher z-order line associated with them. This z-ordering can only be used to compare lines, and cannot be compared with the z_order field in the Area message. The z-order may be negative or zero. Prefer Line.basemap_z_order."]
        pub z_order: ::std::option::Option<::std::primitive::i64>,
    }
    impl Line {
        pub fn builder() -> LineBuilder {
            LineBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a modeled volume in 3D space. Used to represent 3D buildings."]
    pub struct ModeledVolume {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "strips")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The triangle strips present in this mesh."]
        pub strips: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TriangleStrip>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertexOffsets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The vertices present in the mesh defining the modeled volume."]
        pub vertex_offsets: ::std::option::Option<::std::boxed::Box<Vertex3DList>>,
    }
    impl ModeledVolume {
        pub fn builder() -> ModeledVolumeBuilder {
            ModeledVolumeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the data providers that should be included in the attribution string shown by the client."]
    pub struct ProviderInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Attribution string for this provider. This string is not localized."]
        pub description: ::std::option::Option<::std::string::String>,
    }
    impl ProviderInfo {
        pub fn builder() -> ProviderInfoBuilder {
            ProviderInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a relation to another feature in the tile. For example, a building might be occupied by a given POI. The related feature can be retrieved using the related feature index."]
    pub struct Relation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relatedFeatureIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Zero-based index to look up the related feature from the list of features in the tile."]
        pub related_feature_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relation type between the origin feature to the related feature."]
        pub relation_type: ::std::option::Option<RelationRelationTypeEnum>,
    }
    impl Relation {
        pub fn builder() -> RelationBuilder {
            RelationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Relation type between the origin feature to the related feature."]
    pub enum RelationRelationTypeEnum {
        #[serde(rename = "RELATION_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified relation type. Should never happen."]
        RelationTypeUnspecified,
        #[serde(rename = "OCCUPIES")]
        #[doc = "The origin feature occupies the related feature."]
        Occupies,
        #[serde(rename = "PRIMARILY_OCCUPIED_BY")]
        #[doc = "The origin feature is primarily occupied by the related feature."]
        PrimarilyOccupiedBy,
    }
    impl ::std::default::Default for RelationRelationTypeEnum {
        fn default() -> Self {
            Self::RelationTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Extra metadata relating to roads."]
    pub struct RoadInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isPrivate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Road has signage discouraging or prohibiting use by the general public. E.g., roads with signs that say \"Private\", or \"No trespassing.\""]
        pub is_private: ::std::option::Option<::std::primitive::bool>,
    }
    impl RoadInfo {
        pub fn builder() -> RoadInfoBuilder {
            RoadInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A row of altitude points in the elevation grid, ordered from west to east."]
    pub struct Row {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "altitudeDiffs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The difference between each successive pair of altitudes, from west to east. The first, westmost point, is just the altitude rather than a diff. The units are specified by the altitude_multiplier parameter above; the value in meters is given by altitude_multiplier * altitude_diffs[n]. The altitude row (in metres above sea level) can be reconstructed with: a[0] = altitude_diffs[0] * altitude_multiplier when n > 0, a[n] = a[n-1] + altitude_diffs[n-1] * altitude_multiplier."]
        pub altitude_diffs: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    }
    impl Row {
        pub fn builder() -> RowBuilder {
            RowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A packed representation of a 2D grid of uniformly spaced points containing elevation data. Each point within the grid represents the altitude in meters above average sea level at that location within the tile. Elevations provided are (generally) relative to the EGM96 geoid, however some areas will be relative to NAVD88. EGM96 and NAVD88 are off by no more than 2 meters. The grid is oriented north-west to south-east, as illustrated: rows[0].a[0] rows[0].a[m] +-----------------+ | | | N | | ^ | | | | | W <-----> E | | | | | v | | S | | | +-----------------+ rows[n].a[0] rows[n].a[m] Rather than storing the altitudes directly, we store the diffs of the diffs between them as integers at some requested level of precision to take advantage of integer packing. Note that the data is packed in such a way that is fast to decode in Unity and that further optimizes wire size."]
    pub struct SecondDerivativeElevationGrid {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "altitudeMultiplier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A multiplier applied to the elements in the encoded data to extract the actual altitudes in meters."]
        pub altitude_multiplier: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of columns included in the encoded elevation data (i.e. the horizontal resolution of the grid)."]
        pub column_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encodedData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A stream of elements each representing a point on the tile running across each row from left to right, top to bottom. There will be precisely horizontal_resolution * vertical_resolution elements in the stream. The elements are not the heights, rather the second order derivative of the values one would expect in a stream of height data. Each element is a varint with the following encoding: ------------------------------------------------------------------------| | Head Nibble | ------------------------------------------------------------------------| | Bit 0 | Bit 1 | Bits 2-3 | | Terminator| Sign (1=neg) | Least significant 2 bits of absolute error | ------------------------------------------------------------------------| | Tail Nibble #1 | ------------------------------------------------------------------------| | Bit 0 | Bit 1-3 | | Terminator| Least significant 3 bits of absolute error | ------------------------------------------------------------------------| | ... | Tail Nibble #n | ------------------------------------------------------------------------| | Bit 0 | Bit 1-3 | | Terminator| Least significant 3 bits of absolute error | ------------------------------------------------------------------------|"]
        pub encoded_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of rows included in the encoded elevation data (i.e. the vertical resolution of the grid)."]
        pub row_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl SecondDerivativeElevationGrid {
        pub fn builder() -> SecondDerivativeElevationGridBuilder {
            SecondDerivativeElevationGridBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Extra metadata relating to segments."]
    pub struct SegmentInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "roadInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata for features with the ROAD FeatureType."]
        pub road_info: ::std::option::Option<::std::boxed::Box<RoadInfo>>,
    }
    impl SegmentInfo {
        pub fn builder() -> SegmentInfoBuilder {
            SegmentInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A tile containing information about the terrain located in the region it covers."]
    pub struct TerrainTile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "coordinates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The global tile coordinates that uniquely identify this tile."]
        pub coordinates: ::std::option::Option<::std::boxed::Box<TileCoordinates>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstDerivative")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Terrain elevation data encoded as a FirstDerivativeElevationGrid."]
        pub first_derivative:
            ::std::option::Option<::std::boxed::Box<FirstDerivativeElevationGrid>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of the tile. The tile resource name is prefixed by its collection ID `terrain/` followed by the resource ID, which encodes the tile's global x and y coordinates and zoom level as `@,,z`. For example, `terrain/@1,2,3z`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secondDerivative")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Terrain elevation data encoded as a SecondDerivativeElevationGrid. ."]
        pub second_derivative:
            ::std::option::Option<::std::boxed::Box<SecondDerivativeElevationGrid>>,
    }
    impl TerrainTile {
        pub fn builder() -> TerrainTileBuilder {
            TerrainTileBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Global tile coordinates. Global tile coordinates reference a specific tile on the map at a specific zoom level. The origin of this coordinate system is always at the northwest corner of the map, with x values increasing from west to east and y values increasing from north to south. Tiles are indexed using x, y coordinates from that origin. The zoom level containing the entire world in a tile is 0, and it increases as you zoom in. Zoom level n + 1 will contain 4 times as many tiles as zoom level n. The zoom level controls the level of detail of the data that is returned. In particular, this affects the set of feature types returned, their density, and geometry simplification. The exact tile contents may change over time, but care will be taken to keep supporting the most important use cases. For example, zoom level 15 shows roads for orientation and planning in the local neighborhood and zoom level 17 shows buildings to give users on foot a sense of situational awareness."]
    pub struct TileCoordinates {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "x")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The x coordinate."]
        pub x: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "y")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The y coordinate."]
        pub y: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zoom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The Google Maps API zoom level."]
        pub zoom: ::std::option::Option<::std::primitive::i64>,
    }
    impl TileCoordinates {
        pub fn builder() -> TileCoordinatesBuilder {
            TileCoordinatesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a strip of triangles. Each triangle uses the last edge of the previous one. The following diagram shows an example of a triangle strip, with each vertex labeled with its index in the vertex_index array. (1)-----(3) / \\ / \\ / \\ / \\ / \\ / \\ (0)-----(2)-----(4) Vertices may be in either clockwise or counter-clockwise order."]
    pub struct TriangleStrip {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertexIndices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Index into the vertex_offset array representing the next vertex in the triangle strip."]
        pub vertex_indices: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    }
    impl TriangleStrip {
        pub fn builder() -> TriangleStripBuilder {
            TriangleStripBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "2D vertex list used for lines and areas. Each entry represents an offset from the previous one in local tile coordinates. The first entry is offset from (0, 0). For example, the list of vertices [(1,1), (2, 2), (1, 2)] would be encoded in vertex offsets as [(1, 1), (1, 1), (-1, 0)]."]
    pub struct Vertex2DList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "xOffsets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of x-offsets in local tile coordinates."]
        pub x_offsets: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "yOffsets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of y-offsets in local tile coordinates."]
        pub y_offsets: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    }
    impl Vertex2DList {
        pub fn builder() -> Vertex2DListBuilder {
            Vertex2DListBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "3D vertex list used for modeled volumes. Each entry represents an offset from the previous one in local tile coordinates. The first coordinate is offset from (0, 0, 0)."]
    pub struct Vertex3DList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "xOffsets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of x-offsets in local tile coordinates."]
        pub x_offsets: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "yOffsets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of y-offsets in local tile coordinates."]
        pub y_offsets: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zOffsets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of z-offsets in local tile coordinates."]
        pub z_offsets: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    }
    impl Vertex3DList {
        pub fn builder() -> Vertex3DListBuilder {
            Vertex3DListBuilder::default()
        }
    }
}
