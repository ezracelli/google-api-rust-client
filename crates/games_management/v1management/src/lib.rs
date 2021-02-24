#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Achievement reset all response."]
pub struct AchievementResetAllResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#achievementResetAllResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The achievement reset results."]
    pub results:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AchievementResetResponse>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AchievementResetMultipleForAllRequest {
    #[serde(rename = "achievement_ids")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IDs of achievements to reset."]
    pub achievement_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#achievementResetMultipleForAllRequest`."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An achievement reset response."]
pub struct AchievementResetResponse {
    #[serde(rename = "currentState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current state of the achievement. This is the same as the initial state of the achievement. Possible values are: - \"`HIDDEN`\"- Achievement is hidden. - \"`REVEALED`\" - Achievement is revealed. - \"`UNLOCKED`\" - Achievement is unlocked. "]
    pub current_state: ::std::option::Option<::std::string::String>,
    #[serde(rename = "definitionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of an achievement for which player state has been updated."]
    pub definition_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#achievementResetResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateOccurred")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flag to indicate if the requested update actually occurred."]
    pub update_occurred: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Multiple events reset all request."]
pub struct EventsResetMultipleForAllRequest {
    #[serde(rename = "event_ids")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IDs of events to reset."]
    pub event_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#eventsResetMultipleForAllRequest`."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "1P/3P metadata about the player's experience."]
pub struct GamesPlayerExperienceInfoResource {
    #[serde(rename = "currentExperiencePoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current number of experience points for the player."]
    pub current_experience_points: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currentLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current level of the player."]
    pub current_level: ::std::option::Option<::std::boxed::Box<GamesPlayerLevelResource>>,
    #[serde(rename = "lastLevelUpTimestampMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp when the player was leveled up, in millis since Unix epoch UTC."]
    pub last_level_up_timestamp_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The next level of the player. If the current level is the maximum level, this should be same as the current level."]
    pub next_level: ::std::option::Option<::std::boxed::Box<GamesPlayerLevelResource>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "1P/3P metadata about a user's level."]
pub struct GamesPlayerLevelResource {
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The level for the user."]
    pub level: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "maxExperiencePoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum experience points for this level."]
    pub max_experience_points: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minExperiencePoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum experience points for this level."]
    pub min_experience_points: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The HiddenPlayer resource."]
pub struct HiddenPlayer {
    #[serde(rename = "hiddenTimeMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time this player was hidden."]
    pub hidden_time_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#hiddenPlayer`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "player")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The player information."]
    pub player: ::std::option::Option<::std::boxed::Box<Player>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of hidden players."]
pub struct HiddenPlayerList {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The players."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HiddenPlayer>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#hiddenPlayerList`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token for the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Player resource."]
pub struct Player {
    #[serde(rename = "avatarImageUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The base URL for the image that represents the player."]
    pub avatar_image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bannerUrlLandscape")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The url to the landscape mode player banner image."]
    pub banner_url_landscape: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bannerUrlPortrait")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The url to the portrait mode player banner image."]
    pub banner_url_portrait: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name to display for the player."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "experienceInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An object to represent Play Game experience information for the player."]
    pub experience_info:
        ::std::option::Option<::std::boxed::Box<GamesPlayerExperienceInfoResource>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#player`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An object representation of the individual components of the player's name. For some players, these fields may not be present."]
    pub name: ::std::option::Option<PlayerName>,
    #[serde(rename = "originalPlayerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The player ID that was used for this player the first time they signed into the game in question. This is only populated for calls to player.get for the requesting player, only if the player ID has subsequently changed, and only to clients that support remapping player IDs."]
    pub original_player_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "playerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the player."]
    pub player_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "profileSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The player's profile settings. Controls whether or not the player's profile is visible to other players."]
    pub profile_settings: ::std::option::Option<::std::boxed::Box<ProfileSettings>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The player's title rewarded for their game activities."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An object representation of the individual components of the player's name. For some players, these fields may not be present."]
pub struct PlayerName {
    #[serde(rename = "familyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The family name of this player. In some places, this is known as the last name."]
    pub family_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The given name of this player. In some places, this is known as the first name."]
    pub given_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of leaderboard reset resources."]
pub struct PlayerScoreResetAllResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#playerScoreResetAllResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The leaderboard reset results."]
    pub results:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlayerScoreResetResponse>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of reset leaderboard entry resources."]
pub struct PlayerScoreResetResponse {
    #[serde(rename = "definitionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of an leaderboard for which player state has been updated."]
    pub definition_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#playerScoreResetResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resetScoreTimeSpans")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time spans of the updated score. Possible values are: - \"`ALL_TIME`\" - The score is an all-time score. - \"`WEEKLY`\" - The score is a weekly score. - \"`DAILY`\" - The score is a daily score. "]
    pub reset_score_time_spans: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Profile settings"]
pub struct ProfileSettings {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#profileSettings`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "profileVisible")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub profile_visible: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScoresResetMultipleForAllRequest {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#scoresResetMultipleForAllRequest`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "leaderboard_ids")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IDs of leaderboards to reset."]
    pub leaderboard_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
