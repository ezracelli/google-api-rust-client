#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of string parameters to supply when the action method is invoked. For example, consider three snooze buttons: snooze now, snooze 1 day, snooze next week. You might use action method = snooze(), passing the snooze type and snooze time in the list of string parameters."]
pub struct ActionParameter {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the parameter for the action script."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of the parameter."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Parameters that a bot can use to configure how it's response is posted."]
pub struct ActionResponse {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of bot response."]
    pub _type: ::std::option::Option<ActionResponseTypeEnum>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL for users to auth or config. (Only for REQUEST_CONFIG response types.)"]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of bot response."]
pub enum ActionResponseTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Default type; will be handled as NEW_MESSAGE."]
    TypeUnspecified,
    #[serde(rename = "NEW_MESSAGE")]
    #[doc = "Post as a new message in the topic."]
    NewMessage,
    #[serde(rename = "UPDATE_MESSAGE")]
    #[doc = "Update the bot's own message. (Only after CARD_CLICKED events.)"]
    UpdateMessage,
    #[serde(rename = "REQUEST_CONFIG")]
    #[doc = "Privately ask the user for additional auth or config."]
    RequestConfig,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotations associated with the plain-text body of the message. Example plain-text message body: ``` Hello @FooBot how are you!\" ``` The corresponding annotations metadata: ``` \"annotations\":[{ \"type\":\"USER_MENTION\", \"startIndex\":6, \"length\":7, \"userMention\": { \"user\": { \"name\":\"users/107946847022116401880\", \"displayName\":\"FooBot\", \"avatarUrl\":\"https://goo.gl/aeDtrS\", \"type\":\"BOT\" }, \"type\":\"MENTION\" } }] ```"]
pub struct Annotation {
    #[serde(rename = "length")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Length of the substring in the plain-text message body this annotation corresponds to."]
    pub length: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "slashCommand")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The metadata for a slash command."]
    pub slash_command: ::std::option::Option<::std::boxed::Box<SlashCommandMetadata>>,
    #[serde(rename = "startIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Start index (0-based, inclusive) in the plain-text message body this annotation corresponds to."]
    pub start_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of this annotation."]
    pub _type: ::std::option::Option<AnnotationTypeEnum>,
    #[serde(rename = "userMention")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The metadata of user mention."]
    pub user_mention: ::std::option::Option<::std::boxed::Box<UserMentionMetadata>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of this annotation."]
pub enum AnnotationTypeEnum {
    #[serde(rename = "ANNOTATION_TYPE_UNSPECIFIED")]
    #[doc = "Default value for the enum. DO NOT USE."]
    AnnotationTypeUnspecified,
    #[serde(rename = "USER_MENTION")]
    #[doc = "A user is mentioned."]
    UserMention,
    #[serde(rename = "SLASH_COMMAND")]
    #[doc = "A slash command is invoked."]
    SlashCommand,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An attachment in Hangouts Chat."]
pub struct Attachment {
    #[serde(rename = "attachmentDataRef")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A reference to the attachment data. This is used with the media API to download the attachment data."]
    pub attachment_data_ref: ::std::option::Option<::std::boxed::Box<AttachmentDataRef>>,
    #[serde(rename = "contentName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The original file name for the content, not the full path."]
    pub content_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content type (MIME type) of the file."]
    pub content_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "downloadUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The download URL which should be used to allow a human user to download the attachment. Bots should not use this URL to download attachment content."]
    pub download_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "driveDataRef")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A reference to the drive attachment. This is used with the Drive API."]
    pub drive_data_ref: ::std::option::Option<::std::boxed::Box<DriveDataRef>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name of the attachment, in the form \"spaces/*/messages/*/attachments/*\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source of the attachment."]
    pub source: ::std::option::Option<AttachmentSourceEnum>,
    #[serde(rename = "thumbnailUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The thumbnail URL which should be used to preview the attachment to a human user. Bots should not use this URL to download attachment content."]
    pub thumbnail_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The source of the attachment."]
pub enum AttachmentSourceEnum {
    #[serde(rename = "SOURCE_UNSPECIFIED")]
    #[doc = ""]
    SourceUnspecified,
    #[serde(rename = "DRIVE_FILE")]
    #[doc = ""]
    DriveFile,
    #[serde(rename = "UPLOADED_CONTENT")]
    #[doc = ""]
    UploadedContent,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A reference to the data of an attachment."]
pub struct AttachmentDataRef {
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the attachment data. This is used with the media API to download the attachment data."]
    pub resource_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A button. Can be a text button or an image button."]
pub struct Button {
    #[serde(rename = "imageButton")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A button with image and onclick action."]
    pub image_button: ::std::option::Option<::std::boxed::Box<ImageButton>>,
    #[serde(rename = "textButton")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A button with text and onclick action."]
    pub text_button: ::std::option::Option<::std::boxed::Box<TextButton>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A card is a UI element that can contain UI widgets such as texts, images."]
pub struct Card {
    #[serde(rename = "cardActions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The actions of this card."]
    pub card_actions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CardAction>>>,
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The header of the card. A header usually contains a title and an image."]
    pub header: ::std::option::Option<::std::boxed::Box<CardHeader>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the card."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sections")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sections are separated by a line divider."]
    pub sections: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Section>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A card action is the action associated with the card. For an invoice card, a typical action would be: delete invoice, email invoice or open the invoice in browser."]
pub struct CardAction {
    #[serde(rename = "actionLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The label used to be displayed in the action menu item."]
    pub action_label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "onClick")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The onclick action for this action item."]
    pub on_click: ::std::option::Option<::std::boxed::Box<OnClick>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardHeader {
    #[serde(rename = "imageStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The image's type (e.g. square border or circular border)."]
    pub image_style: ::std::option::Option<CardHeaderImageStyleEnum>,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the image in the card header."]
    pub image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subtitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The subtitle of the card header."]
    pub subtitle: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title must be specified. The header has a fixed height: if both a title and subtitle is specified, each will take up 1 line. If only the title is specified, it will take up both lines."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The image's type (e.g. square border or circular border)."]
pub enum CardHeaderImageStyleEnum {
    #[serde(rename = "IMAGE_STYLE_UNSPECIFIED")]
    #[doc = ""]
    ImageStyleUnspecified,
    #[serde(rename = "IMAGE")]
    #[doc = "Square border."]
    Image,
    #[serde(rename = "AVATAR")]
    #[doc = "Circular border."]
    Avatar,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Google Chat events."]
pub struct DeprecatedEvent {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The form action data associated with an interactive card that was clicked. Only populated for CARD_CLICKED events. See the [Interactive Cards guide](/hangouts/chat/how-tos/cards-onclick) for more information."]
    pub action: ::std::option::Option<::std::boxed::Box<FormAction>>,
    #[serde(rename = "configCompleteRedirectUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL the bot should redirect the user to after they have completed an authorization or configuration flow outside of Google Chat. See the [Authorizing access to 3p services guide](/hangouts/chat/how-tos/auth-3p) for more information."]
    pub config_complete_redirect_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "eventTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp indicating when the event was dispatched."]
    pub event_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The message that triggered the event, if applicable."]
    pub message: ::std::option::Option<::std::boxed::Box<Message>>,
    #[serde(rename = "space")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The room or DM in which the event occurred."]
    pub space: ::std::option::Option<::std::boxed::Box<Space>>,
    #[serde(rename = "threadKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bot-defined key for the thread related to the event. See the thread_key field of the `spaces.message.create` request for more information."]
    pub thread_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A secret value that bots can use to verify if a request is from Google. The token is randomly generated by Google, remains static, and can be obtained from the Google Chat API configuration page in the Cloud Console. Developers can revoke/regenerate it if needed from the same page."]
    pub token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the event."]
    pub _type: ::std::option::Option<DeprecatedEventTypeEnum>,
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user that triggered the event."]
    pub user: ::std::option::Option<::std::boxed::Box<User>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the event."]
pub enum DeprecatedEventTypeEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = "Default value for the enum. DO NOT USE."]
    Unspecified,
    #[serde(rename = "MESSAGE")]
    #[doc = "A message was sent in a room or direct message."]
    Message,
    #[serde(rename = "ADDED_TO_SPACE")]
    #[doc = "The bot was added to a room or DM."]
    AddedToSpace,
    #[serde(rename = "REMOVED_FROM_SPACE")]
    #[doc = "The bot was removed from a room or DM."]
    RemovedFromSpace,
    #[serde(rename = "CARD_CLICKED")]
    #[doc = "The bot's interactive card was clicked."]
    CardClicked,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A reference to the data of a drive attachment."]
pub struct DriveDataRef {
    #[serde(rename = "driveFileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id for the drive file, for use with the Drive API."]
    pub drive_file_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A form action describes the behavior when the form is submitted. For example, an Apps Script can be invoked to handle the form."]
pub struct FormAction {
    #[serde(rename = "actionMethodName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The method name is used to identify which part of the form triggered the form submission. This information is echoed back to the bot as part of the card click event. The same method name can be used for several elements that trigger a common behavior if desired."]
    pub action_method_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of action parameters."]
    pub parameters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ActionParameter>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An image that is specified by a URL and can have an onclick action."]
pub struct Image {
    #[serde(rename = "aspectRatio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The aspect ratio of this image (width/height). This field allows clients to reserve the right height for the image while waiting for it to load. It's not meant to override the native aspect ratio of the image. If unset, the server fills it by prefetching the image."]
    pub aspect_ratio: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the image."]
    pub image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "onClick")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The onclick action."]
    pub on_click: ::std::option::Option<::std::boxed::Box<OnClick>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An image button with an onclick action."]
pub struct ImageButton {
    #[serde(rename = "icon")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The icon specified by an enum that indices to an icon provided by Chat API."]
    pub icon: ::std::option::Option<ImageButtonIconEnum>,
    #[serde(rename = "iconUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The icon specified by a URL."]
    pub icon_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this image_button which will be used for accessibility. Default value will be provided if developers don't specify."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "onClick")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The onclick action."]
    pub on_click: ::std::option::Option<::std::boxed::Box<OnClick>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The icon specified by an enum that indices to an icon provided by Chat API."]
pub enum ImageButtonIconEnum {
    #[serde(rename = "ICON_UNSPECIFIED")]
    #[doc = ""]
    IconUnspecified,
    #[serde(rename = "AIRPLANE")]
    #[doc = ""]
    Airplane,
    #[serde(rename = "BOOKMARK")]
    #[doc = ""]
    Bookmark,
    #[serde(rename = "BUS")]
    #[doc = ""]
    Bus,
    #[serde(rename = "CAR")]
    #[doc = ""]
    Car,
    #[serde(rename = "CLOCK")]
    #[doc = ""]
    Clock,
    #[serde(rename = "CONFIRMATION_NUMBER_ICON")]
    #[doc = ""]
    ConfirmationNumberIcon,
    #[serde(rename = "DOLLAR")]
    #[doc = ""]
    Dollar,
    #[serde(rename = "DESCRIPTION")]
    #[doc = ""]
    Description,
    #[serde(rename = "EMAIL")]
    #[doc = ""]
    Email,
    #[serde(rename = "EVENT_PERFORMER")]
    #[doc = ""]
    EventPerformer,
    #[serde(rename = "EVENT_SEAT")]
    #[doc = ""]
    EventSeat,
    #[serde(rename = "FLIGHT_ARRIVAL")]
    #[doc = ""]
    FlightArrival,
    #[serde(rename = "FLIGHT_DEPARTURE")]
    #[doc = ""]
    FlightDeparture,
    #[serde(rename = "HOTEL")]
    #[doc = ""]
    Hotel,
    #[serde(rename = "HOTEL_ROOM_TYPE")]
    #[doc = ""]
    HotelRoomType,
    #[serde(rename = "INVITE")]
    #[doc = ""]
    Invite,
    #[serde(rename = "MAP_PIN")]
    #[doc = ""]
    MapPin,
    #[serde(rename = "MEMBERSHIP")]
    #[doc = ""]
    Membership,
    #[serde(rename = "MULTIPLE_PEOPLE")]
    #[doc = ""]
    MultiplePeople,
    #[serde(rename = "OFFER")]
    #[doc = ""]
    Offer,
    #[serde(rename = "PERSON")]
    #[doc = ""]
    Person,
    #[serde(rename = "PHONE")]
    #[doc = ""]
    Phone,
    #[serde(rename = "RESTAURANT_ICON")]
    #[doc = ""]
    RestaurantIcon,
    #[serde(rename = "SHOPPING_CART")]
    #[doc = ""]
    ShoppingCart,
    #[serde(rename = "STAR")]
    #[doc = ""]
    Star,
    #[serde(rename = "STORE")]
    #[doc = ""]
    Store,
    #[serde(rename = "TICKET")]
    #[doc = ""]
    Ticket,
    #[serde(rename = "TRAIN")]
    #[doc = ""]
    Train,
    #[serde(rename = "VIDEO_CAMERA")]
    #[doc = ""]
    VideoCamera,
    #[serde(rename = "VIDEO_PLAY")]
    #[doc = ""]
    VideoPlay,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A UI element contains a key (label) and a value (content). And this element may also contain some actions such as onclick button."]
pub struct KeyValue {
    #[serde(rename = "bottomLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text of the bottom label. Formatted text supported."]
    pub bottom_label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "button")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A button that can be clicked to trigger an action."]
    pub button: ::std::option::Option<::std::boxed::Box<Button>>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text of the content. Formatted text supported and always required."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentMultiline")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the content should be multiline."]
    pub content_multiline: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "icon")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An enum value that will be replaced by the Chat API with the corresponding icon image."]
    pub icon: ::std::option::Option<KeyValueIconEnum>,
    #[serde(rename = "iconUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The icon specified by a URL."]
    pub icon_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "onClick")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The onclick action. Only the top label, bottom label and content region are clickable."]
    pub on_click: ::std::option::Option<::std::boxed::Box<OnClick>>,
    #[serde(rename = "topLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text of the top label. Formatted text supported."]
    pub top_label: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "An enum value that will be replaced by the Chat API with the corresponding icon image."]
pub enum KeyValueIconEnum {
    #[serde(rename = "ICON_UNSPECIFIED")]
    #[doc = ""]
    IconUnspecified,
    #[serde(rename = "AIRPLANE")]
    #[doc = ""]
    Airplane,
    #[serde(rename = "BOOKMARK")]
    #[doc = ""]
    Bookmark,
    #[serde(rename = "BUS")]
    #[doc = ""]
    Bus,
    #[serde(rename = "CAR")]
    #[doc = ""]
    Car,
    #[serde(rename = "CLOCK")]
    #[doc = ""]
    Clock,
    #[serde(rename = "CONFIRMATION_NUMBER_ICON")]
    #[doc = ""]
    ConfirmationNumberIcon,
    #[serde(rename = "DOLLAR")]
    #[doc = ""]
    Dollar,
    #[serde(rename = "DESCRIPTION")]
    #[doc = ""]
    Description,
    #[serde(rename = "EMAIL")]
    #[doc = ""]
    Email,
    #[serde(rename = "EVENT_PERFORMER")]
    #[doc = ""]
    EventPerformer,
    #[serde(rename = "EVENT_SEAT")]
    #[doc = ""]
    EventSeat,
    #[serde(rename = "FLIGHT_ARRIVAL")]
    #[doc = ""]
    FlightArrival,
    #[serde(rename = "FLIGHT_DEPARTURE")]
    #[doc = ""]
    FlightDeparture,
    #[serde(rename = "HOTEL")]
    #[doc = ""]
    Hotel,
    #[serde(rename = "HOTEL_ROOM_TYPE")]
    #[doc = ""]
    HotelRoomType,
    #[serde(rename = "INVITE")]
    #[doc = ""]
    Invite,
    #[serde(rename = "MAP_PIN")]
    #[doc = ""]
    MapPin,
    #[serde(rename = "MEMBERSHIP")]
    #[doc = ""]
    Membership,
    #[serde(rename = "MULTIPLE_PEOPLE")]
    #[doc = ""]
    MultiplePeople,
    #[serde(rename = "OFFER")]
    #[doc = ""]
    Offer,
    #[serde(rename = "PERSON")]
    #[doc = ""]
    Person,
    #[serde(rename = "PHONE")]
    #[doc = ""]
    Phone,
    #[serde(rename = "RESTAURANT_ICON")]
    #[doc = ""]
    RestaurantIcon,
    #[serde(rename = "SHOPPING_CART")]
    #[doc = ""]
    ShoppingCart,
    #[serde(rename = "STAR")]
    #[doc = ""]
    Star,
    #[serde(rename = "STORE")]
    #[doc = ""]
    Store,
    #[serde(rename = "TICKET")]
    #[doc = ""]
    Ticket,
    #[serde(rename = "TRAIN")]
    #[doc = ""]
    Train,
    #[serde(rename = "VIDEO_CAMERA")]
    #[doc = ""]
    VideoCamera,
    #[serde(rename = "VIDEO_PLAY")]
    #[doc = ""]
    VideoPlay,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListMembershipsResponse {
    #[serde(rename = "memberships")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of memberships in the requested (or first) page."]
    pub memberships: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Membership>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token to retrieve the next page of results. It will be empty for the last page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListSpacesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token to retrieve the next page of results. It will be empty for the last page of results. Tokens expire in an hour. An error is thrown if an expired token is passed."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "spaces")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of spaces in the requested (or first) page."]
    pub spaces: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Space>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Media resource."]
pub struct Media {
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the media resource."]
    pub resource_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a membership relation in Hangouts Chat."]
pub struct Membership {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creation time of the membership a.k.a the time at which the member joined the space, if applicable."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "member")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A User in Hangout Chat"]
    pub member: ::std::option::Option<::std::boxed::Box<User>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "State of the membership."]
    pub state: ::std::option::Option<MembershipStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "State of the membership."]
pub enum MembershipStateEnum {
    #[serde(rename = "MEMBERSHIP_STATE_UNSPECIFIED")]
    #[doc = "Default, do not use."]
    MembershipStateUnspecified,
    #[serde(rename = "JOINED")]
    #[doc = "The user has joined the space."]
    Joined,
    #[serde(rename = "INVITED")]
    #[doc = "The user has been invited, is able to join the space, but currently has not joined."]
    Invited,
    #[serde(rename = "NOT_A_MEMBER")]
    #[doc = "The user is not a member of the space, has not been invited and is not able to join the space."]
    NotAMember,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message in Hangouts Chat."]
pub struct Message {
    #[serde(rename = "actionResponse")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input only. Parameters that a bot can use to configure how its response is posted."]
    pub action_response: ::std::option::Option<::std::boxed::Box<ActionResponse>>,
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Annotations associated with the text in this message."]
    pub annotations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Annotation>>>,
    #[serde(rename = "argumentText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Plain-text body of the message with all bot mentions stripped out."]
    pub argument_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "attachment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User uploaded attachment."]
    pub attachment: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Attachment>>>,
    #[serde(rename = "cards")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rich, formatted and interactive cards that can be used to display UI elements such as: formatted texts, buttons, clickable images. Cards are normally displayed below the plain-text body of the message."]
    pub cards: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Card>>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which the message was created in Hangouts Chat server."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fallbackText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A plain-text description of the message's cards, used when the actual cards cannot be displayed (e.g. mobile notifications)."]
    pub fallback_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name, in the form \"spaces/*/messages/*\". Example: spaces/AAAAMpdlehY/messages/UMxbHmzDlr4.UMxbHmzDlr4"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "previewText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Text for generating preview chips. This text will not be displayed to the user, but any links to images, web pages, videos, etc. included here will generate preview chips."]
    pub preview_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sender")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user who created the message."]
    pub sender: ::std::option::Option<::std::boxed::Box<User>>,
    #[serde(rename = "slashCommand")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Slash command information, if applicable."]
    pub slash_command: ::std::option::Option<::std::boxed::Box<SlashCommand>>,
    #[serde(rename = "space")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The space the message belongs to."]
    pub space: ::std::option::Option<::std::boxed::Box<Space>>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Plain-text body of the message."]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thread")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The thread the message belongs to."]
    pub thread: ::std::option::Option<::std::boxed::Box<Thread>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An onclick action (e.g. open a link)."]
pub struct OnClick {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A form action will be triggered by this onclick if specified."]
    pub action: ::std::option::Option<::std::boxed::Box<FormAction>>,
    #[serde(rename = "openLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This onclick triggers an open link action if specified."]
    pub open_link: ::std::option::Option<::std::boxed::Box<OpenLink>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A link that opens a new window."]
pub struct OpenLink {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL to open."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A section contains a collection of widgets that are rendered (vertically) in the order that they are specified. Across all platforms, cards have a narrow fixed width, so there is currently no need for layout properties (e.g. float)."]
pub struct Section {
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The header of the section, text formatted supported."]
    pub header: ::std::option::Option<::std::string::String>,
    #[serde(rename = "widgets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A section must contain at least 1 widget."]
    pub widgets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WidgetMarkup>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Slash Command in Hangouts Chat."]
pub struct SlashCommand {
    #[serde(rename = "commandId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the slash command invoked."]
    pub command_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotation metadata for slash commands (/)."]
pub struct SlashCommandMetadata {
    #[serde(rename = "bot")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bot whose command was invoked."]
    pub bot: ::std::option::Option<::std::boxed::Box<User>>,
    #[serde(rename = "commandId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The command id of the invoked slash command."]
    pub command_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "commandName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the invoked slash command."]
    pub command_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "triggersDialog")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicating whether the slash command is for a dialog."]
    pub triggers_dialog: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of slash command."]
    pub _type: ::std::option::Option<SlashCommandMetadataTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of slash command."]
pub enum SlashCommandMetadataTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Default value for the enum. DO NOT USE."]
    TypeUnspecified,
    #[serde(rename = "ADD")]
    #[doc = "Add bot to space."]
    Add,
    #[serde(rename = "INVOKE")]
    #[doc = "Invoke slash command in space."]
    Invoke,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A room or DM in Hangouts Chat."]
pub struct Space {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name (only if the space is a room). Please note that this field might not be populated in direct messages between humans."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name of the space, in the form \"spaces/*\". Example: spaces/AAAAMpdlehYs"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "singleUserBotDm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the space is a DM between a bot and a single human."]
    pub single_user_bot_dm: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "threaded")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the messages are threaded in this space."]
    pub threaded: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of a space. This is deprecated. Use `single_user_bot_dm` instead."]
    pub _type: ::std::option::Option<SpaceTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The type of a space. This is deprecated. Use `single_user_bot_dm` instead."]
pub enum SpaceTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = ""]
    TypeUnspecified,
    #[serde(rename = "ROOM")]
    #[doc = "Multi-user spaces such as rooms and DMs between humans."]
    Room,
    #[serde(rename = "DM")]
    #[doc = "1:1 Direct Message between a human and a bot, where all messages are flat."]
    Dm,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A button with text and onclick action."]
pub struct TextButton {
    #[serde(rename = "onClick")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The onclick action of the button."]
    pub on_click: ::std::option::Option<::std::boxed::Box<OnClick>>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text of the button."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A paragraph of text. Formatted text supported."]
pub struct TextParagraph {
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A thread in Hangouts Chat."]
pub struct Thread {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name, in the form \"spaces/*/threads/*\". Example: spaces/AAAAMpdlehY/threads/UMxbHmzDlr4"]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A user in Hangouts Chat."]
pub struct User {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's display name."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "domainId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Obfuscated domain information."]
    pub domain_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isAnonymous")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True when the user is deleted or the user's proifle is not visible."]
    pub is_anonymous: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name, in the format \"users/*\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User type."]
    pub _type: ::std::option::Option<UserTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "User type."]
pub enum UserTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Default value for the enum. DO NOT USE."]
    TypeUnspecified,
    #[serde(rename = "HUMAN")]
    #[doc = "Human user."]
    Human,
    #[serde(rename = "BOT")]
    #[doc = "Bot user."]
    Bot,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotation metadata for user mentions (@)."]
pub struct UserMentionMetadata {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of user mention."]
    pub _type: ::std::option::Option<UserMentionMetadataTypeEnum>,
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user mentioned."]
    pub user: ::std::option::Option<::std::boxed::Box<User>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of user mention."]
pub enum UserMentionMetadataTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Default value for the enum. DO NOT USE."]
    TypeUnspecified,
    #[serde(rename = "ADD")]
    #[doc = "Add user to space."]
    Add,
    #[serde(rename = "MENTION")]
    #[doc = "Mention user in space."]
    Mention,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A widget is a UI element that presents texts, images, etc."]
pub struct WidgetMarkup {
    #[serde(rename = "buttons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of buttons. Buttons is also oneof data and only one of these fields should be set."]
    pub buttons: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Button>>>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display an image in this widget."]
    pub image: ::std::option::Option<::std::boxed::Box<Image>>,
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display a key value item in this widget."]
    pub key_value: ::std::option::Option<::std::boxed::Box<KeyValue>>,
    #[serde(rename = "textParagraph")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display a text paragraph in this widget."]
    pub text_paragraph: ::std::option::Option<::std::boxed::Box<TextParagraph>>,
}
