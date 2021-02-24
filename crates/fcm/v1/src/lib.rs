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
        serde_json::from_str(&"\"json\"").unwrap()
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
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Android specific options for messages sent through [FCM connection server](https://goo.gl/4GLdUl)."]
    pub struct AndroidConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "collapseKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An identifier of a group of messages that can be collapsed, so that only the last message gets sent when delivery can be resumed. A maximum of 4 different collapse keys is allowed at any given time."]
        pub collapse_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Arbitrary key/value payload. If present, it will override google.firebase.fcm.v1.Message.data."]
        pub data:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "directBootOk")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set to true, messages will be allowed to be delivered to the app while the device is in direct boot mode. See [Support Direct Boot mode](https://developer.android.com/training/articles/direct-boot)."]
        pub direct_boot_ok: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fcmOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Options for features provided by the FCM SDK for Android."]
        pub fcm_options: ::std::option::Option<::std::boxed::Box<AndroidFcmOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notification")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Notification to send to android devices."]
        pub notification: ::std::option::Option<::std::boxed::Box<AndroidNotification>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "priority")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Message priority. Can take \"normal\" and \"high\" values. For more information, see [Setting the priority of a message](https://goo.gl/GjONJv)."]
        pub priority: ::std::option::Option<AndroidConfigPriorityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restrictedPackageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Package name of the application where the registration token must match in order to receive the message."]
        pub restricted_package_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ttl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How long (in seconds) the message should be kept in FCM storage if the device is offline. The maximum time to live supported is 4 weeks, and the default value is 4 weeks if not set. Set it to 0 if want to send the message immediately. In JSON format, the Duration type is encoded as a string rather than an object, where the string ends in the suffix \"s\" (indicating seconds) and is preceded by the number of seconds, with nanoseconds expressed as fractional seconds. For example, 3 seconds with 0 nanoseconds should be encoded in JSON format as \"3s\", while 3 seconds and 1 nanosecond should be expressed in JSON format as \"3.000000001s\". The ttl will be rounded down to the nearest second."]
        pub ttl: ::std::option::Option<::std::string::String>,
    }
    impl AndroidConfig {
        pub fn builder() -> AndroidConfigBuilder {
            AndroidConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Message priority. Can take \"normal\" and \"high\" values. For more information, see [Setting the priority of a message](https://goo.gl/GjONJv)."]
    pub enum AndroidConfigPriorityEnum {
        #[serde(rename = "NORMAL")]
        #[doc = "Default priority for data messages. Normal priority messages won't open network connections on a sleeping device, and their delivery may be delayed to conserve the battery. For less time-sensitive messages, such as notifications of new email or other data to sync, choose normal delivery priority."]
        Normal,
        #[serde(rename = "HIGH")]
        #[doc = "Default priority for notification messages. FCM attempts to deliver high priority messages immediately, allowing the FCM service to wake a sleeping device when possible and open a network connection to your app server. Apps with instant messaging, chat, or voice call alerts, for example, generally need to open a network connection and make sure FCM delivers the message to the device without delay. Set high priority if the message is time-critical and requires the user's immediate interaction, but beware that setting your messages to high priority contributes more to battery drain compared with normal priority messages."]
        High,
    }
    impl ::std::default::Default for AndroidConfigPriorityEnum {
        fn default() -> Self {
            Self::Normal
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options for features provided by the FCM SDK for Android."]
    pub struct AndroidFcmOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "analyticsLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label associated with the message's analytics data."]
        pub analytics_label: ::std::option::Option<::std::string::String>,
    }
    impl AndroidFcmOptions {
        pub fn builder() -> AndroidFcmOptionsBuilder {
            AndroidFcmOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Notification to send to android devices."]
    pub struct AndroidNotification {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "body")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The notification's body text. If present, it will override google.firebase.fcm.v1.Notification.body."]
        pub body: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bodyLocArgs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Variable string values to be used in place of the format specifiers in body_loc_key to use to localize the body text to the user's current localization. See [Formatting and Styling](https://goo.gl/MalYE3) for more information."]
        pub body_loc_args: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bodyLocKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key to the body string in the app's string resources to use to localize the body text to the user's current localization. See [String Resources](https://goo.gl/NdFZGI) for more information."]
        pub body_loc_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [notification's channel id](https://developer.android.com/guide/topics/ui/notifiers/notifications#ManageChannels) (new in Android O). The app must create a channel with this channel ID before any notification with this channel ID is received. If you don't send this channel ID in the request, or if the channel ID provided has not yet been created by the app, FCM uses the channel ID specified in the app manifest."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clickAction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The action associated with a user click on the notification. If specified, an activity with a matching intent filter is launched when a user clicks on the notification."]
        pub click_action: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The notification's icon color, expressed in #rrggbb format."]
        pub color: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultLightSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set to true, use the Android framework's default LED light settings for the notification. Default values are specified in [config.xml](https://android.googlesource.com/platform/frameworks/base/+/master/core/res/res/values/config.xml). If `default_light_settings` is set to true and `light_settings` is also set, the user-specified `light_settings` is used instead of the default value."]
        pub default_light_settings: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultSound")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set to true, use the Android framework's default sound for the notification. Default values are specified in [config.xml](https://android.googlesource.com/platform/frameworks/base/+/master/core/res/res/values/config.xml)."]
        pub default_sound: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultVibrateTimings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set to true, use the Android framework's default vibrate pattern for the notification. Default values are specified in [config.xml](https://android.googlesource.com/platform/frameworks/base/+/master/core/res/res/values/config.xml). If `default_vibrate_timings` is set to true and `vibrate_timings` is also set, the default value is used instead of the user-specified `vibrate_timings`."]
        pub default_vibrate_timings: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set the time that the event in the notification occurred. Notifications in the panel are sorted by this time. A point in time is represented using [protobuf.Timestamp](https://developers.google.com/protocol-buffers/docs/reference/java/com/google/protobuf/Timestamp)."]
        pub event_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "icon")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The notification's icon. Sets the notification icon to myicon for drawable resource myicon. If you don't send this key in the request, FCM displays the launcher icon specified in your app manifest."]
        pub icon: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains the URL of an image that is going to be displayed in a notification. If present, it will override google.firebase.fcm.v1.Notification.image."]
        pub image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lightSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Settings to control the notification's LED blinking rate and color if LED is available on the device. The total blinking time is controlled by the OS."]
        pub light_settings: ::std::option::Option<::std::boxed::Box<LightSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set whether or not this notification is relevant only to the current device. Some notifications can be bridged to other devices for remote display, such as a Wear OS watch. This hint can be set to recommend this notification not be bridged. See [Wear OS guides](https://developer.android.com/training/wearables/notifications/bridger#existing-method-of-preventing-bridging)"]
        pub local_only: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notificationCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sets the number of items this notification represents. May be displayed as a badge count for launchers that support badging.See [Notification Badge](https://developer.android.com/training/notify-user/badges). For example, this might be useful if you're using just one notification to represent multiple new messages but you want the count here to represent the number of total new messages. If zero or unspecified, systems that support badging use the default, which is to increment a number displayed on the long-press menu each time a new notification arrives."]
        pub notification_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notificationPriority")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set the relative priority for this notification. Priority is an indication of how much of the user's attention should be consumed by this notification. Low-priority notifications may be hidden from the user in certain situations, while the user might be interrupted for a higher-priority notification. The effect of setting the same priorities may differ slightly on different platforms. Note this priority differs from `AndroidMessagePriority`. This priority is processed by the client after the message has been delivered, whereas [AndroidMessagePriority](https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#androidmessagepriority) is an FCM concept that controls when the message is delivered."]
        pub notification_priority:
            ::std::option::Option<AndroidNotificationNotificationPriorityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sound")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sound to play when the device receives the notification. Supports \"default\" or the filename of a sound resource bundled in the app. Sound files must reside in /res/raw/."]
        pub sound: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sticky")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When set to false or unset, the notification is automatically dismissed when the user clicks it in the panel. When set to true, the notification persists even when the user clicks it."]
        pub sticky: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier used to replace existing notifications in the notification drawer. If not specified, each request creates a new notification. If specified and a notification with the same tag is already being shown, the new notification replaces the existing one in the notification drawer."]
        pub tag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ticker")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sets the \"ticker\" text, which is sent to accessibility services. Prior to API level 21 (`Lollipop`), sets the text that is displayed in the status bar when the notification first arrives."]
        pub ticker: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The notification's title. If present, it will override google.firebase.fcm.v1.Notification.title."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "titleLocArgs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Variable string values to be used in place of the format specifiers in title_loc_key to use to localize the title text to the user's current localization. See [Formatting and Styling](https://goo.gl/MalYE3) for more information."]
        pub title_loc_args: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "titleLocKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key to the title string in the app's string resources to use to localize the title text to the user's current localization. See [String Resources](https://goo.gl/NdFZGI) for more information."]
        pub title_loc_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vibrateTimings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set the vibration pattern to use. Pass in an array of [protobuf.Duration](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Duration) to turn on or off the vibrator. The first value indicates the `Duration` to wait before turning the vibrator on. The next value indicates the `Duration` to keep the vibrator on. Subsequent values alternate between `Duration` to turn the vibrator off and to turn the vibrator on. If `vibrate_timings` is set and `default_vibrate_timings` is set to `true`, the default value is used instead of the user-specified `vibrate_timings`."]
        pub vibrate_timings: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibility")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set the [Notification.visibility](https://developer.android.com/reference/android/app/Notification.html#visibility) of the notification."]
        pub visibility: ::std::option::Option<AndroidNotificationVisibilityEnum>,
    }
    impl AndroidNotification {
        pub fn builder() -> AndroidNotificationBuilder {
            AndroidNotificationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Set the relative priority for this notification. Priority is an indication of how much of the user's attention should be consumed by this notification. Low-priority notifications may be hidden from the user in certain situations, while the user might be interrupted for a higher-priority notification. The effect of setting the same priorities may differ slightly on different platforms. Note this priority differs from `AndroidMessagePriority`. This priority is processed by the client after the message has been delivered, whereas [AndroidMessagePriority](https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#androidmessagepriority) is an FCM concept that controls when the message is delivered."]
    pub enum AndroidNotificationNotificationPriorityEnum {
        #[serde(rename = "PRIORITY_UNSPECIFIED")]
        #[doc = "If priority is unspecified, notification priority is set to `PRIORITY_DEFAULT`."]
        PriorityUnspecified,
        #[serde(rename = "PRIORITY_MIN")]
        #[doc = "Lowest notification priority. Notifications with this `PRIORITY_MIN` might not be shown to the user except under special circumstances, such as detailed notification logs."]
        PriorityMin,
        #[serde(rename = "PRIORITY_LOW")]
        #[doc = "Lower notification priority. The UI may choose to show the notifications smaller, or at a different position in the list, compared with notifications with `PRIORITY_DEFAULT`."]
        PriorityLow,
        #[serde(rename = "PRIORITY_DEFAULT")]
        #[doc = "Default notification priority. If the application does not prioritize its own notifications, use this value for all notifications."]
        PriorityDefault,
        #[serde(rename = "PRIORITY_HIGH")]
        #[doc = "Higher notification priority. Use this for more important notifications or alerts. The UI may choose to show these notifications larger, or at a different position in the notification lists, compared with notifications with `PRIORITY_DEFAULT`."]
        PriorityHigh,
        #[serde(rename = "PRIORITY_MAX")]
        #[doc = "Highest notification priority. Use this for the application's most important items that require the user's prompt attention or input."]
        PriorityMax,
    }
    impl ::std::default::Default for AndroidNotificationNotificationPriorityEnum {
        fn default() -> Self {
            Self::PriorityUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Set the [Notification.visibility](https://developer.android.com/reference/android/app/Notification.html#visibility) of the notification."]
    pub enum AndroidNotificationVisibilityEnum {
        #[serde(rename = "VISIBILITY_UNSPECIFIED")]
        #[doc = "If unspecified, default to `Visibility.PRIVATE`."]
        VisibilityUnspecified,
        #[serde(rename = "PRIVATE")]
        #[doc = "Show this notification on all lockscreens, but conceal sensitive or private information on secure lockscreens."]
        Private,
        #[serde(rename = "PUBLIC")]
        #[doc = "Show this notification in its entirety on all lockscreens."]
        Public,
        #[serde(rename = "SECRET")]
        #[doc = "Do not reveal any part of this notification on a secure lockscreen."]
        Secret,
    }
    impl ::std::default::Default for AndroidNotificationVisibilityEnum {
        fn default() -> Self {
            Self::VisibilityUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "[Apple Push Notification Service](https://goo.gl/MXRTPa) specific options."]
    pub struct ApnsConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fcmOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Options for features provided by the FCM SDK for iOS."]
        pub fcm_options: ::std::option::Option<::std::boxed::Box<ApnsFcmOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP request headers defined in Apple Push Notification Service. Refer to [APNs request headers](https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/sending_notification_requests_to_apns) for supported headers, e.g. \"apns-priority\": \"10\"."]
        pub headers:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "APNs payload as a JSON object, including both `aps` dictionary and custom payload. See [Payload Key Reference](https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/generating_a_remote_notification). If present, it overrides google.firebase.fcm.v1.Notification.title and google.firebase.fcm.v1.Notification.body."]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ApnsConfig {
        pub fn builder() -> ApnsConfigBuilder {
            ApnsConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options for features provided by the FCM SDK for iOS."]
    pub struct ApnsFcmOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "analyticsLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label associated with the message's analytics data."]
        pub analytics_label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains the URL of an image that is going to be displayed in a notification. If present, it will override google.firebase.fcm.v1.Notification.image."]
        pub image: ::std::option::Option<::std::string::String>,
    }
    impl ApnsFcmOptions {
        pub fn builder() -> ApnsFcmOptionsBuilder {
            ApnsFcmOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a color in the RGBA color space. This representation is designed for simplicity of conversion to/from color representations in various languages over compactness; for example, the fields of this representation can be trivially provided to the constructor of \"java.awt.Color\" in Java; it can also be trivially provided to UIColor's \"+colorWithRed:green:blue:alpha\" method in iOS; and, with just a little work, it can be easily formatted into a CSS \"rgba()\" string in JavaScript, as well. Note: this proto does not carry information about the absolute color space that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB, DCI-P3, BT.2020, etc.). By default, applications SHOULD assume the sRGB color space. Note: when color equality needs to be decided, implementations, unless documented otherwise, will treat two colors to be equal if all their red, green, blue and alpha values each differ by at most 1e-5. Example (Java): import com.google.type.Color; // ... public static java.awt.Color fromProto(Color protocolor) { float alpha = protocolor.hasAlpha() ? protocolor.getAlpha().getValue() : 1.0; return new java.awt.Color( protocolor.getRed(), protocolor.getGreen(), protocolor.getBlue(), alpha); } public static Color toProto(java.awt.Color color) { float red = (float) color.getRed(); float green = (float) color.getGreen(); float blue = (float) color.getBlue(); float denominator = 255.0; Color.Builder resultBuilder = Color .newBuilder() .setRed(red / denominator) .setGreen(green / denominator) .setBlue(blue / denominator); int alpha = color.getAlpha(); if (alpha != 255) { result.setAlpha( FloatValue .newBuilder() .setValue(((float) alpha) / denominator) .build()); } return resultBuilder.build(); } // ... Example (iOS / Obj-C): // ... static UIColor* fromProto(Color* protocolor) { float red = [protocolor red]; float green = [protocolor green]; float blue = [protocolor blue]; FloatValue* alpha_wrapper = [protocolor alpha]; float alpha = 1.0; if (alpha_wrapper != nil) { alpha = [alpha_wrapper value]; } return [UIColor colorWithRed:red green:green blue:blue alpha:alpha]; } static Color* toProto(UIColor* color) { CGFloat red, green, blue, alpha; if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) { return nil; } Color* result = [[Color alloc] init]; [result setRed:red]; [result setGreen:green]; [result setBlue:blue]; if (alpha <= 0.9999) { [result setAlpha:floatWrapperWithValue(alpha)]; } [result autorelease]; return result; } // ... Example (JavaScript): // ... var protoToCssColor = function(rgb_color) { var redFrac = rgb_color.red || 0.0; var greenFrac = rgb_color.green || 0.0; var blueFrac = rgb_color.blue || 0.0; var red = Math.floor(redFrac * 255); var green = Math.floor(greenFrac * 255); var blue = Math.floor(blueFrac * 255); if (!('alpha' in rgb_color)) { return rgbToCssColor_(red, green, blue); } var alphaFrac = rgb_color.alpha.value || 0.0; var rgbParams = [red, green, blue].join(','); return ['rgba(', rgbParams, ',', alphaFrac, ')'].join(''); }; var rgbToCssColor_ = function(red, green, blue) { var rgbNumber = new Number((red << 16) | (green << 8) | blue); var hexString = rgbNumber.toString(16); var missingZeros = 6 - hexString.length; var resultBuilder = ['#']; for (var i = 0; i < missingZeros; i++) { resultBuilder.push('0'); } resultBuilder.push(hexString); return resultBuilder.join(''); }; // ..."]
    pub struct Color {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alpha")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fraction of this color that should be applied to the pixel. That is, the final pixel color is defined by the equation: pixel color = alpha * (this color) + (1.0 - alpha) * (background color) This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color. This uses a wrapper message rather than a simple float scalar so that it is possible to distinguish between a default value and the value being unset. If omitted, this color object is to be rendered as a solid color (as if the alpha value had been explicitly given with a value of 1.0)."]
        pub alpha: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of blue in the color as a value in the interval [0, 1]."]
        pub blue: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "green")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of green in the color as a value in the interval [0, 1]."]
        pub green: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "red")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of red in the color as a value in the interval [0, 1]."]
        pub red: ::std::option::Option<::std::primitive::f64>,
    }
    impl Color {
        pub fn builder() -> ColorBuilder {
            ColorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Platform independent options for features provided by the FCM SDKs."]
    pub struct FcmOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "analyticsLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label associated with the message's analytics data."]
        pub analytics_label: ::std::option::Option<::std::string::String>,
    }
    impl FcmOptions {
        pub fn builder() -> FcmOptionsBuilder {
            FcmOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings to control notification LED."]
    pub struct LightSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Set `color` of the LED with [google.type.Color](https://github.com/googleapis/googleapis/blob/master/google/type/color.proto)."]
        pub color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lightOffDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Along with `light_on_duration `, define the blink rate of LED flashes. Resolution defined by [proto.Duration](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Duration)"]
        pub light_off_duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lightOnDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Along with `light_off_duration`, define the blink rate of LED flashes. Resolution defined by [proto.Duration](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Duration)"]
        pub light_on_duration: ::std::option::Option<::std::string::String>,
    }
    impl LightSettings {
        pub fn builder() -> LightSettingsBuilder {
            LightSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message to send by Firebase Cloud Messaging Service."]
    pub struct Message {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "android")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input only. Android specific options for messages sent through [FCM connection server](https://goo.gl/4GLdUl)."]
        pub android: ::std::option::Option<::std::boxed::Box<AndroidConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input only. [Apple Push Notification Service](https://goo.gl/MXRTPa) specific options."]
        pub apns: ::std::option::Option<::std::boxed::Box<ApnsConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Condition to send a message to, e.g. \"'foo' in topics && 'bar' in topics\"."]
        pub condition: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input only. Arbitrary key/value payload. The key should not be a reserved word (\"from\", \"message_type\", or any word starting with \"google\" or \"gcm\")."]
        pub data:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fcmOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input only. Template for FCM SDK feature options to use across all platforms."]
        pub fcm_options: ::std::option::Option<::std::boxed::Box<FcmOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output Only. The identifier of the message sent, in the format of `projects/*/messages/{message_id}`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notification")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input only. Basic notification template to use across all platforms."]
        pub notification: ::std::option::Option<::std::boxed::Box<Notification>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "token")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Registration token to send a message to."]
        pub token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topic name to send a message to, e.g. \"weather\". Note: \"/topics/\" prefix should not be provided."]
        pub topic: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webpush")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input only. [Webpush protocol](https://tools.ietf.org/html/rfc8030) options."]
        pub webpush: ::std::option::Option<::std::boxed::Box<WebpushConfig>>,
    }
    impl Message {
        pub fn builder() -> MessageBuilder {
            MessageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic notification template to use across all platforms."]
    pub struct Notification {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "body")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The notification's body text."]
        pub body: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains the URL of an image that is going to be downloaded on the device and displayed in a notification. JPEG, PNG, BMP have full support across platforms. Animated GIF and video only work on iOS. WebP and HEIF have varying levels of support across platforms and platform versions. Android has 1MB image size limit. Quota usage and implications/costs for hosting image on Firebase Storage: https://firebase.google.com/pricing"]
        pub image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The notification's title."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl Notification {
        pub fn builder() -> NotificationBuilder {
            NotificationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to send a message to specified target."]
    pub struct SendMessageRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Message to send."]
        pub message: ::std::option::Option<::std::boxed::Box<Message>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validateOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag for testing the request without actually delivering the message."]
        pub validate_only: ::std::option::Option<::std::primitive::bool>,
    }
    impl SendMessageRequest {
        pub fn builder() -> SendMessageRequestBuilder {
            SendMessageRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "[Webpush protocol](https://tools.ietf.org/html/rfc8030) options."]
    pub struct WebpushConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Arbitrary key/value payload. If present, it will override google.firebase.fcm.v1.Message.data."]
        pub data:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fcmOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Options for features provided by the FCM SDK for Web."]
        pub fcm_options: ::std::option::Option<::std::boxed::Box<WebpushFcmOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP headers defined in webpush protocol. Refer to [Webpush protocol](https://tools.ietf.org/html/rfc8030#section-5) for supported headers, e.g. \"TTL\": \"15\"."]
        pub headers:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notification")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web Notification options as a JSON object. Supports Notification instance properties as defined in [Web Notification API](https://developer.mozilla.org/en-US/docs/Web/API/Notification). If present, \"title\" and \"body\" fields override [google.firebase.fcm.v1.Notification.title] and [google.firebase.fcm.v1.Notification.body]."]
        pub notification:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl WebpushConfig {
        pub fn builder() -> WebpushConfigBuilder {
            WebpushConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options for features provided by the FCM SDK for Web."]
    pub struct WebpushFcmOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "analyticsLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label associated with the message's analytics data."]
        pub analytics_label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The link to open when the user clicks on the notification. For all URL values, HTTPS is required."]
        pub link: ::std::option::Option<::std::string::String>,
    }
    impl WebpushFcmOptions {
        pub fn builder() -> WebpushFcmOptionsBuilder {
            WebpushFcmOptionsBuilder::default()
        }
    }
}
