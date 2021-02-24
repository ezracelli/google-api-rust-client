#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An achievement definition object."]
pub struct AchievementDefinition {
    #[serde(rename = "achievementType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the achievement."]
    pub achievement_type: ::std::option::Option<AchievementDefinitionAchievementTypeEnum>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of the achievement."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "experiencePoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Experience points which will be earned when unlocking this achievement."]
    pub experience_points: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formattedTotalSteps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total steps for an incremental achievement as a string."]
    pub formatted_total_steps: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the achievement."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "initialState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The initial state of the achievement."]
    pub initial_state: ::std::option::Option<AchievementDefinitionInitialStateEnum>,
    #[serde(rename = "isRevealedIconUrlDefault")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the revealed icon image being returned is a default image, or is provided by the game."]
    pub is_revealed_icon_url_default: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isUnlockedIconUrlDefault")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the unlocked icon image being returned is a default image, or is game-provided."]
    pub is_unlocked_icon_url_default: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementDefinition`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the achievement."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revealedIconUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The image URL for the revealed achievement icon."]
    pub revealed_icon_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalSteps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total steps for an incremental achievement."]
    pub total_steps: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "unlockedIconUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The image URL for the unlocked achievement icon."]
    pub unlocked_icon_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the achievement."]
pub enum AchievementDefinitionAchievementTypeEnum {
    #[serde(rename = "ACHIEVEMENT_TYPE_UNSPECIFIED")]
    #[doc = "Safe default, don't use."]
    AchievementTypeUnspecified,
    #[serde(rename = "STANDARD")]
    #[doc = "Achievement is either locked or unlocked."]
    Standard,
    #[serde(rename = "INCREMENTAL")]
    #[doc = "Achievement is incremental."]
    Incremental,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The initial state of the achievement."]
pub enum AchievementDefinitionInitialStateEnum {
    #[serde(rename = "INITIAL_ACHIEVEMENT_STATE_UNSPECIFIED")]
    #[doc = "Safe default, don't use."]
    InitialAchievementStateUnspecified,
    #[serde(rename = "HIDDEN")]
    #[doc = "Achievement is hidden."]
    Hidden,
    #[serde(rename = "REVEALED")]
    #[doc = "Achievement is revealed."]
    Revealed,
    #[serde(rename = "UNLOCKED")]
    #[doc = "Achievement is unlocked."]
    Unlocked,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of achievement definition objects."]
pub struct AchievementDefinitionsListResponse {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The achievement definitions."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AchievementDefinition>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementDefinitionsListResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token corresponding to the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An achievement increment response"]
pub struct AchievementIncrementResponse {
    #[serde(rename = "currentSteps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current steps recorded for this incremental achievement."]
    pub current_steps: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementIncrementResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "newlyUnlocked")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current steps for the achievement has reached the number of steps required to unlock."]
    pub newly_unlocked: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An achievement reveal response"]
pub struct AchievementRevealResponse {
    #[serde(rename = "currentState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current state of the achievement for which a reveal was attempted. This might be `UNLOCKED` if the achievement was already unlocked."]
    pub current_state: ::std::option::Option<AchievementRevealResponseCurrentStateEnum>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementRevealResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The current state of the achievement for which a reveal was attempted. This might be `UNLOCKED` if the achievement was already unlocked."]
pub enum AchievementRevealResponseCurrentStateEnum {
    #[serde(rename = "REVEAL_ACHIEVEMENT_STATE_UNSPECIFIED")]
    #[doc = "Safe default, don't use."]
    RevealAchievementStateUnspecified,
    #[serde(rename = "REVEALED")]
    #[doc = "Achievement is revealed."]
    Revealed,
    #[serde(rename = "UNLOCKED")]
    #[doc = "Achievement is unlocked."]
    Unlocked,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An achievement set steps at least response."]
pub struct AchievementSetStepsAtLeastResponse {
    #[serde(rename = "currentSteps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current steps recorded for this incremental achievement."]
    pub current_steps: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementSetStepsAtLeastResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "newlyUnlocked")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the current steps for the achievement has reached the number of steps required to unlock."]
    pub newly_unlocked: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An achievement unlock response"]
pub struct AchievementUnlockResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUnlockResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "newlyUnlocked")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this achievement was newly unlocked (that is, whether the unlock request for the achievement was the first for the player)."]
    pub newly_unlocked: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of achievement update requests."]
pub struct AchievementUpdateMultipleRequest {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUpdateMultipleRequest`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The individual achievement update requests."]
    pub updates:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AchievementUpdateRequest>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for UpdateMultipleAchievements rpc."]
pub struct AchievementUpdateMultipleResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUpdateMultipleResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updatedAchievements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The updated state of the achievements."]
    pub updated_achievements:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AchievementUpdateResponse>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to update an achievement."]
pub struct AchievementUpdateRequest {
    #[serde(rename = "achievementId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The achievement this update is being applied to."]
    pub achievement_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "incrementPayload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The payload if an update of type `INCREMENT` was requested for the achievement."]
    pub increment_payload: ::std::option::Option<::std::boxed::Box<GamesAchievementIncrement>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUpdateRequest`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "setStepsAtLeastPayload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The payload if an update of type `SET_STEPS_AT_LEAST` was requested for the achievement."]
    pub set_steps_at_least_payload:
        ::std::option::Option<::std::boxed::Box<GamesAchievementSetStepsAtLeast>>,
    #[serde(rename = "updateType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of update being applied."]
    pub update_type: ::std::option::Option<AchievementUpdateRequestUpdateTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of update being applied."]
pub enum AchievementUpdateRequestUpdateTypeEnum {
    #[serde(rename = "ACHIEVEMENT_UPDATE_TYPE_UNSPECIFIED")]
    #[doc = "Safe default, don't use."]
    AchievementUpdateTypeUnspecified,
    #[serde(rename = "REVEAL")]
    #[doc = "Achievement is revealed."]
    Reveal,
    #[serde(rename = "UNLOCK")]
    #[doc = "Achievement is unlocked."]
    Unlock,
    #[serde(rename = "INCREMENT")]
    #[doc = "Achievement is incremented."]
    Increment,
    #[serde(rename = "SET_STEPS_AT_LEAST")]
    #[doc = "Achievement progress is set to at least the passed value."]
    SetStepsAtLeast,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An updated achievement."]
pub struct AchievementUpdateResponse {
    #[serde(rename = "achievementId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The achievement this update is was applied to."]
    pub achievement_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currentState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current state of the achievement."]
    pub current_state: ::std::option::Option<AchievementUpdateResponseCurrentStateEnum>,
    #[serde(rename = "currentSteps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current steps recorded for this achievement if it is incremental."]
    pub current_steps: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUpdateResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "newlyUnlocked")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this achievement was newly unlocked (that is, whether the unlock request for the achievement was the first for the player)."]
    pub newly_unlocked: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "updateOccurred")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the requested updates actually affected the achievement."]
    pub update_occurred: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The current state of the achievement."]
pub enum AchievementUpdateResponseCurrentStateEnum {
    #[serde(rename = "UPDATED_ACHIEVEMENT_STATE_UNSPECIFIED")]
    #[doc = "Safe default, don't use."]
    UpdatedAchievementStateUnspecified,
    #[serde(rename = "HIDDEN")]
    #[doc = "Achievement is hidden."]
    Hidden,
    #[serde(rename = "REVEALED")]
    #[doc = "Achievement is revealed."]
    Revealed,
    #[serde(rename = "UNLOCKED")]
    #[doc = "Achievement is unlocked."]
    Unlocked,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Application resource."]
pub struct Application {
    #[serde(rename = "achievement_count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of achievements visible to the currently authenticated player."]
    pub achievement_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "assets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The assets of the application."]
    pub assets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ImageAsset>>>,
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The author of the application."]
    pub author: ::std::option::Option<::std::string::String>,
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The category of the application."]
    pub category: ::std::option::Option<::std::boxed::Box<ApplicationCategory>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of the application."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enabledFeatures")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of features that have been enabled for the application."]
    pub enabled_features: ::std::option::Option<::std::vec::Vec<ApplicationEnabledFeaturesEnum>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the application."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The instances of the application."]
    pub instances: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Instance>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#application`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last updated timestamp of the application."]
    pub last_updated_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "leaderboard_count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of leaderboards visible to the currently authenticated player."]
    pub leaderboard_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the application."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "themeColor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A hint to the client UI for what color to use as an app-themed color. The color is given as an RGB triplet (e.g. \"E0E0E0\")."]
    pub theme_color: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ApplicationEnabledFeaturesEnum {
    #[serde(rename = "APPLICATION_FEATURE_UNSPECIFIED")]
    #[doc = "Safe default, don't use."]
    ApplicationFeatureUnspecified,
    #[serde(rename = "SNAPSHOTS")]
    #[doc = "Saved Games (snapshots)."]
    Snapshots,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An application category object."]
pub struct ApplicationCategory {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#applicationCategory`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "primary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The primary category."]
    pub primary: ::std::option::Option<::std::string::String>,
    #[serde(rename = "secondary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The secondary category."]
    pub secondary: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A third party application verification response resource."]
pub struct ApplicationVerifyResponse {
    #[serde(rename = "alternate_player_id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An alternate ID that was once used for the player that was issued the auth token used in this request. (This field is not normally populated.)"]
    pub alternate_player_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#applicationVerifyResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "player_id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the player that was issued the auth token used in this request."]
    pub player_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Data related to individual game categories."]
pub struct Category {
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The category name."]
    pub category: ::std::option::Option<::std::string::String>,
    #[serde(rename = "experiencePoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Experience points earned in this category."]
    pub experience_points: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#category`."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A third party list metagame categories response."]
pub struct CategoryListResponse {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of categories with usage data."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Category>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#categoryListResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token corresponding to the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Container for a URL end point of the requested type."]
pub struct EndPoint {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL suitable for loading in a web browser for the requested endpoint."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch update failure resource."]
pub struct EventBatchRecordFailure {
    #[serde(rename = "failureCause")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cause for the update failure."]
    pub failure_cause: ::std::option::Option<EventBatchRecordFailureFailureCauseEnum>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventBatchRecordFailure`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time range which was rejected; empty for a request-wide failure."]
    pub range: ::std::option::Option<::std::boxed::Box<EventPeriodRange>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The cause for the update failure."]
pub enum EventBatchRecordFailureFailureCauseEnum {
    #[serde(rename = "EVENT_FAILURE_CAUSE_UNSPECIFIED")]
    #[doc = "Default value. Should not be used."]
    EventFailureCauseUnspecified,
    #[serde(rename = "TOO_LARGE")]
    #[doc = "A batch request was issued with more events than are allowed in a single batch."]
    TooLarge,
    #[serde(rename = "TIME_PERIOD_EXPIRED")]
    #[doc = "A batch was sent with data too far in the past to record."]
    TimePeriodExpired,
    #[serde(rename = "TIME_PERIOD_SHORT")]
    #[doc = "A batch was sent with a time range that was too short."]
    TimePeriodShort,
    #[serde(rename = "TIME_PERIOD_LONG")]
    #[doc = "A batch was sent with a time range that was too long."]
    TimePeriodLong,
    #[serde(rename = "ALREADY_UPDATED")]
    #[doc = "An attempt was made to record a batch of data which was already seen."]
    AlreadyUpdated,
    #[serde(rename = "RECORD_RATE_HIGH")]
    #[doc = "An attempt was made to record data faster than the server will apply updates."]
    RecordRateHigh,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event child relationship resource."]
pub struct EventChild {
    #[serde(rename = "childId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the child event."]
    pub child_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventChild`."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event definition resource."]
pub struct EventDefinition {
    #[serde(rename = "childEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of events that are a child of this event."]
    pub child_events: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventChild>>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of what this event represents."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name to display for the event."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the event."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The base URL for the image that represents the event."]
    pub image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isDefaultImageUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the icon image being returned is a default image, or is game-provided."]
    pub is_default_image_url: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventDefinition`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The visibility of event being tracked in this definition."]
    pub visibility: ::std::option::Option<EventDefinitionVisibilityEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The visibility of event being tracked in this definition."]
pub enum EventDefinitionVisibilityEnum {
    #[serde(rename = "EVENT_VISIBILITY_UNSPECIFIED")]
    #[doc = "Default value. Should not be used."]
    EventVisibilityUnspecified,
    #[serde(rename = "REVEALED")]
    #[doc = "This event should be visible to all users."]
    Revealed,
    #[serde(rename = "HIDDEN")]
    #[doc = "This event should only be shown to users that have recorded this event at least once."]
    Hidden,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ListDefinitions response."]
pub struct EventDefinitionListResponse {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The event definitions."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventDefinition>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventDefinitionListResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token for the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event period time range."]
pub struct EventPeriodRange {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventPeriodRange`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "periodEndMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when this update period ends, in millis, since 1970 UTC (Unix Epoch)."]
    pub period_end_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "periodStartMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when this update period begins, in millis, since 1970 UTC (Unix Epoch)."]
    pub period_start_millis: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event period update resource."]
pub struct EventPeriodUpdate {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventPeriodUpdate`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timePeriod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time period being covered by this update."]
    pub time_period: ::std::option::Option<::std::boxed::Box<EventPeriodRange>>,
    #[serde(rename = "updates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The updates being made for this time period."]
    pub updates: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventUpdateRequest>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event update failure resource."]
pub struct EventRecordFailure {
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the event that was not updated."]
    pub event_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "failureCause")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cause for the update failure."]
    pub failure_cause: ::std::option::Option<EventRecordFailureFailureCauseEnum>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventRecordFailure`."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The cause for the update failure."]
pub enum EventRecordFailureFailureCauseEnum {
    #[serde(rename = "EVENT_UPDATE_FAILURE_CAUSE_UNSPECIFIED")]
    #[doc = "Default value. Should not use."]
    EventUpdateFailureCauseUnspecified,
    #[serde(rename = "NOT_FOUND")]
    #[doc = "An attempt was made to set an event that was not defined."]
    NotFound,
    #[serde(rename = "INVALID_UPDATE_VALUE")]
    #[doc = "An attempt was made to increment an event by a non-positive value."]
    InvalidUpdateValue,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event period update resource."]
pub struct EventRecordRequest {
    #[serde(rename = "currentTimeMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current time when this update was sent, in milliseconds, since 1970 UTC (Unix Epoch)."]
    pub current_time_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventRecordRequest`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request ID used to identify this attempt to record events."]
    pub request_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timePeriods")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of the time period updates being made in this request."]
    pub time_periods: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventPeriodUpdate>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event period update resource."]
pub struct EventUpdateRequest {
    #[serde(rename = "definitionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the event being modified in this update."]
    pub definition_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventUpdateRequest`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of times this event occurred in this time period."]
    pub update_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event period update resource."]
pub struct EventUpdateResponse {
    #[serde(rename = "batchFailures")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Any batch-wide failures which occurred applying updates."]
    pub batch_failures:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventBatchRecordFailure>>>,
    #[serde(rename = "eventFailures")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Any failures updating a particular event."]
    pub event_failures:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventRecordFailure>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventUpdateResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "playerEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current status of any updated events"]
    pub player_events: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlayerEvent>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The payload to request to increment an achievement."]
pub struct GamesAchievementIncrement {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#GamesAchievementIncrement`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The requestId associated with an increment to an achievement."]
    pub request_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "steps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of steps to be incremented."]
    pub steps: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The payload to request to increment an achievement."]
pub struct GamesAchievementSetStepsAtLeast {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#GamesAchievementSetStepsAtLeast`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "steps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum number of steps for the achievement to be set to."]
    pub steps: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An image asset object."]
pub struct ImageAsset {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The height of the asset."]
    pub height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#imageAsset`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the asset."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the asset."]
    pub url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width of the asset."]
    pub width: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Instance resource."]
pub struct Instance {
    #[serde(rename = "acquisitionUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI which shows where a user can acquire this instance."]
    pub acquisition_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "androidInstance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Platform dependent details for Android."]
    pub android_instance: ::std::option::Option<::std::boxed::Box<InstanceAndroidDetails>>,
    #[serde(rename = "iosInstance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Platform dependent details for iOS."]
    pub ios_instance: ::std::option::Option<::std::boxed::Box<InstanceIosDetails>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#instance`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Localized display name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "platformType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The platform type."]
    pub platform_type: ::std::option::Option<InstancePlatformTypeEnum>,
    #[serde(rename = "realtimePlay")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flag to show if this game instance supports realtime play."]
    pub realtime_play: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "turnBasedPlay")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flag to show if this game instance supports turn based play."]
    pub turn_based_play: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "webInstance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Platform dependent details for Web."]
    pub web_instance: ::std::option::Option<::std::boxed::Box<InstanceWebDetails>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The platform type."]
pub enum InstancePlatformTypeEnum {
    #[serde(rename = "PLATFORM_TYPE_UNSPECIFIED")]
    #[doc = "Default value. Should be unused."]
    PlatformTypeUnspecified,
    #[serde(rename = "ANDROID")]
    #[doc = "Instance is for Android."]
    Android,
    #[serde(rename = "IOS")]
    #[doc = "Instance is for iOS."]
    Ios,
    #[serde(rename = "WEB_APP")]
    #[doc = "Instance is for Web App."]
    WebApp,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Android instance details resource."]
pub struct InstanceAndroidDetails {
    #[serde(rename = "enablePiracyCheck")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flag indicating whether the anti-piracy check is enabled."]
    pub enable_piracy_check: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#instanceAndroidDetails`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "packageName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Android package name which maps to Google Play URL."]
    pub package_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "preferred")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that this instance is the default for new installations."]
    pub preferred: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The iOS details resource."]
pub struct InstanceIosDetails {
    #[serde(rename = "bundleIdentifier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Bundle identifier."]
    pub bundle_identifier: ::std::option::Option<::std::string::String>,
    #[serde(rename = "itunesAppId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "iTunes App ID."]
    pub itunes_app_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#instanceIosDetails`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "preferredForIpad")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that this instance is the default for new installations on iPad devices."]
    pub preferred_for_ipad: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "preferredForIphone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that this instance is the default for new installations on iPhone devices."]
    pub preferred_for_iphone: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "supportIpad")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flag to indicate if this instance supports iPad."]
    pub support_ipad: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "supportIphone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flag to indicate if this instance supports iPhone."]
    pub support_iphone: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Web details resource."]
pub struct InstanceWebDetails {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#instanceWebDetails`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "launchUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Launch URL for the game."]
    pub launch_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "preferred")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that this instance is the default for new installations."]
    pub preferred: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Leaderboard resource."]
pub struct Leaderboard {
    #[serde(rename = "iconUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The icon for the leaderboard."]
    pub icon_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The leaderboard ID."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isIconUrlDefault")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the icon image being returned is a default image, or is game-provided."]
    pub is_icon_url_default: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboard`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the leaderboard."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "order")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How scores are ordered."]
    pub order: ::std::option::Option<LeaderboardOrderEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "How scores are ordered."]
pub enum LeaderboardOrderEnum {
    #[serde(rename = "SCORE_ORDER_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    ScoreOrderUnspecified,
    #[serde(rename = "LARGER_IS_BETTER")]
    #[doc = "Larger values are better; scores are sorted in descending order"]
    LargerIsBetter,
    #[serde(rename = "SMALLER_IS_BETTER")]
    #[doc = "Smaller values are better; scores are sorted in ascending order"]
    SmallerIsBetter,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Leaderboard Entry resource."]
pub struct LeaderboardEntry {
    #[serde(rename = "formattedScore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The localized string for the numerical value of this score."]
    pub formatted_score: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formattedScoreRank")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The localized string for the rank of this score for this leaderboard."]
    pub formatted_score_rank: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboardEntry`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "player")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The player who holds this score."]
    pub player: ::std::option::Option<::std::boxed::Box<Player>>,
    #[serde(rename = "scoreRank")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The rank of this score for this leaderboard."]
    pub score_rank: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scoreTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional information about the score. Values must contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986."]
    pub score_tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scoreValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The numerical value of this score."]
    pub score_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeSpan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time span of this high score."]
    pub time_span: ::std::option::Option<LeaderboardEntryTimeSpanEnum>,
    #[serde(rename = "writeTimestampMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp at which this score was recorded, in milliseconds since the epoch in UTC."]
    pub write_timestamp_millis: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The time span of this high score."]
pub enum LeaderboardEntryTimeSpanEnum {
    #[serde(rename = "SCORE_TIME_SPAN_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    ScoreTimeSpanUnspecified,
    #[serde(rename = "ALL_TIME")]
    #[doc = "The score is an all-time score."]
    AllTime,
    #[serde(rename = "WEEKLY")]
    #[doc = "The score is a weekly score."]
    Weekly,
    #[serde(rename = "DAILY")]
    #[doc = "The score is a daily score."]
    Daily,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of leaderboard objects."]
pub struct LeaderboardListResponse {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The leaderboards."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Leaderboard>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboardListResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token corresponding to the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A score rank in a leaderboard."]
pub struct LeaderboardScoreRank {
    #[serde(rename = "formattedNumScores")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of scores in the leaderboard as a string."]
    pub formatted_num_scores: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formattedRank")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The rank in the leaderboard as a string."]
    pub formatted_rank: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboardScoreRank`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numScores")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of scores in the leaderboard."]
    pub num_scores: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rank")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The rank in the leaderboard."]
    pub rank: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ListScores response."]
pub struct LeaderboardScores {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The scores in the leaderboard."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LeaderboardEntry>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboardScores`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token for the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numScores")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of scores in the leaderboard."]
    pub num_scores: ::std::option::Option<::std::string::String>,
    #[serde(rename = "playerScore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The score of the requesting player on the leaderboard. The player's score may appear both here and in the list of scores above. If you are viewing a public leaderboard and the player is not sharing their gameplay information publicly, the `scoreRank`and `formattedScoreRank` values will not be present."]
    pub player_score: ::std::option::Option<::std::boxed::Box<LeaderboardEntry>>,
    #[serde(rename = "prevPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token for the previous page of results."]
    pub prev_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The metagame config resource"]
pub struct MetagameConfig {
    #[serde(rename = "currentVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Current version of the metagame configuration data. When this data is updated, the version number will be increased by one."]
    pub current_version: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#metagameConfig`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "playerLevels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of player levels."]
    pub player_levels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlayerLevel>>>,
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
    pub experience_info: ::std::option::Option<::std::boxed::Box<PlayerExperienceInfo>>,
    #[serde(rename = "friendStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The friend status of the given player, relative to the requester. This is unset if the player is not sharing their friends list with the game."]
    pub friend_status: ::std::option::Option<PlayerFriendStatusEnum>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#player`"]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A representation of the individual components of the name."]
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
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The friend status of the given player, relative to the requester. This is unset if the player is not sharing their friends list with the game."]
pub enum PlayerFriendStatusEnum {
    #[serde(rename = "FRIEND_STATUS_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    FriendStatusUnspecified,
    #[serde(rename = "NO_RELATIONSHIP")]
    #[doc = "There is no relationship between the players."]
    NoRelationship,
    #[serde(rename = "FRIEND")]
    #[doc = "The player and requester are friends."]
    Friend,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A representation of the individual components of the name."]
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
#[doc = "An achievement object."]
pub struct PlayerAchievement {
    #[serde(rename = "achievementState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of the achievement."]
    pub achievement_state: ::std::option::Option<PlayerAchievementAchievementStateEnum>,
    #[serde(rename = "currentSteps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current steps for an incremental achievement."]
    pub current_steps: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "experiencePoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Experience points earned for the achievement. This field is absent for achievements that have not yet been unlocked and 0 for achievements that have been unlocked by testers but that are unpublished."]
    pub experience_points: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formattedCurrentStepsString")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current steps for an incremental achievement as a string."]
    pub formatted_current_steps_string: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the achievement."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerAchievement`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp of the last modification to this achievement's state."]
    pub last_updated_timestamp: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of the achievement."]
pub enum PlayerAchievementAchievementStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    StateUnspecified,
    #[serde(rename = "HIDDEN")]
    #[doc = "Achievement is hidden."]
    Hidden,
    #[serde(rename = "REVEALED")]
    #[doc = "Achievement is revealed."]
    Revealed,
    #[serde(rename = "UNLOCKED")]
    #[doc = "Achievement is unlocked."]
    Unlocked,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of achievement objects."]
pub struct PlayerAchievementListResponse {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The achievements."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlayerAchievement>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerAchievementListResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token corresponding to the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event status resource."]
pub struct PlayerEvent {
    #[serde(rename = "definitionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the event definition."]
    pub definition_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formattedNumEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current number of times this event has occurred, as a string. The formatting of this string depends on the configuration of your event in the Play Games Developer Console."]
    pub formatted_num_events: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerEvent`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current number of times this event has occurred."]
    pub num_events: ::std::option::Option<::std::string::String>,
    #[serde(rename = "playerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the player."]
    pub player_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ListByPlayer response."]
pub struct PlayerEventListResponse {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The player events."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlayerEvent>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerEventListResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token for the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "1P/3P metadata about the player's experience."]
pub struct PlayerExperienceInfo {
    #[serde(rename = "currentExperiencePoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current number of experience points for the player."]
    pub current_experience_points: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currentLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current level of the player."]
    pub current_level: ::std::option::Option<::std::boxed::Box<PlayerLevel>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerExperienceInfo`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastLevelUpTimestampMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp when the player was leveled up, in millis since Unix epoch UTC."]
    pub last_level_up_timestamp_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The next level of the player. If the current level is the maximum level, this should be same as the current level."]
    pub next_level: ::std::option::Option<::std::boxed::Box<PlayerLevel>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A player leaderboard score object."]
pub struct PlayerLeaderboardScore {
    #[serde(rename = "friendsRank")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The rank of the score in the friends collection for this leaderboard."]
    pub friends_rank: ::std::option::Option<::std::boxed::Box<LeaderboardScoreRank>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerLeaderboardScore`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "leaderboard_id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the leaderboard this score is in."]
    pub leaderboard_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publicRank")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The public rank of the score in this leaderboard. This object will not be present if the user is not sharing their scores publicly."]
    pub public_rank: ::std::option::Option<::std::boxed::Box<LeaderboardScoreRank>>,
    #[serde(rename = "scoreString")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The formatted value of this score."]
    pub score_string: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scoreTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional information about the score. Values must contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986."]
    pub score_tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scoreValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The numerical value of this score."]
    pub score_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "socialRank")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The social rank of the score in this leaderboard."]
    pub social_rank: ::std::option::Option<::std::boxed::Box<LeaderboardScoreRank>>,
    #[serde(rename = "timeSpan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time span of this score."]
    pub time_span: ::std::option::Option<PlayerLeaderboardScoreTimeSpanEnum>,
    #[serde(rename = "writeTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp at which this score was recorded, in milliseconds since the epoch in UTC."]
    pub write_timestamp: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The time span of this score."]
pub enum PlayerLeaderboardScoreTimeSpanEnum {
    #[serde(rename = "SCORE_TIME_SPAN_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    ScoreTimeSpanUnspecified,
    #[serde(rename = "ALL_TIME")]
    #[doc = "The score is an all-time score."]
    AllTime,
    #[serde(rename = "WEEKLY")]
    #[doc = "The score is a weekly score."]
    Weekly,
    #[serde(rename = "DAILY")]
    #[doc = "The score is a daily score."]
    Daily,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of player leaderboard scores."]
pub struct PlayerLeaderboardScoreListResponse {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The leaderboard scores."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlayerLeaderboardScore>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerLeaderboardScoreListResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token for the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "player")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Player resources for the owner of this score."]
    pub player: ::std::option::Option<::std::boxed::Box<Player>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "1P/3P metadata about a user's level."]
pub struct PlayerLevel {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerLevel`."]
    pub kind: ::std::option::Option<::std::string::String>,
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
#[doc = "A third party player list response."]
pub struct PlayerListResponse {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The players."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Player>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerListResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token corresponding to the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A player score."]
pub struct PlayerScore {
    #[serde(rename = "formattedScore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The formatted score for this player score."]
    pub formatted_score: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerScore`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The numerical value for this player score."]
    pub score: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scoreTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional information about this score. Values will contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986."]
    pub score_tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeSpan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time span for this player score."]
    pub time_span: ::std::option::Option<PlayerScoreTimeSpanEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The time span for this player score."]
pub enum PlayerScoreTimeSpanEnum {
    #[serde(rename = "SCORE_TIME_SPAN_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    ScoreTimeSpanUnspecified,
    #[serde(rename = "ALL_TIME")]
    #[doc = "The score is an all-time score."]
    AllTime,
    #[serde(rename = "WEEKLY")]
    #[doc = "The score is a weekly score."]
    Weekly,
    #[serde(rename = "DAILY")]
    #[doc = "The score is a daily score."]
    Daily,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of score submission statuses."]
pub struct PlayerScoreListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerScoreListResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "submittedScores")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The score submissions statuses."]
    pub submitted_scores:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlayerScoreResponse>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of leaderboard entry resources."]
pub struct PlayerScoreResponse {
    #[serde(rename = "beatenScoreTimeSpans")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time spans where the submitted score is better than the existing score for that time span."]
    pub beaten_score_time_spans:
        ::std::option::Option<::std::vec::Vec<PlayerScoreResponseBeatenScoreTimeSpansEnum>>,
    #[serde(rename = "formattedScore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The formatted value of the submitted score."]
    pub formatted_score: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerScoreResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "leaderboardId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The leaderboard ID that this score was submitted to."]
    pub leaderboard_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scoreTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional information about this score. Values will contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986."]
    pub score_tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unbeatenScores")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The scores in time spans that have not been beaten. As an example, the submitted score may be better than the player's `DAILY` score, but not better than the player's scores for the `WEEKLY` or `ALL_TIME` time spans."]
    pub unbeaten_scores: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlayerScore>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum PlayerScoreResponseBeatenScoreTimeSpansEnum {
    #[serde(rename = "SCORE_TIME_SPAN_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    ScoreTimeSpanUnspecified,
    #[serde(rename = "ALL_TIME")]
    #[doc = "The score is an all-time score."]
    AllTime,
    #[serde(rename = "WEEKLY")]
    #[doc = "The score is a weekly score."]
    Weekly,
    #[serde(rename = "DAILY")]
    #[doc = "The score is a daily score."]
    Daily,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of score submission requests."]
pub struct PlayerScoreSubmissionList {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerScoreSubmissionList`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scores")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The score submissions."]
    pub scores: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ScoreSubmission>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Profile settings"]
pub struct ProfileSettings {
    #[serde(rename = "friendsListVisibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub friends_list_visibility: ::std::option::Option<ProfileSettingsFriendsListVisibilityEnum>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#profileSettings`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "profileVisible")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the player's profile is visible to the currently signed in player."]
    pub profile_visible: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ProfileSettingsFriendsListVisibilityEnum {
    #[serde(rename = "FRIENDS_LIST_VISIBILITY_UNSPECIFIED")]
    #[doc = "Unused."]
    FriendsListVisibilityUnspecified,
    #[serde(rename = "VISIBLE")]
    #[doc = "The friends list is currently visible to the game."]
    Visible,
    #[serde(rename = "REQUEST_REQUIRED")]
    #[doc = "The developer does not have access to the friends list, but can call the Android API to show a consent dialog."]
    RequestRequired,
    #[serde(rename = "UNAVAILABLE")]
    #[doc = "The friends list is currently unavailable for this user, and it is not possible to request access at this time, either because the user has permanently declined or the friends feature is not available to them. In this state, any attempts to request access to the friends list will be unsuccessful."]
    Unavailable,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for ResolveSnapshotHead RPC."]
pub struct ResolveSnapshotHeadRequest {
    #[serde(rename = "maxConflictsPerSnapshot")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number of SnapshotRevision resources for `conflictingRevisions` to return per SnapshotExtended resource in the response. For any response, the actual number of resources returned may be less than specified by `maxConflictsPerSnapshot`. The value provided should be greater or equal to 0. If no value is provided, the server will use a sensible default."]
    pub max_conflicts_per_snapshot: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "resolutionPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The automatic resolution policy. All conflicts are resolved in chronological order, starting from the/ least recent. If the comparison metric is equal for the tentative head and the conflict, the head wins."]
    pub resolution_policy: ::std::option::Option<ResolveSnapshotHeadRequestResolutionPolicyEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The automatic resolution policy. All conflicts are resolved in chronological order, starting from the/ least recent. If the comparison metric is equal for the tentative head and the conflict, the head wins."]
pub enum ResolveSnapshotHeadRequestResolutionPolicyEnum {
    #[serde(rename = "RESOLUTION_POLICY_UNSPECIFIED")]
    #[doc = "Safe default, don't use explicitly."]
    ResolutionPolicyUnspecified,
    #[serde(rename = "USE_HEAD")]
    #[doc = "Drops all conflicts and keeps the current head only."]
    UseHead,
    #[serde(rename = "LONGEST_PLAYTIME")]
    #[doc = "Use the snapshot with the longest played time."]
    LongestPlaytime,
    #[serde(rename = "MOST_RECENTLY_MODIFIED")]
    #[doc = "Use the snapshot that was most recently modified."]
    MostRecentlyModified,
    #[serde(rename = "HIGHEST_PROGRESS")]
    #[doc = "Use the snapshot with the highest progress value."]
    HighestProgress,
    #[serde(rename = "NO_AUTOMATIC_RESOLUTION")]
    #[doc = "Don't resolve conflicts at all. Effectively only returns the current head revision of the snapshot. Corresponds to a game opening the snapshot with manual resolution policy."]
    NoAutomaticResolution,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ResolveSnapshotHead RPC."]
pub struct ResolveSnapshotHeadResponse {
    #[serde(rename = "snapshot")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of the snapshot."]
    pub snapshot: ::std::option::Option<::std::boxed::Box<SnapshotExtended>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A third party checking a revision response."]
pub struct RevisionCheckResponse {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of the API this client revision should use when calling API methods."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#revisionCheckResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revisionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the revision check."]
    pub revision_status: ::std::option::Option<RevisionCheckResponseRevisionStatusEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The result of the revision check."]
pub enum RevisionCheckResponseRevisionStatusEnum {
    #[serde(rename = "REVISION_STATUS_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    RevisionStatusUnspecified,
    #[serde(rename = "OK")]
    #[doc = "The revision being used is current."]
    Ok,
    #[serde(rename = "DEPRECATED")]
    #[doc = "There is currently a newer version available, but the revision being used still works."]
    Deprecated,
    #[serde(rename = "INVALID")]
    #[doc = "The revision being used is not supported in any released version."]
    Invalid,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to submit a score to leaderboards."]
pub struct ScoreSubmission {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#scoreSubmission`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "leaderboardId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The leaderboard this score is being submitted to."]
    pub leaderboard_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The new score being submitted."]
    pub score: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scoreTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional information about this score. Values will contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986."]
    pub score_tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "signature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Signature Values will contain URI-safe characters as defined by section 2.3 of RFC 3986."]
    pub signature: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An snapshot object."]
pub struct Snapshot {
    #[serde(rename = "coverImage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cover image of this snapshot. May be absent if there is no image."]
    pub cover_image: ::std::option::Option<::std::boxed::Box<SnapshotImage>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of this snapshot."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "driveId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the file underlying this snapshot in the Drive API. Only present if the snapshot is a view on a Drive file and the file is owned by the caller."]
    pub drive_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "durationMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The duration associated with this snapshot, in millis."]
    pub duration_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the snapshot."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#snapshot`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp (in millis since Unix epoch) of the last modification to this snapshot."]
    pub last_modified_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The progress value (64-bit integer set by developer) associated with this snapshot."]
    pub progress_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of this snapshot."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of this snapshot."]
    pub _type: ::std::option::Option<SnapshotTypeEnum>,
    #[serde(rename = "uniqueName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique name provided when the snapshot was created."]
    pub unique_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of this snapshot."]
pub enum SnapshotTypeEnum {
    #[serde(rename = "SNAPSHOT_TYPE_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    SnapshotTypeUnspecified,
    #[serde(rename = "SAVE_GAME")]
    #[doc = "A snapshot representing a save game."]
    SaveGame,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Identifies a snapshot cover image resource. The image is provided by the game."]
pub struct SnapshotCoverImageResource {
    #[serde(rename = "contentHash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Hash-like weak identifier of the uploaded image bytes, consistent per player per application. The content hash for a given resource will not change if the binary data hasn't changed. Except in very rare circumstances, the content_hash for matching binary data will be the same within a given player and application."]
    pub content_hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "downloadUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A URL the client can use to download the image. May vary across requests, and only guaranteed to be valid for a short time after it is returned."]
    pub download_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The height of the image in pixels."]
    pub height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The MIME type of the image."]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the image resource. It's guaranteed that if two IDs are equal then the contents are equal as well. It's not guaranteed that two identical blobs coming from separate uploads have the same ID. The resource ID can only be used within the application, user and resource type it was originally returned for. For example, it's not possible to use SnapshotDataResource's resource ID as the resource_id of a SnapshotCoverImageResource, even if the blob is a valid image file."]
    pub resource_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width of the image in pixels."]
    pub width: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Identifies a snapshot data resource. The data is provided by the game."]
pub struct SnapshotDataResource {
    #[serde(rename = "contentHash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Hash-like weak identifier of the uploaded blob bytes, consistent per player per application. The content hash for a given resource will not change if the binary data hasn't changed. Except in very rare circumstances, the content_hash for matching binary data will be the same within a given player and application."]
    pub content_hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "downloadUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A URL that the client can use to download the blob. May vary across requests, and only guaranteed to be valid for a short time after it is returned."]
    pub download_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the blob resource. It's guaranteed that if two IDs are equal then the contents are equal as well. It's not guaranteed that two identical blobs coming from separate uploads have the same resource ID. The resource ID can only be used within the application, user and resource type it was originally returned for. For example, it's not possible to use SnapshotDataResource's resource ID as the resource_id of a SnapshotCoverImageResource, even if the blob is a valid image file."]
    pub resource_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Size of the saved game blob in bytes."]
    pub size: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A snapshot represents a saved game state referred to using the developer-provided snapshot_name. The set of attributes and binary data for a specific state is called a revision. Each revision is itself immutable, and referred to by a snapshot revision id. At any time, a snapshot has a \"head\" revision, and updates are made against that revision. If a snapshot update is received that isn't against the current head revision, then instead of changing the head revision it will result in a conflicting revision that must be specifically resolved."]
pub struct SnapshotExtended {
    #[serde(rename = "conflictingRevisions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of conflicting revisions. Only set if explicitly requested (e.g. using a field mask or a request flag), or if the RPC guarantees that this field is set. The conflicting revisions are sorted chronologically by their server creation time (oldest first). If there are too many conflicting revisions to return all of them in a single request this will only contain the first batch. In such case, the presented conflicting revisions must be resolved first in order to fetch the next batch."]
    pub conflicting_revisions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SnapshotRevision>>>,
    #[serde(rename = "hasConflictingRevisions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An indicator whether the snapshot has any conflicting revisions or not. Always set."]
    pub has_conflicting_revisions: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "headRevision")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current head revision (the canonical revision as understood by the server)."]
    pub head_revision: ::std::option::Option<::std::boxed::Box<SnapshotRevision>>,
    #[serde(rename = "snapshotName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An identifier of the snapshot, developer-specified. It must match the pattern [0-9a-zA-Z-._~]{1,100}."]
    pub snapshot_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An image of a snapshot."]
pub struct SnapshotImage {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The height of the image."]
    pub height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#snapshotImage`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mime_type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MIME type of the image."]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the image. This URL may be invalidated at any time and should not be cached."]
    pub url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width of the image."]
    pub width: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A third party list snapshots response."]
pub struct SnapshotListResponse {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The snapshots."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Snapshot>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#snapshotListResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token corresponding to the next page of results. If there are no more results, the token is omitted."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata about a snapshot revision. Snapshot metadata is immutable - a metadata change corresponds to a new snapshot revision."]
pub struct SnapshotMetadata {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of this snapshot."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deviceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The device that created the current revision."]
    pub device_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gameplayDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The duration associated with this snapshot. Values with sub-millisecond precision can be rounded or trimmed to the closest millisecond."]
    pub gameplay_duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifyTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp of the last modification to this snapshot as provided by the client. Values with sub-millisecond precision can be rounded or trimmed to the closest millisecond."]
    pub last_modify_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The progress value (64-bit integer set by developer) associated with this snapshot."]
    pub progress_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Snapshot revision resource. Snapshot revisions are immutable."]
pub struct SnapshotRevision {
    #[serde(rename = "blob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reference to the game provided blob for this revision."]
    pub blob: ::std::option::Option<::std::boxed::Box<SnapshotDataResource>>,
    #[serde(rename = "coverImage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reference to the cover image for this revision."]
    pub cover_image: ::std::option::Option<::std::boxed::Box<SnapshotCoverImageResource>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A server generated identifier of the snapshot revision."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata for this snapshot revision."]
    pub metadata: ::std::option::Option<::std::boxed::Box<SnapshotMetadata>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A third party stats resource."]
pub struct StatsResponse {
    #[serde(rename = "avg_session_length_minutes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Average session length in minutes of the player. E.g., 1, 30, 60, ... . Not populated if there is not enough information."]
    pub avg_session_length_minutes: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "churn_probability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The probability of the player not returning to play the game in the next day. E.g., 0, 0.1, 0.5, ..., 1.0. Not populated if there is not enough information."]
    pub churn_probability: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "days_since_last_played")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of days since the player last played this game. E.g., 0, 1, 5, 10, ... . Not populated if there is not enough information."]
    pub days_since_last_played: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "high_spender_probability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The probability of the player going to spend beyond a threshold amount of money. E.g., 0, 0.25, 0.50, 0.75. Not populated if there is not enough information."]
    pub high_spender_probability: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#statsResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "num_purchases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of in-app purchases made by the player in this game. E.g., 0, 1, 5, 10, ... . Not populated if there is not enough information."]
    pub num_purchases: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "num_sessions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The approximate number of sessions of the player within the last 28 days, where a session begins when the player is connected to Play Games Services and ends when they are disconnected. E.g., 0, 1, 5, 10, ... . Not populated if there is not enough information."]
    pub num_sessions: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "num_sessions_percentile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The approximation of the sessions percentile of the player within the last 30 days, where a session begins when the player is connected to Play Games Services and ends when they are disconnected. E.g., 0, 0.25, 0.5, 0.75. Not populated if there is not enough information."]
    pub num_sessions_percentile: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "spend_percentile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The approximate spend percentile of the player in this game. E.g., 0, 0.25, 0.5, 0.75. Not populated if there is not enough information."]
    pub spend_percentile: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "spend_probability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The probability of the player going to spend the game in the next seven days. E.g., 0, 0.25, 0.50, 0.75. Not populated if there is not enough information."]
    pub spend_probability: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "total_spend_next_28_days")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The predicted amount of money that the player going to spend in the next 28 days. E.g., 1, 30, 60, ... . Not populated if there is not enough information."]
    pub total_spend_next_28_days: ::std::option::Option<::std::primitive::f64>,
}
