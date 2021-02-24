#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An achievement configuration resource."]
pub struct AchievementConfiguration {
    #[serde(rename = "achievementType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the achievement."]
    pub achievement_type: ::std::option::Option<AchievementConfigurationAchievementTypeEnum>,
    #[serde(rename = "draft")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The draft data of the achievement."]
    pub draft: ::std::option::Option<::std::boxed::Box<AchievementConfigurationDetail>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the achievement."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "initialState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The initial state of the achievement."]
    pub initial_state: ::std::option::Option<AchievementConfigurationInitialStateEnum>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#achievementConfiguration`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "published")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The read-only published data of the achievement."]
    pub published: ::std::option::Option<::std::boxed::Box<AchievementConfigurationDetail>>,
    #[serde(rename = "stepsToUnlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Steps to unlock. Only applicable to incremental achievements."]
    pub steps_to_unlock: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for this resource."]
    pub token: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An achievement configuration detail."]
pub struct AchievementConfigurationDetail {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Localized strings for the achievement description."]
    pub description: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
    #[serde(rename = "iconUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The icon url of this achievement. Writes to this field are ignored."]
    pub icon_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#achievementConfigurationDetail`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Localized strings for the achievement name."]
    pub name: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
    #[serde(rename = "pointValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Point value for the achievement."]
    pub point_value: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "sortRank")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sort rank of this achievement. Writes to this field are ignored."]
    pub sort_rank: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ListConfigurations response."]
pub struct AchievementConfigurationListResponse {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The achievement configurations."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AchievementConfiguration>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#achievementConfigurationListResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token for the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A number affix resource."]
pub struct GamesNumberAffixConfiguration {
    #[serde(rename = "few")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When the language requires special treatment of \"small\" numbers (as with 2, 3, and 4 in Czech; or numbers ending 2, 3, or 4 but not 12, 13, or 14 in Polish)."]
    pub few: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
    #[serde(rename = "many")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When the language requires special treatment of \"large\" numbers (as with numbers ending 11-99 in Maltese)."]
    pub many: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
    #[serde(rename = "one")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When the language requires special treatment of numbers like one (as with the number 1 in English and most other languages; in Russian, any number ending in 1 but not ending in 11 is in this class)."]
    pub one: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
    #[serde(rename = "other")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When the language does not require special treatment of the given quantity (as with all numbers in Chinese, or 42 in English)."]
    pub other: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
    #[serde(rename = "two")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When the language requires special treatment of numbers like two (as with 2 in Welsh, or 102 in Slovenian)."]
    pub two: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
    #[serde(rename = "zero")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When the language requires special treatment of the number 0 (as in Arabic)."]
    pub zero: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A number format resource."]
pub struct GamesNumberFormatConfiguration {
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The curreny code string. Only used for CURRENCY format type."]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numDecimalPlaces")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of decimal places for number. Only used for NUMERIC format type."]
    pub num_decimal_places: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "numberFormatType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The formatting for the number."]
    pub number_format_type:
        ::std::option::Option<GamesNumberFormatConfigurationNumberFormatTypeEnum>,
    #[serde(rename = "suffix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional suffix for the NUMERIC format type. These strings follow the same plural rules as all Android string resources."]
    pub suffix: ::std::option::Option<::std::boxed::Box<GamesNumberAffixConfiguration>>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An image configuration resource."]
pub struct ImageConfiguration {
    #[serde(rename = "imageType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The image type for the image."]
    pub image_type: ::std::option::Option<ImageConfigurationImageTypeEnum>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#imageConfiguration`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource ID of resource which the image belongs to."]
    pub resource_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The url for this image."]
    pub url: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An leaderboard configuration resource."]
pub struct LeaderboardConfiguration {
    #[serde(rename = "draft")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The draft data of the leaderboard."]
    pub draft: ::std::option::Option<::std::boxed::Box<LeaderboardConfigurationDetail>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the leaderboard."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#leaderboardConfiguration`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "published")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The read-only published data of the leaderboard."]
    pub published: ::std::option::Option<::std::boxed::Box<LeaderboardConfigurationDetail>>,
    #[serde(rename = "scoreMax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum score that can be posted to this leaderboard."]
    pub score_max: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scoreMin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minimum score that can be posted to this leaderboard."]
    pub score_min: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scoreOrder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub score_order: ::std::option::Option<LeaderboardConfigurationScoreOrderEnum>,
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for this resource."]
    pub token: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A leaderboard configuration detail."]
pub struct LeaderboardConfigurationDetail {
    #[serde(rename = "iconUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The icon url of this leaderboard. Writes to this field are ignored."]
    pub icon_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#leaderboardConfigurationDetail`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Localized strings for the leaderboard name."]
    pub name: ::std::option::Option<::std::boxed::Box<LocalizedStringBundle>>,
    #[serde(rename = "scoreFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The score formatting for the leaderboard."]
    pub score_format: ::std::option::Option<::std::boxed::Box<GamesNumberFormatConfiguration>>,
    #[serde(rename = "sortRank")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sort rank of this leaderboard. Writes to this field are ignored."]
    pub sort_rank: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ListConfigurations response."]
pub struct LeaderboardConfigurationListResponse {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The leaderboard configurations."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LeaderboardConfiguration>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#leaderboardConfigurationListResponse`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token for the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A localized string resource."]
pub struct LocalizedString {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#localizedString`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The locale string."]
    pub locale: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The string value."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A localized string bundle resource."]
pub struct LocalizedStringBundle {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#localizedStringBundle`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "translations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The locale strings."]
    pub translations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LocalizedString>>>,
}
