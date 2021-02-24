#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's physical address. May be a P.O. box or street address. All fields are optional."]
pub struct Address {
    #[serde(rename = "city")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The city of the address."]
    pub city: ::std::option::Option<::std::string::String>,
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The country of the address."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The [ISO 3166-1 alpha-2](http://www.iso.org/iso/country_codes.htm) country code of the address."]
    pub country_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "extendedAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The extended address of the address; for example, the apartment number."]
    pub extended_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formattedType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of the address translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale."]
    pub formatted_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formattedValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unstructured value of the address. If this is not set by the user it will be automatically constructed from structured values."]
    pub formatted_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the address."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "poBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The P.O. box of the address."]
    pub po_box: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The postal code of the address."]
    pub postal_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The region of the address; for example, the state or province."]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "streetAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The street address."]
    pub street_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the address. The type can be custom or one of these predefined values: * `home` * `work` * `other`"]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's age range."]
pub struct AgeRangeType {
    #[serde(rename = "ageRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The age range."]
    pub age_range: ::std::option::Option<AgeRangeTypeAgeRangeEnum>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the age range."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The age range."]
pub enum AgeRangeTypeAgeRangeEnum {
    #[serde(rename = "AGE_RANGE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    AgeRangeUnspecified,
    #[serde(rename = "LESS_THAN_EIGHTEEN")]
    #[doc = "Younger than eighteen."]
    LessThanEighteen,
    #[serde(rename = "EIGHTEEN_TO_TWENTY")]
    #[doc = "Between eighteen and twenty."]
    EighteenToTwenty,
    #[serde(rename = "TWENTY_ONE_OR_OLDER")]
    #[doc = "Twenty-one and older."]
    TwentyOneOrOlder,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response to a batch get contact groups request."]
pub struct BatchGetContactGroupsResponse {
    #[serde(rename = "responses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of responses for each requested contact group resource."]
    pub responses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContactGroupResponse>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's short biography."]
pub struct Biography {
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content type of the biography."]
    pub content_type: ::std::option::Option<BiographyContentTypeEnum>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the biography."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The short biography."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The content type of the biography."]
pub enum BiographyContentTypeEnum {
    #[serde(rename = "CONTENT_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    ContentTypeUnspecified,
    #[serde(rename = "TEXT_PLAIN")]
    #[doc = "Plain text."]
    TextPlain,
    #[serde(rename = "TEXT_HTML")]
    #[doc = "HTML text."]
    TextHtml,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's birthday. At least one of the `date` and `text` fields are specified. The `date` and `text` fields typically represent the same date, but are not guaranteed to."]
pub struct Birthday {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date of the birthday."]
    pub date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the birthday."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A free-form string representing the user's birthday."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "**DEPRECATED**: No data will be returned A person's bragging rights."]
pub struct BraggingRights {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the bragging rights."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bragging rights; for example, `climbed mount everest`."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's calendar URL."]
pub struct CalendarUrl {
    #[serde(rename = "formattedType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of the calendar URL translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale."]
    pub formatted_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the calendar URL."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the calendar URL. The type can be custom or one of these predefined values: * `home` * `freeBusy` * `work`"]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The calendar URL."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Arbitrary client data that is populated by clients. Duplicate keys and values are allowed."]
pub struct ClientData {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The client specified key of the client data."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the client data."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The client specified value of the client data."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A contact group."]
pub struct ContactGroup {
    #[serde(rename = "clientData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The group's client data."]
    pub client_data: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GroupClientData>>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the resource. Used for web cache validation."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formattedName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale for system groups names. Group names set by the owner are the same as name."]
    pub formatted_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "groupType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The contact group type."]
    pub group_type: ::std::option::Option<ContactGroupGroupTypeEnum>,
    #[serde(rename = "memberCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The total number of contacts in the group irrespective of max members in specified in the request."]
    pub member_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "memberResourceNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The list of contact person resource names that are members of the contact group. The field is only populated for GET requests and will only return as many members as `maxMembers` in the get request."]
    pub member_resource_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Metadata about the contact group."]
    pub metadata: ::std::option::Option<::std::boxed::Box<ContactGroupMetadata>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The contact group name set by the group owner or a system provided name for system groups."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name for the contact group, assigned by the server. An ASCII string, in the form of `contactGroups/{contact_group_id}`."]
    pub resource_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The contact group type."]
pub enum ContactGroupGroupTypeEnum {
    #[serde(rename = "GROUP_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    GroupTypeUnspecified,
    #[serde(rename = "USER_CONTACT_GROUP")]
    #[doc = "User defined contact group."]
    UserContactGroup,
    #[serde(rename = "SYSTEM_CONTACT_GROUP")]
    #[doc = "System defined contact group."]
    SystemContactGroup,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Google contact group membership."]
pub struct ContactGroupMembership {
    #[serde(rename = "contactGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The contact group ID for the contact group membership."]
    pub contact_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contactGroupResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name for the contact group, assigned by the server. An ASCII string, in the form of `contactGroups/{contact_group_id}`. Only contact_group_resource_name can be used for modifying memberships. Any contact group membership can be removed, but only user group or \"myContacts\" or \"starred\" system groups memberships can be added. A contact must always have at least one contact group membership."]
    pub contact_group_resource_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The metadata about a contact group."]
pub struct ContactGroupMetadata {
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. True if the contact group resource has been deleted. Populated only for [`ListContactGroups`](/people/api/rest/v1/contactgroups/list) requests that include a sync token."]
    pub deleted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the group was last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for a specific contact group."]
pub struct ContactGroupResponse {
    #[serde(rename = "contactGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The contact group."]
    pub contact_group: ::std::option::Option<::std::boxed::Box<ContactGroup>>,
    #[serde(rename = "requestedResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The original requested resource name."]
    pub requested_resource_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the response."]
    pub status: ::std::option::Option<::std::boxed::Box<Status>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to copy an \"Other contact\" to my contacts group."]
pub struct CopyOtherContactToMyContactsGroupRequest {
    #[serde(rename = "copyMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A field mask to restrict which fields are copied into the new contact. Valid values are: * emailAddresses * names * phoneNumbers"]
    pub copy_mask: ::std::option::Option<::std::string::String>,
    #[serde(rename = "readMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A field mask to restrict which fields on the person are returned. Multiple fields can be specified by separating them with commas. Defaults to the copy mask with metadata and membership fields if not set. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined"]
    pub read_mask: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set."]
    pub sources:
        ::std::option::Option<::std::vec::Vec<CopyOtherContactToMyContactsGroupRequestSourcesEnum>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum CopyOtherContactToMyContactsGroupRequestSourcesEnum {
    #[serde(rename = "READ_SOURCE_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    ReadSourceTypeUnspecified,
    #[serde(rename = "READ_SOURCE_TYPE_PROFILE")]
    #[doc = "Returns SourceType.ACCOUNT, SourceType.DOMAIN_PROFILE, and SourceType.PROFILE."]
    ReadSourceTypeProfile,
    #[serde(rename = "READ_SOURCE_TYPE_CONTACT")]
    #[doc = "Returns SourceType.CONTACT."]
    ReadSourceTypeContact,
    #[serde(rename = "READ_SOURCE_TYPE_DOMAIN_CONTACT")]
    #[doc = "Returns SourceType.DOMAIN_CONTACT."]
    ReadSourceTypeDomainContact,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's cover photo. A large image shown on the person's profile page that represents who they are or what they care about."]
pub struct CoverPhoto {
    #[serde(rename = "default")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if the cover photo is the default cover photo; false if the cover photo is a user-provided cover photo."]
    pub _default: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the cover photo."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the cover photo."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to create a new contact group."]
pub struct CreateContactGroupRequest {
    #[serde(rename = "contactGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The contact group to create."]
    pub contact_group: ::std::option::Option<::std::boxed::Box<ContactGroup>>,
    #[serde(rename = "readGroupFields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A field mask to restrict which fields on the group are returned. Defaults to `metadata`, `groupType`, and `name` if not set or set to empty. Valid fields are: * clientData * groupType * metadata * name"]
    pub read_group_fields: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."]
pub struct Date {
    #[serde(rename = "day")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub day: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "month")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub month: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "year")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub year: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for deleteing a contact's photo."]
pub struct DeleteContactPhotoResponse {
    #[serde(rename = "person")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The updated person, if person_fields is set in the DeleteContactPhotoRequest; otherwise this will be unset."]
    pub person: ::std::option::Option<::std::boxed::Box<Person>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A G Suite Domain membership."]
pub struct DomainMembership {
    #[serde(rename = "inViewerDomain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if the person is in the viewer's G Suite domain."]
    pub in_viewer_domain: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's email address."]
pub struct EmailAddress {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name of the email."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formattedType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of the email address translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale."]
    pub formatted_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the email address."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the email address. The type can be custom or one of these predefined values: * `home` * `work` * `other`"]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event related to the person."]
pub struct Event {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date of the event."]
    pub date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "formattedType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of the event translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale."]
    pub formatted_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the event."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the event. The type can be custom or one of these predefined values: * `anniversary` * `other`"]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An identifier from an external entity related to the person."]
pub struct ExternalId {
    #[serde(rename = "formattedType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of the event translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale."]
    pub formatted_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the external ID."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the external ID. The type can be custom or one of these predefined values: * `account` * `customer` * `loginId` * `network` * `organization`"]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of the external ID."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata about a field."]
pub struct FieldMetadata {
    #[serde(rename = "primary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if the field is the primary field; false if the field is a secondary field."]
    pub primary: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source of the field."]
    pub source: ::std::option::Option<::std::boxed::Box<Source>>,
    #[serde(rename = "verified")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. True if the field is verified; false if the field is unverified. A verified field is typically a name, email address, phone number, or website that has been confirmed to be owned by the person."]
    pub verified: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The name that should be used to sort the person in a list."]
pub struct FileAs {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the file-as."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The file-as value"]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's gender."]
pub struct Gender {
    #[serde(rename = "addressMeAs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of pronouns that should be used to address the person. The value can be custom or one of these predefined values: * `male` * `female` * `other`"]
    pub address_me_as: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formattedValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The value of the gender translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale. Unspecified or custom value are not localized."]
    pub formatted_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the gender."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The gender for the person. The gender can be custom or one of these predefined values: * `male` * `female` * `unspecified`"]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response to a get request for a list of people by resource name."]
pub struct GetPeopleResponse {
    #[serde(rename = "responses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The response for each requested resource name."]
    pub responses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PersonResponse>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Arbitrary client data that is populated by clients. Duplicate keys and values are allowed. LINT.IfChange(GroupClientData)"]
pub struct GroupClientData {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The client specified key of the client data."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The client specified value of the client data."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's instant messaging client."]
pub struct ImClient {
    #[serde(rename = "formattedProtocol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The protocol of the IM client formatted in the viewer's account locale or the `Accept-Language` HTTP header locale."]
    pub formatted_protocol: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formattedType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of the IM client translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale."]
    pub formatted_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the IM client."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The protocol of the IM client. The protocol can be custom or one of these predefined values: * `aim` * `msn` * `yahoo` * `skype` * `qq` * `googleTalk` * `icq` * `jabber` * `netMeeting`"]
    pub protocol: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the IM client. The type can be custom or one of these predefined values: * `home` * `work` * `other`"]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user name used in the IM client."]
    pub username: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "One of the person's interests."]
pub struct Interest {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the interest."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The interest; for example, `stargazing`."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response to a request for the authenticated user's connections."]
pub struct ListConnectionsResponse {
    #[serde(rename = "connections")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of people that the requestor is connected to."]
    pub connections: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Person>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextSyncToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `sync_token` to retrieve changes since the last request. Request must set `request_sync_token` to return the sync token. When the response is paginated, only the last page will contain `nextSyncToken`."]
    pub next_sync_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of items in the list without pagination."]
    pub total_items: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "totalPeople")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "**DEPRECATED** (Please use totalItems) The total number of people in the list without pagination."]
    pub total_people: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response to a list contact groups request."]
pub struct ListContactGroupsResponse {
    #[serde(rename = "contactGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of contact groups. Members of the contact groups are not populated."]
    pub contact_groups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContactGroup>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token that can be used to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextSyncToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token that can be used to retrieve changes since the last request."]
    pub next_sync_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of items in the list without pagination."]
    pub total_items: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response to a request for the authenticated user's domain directory."]
pub struct ListDirectoryPeopleResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextSyncToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `sync_token` to retrieve changes since the last request. Request must set `request_sync_token` to return the sync token."]
    pub next_sync_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "people")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of people in the domain directory."]
    pub people: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Person>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response to a request for the authenticated user's \"Other contacts\"."]
pub struct ListOtherContactsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextSyncToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `sync_token` to retrieve changes since the last request. Request must set `request_sync_token` to return the sync token."]
    pub next_sync_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "otherContacts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of \"Other contacts\" returned as Person resources. \"Other contacts\" support a limited subset of fields. See ListOtherContactsRequest.request_mask for more detailed information."]
    pub other_contacts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Person>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's locale preference."]
pub struct Locale {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the locale."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The well-formed [IETF BCP 47](https://tools.ietf.org/html/bcp47) language tag representing the locale."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's location."]
pub struct Location {
    #[serde(rename = "buildingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The building identifier."]
    pub building_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "current")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the location is the current location."]
    pub current: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "deskCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The individual desk location."]
    pub desk_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "floor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The floor name or number."]
    pub floor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "floorSection")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The floor section in `floor_name`."]
    pub floor_section: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the location."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the location. The type can be custom or one of these predefined values: * `desk` * `grewUp`"]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The free-form value of the location."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's membership in a group. Only contact group memberships can be modified."]
pub struct Membership {
    #[serde(rename = "contactGroupMembership")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The contact group membership."]
    pub contact_group_membership: ::std::option::Option<::std::boxed::Box<ContactGroupMembership>>,
    #[serde(rename = "domainMembership")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The domain membership."]
    pub domain_membership: ::std::option::Option<::std::boxed::Box<DomainMembership>>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the membership."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's miscellaneous keyword."]
pub struct MiscKeyword {
    #[serde(rename = "formattedType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of the miscellaneous keyword translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale."]
    pub formatted_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the miscellaneous keyword."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The miscellaneous keyword type."]
    pub _type: ::std::option::Option<MiscKeywordTypeEnum>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of the miscellaneous keyword."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The miscellaneous keyword type."]
pub enum MiscKeywordTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    TypeUnspecified,
    #[serde(rename = "OUTLOOK_BILLING_INFORMATION")]
    #[doc = "Outlook field for billing information."]
    OutlookBillingInformation,
    #[serde(rename = "OUTLOOK_DIRECTORY_SERVER")]
    #[doc = "Outlook field for directory server."]
    OutlookDirectoryServer,
    #[serde(rename = "OUTLOOK_KEYWORD")]
    #[doc = "Outlook field for keyword."]
    OutlookKeyword,
    #[serde(rename = "OUTLOOK_MILEAGE")]
    #[doc = "Outlook field for mileage."]
    OutlookMileage,
    #[serde(rename = "OUTLOOK_PRIORITY")]
    #[doc = "Outlook field for priority."]
    OutlookPriority,
    #[serde(rename = "OUTLOOK_SENSITIVITY")]
    #[doc = "Outlook field for sensitivity."]
    OutlookSensitivity,
    #[serde(rename = "OUTLOOK_SUBJECT")]
    #[doc = "Outlook field for subject."]
    OutlookSubject,
    #[serde(rename = "OUTLOOK_USER")]
    #[doc = "Outlook field for user."]
    OutlookUser,
    #[serde(rename = "HOME")]
    #[doc = "Home."]
    Home,
    #[serde(rename = "WORK")]
    #[doc = "Work."]
    Work,
    #[serde(rename = "OTHER")]
    #[doc = "Other."]
    Other,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to modify an existing contact group's members. Contacts can be removed from any group but they can only be added to a user group or \"myContacts\" or \"starred\" system groups."]
pub struct ModifyContactGroupMembersRequest {
    #[serde(rename = "resourceNamesToAdd")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The resource names of the contact people to add in the form of `people/{person_id}`. The total number of resource names in `resource_names_to_add` and `resource_names_to_remove` must be less than or equal to 1000."]
    pub resource_names_to_add: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "resourceNamesToRemove")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The resource names of the contact people to remove in the form of `people/{person_id}`. The total number of resource names in `resource_names_to_add` and `resource_names_to_remove` must be less than or equal to 1000."]
    pub resource_names_to_remove: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response to a modify contact group members request."]
pub struct ModifyContactGroupMembersResponse {
    #[serde(rename = "canNotRemoveLastContactGroupResourceNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The contact people resource names that cannot be removed from their last contact group."]
    pub can_not_remove_last_contact_group_resource_names:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "notFoundResourceNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The contact people resource names that were not found."]
    pub not_found_resource_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's name. If the name is a mononym, the family name is empty."]
pub struct Name {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name formatted according to the locale specified by the viewer's account or the `Accept-Language` HTTP header."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayNameLastFirst")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name with the last name first formatted according to the locale specified by the viewer's account or the `Accept-Language` HTTP header."]
    pub display_name_last_first: ::std::option::Option<::std::string::String>,
    #[serde(rename = "familyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The family name."]
    pub family_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The given name."]
    pub given_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "honorificPrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The honorific prefixes, such as `Mrs.` or `Dr.`"]
    pub honorific_prefix: ::std::option::Option<::std::string::String>,
    #[serde(rename = "honorificSuffix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The honorific suffixes, such as `Jr.`"]
    pub honorific_suffix: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the name."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "middleName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The middle name(s)."]
    pub middle_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "phoneticFamilyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The family name spelled as it sounds."]
    pub phonetic_family_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "phoneticFullName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full name spelled as it sounds."]
    pub phonetic_full_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "phoneticGivenName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The given name spelled as it sounds."]
    pub phonetic_given_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "phoneticHonorificPrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The honorific prefixes spelled as they sound."]
    pub phonetic_honorific_prefix: ::std::option::Option<::std::string::String>,
    #[serde(rename = "phoneticHonorificSuffix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The honorific suffixes spelled as they sound."]
    pub phonetic_honorific_suffix: ::std::option::Option<::std::string::String>,
    #[serde(rename = "phoneticMiddleName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The middle name(s) spelled as they sound."]
    pub phonetic_middle_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unstructuredName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The free form name value."]
    pub unstructured_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's nickname."]
pub struct Nickname {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the nickname."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the nickname."]
    pub _type: ::std::option::Option<NicknameTypeEnum>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The nickname."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the nickname."]
pub enum NicknameTypeEnum {
    #[serde(rename = "DEFAULT")]
    #[doc = "Generic nickname."]
    Default,
    #[serde(rename = "MAIDEN_NAME")]
    #[doc = "Maiden name or birth family name. Used when the person's family name has changed as a result of marriage."]
    MaidenName,
    #[serde(rename = "INITIALS")]
    #[doc = "Initials."]
    Initials,
    #[serde(rename = "GPLUS")]
    #[doc = "Google+ profile nickname."]
    Gplus,
    #[serde(rename = "OTHER_NAME")]
    #[doc = "A professional affiliation or other name; for example, `Dr. Smith.`"]
    OtherName,
    #[serde(rename = "ALTERNATE_NAME")]
    #[doc = "Alternate name person is known by."]
    AlternateName,
    #[serde(rename = "SHORT_NAME")]
    #[doc = "A shorter version of the person's name."]
    ShortName,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's occupation."]
pub struct Occupation {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the occupation."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The occupation; for example, `carpenter`."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's past or current organization. Overlapping date ranges are permitted."]
pub struct Organization {
    #[serde(rename = "current")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if the organization is the person's current organization; false if the organization is a past organization."]
    pub current: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "department")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's department at the organization."]
    pub department: ::std::option::Option<::std::string::String>,
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The domain name associated with the organization; for example, `google.com`."]
    pub domain: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end date when the person left the organization."]
    pub end_date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "formattedType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of the organization translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale."]
    pub formatted_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "jobDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's job description at the organization."]
    pub job_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location of the organization office the person works at."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the organization."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the organization."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "phoneticName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The phonetic name of the organization."]
    pub phonetic_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The start date when the person joined the organization."]
    pub start_date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "symbol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The symbol associated with the organization; for example, a stock ticker symbol, abbreviation, or acronym."]
    pub symbol: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's job title at the organization."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the organization. The type can be custom or one of these predefined values: * `work` * `school`"]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about a person merged from various data sources such as the authenticated user's contacts and profile data. Most fields can have multiple items. The items in a field have no guaranteed order, but each non-empty field is guaranteed to have exactly one field with `metadata.primary` set to true."]
pub struct Person {
    #[serde(rename = "addresses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's street addresses."]
    pub addresses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Address>>>,
    #[serde(rename = "ageRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. **DEPRECATED** (Please use `person.ageRanges` instead) The person's age range."]
    pub age_range: ::std::option::Option<PersonAgeRangeEnum>,
    #[serde(rename = "ageRanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The person's age ranges."]
    pub age_ranges: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AgeRangeType>>>,
    #[serde(rename = "biographies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's biographies. This field is a singleton for contact sources."]
    pub biographies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Biography>>>,
    #[serde(rename = "birthdays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's birthdays. This field is a singleton for contact sources."]
    pub birthdays: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Birthday>>>,
    #[serde(rename = "braggingRights")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "**DEPRECATED**: No data will be returned The person's bragging rights."]
    pub bragging_rights: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BraggingRights>>>,
    #[serde(rename = "calendarUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's calendar URLs."]
    pub calendar_urls: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CalendarUrl>>>,
    #[serde(rename = "clientData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's client data."]
    pub client_data: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ClientData>>>,
    #[serde(rename = "coverPhotos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The person's cover photos."]
    pub cover_photos: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CoverPhoto>>>,
    #[serde(rename = "emailAddresses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's email addresses."]
    pub email_addresses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EmailAddress>>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the resource. Used for web cache validation."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's events."]
    pub events: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Event>>>,
    #[serde(rename = "externalIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's external IDs."]
    pub external_ids: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ExternalId>>>,
    #[serde(rename = "fileAses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's file-ases."]
    pub file_ases: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FileAs>>>,
    #[serde(rename = "genders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's genders. This field is a singleton for contact sources."]
    pub genders: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Gender>>>,
    #[serde(rename = "imClients")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's instant messaging clients."]
    pub im_clients: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ImClient>>>,
    #[serde(rename = "interests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's interests."]
    pub interests: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Interest>>>,
    #[serde(rename = "locales")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's locale preferences."]
    pub locales: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Locale>>>,
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's locations."]
    pub locations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Location>>>,
    #[serde(rename = "memberships")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's group memberships."]
    pub memberships: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Membership>>>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Metadata about the person."]
    pub metadata: ::std::option::Option<::std::boxed::Box<PersonMetadata>>,
    #[serde(rename = "miscKeywords")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's miscellaneous keywords."]
    pub misc_keywords: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MiscKeyword>>>,
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's names. This field is a singleton for contact sources."]
    pub names: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Name>>>,
    #[serde(rename = "nicknames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's nicknames."]
    pub nicknames: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Nickname>>>,
    #[serde(rename = "occupations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's occupations."]
    pub occupations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Occupation>>>,
    #[serde(rename = "organizations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's past or current organizations."]
    pub organizations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Organization>>>,
    #[serde(rename = "phoneNumbers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's phone numbers."]
    pub phone_numbers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PhoneNumber>>>,
    #[serde(rename = "photos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The person's photos."]
    pub photos: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Photo>>>,
    #[serde(rename = "relations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's relations."]
    pub relations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Relation>>>,
    #[serde(rename = "relationshipInterests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. **DEPRECATED**: No data will be returned The person's relationship interests."]
    pub relationship_interests:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RelationshipInterest>>>,
    #[serde(rename = "relationshipStatuses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. **DEPRECATED**: No data will be returned The person's relationship statuses."]
    pub relationship_statuses:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RelationshipStatus>>>,
    #[serde(rename = "residences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "**DEPRECATED**: (Please use `person.locations` instead) The person's residences."]
    pub residences: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Residence>>>,
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name for the person, assigned by the server. An ASCII string with a max length of 27 characters, in the form of `people/{person_id}`."]
    pub resource_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sipAddresses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's SIP addresses."]
    pub sip_addresses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SipAddress>>>,
    #[serde(rename = "skills")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's skills."]
    pub skills: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Skill>>>,
    #[serde(rename = "taglines")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. **DEPRECATED**: No data will be returned The person's taglines."]
    pub taglines: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Tagline>>>,
    #[serde(rename = "urls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's associated URLs."]
    pub urls: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Url>>>,
    #[serde(rename = "userDefined")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's user defined data."]
    pub user_defined: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserDefined>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. **DEPRECATED** (Please use `person.ageRanges` instead) The person's age range."]
pub enum PersonAgeRangeEnum {
    #[serde(rename = "AGE_RANGE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    AgeRangeUnspecified,
    #[serde(rename = "LESS_THAN_EIGHTEEN")]
    #[doc = "Younger than eighteen."]
    LessThanEighteen,
    #[serde(rename = "EIGHTEEN_TO_TWENTY")]
    #[doc = "Between eighteen and twenty."]
    EighteenToTwenty,
    #[serde(rename = "TWENTY_ONE_OR_OLDER")]
    #[doc = "Twenty-one and older."]
    TwentyOneOrOlder,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The metadata about a person."]
pub struct PersonMetadata {
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. True if the person resource has been deleted. Populated only for [`connections.list`](/people/api/rest/v1/people.connections/list) requests that include a sync token."]
    pub deleted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "linkedPeopleResourceNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Resource names of people linked to this resource."]
    pub linked_people_resource_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "objectType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. **DEPRECATED** (Please use `person.metadata.sources.profileMetadata.objectType` instead) The type of the person object."]
    pub object_type: ::std::option::Option<PersonMetadataObjectTypeEnum>,
    #[serde(rename = "previousResourceNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Any former resource names this person has had. Populated only for [`connections.list`](/people/api/rest/v1/people.connections/list) requests that include a sync token. The resource name may change when adding or removing fields that link a contact and profile such as a verified email, verified phone number, or profile URL."]
    pub previous_resource_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sources of data for the person."]
    pub sources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Source>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. **DEPRECATED** (Please use `person.metadata.sources.profileMetadata.objectType` instead) The type of the person object."]
pub enum PersonMetadataObjectTypeEnum {
    #[serde(rename = "OBJECT_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    ObjectTypeUnspecified,
    #[serde(rename = "PERSON")]
    #[doc = "Person."]
    Person,
    #[serde(rename = "PAGE")]
    #[doc = "[Currents Page.](https://gsuite.google.com/products/currents/)"]
    Page,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for a single person"]
pub struct PersonResponse {
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "**DEPRECATED** (Please use status instead) [HTTP 1.1 status code] (http://www.w3.org/Protocols/rfc2616/rfc2616-sec10.html)."]
    pub http_status_code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "person")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person."]
    pub person: ::std::option::Option<::std::boxed::Box<Person>>,
    #[serde(rename = "requestedResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The original requested resource name. May be different than the resource name on the returned person. The resource name can change when adding or removing fields that link a contact and profile such as a verified email, verified phone number, or a profile URL."]
    pub requested_resource_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the response."]
    pub status: ::std::option::Option<::std::boxed::Box<Status>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's phone number."]
pub struct PhoneNumber {
    #[serde(rename = "canonicalForm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The canonicalized [ITU-T E.164](https://law.resource.org/pub/us/cfr/ibr/004/itu-t.E.164.1.2008.pdf) form of the phone number."]
    pub canonical_form: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formattedType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of the phone number translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale."]
    pub formatted_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the phone number."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the phone number. The type can be custom or one of these predefined values: * `home` * `work` * `mobile` * `homeFax` * `workFax` * `otherFax` * `pager` * `workMobile` * `workPager` * `main` * `googleVoice` * `other`"]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The phone number."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's photo. A picture shown next to the person's name to help others recognize the person."]
pub struct Photo {
    #[serde(rename = "default")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if the photo is a default photo; false if the photo is a user-provided photo."]
    pub _default: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the photo."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the photo. You can change the desired size by appending a query parameter `sz={size}` at the end of the url, where {size} is the size in pixels. Example: https://lh3.googleusercontent.com/-T_wVWLlmg7w/AAAAAAAAAAI/AAAAAAAABa8/00gzXvDBYqw/s100/photo.jpg?sz=50"]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The metadata about a profile."]
pub struct ProfileMetadata {
    #[serde(rename = "objectType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The profile object type."]
    pub object_type: ::std::option::Option<ProfileMetadataObjectTypeEnum>,
    #[serde(rename = "userTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The user types."]
    pub user_types: ::std::option::Option<::std::vec::Vec<ProfileMetadataUserTypesEnum>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The profile object type."]
pub enum ProfileMetadataObjectTypeEnum {
    #[serde(rename = "OBJECT_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    ObjectTypeUnspecified,
    #[serde(rename = "PERSON")]
    #[doc = "Person."]
    Person,
    #[serde(rename = "PAGE")]
    #[doc = "[Currents Page.](https://gsuite.google.com/products/currents/)"]
    Page,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ProfileMetadataUserTypesEnum {
    #[serde(rename = "USER_TYPE_UNKNOWN")]
    #[doc = "The user type is not known."]
    UserTypeUnknown,
    #[serde(rename = "GOOGLE_USER")]
    #[doc = "The user is a Google user."]
    GoogleUser,
    #[serde(rename = "GPLUS_USER")]
    #[doc = "The user is a Currents user."]
    GplusUser,
    #[serde(rename = "GOOGLE_APPS_USER")]
    #[doc = "The user is a G Suite user."]
    GoogleAppsUser,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's relation to another person."]
pub struct Relation {
    #[serde(rename = "formattedType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of the relation translated and formatted in the viewer's account locale or the locale specified in the Accept-Language HTTP header."]
    pub formatted_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the relation."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "person")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the other person this relation refers to."]
    pub person: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's relation to the other person. The type can be custom or one of these predefined values: * `spouse` * `child` * `mother` * `father` * `parent` * `brother` * `sister` * `friend` * `relative` * `domesticPartner` * `manager` * `assistant` * `referredBy` * `partner`"]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "**DEPRECATED**: No data will be returned A person's relationship interest ."]
pub struct RelationshipInterest {
    #[serde(rename = "formattedValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The value of the relationship interest translated and formatted in the viewer's account locale or the locale specified in the Accept-Language HTTP header."]
    pub formatted_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the relationship interest."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of relationship the person is looking for. The value can be custom or one of these predefined values: * `friend` * `date` * `relationship` * `networking`"]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "**DEPRECATED**: No data will be returned A person's relationship status."]
pub struct RelationshipStatus {
    #[serde(rename = "formattedValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The value of the relationship status translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale."]
    pub formatted_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the relationship status."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relationship status. The value can be custom or one of these predefined values: * `single` * `inARelationship` * `engaged` * `married` * `itsComplicated` * `openRelationship` * `widowed` * `inDomesticPartnership` * `inCivilUnion`"]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "**DEPRECATED**: Please use `person.locations` instead. A person's past or current residence."]
pub struct Residence {
    #[serde(rename = "current")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if the residence is the person's current residence; false if the residence is a past residence."]
    pub current: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the residence."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The address of the residence."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response to a request for people in the authenticated user's domain directory that match the specified query."]
pub struct SearchDirectoryPeopleResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "people")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of people in the domain directory that match the query."]
    pub people: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Person>>>,
    #[serde(rename = "totalSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of items in the list without pagination."]
    pub total_size: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response to a search request for the authenticated user, given a query."]
pub struct SearchResponse {
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The results of the request."]
    pub results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SearchResult>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A result of a search query."]
pub struct SearchResult {
    #[serde(rename = "person")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The matched Person."]
    pub person: ::std::option::Option<::std::boxed::Box<Person>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's SIP address. Session Initial Protocol addresses are used for VoIP communications to make voice or video calls over the internet."]
pub struct SipAddress {
    #[serde(rename = "formattedType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of the SIP address translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale."]
    pub formatted_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the SIP address."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the SIP address. The type can be custom or or one of these predefined values: * `home` * `work` * `mobile` * `other`"]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The SIP address in the [RFC 3261 19.1](https://tools.ietf.org/html/rfc3261#section-19.1) SIP URI format."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A skill that the person has."]
pub struct Skill {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the skill."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The skill; for example, `underwater basket weaving`."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The source of a field."]
pub struct Source {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "**Only populated in `person.metadata.sources`.** The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the source. Used for web cache validation."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier within the source type generated by the server."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "profileMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. **Only populated in `person.metadata.sources`.** Metadata about a source of type PROFILE."]
    pub profile_metadata: ::std::option::Option<::std::boxed::Box<ProfileMetadata>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source type."]
    pub _type: ::std::option::Option<SourceTypeEnum>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. **Only populated in `person.metadata.sources`.** Last update timestamp of this source."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The source type."]
pub enum SourceTypeEnum {
    #[serde(rename = "SOURCE_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    SourceTypeUnspecified,
    #[serde(rename = "ACCOUNT")]
    #[doc = "[Google Account](https://accounts.google.com)."]
    Account,
    #[serde(rename = "PROFILE")]
    #[doc = "[Google profile](https://profiles.google.com). You can view the profile at [https://profiles.google.com/](https://profiles.google.com/){id}, where {id} is the source id."]
    Profile,
    #[serde(rename = "DOMAIN_PROFILE")]
    #[doc = "[G Suite domain profile](https://support.google.com/a/answer/1628008)."]
    DomainProfile,
    #[serde(rename = "CONTACT")]
    #[doc = "[Google contact](https://contacts.google.com). You can view the contact at [https://contact.google.com/](https://contact.google.com/){id}, where {id} is the source id."]
    Contact,
    #[serde(rename = "OTHER_CONTACT")]
    #[doc = "[Google \"Other contact\"](https://contacts.google.com/other)."]
    OtherContact,
    #[serde(rename = "DOMAIN_CONTACT")]
    #[doc = "[G Suite domain shared contact](https://support.google.com/a/answer/9281635)."]
    DomainContact,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
pub struct Status {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status code, which should be an enum value of google.rpc.Code."]
    pub code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
    pub details: ::std::option::Option<
        ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    >,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "**DEPRECATED**: No data will be returned A brief one-line description of the person."]
pub struct Tagline {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the tagline."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tagline."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to update an existing user contact group. All updated fields will be replaced."]
pub struct UpdateContactGroupRequest {
    #[serde(rename = "contactGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The contact group to update."]
    pub contact_group: ::std::option::Option<::std::boxed::Box<ContactGroup>>,
    #[serde(rename = "readGroupFields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A field mask to restrict which fields on the group are returned. Defaults to `metadata`, `groupType`, and `name` if not set or set to empty. Valid fields are: * clientData * groupType * memberCount * metadata * name"]
    pub read_group_fields: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateGroupFields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A field mask to restrict which fields on the group are updated. Multiple fields can be specified by separating them with commas. Defaults to `name` if not set or set to empty. Updated fields are replaced. Valid values are: * clientData * name"]
    pub update_group_fields: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to update an existing contact's photo. All requests must have a valid photo format: JPEG or PNG."]
pub struct UpdateContactPhotoRequest {
    #[serde(rename = "personFields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A field mask to restrict which fields on the person are returned. Multiple fields can be specified by separating them with commas. Defaults to empty if not set, which will skip the post mutate get. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined"]
    pub person_fields: ::std::option::Option<::std::string::String>,
    #[serde(rename = "photoBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Raw photo bytes"]
    pub photo_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set."]
    pub sources: ::std::option::Option<::std::vec::Vec<UpdateContactPhotoRequestSourcesEnum>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum UpdateContactPhotoRequestSourcesEnum {
    #[serde(rename = "READ_SOURCE_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    ReadSourceTypeUnspecified,
    #[serde(rename = "READ_SOURCE_TYPE_PROFILE")]
    #[doc = "Returns SourceType.ACCOUNT, SourceType.DOMAIN_PROFILE, and SourceType.PROFILE."]
    ReadSourceTypeProfile,
    #[serde(rename = "READ_SOURCE_TYPE_CONTACT")]
    #[doc = "Returns SourceType.CONTACT."]
    ReadSourceTypeContact,
    #[serde(rename = "READ_SOURCE_TYPE_DOMAIN_CONTACT")]
    #[doc = "Returns SourceType.DOMAIN_CONTACT."]
    ReadSourceTypeDomainContact,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for updating a contact's photo."]
pub struct UpdateContactPhotoResponse {
    #[serde(rename = "person")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The updated person, if person_fields is set in the UpdateContactPhotoRequest; otherwise this will be unset."]
    pub person: ::std::option::Option<::std::boxed::Box<Person>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's associated URLs."]
pub struct Url {
    #[serde(rename = "formattedType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of the URL translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale."]
    pub formatted_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the URL."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the URL. The type can be custom or one of these predefined values: * `home` * `work` * `blog` * `profile` * `homePage` * `ftp` * `reservations` * `appInstallPage`: website for a Currents application. * `other`"]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Arbitrary user data that is populated by the end users."]
pub struct UserDefined {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end user specified key of the user defined data."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the user defined data."]
    pub metadata: ::std::option::Option<::std::boxed::Box<FieldMetadata>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end user specified value of the user defined data."]
    pub value: ::std::option::Option<::std::string::String>,
}
