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
    pub mod applications {
        pub mod methods {
            pub mod list_hidden {
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
                    #[doc = "The maximum number of player resources to return in the response, used for paging. For any response, the actual number of player resources returned may be less than the specified `maxResults`."]
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
    #[doc = "Achievement reset all response."]
    pub struct AchievementResetAllResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#achievementResetAllResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The achievement reset results."]
        pub results:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AchievementResetResponse>>>,
    }
    impl AchievementResetAllResponse {
        pub fn builder() -> AchievementResetAllResponseBuilder {
            AchievementResetAllResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AchievementResetMultipleForAllRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "achievement_ids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IDs of achievements to reset."]
        pub achievement_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#achievementResetMultipleForAllRequest`."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl AchievementResetMultipleForAllRequest {
        pub fn builder() -> AchievementResetMultipleForAllRequestBuilder {
            AchievementResetMultipleForAllRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An achievement reset response."]
    pub struct AchievementResetResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current state of the achievement. This is the same as the initial state of the achievement. Possible values are: - \"`HIDDEN`\"- Achievement is hidden. - \"`REVEALED`\" - Achievement is revealed. - \"`UNLOCKED`\" - Achievement is unlocked. "]
        pub current_state: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "definitionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of an achievement for which player state has been updated."]
        pub definition_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#achievementResetResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateOccurred")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag to indicate if the requested update actually occurred."]
        pub update_occurred: ::std::option::Option<::std::primitive::bool>,
    }
    impl AchievementResetResponse {
        pub fn builder() -> AchievementResetResponseBuilder {
            AchievementResetResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Multiple events reset all request."]
    pub struct EventsResetMultipleForAllRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "event_ids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IDs of events to reset."]
        pub event_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#eventsResetMultipleForAllRequest`."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl EventsResetMultipleForAllRequest {
        pub fn builder() -> EventsResetMultipleForAllRequestBuilder {
            EventsResetMultipleForAllRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "1P/3P metadata about the player's experience."]
    pub struct GamesPlayerExperienceInfoResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentExperiencePoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current number of experience points for the player."]
        pub current_experience_points: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current level of the player."]
        pub current_level: ::std::option::Option<::std::boxed::Box<GamesPlayerLevelResource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastLevelUpTimestampMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the player was leveled up, in millis since Unix epoch UTC."]
        pub last_level_up_timestamp_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The next level of the player. If the current level is the maximum level, this should be same as the current level."]
        pub next_level: ::std::option::Option<::std::boxed::Box<GamesPlayerLevelResource>>,
    }
    impl GamesPlayerExperienceInfoResource {
        pub fn builder() -> GamesPlayerExperienceInfoResourceBuilder {
            GamesPlayerExperienceInfoResourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "1P/3P metadata about a user's level."]
    pub struct GamesPlayerLevelResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "level")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The level for the user."]
        pub level: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxExperiencePoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum experience points for this level."]
        pub max_experience_points: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minExperiencePoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum experience points for this level."]
        pub min_experience_points: ::std::option::Option<::std::string::String>,
    }
    impl GamesPlayerLevelResource {
        pub fn builder() -> GamesPlayerLevelResourceBuilder {
            GamesPlayerLevelResourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The HiddenPlayer resource."]
    pub struct HiddenPlayer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hiddenTimeMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this player was hidden."]
        pub hidden_time_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#hiddenPlayer`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "player")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The player information."]
        pub player: ::std::option::Option<::std::boxed::Box<Player>>,
    }
    impl HiddenPlayer {
        pub fn builder() -> HiddenPlayerBuilder {
            HiddenPlayerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of hidden players."]
    pub struct HiddenPlayerList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The players."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HiddenPlayer>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#hiddenPlayerList`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pagination token for the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl HiddenPlayerList {
        pub fn builder() -> HiddenPlayerListBuilder {
            HiddenPlayerListBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Player resource."]
    pub struct Player {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "avatarImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The base URL for the image that represents the player."]
        pub avatar_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerUrlLandscape")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The url to the landscape mode player banner image."]
        pub banner_url_landscape: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerUrlPortrait")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The url to the portrait mode player banner image."]
        pub banner_url_portrait: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name to display for the player."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "experienceInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An object to represent Play Game experience information for the player."]
        pub experience_info:
            ::std::option::Option<::std::boxed::Box<GamesPlayerExperienceInfoResource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#player`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An object representation of the individual components of the player's name. For some players, these fields may not be present."]
        pub name: ::std::option::Option<PlayerName>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalPlayerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The player ID that was used for this player the first time they signed into the game in question. This is only populated for calls to player.get for the requesting player, only if the player ID has subsequently changed, and only to clients that support remapping player IDs."]
        pub original_player_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "playerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the player."]
        pub player_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The player's profile settings. Controls whether or not the player's profile is visible to other players."]
        pub profile_settings: ::std::option::Option<::std::boxed::Box<ProfileSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The player's title rewarded for their game activities."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl Player {
        pub fn builder() -> PlayerBuilder {
            PlayerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object representation of the individual components of the player's name. For some players, these fields may not be present."]
    pub struct PlayerName {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "familyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The family name of this player. In some places, this is known as the last name."]
        pub family_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "givenName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The given name of this player. In some places, this is known as the first name."]
        pub given_name: ::std::option::Option<::std::string::String>,
    }
    impl PlayerName {
        pub fn builder() -> PlayerNameBuilder {
            PlayerNameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of leaderboard reset resources."]
    pub struct PlayerScoreResetAllResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#playerScoreResetAllResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The leaderboard reset results."]
        pub results:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlayerScoreResetResponse>>>,
    }
    impl PlayerScoreResetAllResponse {
        pub fn builder() -> PlayerScoreResetAllResponseBuilder {
            PlayerScoreResetAllResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of reset leaderboard entry resources."]
    pub struct PlayerScoreResetResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "definitionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of an leaderboard for which player state has been updated."]
        pub definition_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#playerScoreResetResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resetScoreTimeSpans")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time spans of the updated score. Possible values are: - \"`ALL_TIME`\" - The score is an all-time score. - \"`WEEKLY`\" - The score is a weekly score. - \"`DAILY`\" - The score is a daily score. "]
        pub reset_score_time_spans: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl PlayerScoreResetResponse {
        pub fn builder() -> PlayerScoreResetResponseBuilder {
            PlayerScoreResetResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Profile settings"]
    pub struct ProfileSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#profileSettings`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileVisible")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub profile_visible: ::std::option::Option<::std::primitive::bool>,
    }
    impl ProfileSettings {
        pub fn builder() -> ProfileSettingsBuilder {
            ProfileSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ScoresResetMultipleForAllRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#scoresResetMultipleForAllRequest`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "leaderboard_ids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IDs of leaderboards to reset."]
        pub leaderboard_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ScoresResetMultipleForAllRequest {
        pub fn builder() -> ScoresResetMultipleForAllRequestBuilder {
            ScoresResetMultipleForAllRequestBuilder::default()
        }
    }
}
