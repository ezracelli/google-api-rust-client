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
    pub mod achievement_configurations {
        pub mod methods {
            pub mod list {
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of resource configurations to return in the response, used for paging. For any response, the actual number of resources returned may be less than the specified `maxResults`."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod leaderboard_configurations {
        pub mod methods {
            pub mod list {
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of resource configurations to return in the response, used for paging. For any response, the actual number of resources returned may be less than the specified `maxResults`."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
    #[doc = "An achievement configuration resource."]
    pub struct AchievementConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "achievementType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the achievement."]
        pub achievement_type: ::std::option::Option<AchievementConfigurationAchievementTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "draft")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The draft data of the achievement."]
        pub draft: ::std::option::Option<::std::boxed::Box<AchievementConfigurationDetail>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the achievement."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "initialState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The initial state of the achievement."]
        pub initial_state: ::std::option::Option<AchievementConfigurationInitialStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#achievementConfiguration`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "published")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The read-only published data of the achievement."]
        pub published: ::std::option::Option<::std::boxed::Box<AchievementConfigurationDetail>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stepsToUnlock")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Steps to unlock. Only applicable to incremental achievements."]
        pub steps_to_unlock: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "token")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token for this resource."]
        pub token: ::std::option::Option<::std::string::String>,
    }
    impl AchievementConfiguration {
        pub fn builder() -> AchievementConfigurationBuilder {
            AchievementConfigurationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the achievement."]
    pub enum AchievementConfigurationAchievementTypeEnum {
        #[serde(rename = "ACHIEVEMENT_TYPE_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        AchievementTypeUnspecified,
        #[serde(rename = "STANDARD")]
        #[doc = "Achievement is either locked or unlocked."]
        Standard,
        #[serde(rename = "INCREMENTAL")]
        #[doc = "Achievement is incremental."]
        Incremental,
    }
    impl ::std::default::Default for AchievementConfigurationAchievementTypeEnum {
        fn default() -> Self {
            Self::AchievementTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The initial state of the achievement."]
    pub enum AchievementConfigurationInitialStateEnum {
        #[serde(rename = "INITIAL_STATE_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        InitialStateUnspecified,
        #[serde(rename = "HIDDEN")]
        #[doc = "Achievement is hidden."]
        Hidden,
        #[serde(rename = "REVEALED")]
        #[doc = "Achievement is revealed."]
        Revealed,
    }
    impl ::std::default::Default for AchievementConfigurationInitialStateEnum {
        fn default() -> Self {
            Self::InitialStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An achievement configuration detail."]
    pub struct AchievementConfigurationDetail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Localized strings for the achievement description."]
        pub description: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iconUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The icon url of this achievement. Writes to this field are ignored."]
        pub icon_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#achievementConfigurationDetail`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Localized strings for the achievement name."]
        pub name: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pointValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Point value for the achievement."]
        pub point_value: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortRank")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sort rank of this achievement. Writes to this field are ignored."]
        pub sort_rank: ::std::option::Option<::std::primitive::i64>,
    }
    impl AchievementConfigurationDetail {
        pub fn builder() -> AchievementConfigurationDetailBuilder {
            AchievementConfigurationDetailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A ListConfigurations response."]
    pub struct AchievementConfigurationListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The achievement configurations."]
        pub items:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AchievementConfiguration>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#achievementConfigurationListResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pagination token for the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl AchievementConfigurationListResponse {
        pub fn builder() -> AchievementConfigurationListResponseBuilder {
            AchievementConfigurationListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A number affix resource."]
    pub struct GamesNumberAffixConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "few")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the language requires special treatment of \"small\" numbers (as with 2, 3, and 4 in Czech; or numbers ending 2, 3, or 4 but not 12, 13, or 14 in Polish)."]
        pub few: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "many")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the language requires special treatment of \"large\" numbers (as with numbers ending 11-99 in Maltese)."]
        pub many: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "one")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the language requires special treatment of numbers like one (as with the number 1 in English and most other languages; in Russian, any number ending in 1 but not ending in 11 is in this class)."]
        pub one: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "other")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the language does not require special treatment of the given quantity (as with all numbers in Chinese, or 42 in English)."]
        pub other: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "two")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the language requires special treatment of numbers like two (as with 2 in Welsh, or 102 in Slovenian)."]
        pub two: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zero")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the language requires special treatment of the number 0 (as in Arabic)."]
        pub zero: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
    }
    impl GamesNumberAffixConfiguration {
        pub fn builder() -> GamesNumberAffixConfigurationBuilder {
            GamesNumberAffixConfigurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A number format resource."]
    pub struct GamesNumberFormatConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The curreny code string. Only used for CURRENCY format type."]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numDecimalPlaces")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of decimal places for number. Only used for NUMERIC format type."]
        pub num_decimal_places: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numberFormatType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The formatting for the number."]
        pub number_format_type:
            ::std::option::Option<GamesNumberFormatConfigurationNumberFormatTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suffix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional suffix for the NUMERIC format type. These strings follow the same plural rules as all Android string resources."]
        pub suffix: ::std::option::Option<::std::boxed::Box<GamesNumberAffixConfiguration>>,
    }
    impl GamesNumberFormatConfiguration {
        pub fn builder() -> GamesNumberFormatConfigurationBuilder {
            GamesNumberFormatConfigurationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The formatting for the number."]
    pub enum GamesNumberFormatConfigurationNumberFormatTypeEnum {
        #[serde(rename = "NUMBER_FORMAT_TYPE_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        NumberFormatTypeUnspecified,
        #[serde(rename = "NUMERIC")]
        #[doc = "Numbers are formatted to have no digits or fixed number of digits after the decimal point according to locale. An optional custom unit can be added."]
        Numeric,
        #[serde(rename = "TIME_DURATION")]
        #[doc = "Numbers are formatted to hours, minutes and seconds."]
        TimeDuration,
        #[serde(rename = "CURRENCY")]
        #[doc = "Numbers are formatted to currency according to locale."]
        Currency,
    }
    impl ::std::default::Default for GamesNumberFormatConfigurationNumberFormatTypeEnum {
        fn default() -> Self {
            Self::NumberFormatTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An image configuration resource."]
    pub struct ImageConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The image type for the image."]
        pub image_type: ::std::option::Option<ImageConfigurationImageTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#imageConfiguration`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource ID of resource which the image belongs to."]
        pub resource_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The url for this image."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl ImageConfiguration {
        pub fn builder() -> ImageConfigurationBuilder {
            ImageConfigurationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The image type for the image."]
    pub enum ImageConfigurationImageTypeEnum {
        #[serde(rename = "IMAGE_TYPE_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        ImageTypeUnspecified,
        #[serde(rename = "ACHIEVEMENT_ICON")]
        #[doc = "The icon image for an achievement resource."]
        AchievementIcon,
        #[serde(rename = "LEADERBOARD_ICON")]
        #[doc = "The icon image for a leaderboard resource."]
        LeaderboardIcon,
    }
    impl ::std::default::Default for ImageConfigurationImageTypeEnum {
        fn default() -> Self {
            Self::ImageTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An leaderboard configuration resource."]
    pub struct LeaderboardConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "draft")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The draft data of the leaderboard."]
        pub draft: ::std::option::Option<::std::boxed::Box<LeaderboardConfigurationDetail>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the leaderboard."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#leaderboardConfiguration`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "published")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The read-only published data of the leaderboard."]
        pub published: ::std::option::Option<::std::boxed::Box<LeaderboardConfigurationDetail>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scoreMax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum score that can be posted to this leaderboard."]
        pub score_max: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scoreMin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum score that can be posted to this leaderboard."]
        pub score_min: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scoreOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub score_order: ::std::option::Option<LeaderboardConfigurationScoreOrderEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "token")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token for this resource."]
        pub token: ::std::option::Option<::std::string::String>,
    }
    impl LeaderboardConfiguration {
        pub fn builder() -> LeaderboardConfigurationBuilder {
            LeaderboardConfigurationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum LeaderboardConfigurationScoreOrderEnum {
        #[serde(rename = "SCORE_ORDER_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        ScoreOrderUnspecified,
        #[serde(rename = "LARGER_IS_BETTER")]
        #[doc = "Larger scores posted are ranked higher."]
        LargerIsBetter,
        #[serde(rename = "SMALLER_IS_BETTER")]
        #[doc = "Smaller scores posted are ranked higher."]
        SmallerIsBetter,
    }
    impl ::std::default::Default for LeaderboardConfigurationScoreOrderEnum {
        fn default() -> Self {
            Self::ScoreOrderUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A leaderboard configuration detail."]
    pub struct LeaderboardConfigurationDetail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iconUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The icon url of this leaderboard. Writes to this field are ignored."]
        pub icon_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#leaderboardConfigurationDetail`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Localized strings for the leaderboard name."]
        pub name: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scoreFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The score formatting for the leaderboard."]
        pub score_format: ::std::option::Option<::std::boxed::Box<GamesNumberFormatConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortRank")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sort rank of this leaderboard. Writes to this field are ignored."]
        pub sort_rank: ::std::option::Option<::std::primitive::i64>,
    }
    impl LeaderboardConfigurationDetail {
        pub fn builder() -> LeaderboardConfigurationDetailBuilder {
            LeaderboardConfigurationDetailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A ListConfigurations response."]
    pub struct LeaderboardConfigurationListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The leaderboard configurations."]
        pub items:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LeaderboardConfiguration>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#leaderboardConfigurationListResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pagination token for the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl LeaderboardConfigurationListResponse {
        pub fn builder() -> LeaderboardConfigurationListResponseBuilder {
            LeaderboardConfigurationListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A localized string resource."]
    pub struct LocalizedString {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#localizedString`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The locale string."]
        pub locale: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The string value."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl LocalizedString {
        pub fn builder() -> LocalizedStringBuilder {
            LocalizedStringBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A localized string bundle resource."]
    pub struct LocalizedStringBundle {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#localizedStringBundle`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "translations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The locale strings."]
        pub translations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LocalizedString>>>,
    }
    impl LocalizedStringBundle {
        pub fn builder() -> LocalizedStringBundleBuilder {
            LocalizedStringBundleBuilder::default()
        }
    }
}
