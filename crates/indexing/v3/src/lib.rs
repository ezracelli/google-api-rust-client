#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Output for PublishUrlNotification"]
pub struct PublishUrlNotificationResponse {
    #[serde(rename = "urlNotificationMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the notification events received for this URL."]
    pub url_notification_metadata:
        ::std::option::Option<::std::boxed::Box<UrlNotificationMetadata>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`UrlNotification` is the resource used in all Indexing API calls. It describes one event in the life cycle of a Web Document."]
pub struct UrlNotification {
    #[serde(rename = "notifyTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creation timestamp for this notification. Users should _not_ specify it, the field is ignored at the request time."]
    pub notify_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL life cycle event that Google is being notified about."]
    pub _type: ::std::option::Option<UrlNotificationTypeEnum>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The object of this notification. The URL must be owned by the publisher of this notification and, in case of `URL_UPDATED` notifications, it _must_ be crawlable by Google."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The URL life cycle event that Google is being notified about."]
pub enum UrlNotificationTypeEnum {
    #[serde(rename = "URL_NOTIFICATION_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    UrlNotificationTypeUnspecified,
    #[serde(rename = "URL_UPDATED")]
    #[doc = "The given URL (Web document) has been updated."]
    UrlUpdated,
    #[serde(rename = "URL_DELETED")]
    #[doc = "The given URL (Web document) has been deleted."]
    UrlDeleted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Summary of the most recent Indexing API notifications successfully received, for a given URL."]
pub struct UrlNotificationMetadata {
    #[serde(rename = "latestRemove")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Latest notification received with type `URL_REMOVED`."]
    pub latest_remove: ::std::option::Option<::std::boxed::Box<UrlNotification>>,
    #[serde(rename = "latestUpdate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Latest notification received with type `URL_UPDATED`."]
    pub latest_update: ::std::option::Option<::std::boxed::Box<UrlNotification>>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to which this metadata refers."]
    pub url: ::std::option::Option<::std::string::String>,
}
