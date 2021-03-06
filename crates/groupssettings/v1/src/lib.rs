#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
pub struct QueryParameters {
    #[builder(default = "{ query_parameters_defaults :: alt () }", setter(into))]
    #[serde(rename = "alt")]
    #[serde(default = "query_parameters_defaults :: alt")]
    #[doc = "Data format for the response."]
    pub alt: QueryParametersAltEnum,
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
    #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
    pub quota_user: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "userIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use quotaUser instead."]
    pub user_ip: ::std::option::Option<::std::string::String>,
}
impl QueryParameters {
    pub fn builder() -> QueryParametersBuilder {
        QueryParametersBuilder::default()
    }
}
mod query_parameters_defaults {
    pub fn alt() -> super::QueryParametersAltEnum {
        serde_json::from_str(&"\"atom\"").unwrap()
    }
    pub fn pretty_print() -> ::std::primitive::bool {
        serde_json::from_str(&"true").unwrap()
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Data format for the response."]
pub enum QueryParametersAltEnum {
    #[serde(rename = "atom")]
    #[doc = "Responses with Content-Type of application/atom+xml"]
    Atom,
    #[serde(rename = "json")]
    #[doc = "Responses with Content-Type of application/json"]
    Json,
}
impl ::std::default::Default for QueryParametersAltEnum {
    fn default() -> Self {
        Self::Atom
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for Group resource"]
    pub struct Groups {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowExternalMembers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies whether members external to your organization can join the group. Possible values are:  \n- true: G Suite users external to your organization can become members of this group. \n- false: Users not belonging to the organization are not allowed to become members of this group."]
        pub allow_external_members: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowGoogleCommunication")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Allows Google to contact administrator of the group.  \n- true: Allow Google to contact managers of this group. Occasionally Google may send updates on the latest features, ask for input on new features, or ask for permission to highlight your group. \n- false: Google can not contact managers of this group."]
        pub allow_google_communication: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowWebPosting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allows posting from web. Possible values are:  \n- true: Allows any member to post to the group forum. \n- false: Members only use Gmail to communicate with the group."]
        pub allow_web_posting: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "archiveOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allows the group to be archived only. Possible values are:  \n- true: Group is archived and the group is inactive. New messages to this group are rejected. The older archived messages are browseable and searchable.  \n- If true, the whoCanPostMessage property is set to NONE_CAN_POST.  \n- If reverted from true to false, whoCanPostMessages is set to ALL_MANAGERS_CAN_POST.  \n- false: The group is active and can receive messages.  \n- When false, updating whoCanPostMessage to NONE_CAN_POST, results in an error."]
        pub archive_only: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customFooterText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set the content of custom footer text. The maximum number of characters is 1,000."]
        pub custom_footer_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customReplyTo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An email address used when replying to a message if the replyTo property is set to REPLY_TO_CUSTOM. This address is defined by an account administrator.  \n- When the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property holds a custom email address used when replying to a message. \n- If the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property must have a text value or an error is returned."]
        pub custom_reply_to: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customRolesEnabledForSettingsToBeMerged")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies whether the group has a custom role that's included in one of the settings being merged. This field is read-only and update/patch requests to it are ignored. Possible values are:  \n- true \n- false"]
        pub custom_roles_enabled_for_settings_to_be_merged:
            ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultMessageDenyNotificationText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When a message is rejected, this is text for the rejection notification sent to the message's author. By default, this property is empty and has no value in the API's response body. The maximum notification text size is 10,000 characters. Note: Requires sendMessageDenyNotification property to be true."]
        pub default_message_deny_notification_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the group. This property value may be an empty string if no group description has been entered. If entered, the maximum group description is no more than 300 characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The group's email address. This property can be updated using the Directory API. Note: Only a group owner can change a group's email address. A group manager can't do this.\nWhen you change your group's address using the Directory API or the control panel, you are changing the address your subscribers use to send email and the web address people use to access your group. People can't reach your group by visiting the old address."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableCollaborativeInbox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies whether a collaborative inbox will remain turned on for the group. Possible values are:  \n- true \n- false"]
        pub enable_collaborative_inbox: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "favoriteRepliesOnTop")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if favorite replies should be displayed above other replies.  \n- true: Favorite replies will be displayed above other replies. \n- false: Favorite replies will not be displayed above other replies."]
        pub favorite_replies_on_top: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeCustomFooter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to include custom footer. Possible values are:  \n- true \n- false"]
        pub include_custom_footer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeInGlobalAddressList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enables the group to be included in the Global Address List. For more information, see the help center. Possible values are:  \n- true: Group is included in the Global Address List. \n- false: Group is not included in the Global Address List."]
        pub include_in_global_address_list: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isArchived")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allows the Group contents to be archived. Possible values are:  \n- true: Archive messages sent to the group. \n- false: Do not keep an archive of messages sent to this group. If false, previously archived messages remain in the archive."]
        pub is_archived: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ groups_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "groups_defaults :: kind")]
        #[doc = "The type of the resource. It is always groupsSettings#groups."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxMessageBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The maximum size of a message is 25Mb."]
        pub max_message_bytes: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "membersCanPostAsTheGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enables members to post messages as the group. Possible values are:  \n- true: Group member can post messages using the group's email address instead of their own email address. Message appear to originate from the group itself. Note: When true, any message moderation settings on individual users or new members do not apply to posts made on behalf of the group. \n- false: Members can not post in behalf of the group's email address."]
        pub members_can_post_as_the_group: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageDisplayFont")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The default message display font always has a value of \"DEFAULT_FONT\"."]
        pub message_display_font: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageModerationLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Moderation level of incoming messages. Possible values are:  \n- MODERATE_ALL_MESSAGES: All messages are sent to the group owner's email address for approval. If approved, the message is sent to the group. \n- MODERATE_NON_MEMBERS: All messages from non group members are sent to the group owner's email address for approval. If approved, the message is sent to the group. \n- MODERATE_NEW_MEMBERS: All messages from new members are sent to the group owner's email address for approval. If approved, the message is sent to the group. \n- MODERATE_NONE: No moderator approval is required. Messages are delivered directly to the group. Note: When the whoCanPostMessage is set to ANYONE_CAN_POST, we recommend the messageModerationLevel be set to MODERATE_NON_MEMBERS to protect the group from possible spam.\nWhen memberCanPostAsTheGroup is true, any message moderation settings on individual users or new members will not apply to posts made on behalf of the group."]
        pub message_moderation_level: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the group, which has a maximum size of 75 characters."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The primary language for group. For a group's primary language use the language tags from the G Suite languages found at G Suite Email Settings API Email Language Tags."]
        pub primary_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replyTo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies who receives the default reply. Possible values are:  \n- REPLY_TO_CUSTOM: For replies to messages, use the group's custom email address.\nWhen the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property holds the custom email address used when replying to a message. If the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property must have a value. Otherwise an error is returned.\n \n- REPLY_TO_SENDER: The reply sent to author of message. \n- REPLY_TO_LIST: This reply message is sent to the group. \n- REPLY_TO_OWNER: The reply is sent to the owner(s) of the group. This does not include the group's managers. \n- REPLY_TO_IGNORE: Group users individually decide where the message reply is sent. \n- REPLY_TO_MANAGERS: This reply message is sent to the group's managers, which includes all managers and the group owner."]
        pub reply_to: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sendMessageDenyNotification")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allows a member to be notified if the member's message to the group is denied by the group owner. Possible values are:  \n- true: When a message is rejected, send the deny message notification to the message author.\nThe defaultMessageDenyNotificationText property is dependent on the sendMessageDenyNotification property being true.\n \n- false: When a message is rejected, no notification is sent."]
        pub send_message_deny_notification: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "showInGroupDirectory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanDiscoverGroup setting. Allows the group to be visible in the Groups Directory. Possible values are:  \n- true: All groups in the account are listed in the Groups directory. \n- false: All groups in the account are not listed in the directory."]
        pub show_in_group_directory: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spamModerationLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies moderation levels for messages detected as spam. Possible values are:  \n- ALLOW: Post the message to the group. \n- MODERATE: Send the message to the moderation queue. This is the default. \n- SILENTLY_MODERATE: Send the message to the moderation queue, but do not send notification to moderators. \n- REJECT: Immediately reject the message."]
        pub spam_moderation_level: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanAdd")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanModerateMembers setting. Permissions to add members. Possible values are:  \n- ALL_MEMBERS_CAN_ADD: Managers and members can directly add new members. \n- ALL_MANAGERS_CAN_ADD: Only managers can directly add new members. this includes the group's owner. \n- ALL_OWNERS_CAN_ADD: Only owners can directly add new members. \n- NONE_CAN_ADD: No one can directly add new members."]
        pub who_can_add: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanAddReferences")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This functionality is no longer supported in the Google Groups UI. The value is always \"NONE\"."]
        pub who_can_add_references: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanApproveMembers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies who can approve members who ask to join groups. This permission will be deprecated once it is merged into the new whoCanModerateMembers setting. Possible values are:  \n- ALL_MEMBERS_CAN_APPROVE \n- ALL_MANAGERS_CAN_APPROVE \n- ALL_OWNERS_CAN_APPROVE \n- NONE_CAN_APPROVE"]
        pub who_can_approve_members: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanApproveMessages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can approve pending messages in the moderation queue. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- OWNERS_ONLY \n- NONE"]
        pub who_can_approve_messages: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanAssignTopics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to assign topics in a forum to another user. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- MANAGERS_ONLY \n- OWNERS_ONLY \n- NONE"]
        pub who_can_assign_topics: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanAssistContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies who can moderate metadata. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- MANAGERS_ONLY \n- OWNERS_ONLY \n- NONE"]
        pub who_can_assist_content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanBanUsers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies who can deny membership to users. This permission will be deprecated once it is merged into the new whoCanModerateMembers setting. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- OWNERS_ONLY \n- NONE"]
        pub who_can_ban_users: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanContactOwner")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Permission to contact owner of the group via web UI. Possible values are:  \n- ALL_IN_DOMAIN_CAN_CONTACT \n- ALL_MANAGERS_CAN_CONTACT \n- ALL_MEMBERS_CAN_CONTACT \n- ANYONE_CAN_CONTACT \n- ALL_OWNERS_CAN_CONTACT"]
        pub who_can_contact_owner: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanDeleteAnyPost")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can delete replies to topics. (Authors can always delete their own posts). Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- OWNERS_ONLY \n- NONE"]
        pub who_can_delete_any_post: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanDeleteTopics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can delete topics. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- OWNERS_ONLY \n- NONE"]
        pub who_can_delete_topics: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanDiscoverGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the set of users for whom this group is discoverable. Possible values are:  \n- ANYONE_CAN_DISCOVER \n- ALL_IN_DOMAIN_CAN_DISCOVER \n- ALL_MEMBERS_CAN_DISCOVER"]
        pub who_can_discover_group: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanEnterFreeFormTags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to enter free form tags for topics in a forum. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- MANAGERS_ONLY \n- OWNERS_ONLY \n- NONE"]
        pub who_can_enter_free_form_tags: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanHideAbuse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can hide posts by reporting them as abuse. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- OWNERS_ONLY \n- NONE"]
        pub who_can_hide_abuse: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanInvite")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanModerateMembers setting. Permissions to invite new members. Possible values are:  \n- ALL_MEMBERS_CAN_INVITE: Managers and members can invite a new member candidate. \n- ALL_MANAGERS_CAN_INVITE: Only managers can invite a new member. This includes the group's owner. \n- ALL_OWNERS_CAN_INVITE: Only owners can invite a new member. \n- NONE_CAN_INVITE: No one can invite a new member candidate."]
        pub who_can_invite: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanJoin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Permission to join group. Possible values are:  \n- ANYONE_CAN_JOIN: Anyone in the account domain can join. This includes accounts with multiple domains. \n- ALL_IN_DOMAIN_CAN_JOIN: Any Internet user who is outside your domain can access your Google Groups service and view the list of groups in your Groups directory. Warning: Group owners can add external addresses, outside of the domain to their groups. They can also allow people outside your domain to join their groups. If you later disable this option, any external addresses already added to users' groups remain in those groups. \n- INVITED_CAN_JOIN: Candidates for membership can be invited to join.  \n- CAN_REQUEST_TO_JOIN: Non members can request an invitation to join."]
        pub who_can_join: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanLeaveGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Permission to leave the group. Possible values are:  \n- ALL_MANAGERS_CAN_LEAVE \n- ALL_MEMBERS_CAN_LEAVE \n- NONE_CAN_LEAVE"]
        pub who_can_leave_group: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanLockTopics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can prevent users from posting replies to topics. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- OWNERS_ONLY \n- NONE"]
        pub who_can_lock_topics: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanMakeTopicsSticky")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can make topics appear at the top of the topic list. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- OWNERS_ONLY \n- NONE"]
        pub who_can_make_topics_sticky: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanMarkDuplicate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a topic as a duplicate of another topic. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- MANAGERS_ONLY \n- OWNERS_ONLY \n- NONE"]
        pub who_can_mark_duplicate: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanMarkFavoriteReplyOnAnyTopic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark any other user's post as a favorite reply. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- MANAGERS_ONLY \n- OWNERS_ONLY \n- NONE"]
        pub who_can_mark_favorite_reply_on_any_topic: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanMarkFavoriteReplyOnOwnTopic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a post for a topic they started as a favorite reply. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- MANAGERS_ONLY \n- OWNERS_ONLY \n- NONE"]
        pub who_can_mark_favorite_reply_on_own_topic: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanMarkNoResponseNeeded")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a topic as not needing a response. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- MANAGERS_ONLY \n- OWNERS_ONLY \n- NONE"]
        pub who_can_mark_no_response_needed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanModerateContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies who can moderate content. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- OWNERS_ONLY \n- NONE"]
        pub who_can_moderate_content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanModerateMembers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies who can manage members. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- OWNERS_ONLY \n- NONE"]
        pub who_can_moderate_members: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanModifyMembers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanModerateMembers setting. Specifies who can change group members' roles. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- OWNERS_ONLY \n- NONE"]
        pub who_can_modify_members: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanModifyTagsAndCategories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to change tags and categories. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- MANAGERS_ONLY \n- OWNERS_ONLY \n- NONE"]
        pub who_can_modify_tags_and_categories: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanMoveTopicsIn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can move topics into the group or forum. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- OWNERS_ONLY \n- NONE"]
        pub who_can_move_topics_in: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanMoveTopicsOut")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can move topics out of the group or forum. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- OWNERS_ONLY \n- NONE"]
        pub who_can_move_topics_out: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanPostAnnouncements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can post announcements, a special topic type. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- OWNERS_ONLY \n- NONE"]
        pub who_can_post_announcements: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanPostMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Permissions to post messages. Possible values are:  \n- NONE_CAN_POST: The group is disabled and archived. No one can post a message to this group.  \n- When archiveOnly is false, updating whoCanPostMessage to NONE_CAN_POST, results in an error. \n- If archiveOnly is reverted from true to false, whoCanPostMessages is set to ALL_MANAGERS_CAN_POST.  \n- ALL_MANAGERS_CAN_POST: Managers, including group owners, can post messages. \n- ALL_MEMBERS_CAN_POST: Any group member can post a message. \n- ALL_OWNERS_CAN_POST: Only group owners can post a message. \n- ALL_IN_DOMAIN_CAN_POST: Anyone in the account can post a message.  \n- ANYONE_CAN_POST: Any Internet user who outside your account can access your Google Groups service and post a message. Note: When whoCanPostMessage is set to ANYONE_CAN_POST, we recommend the messageModerationLevel be set to MODERATE_NON_MEMBERS to protect the group from possible spam."]
        pub who_can_post_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanTakeTopics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to take topics in a forum. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- MANAGERS_ONLY \n- OWNERS_ONLY \n- NONE"]
        pub who_can_take_topics: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanUnassignTopic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to unassign any topic in a forum. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- MANAGERS_ONLY \n- OWNERS_ONLY \n- NONE"]
        pub who_can_unassign_topic: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanUnmarkFavoriteReplyOnAnyTopic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to unmark any post from a favorite reply. Possible values are:  \n- ALL_MEMBERS \n- OWNERS_AND_MANAGERS \n- MANAGERS_ONLY \n- OWNERS_ONLY \n- NONE"]
        pub who_can_unmark_favorite_reply_on_any_topic:
            ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanViewGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Permissions to view group messages. Possible values are:  \n- ANYONE_CAN_VIEW: Any Internet user can view the group's messages.  \n- ALL_IN_DOMAIN_CAN_VIEW: Anyone in your account can view this group's messages. \n- ALL_MEMBERS_CAN_VIEW: All group members can view the group's messages. \n- ALL_MANAGERS_CAN_VIEW: Any group manager can view this group's messages."]
        pub who_can_view_group: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whoCanViewMembership")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Permissions to view membership. Possible values are:  \n- ALL_IN_DOMAIN_CAN_VIEW: Anyone in the account can view the group members list.\nIf a group already has external members, those members can still send email to this group.\n \n- ALL_MEMBERS_CAN_VIEW: The group members can view the group members list. \n- ALL_MANAGERS_CAN_VIEW: The group managers can view group members list."]
        pub who_can_view_membership: ::std::option::Option<::std::string::String>,
    }
    impl Groups {
        pub fn builder() -> GroupsBuilder {
            GroupsBuilder::default()
        }
    }
    mod groups_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"groupsSettings#groups\"").unwrap()
        }
    }
}
