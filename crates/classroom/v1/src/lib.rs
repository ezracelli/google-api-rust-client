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
    pub mod courses {
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
                    #[serde(rename = "courseStates")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restricts returned courses to those in one of the specified states The default value is ACTIVE, ARCHIVED, PROVISIONED, DECLINED."]
                    pub course_states: ::std::option::Option<QueryParametersCourseStatesEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of items to return. Zero or unspecified indicates that the server may assign a maximum. The server may return fewer than the specified number of results."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "nextPageToken value returned from a previous list call, indicating that the subsequent page of results should be returned. The list request must be otherwise identical to the one that resulted in this token."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "studentId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restricts returned courses to those having a student with the specified identifier. The identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `\"me\"`, indicating the requesting user"]
                    pub student_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "teacherId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restricts returned courses to those having a teacher with the specified identifier. The identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `\"me\"`, indicating the requesting user"]
                    pub teacher_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Restricts returned courses to those in one of the specified states The default value is ACTIVE, ARCHIVED, PROVISIONED, DECLINED."]
                pub enum QueryParametersCourseStatesEnum {
                    #[serde(rename = "COURSE_STATE_UNSPECIFIED")]
                    #[doc = "No course state. No returned Course message will use this value."]
                    CourseStateUnspecified,
                    #[serde(rename = "ACTIVE")]
                    #[doc = "The course is active."]
                    Active,
                    #[serde(rename = "ARCHIVED")]
                    #[doc = "The course has been archived. You cannot modify it except to change it to a different state."]
                    Archived,
                    #[serde(rename = "PROVISIONED")]
                    #[doc = "The course has been created, but not yet activated. It is accessible by the primary teacher and domain administrators, who may modify it or change it to the `ACTIVE` or `DECLINED` states. A course may only be changed to `PROVISIONED` if it is in the `DECLINED` state."]
                    Provisioned,
                    #[serde(rename = "DECLINED")]
                    #[doc = "The course has been created, but declined. It is accessible by the course owner and domain administrators, though it will not be displayed in the web UI. You cannot modify the course except to change it to the `PROVISIONED` state. A course may only be changed to `DECLINED` if it is in the `PROVISIONED` state."]
                    Declined,
                    #[serde(rename = "SUSPENDED")]
                    #[doc = "The course has been suspended. You cannot modify the course, and only the user identified by the `owner_id` can view the course. A course may be placed in this state if it potentially violates the Terms of Service."]
                    Suspended,
                }
                impl ::std::default::Default for QueryParametersCourseStatesEnum {
                    fn default() -> Self {
                        Self::CourseStateUnspecified
                    }
                }
            }
            pub mod patch {
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
                    #[serde(rename = "updateMask")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Mask that identifies which fields on the course to update. This field is required to do an update. The update will fail if invalid fields are specified. The following fields are valid: * `name` * `section` * `descriptionHeading` * `description` * `room` * `courseState` * `ownerId` Note: patches to ownerId are treated as being effective immediately, but in practice it may take some time for the ownership transfer of all affected resources to complete. When set in a query parameter, this field should be specified as `updateMask=,,...`"]
                    pub update_mask: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
        pub mod resources {
            pub mod aliases {
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of items to return. Zero or unspecified indicates that the server may assign a maximum. The server may return fewer than the specified number of results."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "nextPageToken value returned from a previous list call, indicating that the subsequent page of results should be returned. The list request must be otherwise identical to the one that resulted in this token."]
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
            pub mod announcements {
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "announcementStates")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Restriction on the `state` of announcements returned. If this argument is left unspecified, the default value is `PUBLISHED`."]
                            pub announcement_states:
                                ::std::option::Option<QueryParametersAnnouncementStatesEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional sort ordering for results. A comma-separated list of fields with an optional sort direction keyword. Supported field is `updateTime`. Supported direction keywords are `asc` and `desc`. If not specified, `updateTime desc` is the default behavior. Examples: `updateTime asc`, `updateTime`"]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of items to return. Zero or unspecified indicates that the server may assign a maximum. The server may return fewer than the specified number of results."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "nextPageToken value returned from a previous list call, indicating that the subsequent page of results should be returned. The list request must be otherwise identical to the one that resulted in this token."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Restriction on the `state` of announcements returned. If this argument is left unspecified, the default value is `PUBLISHED`."]
                        pub enum QueryParametersAnnouncementStatesEnum {
                            #[serde(rename = "ANNOUNCEMENT_STATE_UNSPECIFIED")]
                            #[doc = "No state specified. This is never returned."]
                            AnnouncementStateUnspecified,
                            #[serde(rename = "PUBLISHED")]
                            #[doc = "Status for announcement that has been published. This is the default state."]
                            Published,
                            #[serde(rename = "DRAFT")]
                            #[doc = "Status for an announcement that is not yet published. Announcement in this state is visible only to course teachers and domain administrators."]
                            Draft,
                            #[serde(rename = "DELETED")]
                            #[doc = "Status for announcement that was published but is now deleted. Announcement in this state is visible only to course teachers and domain administrators. Announcement in this state is deleted after some time."]
                            Deleted,
                        }
                        impl ::std::default::Default for QueryParametersAnnouncementStatesEnum {
                            fn default() -> Self {
                                Self::AnnouncementStateUnspecified
                            }
                        }
                    }
                    pub mod patch {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Mask that identifies which fields on the announcement to update. This field is required to do an update. The update fails if invalid fields are specified. If a field supports empty values, it can be cleared by specifying it in the update mask and not in the Announcement object. If a field that does not support empty values is included in the update mask and not set in the Announcement object, an `INVALID_ARGUMENT` error is returned. The following fields may be specified by teachers: * `text` * `state` * `scheduled_time`"]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod course_work {
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "courseWorkStates")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Restriction on the work status to return. Only courseWork that matches is returned. If unspecified, items with a work status of `PUBLISHED` is returned."]
                            pub course_work_states:
                                ::std::option::Option<QueryParametersCourseWorkStatesEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional sort ordering for results. A comma-separated list of fields with an optional sort direction keyword. Supported fields are `updateTime` and `dueDate`. Supported direction keywords are `asc` and `desc`. If not specified, `updateTime desc` is the default behavior. Examples: `dueDate asc,updateTime desc`, `updateTime,dueDate desc`"]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of items to return. Zero or unspecified indicates that the server may assign a maximum. The server may return fewer than the specified number of results."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "nextPageToken value returned from a previous list call, indicating that the subsequent page of results should be returned. The list request must be otherwise identical to the one that resulted in this token."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Restriction on the work status to return. Only courseWork that matches is returned. If unspecified, items with a work status of `PUBLISHED` is returned."]
                        pub enum QueryParametersCourseWorkStatesEnum {
                            #[serde(rename = "COURSE_WORK_STATE_UNSPECIFIED")]
                            #[doc = "No state specified. This is never returned."]
                            CourseWorkStateUnspecified,
                            #[serde(rename = "PUBLISHED")]
                            #[doc = "Status for work that has been published. This is the default state."]
                            Published,
                            #[serde(rename = "DRAFT")]
                            #[doc = "Status for work that is not yet published. Work in this state is visible only to course teachers and domain administrators."]
                            Draft,
                            #[serde(rename = "DELETED")]
                            #[doc = "Status for work that was published but is now deleted. Work in this state is visible only to course teachers and domain administrators. Work in this state is deleted after some time."]
                            Deleted,
                        }
                        impl ::std::default::Default for QueryParametersCourseWorkStatesEnum {
                            fn default() -> Self {
                                Self::CourseWorkStateUnspecified
                            }
                        }
                    }
                    pub mod patch {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Mask that identifies which fields on the course work to update. This field is required to do an update. The update fails if invalid fields are specified. If a field supports empty values, it can be cleared by specifying it in the update mask and not in the CourseWork object. If a field that does not support empty values is included in the update mask and not set in the CourseWork object, an `INVALID_ARGUMENT` error is returned. The following fields may be specified by teachers: * `title` * `description` * `state` * `due_date` * `due_time` * `max_points` * `scheduled_time` * `submission_modification_mode` * `topic_id`"]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod student_submissions {
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "late")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Requested lateness value. If specified, returned student submissions are restricted by the requested value. If unspecified, submissions are returned regardless of `late` value."]
                                    pub late: ::std::option::Option<QueryParametersLateEnum>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Maximum number of items to return. Zero or unspecified indicates that the server may assign a maximum. The server may return fewer than the specified number of results."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "nextPageToken value returned from a previous list call, indicating that the subsequent page of results should be returned. The list request must be otherwise identical to the one that resulted in this token."]
                                    pub page_token: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "states")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Requested submission states. If specified, returned student submissions match one of the specified submission states."]
                                    pub states: ::std::option::Option<QueryParametersStatesEnum>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "userId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional argument to restrict returned student work to those owned by the student with the specified identifier. The identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `\"me\"`, indicating the requesting user"]
                                    pub user_id: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                                #[derive(
                                    Debug,
                                    PartialEq,
                                    Copy,
                                    Clone,
                                    serde :: Serialize,
                                    serde :: Deserialize,
                                )]
                                #[doc = "Requested lateness value. If specified, returned student submissions are restricted by the requested value. If unspecified, submissions are returned regardless of `late` value."]
                                pub enum QueryParametersLateEnum {
                                    #[serde(rename = "LATE_VALUES_UNSPECIFIED")]
                                    #[doc = "No restriction on submission late values specified."]
                                    LateValuesUnspecified,
                                    #[serde(rename = "LATE_ONLY")]
                                    #[doc = "Return StudentSubmissions where late is true."]
                                    LateOnly,
                                    #[serde(rename = "NOT_LATE_ONLY")]
                                    #[doc = "Return StudentSubmissions where late is false."]
                                    NotLateOnly,
                                }
                                impl ::std::default::Default for QueryParametersLateEnum {
                                    fn default() -> Self {
                                        Self::LateValuesUnspecified
                                    }
                                }
                                #[derive(
                                    Debug,
                                    PartialEq,
                                    Copy,
                                    Clone,
                                    serde :: Serialize,
                                    serde :: Deserialize,
                                )]
                                #[doc = "Requested submission states. If specified, returned student submissions match one of the specified submission states."]
                                pub enum QueryParametersStatesEnum {
                                    #[serde(rename = "SUBMISSION_STATE_UNSPECIFIED")]
                                    #[doc = "No state specified. This should never be returned."]
                                    SubmissionStateUnspecified,
                                    #[serde(rename = "NEW")]
                                    #[doc = "The student has never accessed this submission. Attachments are not returned and timestamps is not set."]
                                    New,
                                    #[serde(rename = "CREATED")]
                                    #[doc = "Has been created."]
                                    Created,
                                    #[serde(rename = "TURNED_IN")]
                                    #[doc = "Has been turned in to the teacher."]
                                    TurnedIn,
                                    #[serde(rename = "RETURNED")]
                                    #[doc = "Has been returned to the student."]
                                    Returned,
                                    #[serde(rename = "RECLAIMED_BY_STUDENT")]
                                    #[doc = "Student chose to \"unsubmit\" the assignment."]
                                    ReclaimedByStudent,
                                }
                                impl ::std::default::Default for QueryParametersStatesEnum {
                                    fn default() -> Self {
                                        Self::SubmissionStateUnspecified
                                    }
                                }
                            }
                            pub mod patch {
                                #[derive(
                                    Clone,
                                    Debug,
                                    PartialEq,
                                    derive_builder :: Builder,
                                    serde :: Serialize,
                                    serde :: Deserialize,
                                )]
                                pub struct QueryParameters {
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "updateMask")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Mask that identifies which fields on the student submission to update. This field is required to do an update. The update fails if invalid fields are specified. The following fields may be specified by teachers: * `draft_grade` * `assigned_grade`"]
                                    pub update_mask: ::std::option::Option<::std::string::String>,
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
            }
            pub mod course_work_materials {
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "courseWorkMaterialStates")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Restriction on the work status to return. Only course work material that matches is returned. If unspecified, items with a work status of `PUBLISHED` is returned."]
                            pub course_work_material_states:
                                ::std::option::Option<QueryParametersCourseWorkMaterialStatesEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "materialDriveId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional filtering for course work material with at least one Drive material whose ID matches the provided string. If `material_link` is also specified, course work material must have materials matching both filters."]
                            pub material_drive_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "materialLink")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional filtering for course work material with at least one link material whose URL partially matches the provided string."]
                            pub material_link: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional sort ordering for results. A comma-separated list of fields with an optional sort direction keyword. Supported field is `updateTime`. Supported direction keywords are `asc` and `desc`. If not specified, `updateTime desc` is the default behavior. Examples: `updateTime asc`, `updateTime`"]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of items to return. Zero or unspecified indicates that the server may assign a maximum. The server may return fewer than the specified number of results."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "nextPageToken value returned from a previous list call, indicating that the subsequent page of results should be returned. The list request must be otherwise identical to the one that resulted in this token."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Restriction on the work status to return. Only course work material that matches is returned. If unspecified, items with a work status of `PUBLISHED` is returned."]
                        pub enum QueryParametersCourseWorkMaterialStatesEnum {
                            #[serde(rename = "COURSEWORK_MATERIAL_STATE_UNSPECIFIED")]
                            #[doc = "No state specified. This is never returned."]
                            CourseworkMaterialStateUnspecified,
                            #[serde(rename = "PUBLISHED")]
                            #[doc = "Status for course work material that has been published. This is the default state."]
                            Published,
                            #[serde(rename = "DRAFT")]
                            #[doc = "Status for an course work material that is not yet published. Course work material in this state is visible only to course teachers and domain administrators."]
                            Draft,
                            #[serde(rename = "DELETED")]
                            #[doc = "Status for course work material that was published but is now deleted. Course work material in this state is visible only to course teachers and domain administrators. Course work material in this state is deleted after some time."]
                            Deleted,
                        }
                        impl ::std::default::Default for QueryParametersCourseWorkMaterialStatesEnum {
                            fn default() -> Self {
                                Self::CourseworkMaterialStateUnspecified
                            }
                        }
                    }
                    pub mod patch {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Mask that identifies which fields on the course work material to update. This field is required to do an update. The update fails if invalid fields are specified. If a field supports empty values, it can be cleared by specifying it in the update mask and not in the course work material object. If a field that does not support empty values is included in the update mask and not set in the course work material object, an `INVALID_ARGUMENT` error is returned. The following fields may be specified by teachers: * `title` * `description` * `state` * `scheduled_time` * `topic_id`"]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod students {
                pub mod methods {
                    pub mod create {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "enrollmentCode")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Enrollment code of the course to create the student in. This code is required if userId corresponds to the requesting user; it may be omitted if the requesting user has administrative permissions to create students for any user."]
                            pub enrollment_code: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of items to return. The default is 30 if unspecified or `0`. The server may return fewer than the specified number of results."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "nextPageToken value returned from a previous list call, indicating that the subsequent page of results should be returned. The list request must be otherwise identical to the one that resulted in this token."]
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
            pub mod teachers {
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of items to return. The default is 30 if unspecified or `0`. The server may return fewer than the specified number of results."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "nextPageToken value returned from a previous list call, indicating that the subsequent page of results should be returned. The list request must be otherwise identical to the one that resulted in this token."]
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
            pub mod topics {
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of items to return. Zero or unspecified indicates that the server may assign a maximum. The server may return fewer than the specified number of results."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "nextPageToken value returned from a previous list call, indicating that the subsequent page of results should be returned. The list request must be otherwise identical to the one that resulted in this token."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod patch {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Mask that identifies which fields on the topic to update. This field is required to do an update. The update fails if invalid fields are specified. If a field supports empty values, it can be cleared by specifying it in the update mask and not in the Topic object. If a field that does not support empty values is included in the update mask and not set in the Topic object, an `INVALID_ARGUMENT` error is returned. The following fields may be specified: * `name`"]
                            pub update_mask: ::std::option::Option<::std::string::String>,
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
    }
    pub mod invitations {
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
                    #[serde(rename = "courseId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restricts returned invitations to those for a course with the specified identifier."]
                    pub course_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of items to return. The default is 500 if unspecified or `0`. The server may return fewer than the specified number of results."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "nextPageToken value returned from a previous list call, indicating that the subsequent page of results should be returned. The list request must be otherwise identical to the one that resulted in this token."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restricts returned invitations to those for a specific user. The identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `\"me\"`, indicating the requesting user"]
                    pub user_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod user_profiles {
        pub mod resources {
            pub mod guardian_invitations {
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "invitedEmailAddress")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If specified, only results with the specified `invited_email_address` are returned."]
                            pub invited_email_address: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of items to return. Zero or unspecified indicates that the server may assign a maximum. The server may return fewer than the specified number of results."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "nextPageToken value returned from a previous list call, indicating that the subsequent page of results should be returned. The list request must be otherwise identical to the one that resulted in this token."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "states")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If specified, only results with the specified `state` values are returned. Otherwise, results with a `state` of `PENDING` are returned."]
                            pub states: ::std::option::Option<QueryParametersStatesEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "If specified, only results with the specified `state` values are returned. Otherwise, results with a `state` of `PENDING` are returned."]
                        pub enum QueryParametersStatesEnum {
                            #[serde(rename = "GUARDIAN_INVITATION_STATE_UNSPECIFIED")]
                            #[doc = "Should never be returned."]
                            GuardianInvitationStateUnspecified,
                            #[serde(rename = "PENDING")]
                            #[doc = "The invitation is active and awaiting a response."]
                            Pending,
                            #[serde(rename = "COMPLETE")]
                            #[doc = "The invitation is no longer active. It may have been accepted, declined, withdrawn or it may have expired."]
                            Complete,
                        }
                        impl ::std::default::Default for QueryParametersStatesEnum {
                            fn default() -> Self {
                                Self::GuardianInvitationStateUnspecified
                            }
                        }
                    }
                    pub mod patch {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Mask that identifies which fields on the course to update. This field is required to do an update. The update fails if invalid fields are specified. The following fields are valid: * `state` When set in a query parameter, this field should be specified as `updateMask=,,...`"]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod guardians {
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "invitedEmailAddress")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Filter results by the email address that the original invitation was sent to, resulting in this guardian link. This filter can only be used by domain administrators."]
                            pub invited_email_address: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of items to return. Zero or unspecified indicates that the server may assign a maximum. The server may return fewer than the specified number of results."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "nextPageToken value returned from a previous list call, indicating that the subsequent page of results should be returned. The list request must be otherwise identical to the one that resulted in this token."]
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
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Announcement created by a teacher for students of the course"]
    pub struct Announcement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternateLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Absolute link to this announcement in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only."]
        pub alternate_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assigneeMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Assignee mode of the announcement. If unspecified, the default value is `ALL_STUDENTS`."]
        pub assignee_mode: ::std::option::Option<AnnouncementAssigneeModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of the course. Read-only."]
        pub course_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp when this announcement was created. Read-only."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creatorUserId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier for the user that created the announcement. Read-only."]
        pub creator_user_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Classroom-assigned identifier of this announcement, unique per course. Read-only."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "individualStudentsOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifiers of students with access to the announcement. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. If the `assigneeMode` is `INDIVIDUAL_STUDENTS`, then only students specified in this field can see the announcement."]
        pub individual_students_options:
            ::std::option::Option<::std::boxed::Box<IndividualStudentsOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "materials")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional materials. Announcements must have no more than 20 material items."]
        pub materials: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Material>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduledTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional timestamp when this announcement is scheduled to be published."]
        pub scheduled_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of this announcement. If unspecified, the default state is `DRAFT`."]
        pub state: ::std::option::Option<AnnouncementStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of this announcement. The text must be a valid UTF-8 string containing no more than 30,000 characters."]
        pub text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of the most recent change to this announcement. Read-only."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl Announcement {
        pub fn builder() -> AnnouncementBuilder {
            AnnouncementBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Assignee mode of the announcement. If unspecified, the default value is `ALL_STUDENTS`."]
    pub enum AnnouncementAssigneeModeEnum {
        #[serde(rename = "ASSIGNEE_MODE_UNSPECIFIED")]
        #[doc = "No mode specified. This is never returned."]
        AssigneeModeUnspecified,
        #[serde(rename = "ALL_STUDENTS")]
        #[doc = "All students can see the item. This is the default state."]
        AllStudents,
        #[serde(rename = "INDIVIDUAL_STUDENTS")]
        #[doc = "A subset of the students can see the item."]
        IndividualStudents,
    }
    impl ::std::default::Default for AnnouncementAssigneeModeEnum {
        fn default() -> Self {
            Self::AssigneeModeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Status of this announcement. If unspecified, the default state is `DRAFT`."]
    pub enum AnnouncementStateEnum {
        #[serde(rename = "ANNOUNCEMENT_STATE_UNSPECIFIED")]
        #[doc = "No state specified. This is never returned."]
        AnnouncementStateUnspecified,
        #[serde(rename = "PUBLISHED")]
        #[doc = "Status for announcement that has been published. This is the default state."]
        Published,
        #[serde(rename = "DRAFT")]
        #[doc = "Status for an announcement that is not yet published. Announcement in this state is visible only to course teachers and domain administrators."]
        Draft,
        #[serde(rename = "DELETED")]
        #[doc = "Status for announcement that was published but is now deleted. Announcement in this state is visible only to course teachers and domain administrators. Announcement in this state is deleted after some time."]
        Deleted,
    }
    impl ::std::default::Default for AnnouncementStateEnum {
        fn default() -> Self {
            Self::AnnouncementStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional details for assignments."]
    pub struct Assignment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "studentWorkFolder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Drive folder where attachments from student submissions are placed. This is only populated for course teachers and administrators."]
        pub student_work_folder: ::std::option::Option<::std::boxed::Box<DriveFolder>>,
    }
    impl Assignment {
        pub fn builder() -> AssignmentBuilder {
            AssignmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Student work for an assignment."]
    pub struct AssignmentSubmission {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attachments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Attachments added by the student. Drive files that correspond to materials with a share mode of STUDENT_COPY may not exist yet if the student has not accessed the assignment in Classroom. Some attachment metadata is only populated if the requesting user has permission to access it. Identifier and alternate_link fields are always available, but others (for example, title) may not be."]
        pub attachments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Attachment>>>,
    }
    impl AssignmentSubmission {
        pub fn builder() -> AssignmentSubmissionBuilder {
            AssignmentSubmissionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Attachment added to student assignment work. When creating attachments, setting the `form` field is not supported."]
    pub struct Attachment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveFile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Drive file attachment."]
        pub drive_file: ::std::option::Option<::std::boxed::Box<DriveFile>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "form")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Forms attachment."]
        pub form: ::std::option::Option<::std::boxed::Box<Form>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link attachment."]
        pub link: ::std::option::Option<::std::boxed::Box<Link>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "youTubeVideo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Youtube video attachment."]
        pub you_tube_video: ::std::option::Option<::std::boxed::Box<YouTubeVideo>>,
    }
    impl Attachment {
        pub fn builder() -> AttachmentBuilder {
            AttachmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A reference to a Cloud Pub/Sub topic. To register for notifications, the owner of the topic must grant `classroom-notifications@system.gserviceaccount.com` the `projects.topics.publish` permission."]
    pub struct CloudPubsubTopic {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `name` field of a Cloud Pub/Sub [Topic](https://cloud.google.com/pubsub/docs/reference/rest/v1/projects.topics#Topic)."]
        pub topic_name: ::std::option::Option<::std::string::String>,
    }
    impl CloudPubsubTopic {
        pub fn builder() -> CloudPubsubTopicBuilder {
            CloudPubsubTopicBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Course in Classroom."]
    pub struct Course {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternateLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Absolute link to this course in the Classroom web UI. Read-only."]
        pub alternate_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "calendarId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Calendar ID for a calendar that all course members can see, to which Classroom adds events for course work and announcements in the course. Read-only."]
        pub calendar_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseGroupEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of a Google group containing all members of the course. This group does not accept email and can only be used for permissions. Read-only."]
        pub course_group_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseMaterialSets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sets of materials that appear on the \"about\" page of this course. Read-only."]
        pub course_material_sets:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CourseMaterialSet>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State of the course. If unspecified, the default state is `PROVISIONED`."]
        pub course_state: ::std::option::Option<CourseCourseStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creation time of the course. Specifying this field in a course update mask results in an error. Read-only."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional description. For example, \"We'll be learning about the structure of living creatures from a combination of textbooks, guest lectures, and lab work. Expect to be excited!\" If set, this field must be a valid UTF-8 string and no longer than 30,000 characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "descriptionHeading")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional heading for the description. For example, \"Welcome to 10th Grade Biology.\" If set, this field must be a valid UTF-8 string and no longer than 3600 characters."]
        pub description_heading: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enrollmentCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enrollment code to use when joining this course. Specifying this field in a course update mask results in an error. Read-only."]
        pub enrollment_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "guardiansEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not guardian notifications are enabled for this course. Read-only."]
        pub guardians_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier for this course assigned by Classroom. When creating a course, you may optionally set this identifier to an alias string in the request to create a corresponding alias. The `id` is still assigned by Classroom and cannot be updated after the course is created. Specifying this field in a course update mask results in an error."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the course. For example, \"10th Grade Biology\". The name is required. It must be between 1 and 750 characters and a valid UTF-8 string."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of the owner of a course. When specified as a parameter of a create course request, this field is required. The identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `\"me\"`, indicating the requesting user This must be set in a create request. Admins can also specify this field in a patch course request to transfer ownership. In other contexts, it is read-only."]
        pub owner_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "room")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional room location. For example, \"301\". If set, this field must be a valid UTF-8 string and no longer than 650 characters."]
        pub room: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "section")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Section of the course. For example, \"Period 2\". If set, this field must be a valid UTF-8 string and no longer than 2800 characters."]
        pub section: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teacherFolder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about a Drive Folder that is shared with all teachers of the course. This field will only be set for teachers of the course and domain administrators. Read-only."]
        pub teacher_folder: ::std::option::Option<::std::boxed::Box<DriveFolder>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teacherGroupEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of a Google group containing all teachers of the course. This group does not accept email and can only be used for permissions. Read-only."]
        pub teacher_group_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time of the most recent update to this course. Specifying this field in a course update mask results in an error. Read-only."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl Course {
        pub fn builder() -> CourseBuilder {
            CourseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "State of the course. If unspecified, the default state is `PROVISIONED`."]
    pub enum CourseCourseStateEnum {
        #[serde(rename = "COURSE_STATE_UNSPECIFIED")]
        #[doc = "No course state. No returned Course message will use this value."]
        CourseStateUnspecified,
        #[serde(rename = "ACTIVE")]
        #[doc = "The course is active."]
        Active,
        #[serde(rename = "ARCHIVED")]
        #[doc = "The course has been archived. You cannot modify it except to change it to a different state."]
        Archived,
        #[serde(rename = "PROVISIONED")]
        #[doc = "The course has been created, but not yet activated. It is accessible by the primary teacher and domain administrators, who may modify it or change it to the `ACTIVE` or `DECLINED` states. A course may only be changed to `PROVISIONED` if it is in the `DECLINED` state."]
        Provisioned,
        #[serde(rename = "DECLINED")]
        #[doc = "The course has been created, but declined. It is accessible by the course owner and domain administrators, though it will not be displayed in the web UI. You cannot modify the course except to change it to the `PROVISIONED` state. A course may only be changed to `DECLINED` if it is in the `PROVISIONED` state."]
        Declined,
        #[serde(rename = "SUSPENDED")]
        #[doc = "The course has been suspended. You cannot modify the course, and only the user identified by the `owner_id` can view the course. A course may be placed in this state if it potentially violates the Terms of Service."]
        Suspended,
    }
    impl ::std::default::Default for CourseCourseStateEnum {
        fn default() -> Self {
            Self::CourseStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Alternative identifier for a course. An alias uniquely identifies a course. It must be unique within one of the following scopes: * domain: A domain-scoped alias is visible to all users within the alias creator's domain and can be created only by a domain admin. A domain-scoped alias is often used when a course has an identifier external to Classroom. * project: A project-scoped alias is visible to any request from an application using the Developer Console project ID that created the alias and can be created by any project. A project-scoped alias is often used when an application has alternative identifiers. A random value can also be used to avoid duplicate courses in the event of transmission failures, as retrying a request will return `ALREADY_EXISTS` if a previous one has succeeded."]
    pub struct CourseAlias {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alias")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Alias string. The format of the string indicates the desired alias scoping. * `d:` indicates a domain-scoped alias. Example: `d:math_101` * `p:` indicates a project-scoped alias. Example: `p:abc123` This field has a maximum length of 256 characters."]
        pub alias: ::std::option::Option<::std::string::String>,
    }
    impl CourseAlias {
        pub fn builder() -> CourseAliasBuilder {
            CourseAliasBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A material attached to a course as part of a material set."]
    pub struct CourseMaterial {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveFile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Drive file attachment."]
        pub drive_file: ::std::option::Option<::std::boxed::Box<DriveFile>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "form")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Forms attachment."]
        pub form: ::std::option::Option<::std::boxed::Box<Form>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link atatchment."]
        pub link: ::std::option::Option<::std::boxed::Box<Link>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "youTubeVideo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Youtube video attachment."]
        pub you_tube_video: ::std::option::Option<::std::boxed::Box<YouTubeVideo>>,
    }
    impl CourseMaterial {
        pub fn builder() -> CourseMaterialBuilder {
            CourseMaterialBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A set of materials that appears on the \"About\" page of the course. These materials might include a syllabus, schedule, or other background information relating to the course as a whole."]
    pub struct CourseMaterialSet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "materials")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Materials attached to this set."]
        pub materials: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CourseMaterial>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title for this set."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl CourseMaterialSet {
        pub fn builder() -> CourseMaterialSetBuilder {
            CourseMaterialSetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a `Feed` with a `feed_type` of `COURSE_ROSTER_CHANGES`."]
    pub struct CourseRosterChangesInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `course_id` of the course to subscribe to roster changes for."]
        pub course_id: ::std::option::Option<::std::string::String>,
    }
    impl CourseRosterChangesInfo {
        pub fn builder() -> CourseRosterChangesInfoBuilder {
            CourseRosterChangesInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Course work created by a teacher for students of the course."]
    pub struct CourseWork {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternateLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Absolute link to this course work in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only."]
        pub alternate_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assigneeMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Assignee mode of the coursework. If unspecified, the default value is `ALL_STUDENTS`."]
        pub assignee_mode: ::std::option::Option<CourseWorkAssigneeModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assignment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Assignment details. This is populated only when `work_type` is `ASSIGNMENT`. Read-only."]
        pub assignment: ::std::option::Option<::std::boxed::Box<Assignment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "associatedWithDeveloper")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this course work item is associated with the Developer Console project making the request. See CreateCourseWork for more details. Read-only."]
        pub associated_with_developer: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of the course. Read-only."]
        pub course_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp when this course work was created. Read-only."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creatorUserId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier for the user that created the coursework. Read-only."]
        pub creator_user_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional description of this course work. If set, the description must be a valid UTF-8 string containing no more than 30,000 characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dueDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional date, in UTC, that submissions for this course work are due. This must be specified if `due_time` is specified."]
        pub due_date: ::std::option::Option<::std::boxed::Box<Date>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dueTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional time of day, in UTC, that submissions for this course work are due. This must be specified if `due_date` is specified."]
        pub due_time: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Classroom-assigned identifier of this course work, unique per course. Read-only."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "individualStudentsOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifiers of students with access to the coursework. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. If the `assigneeMode` is `INDIVIDUAL_STUDENTS`, then only students specified in this field are assigned the coursework."]
        pub individual_students_options:
            ::std::option::Option<::std::boxed::Box<IndividualStudentsOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "materials")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional materials. CourseWork must have no more than 20 material items."]
        pub materials: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Material>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxPoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum grade for this course work. If zero or unspecified, this assignment is considered ungraded. This must be a non-negative integer value."]
        pub max_points: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "multipleChoiceQuestion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Multiple choice question details. For read operations, this field is populated only when `work_type` is `MULTIPLE_CHOICE_QUESTION`. For write operations, this field must be specified when creating course work with a `work_type` of `MULTIPLE_CHOICE_QUESTION`, and it must not be set otherwise."]
        pub multiple_choice_question:
            ::std::option::Option<::std::boxed::Box<MultipleChoiceQuestion>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduledTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional timestamp when this course work is scheduled to be published."]
        pub scheduled_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of this course work. If unspecified, the default state is `DRAFT`."]
        pub state: ::std::option::Option<CourseWorkStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "submissionModificationMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Setting to determine when students are allowed to modify submissions. If unspecified, the default value is `MODIFIABLE_UNTIL_TURNED_IN`."]
        pub submission_modification_mode:
            ::std::option::Option<CourseWorkSubmissionModificationModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of this course work. The title must be a valid UTF-8 string containing between 1 and 3000 characters."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier for the topic that this coursework is associated with. Must match an existing topic in the course."]
        pub topic_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of the most recent change to this course work. Read-only."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of this course work. The type is set when the course work is created and cannot be changed."]
        pub work_type: ::std::option::Option<CourseWorkWorkTypeEnum>,
    }
    impl CourseWork {
        pub fn builder() -> CourseWorkBuilder {
            CourseWorkBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Assignee mode of the coursework. If unspecified, the default value is `ALL_STUDENTS`."]
    pub enum CourseWorkAssigneeModeEnum {
        #[serde(rename = "ASSIGNEE_MODE_UNSPECIFIED")]
        #[doc = "No mode specified. This is never returned."]
        AssigneeModeUnspecified,
        #[serde(rename = "ALL_STUDENTS")]
        #[doc = "All students can see the item. This is the default state."]
        AllStudents,
        #[serde(rename = "INDIVIDUAL_STUDENTS")]
        #[doc = "A subset of the students can see the item."]
        IndividualStudents,
    }
    impl ::std::default::Default for CourseWorkAssigneeModeEnum {
        fn default() -> Self {
            Self::AssigneeModeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Status of this course work. If unspecified, the default state is `DRAFT`."]
    pub enum CourseWorkStateEnum {
        #[serde(rename = "COURSE_WORK_STATE_UNSPECIFIED")]
        #[doc = "No state specified. This is never returned."]
        CourseWorkStateUnspecified,
        #[serde(rename = "PUBLISHED")]
        #[doc = "Status for work that has been published. This is the default state."]
        Published,
        #[serde(rename = "DRAFT")]
        #[doc = "Status for work that is not yet published. Work in this state is visible only to course teachers and domain administrators."]
        Draft,
        #[serde(rename = "DELETED")]
        #[doc = "Status for work that was published but is now deleted. Work in this state is visible only to course teachers and domain administrators. Work in this state is deleted after some time."]
        Deleted,
    }
    impl ::std::default::Default for CourseWorkStateEnum {
        fn default() -> Self {
            Self::CourseWorkStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Setting to determine when students are allowed to modify submissions. If unspecified, the default value is `MODIFIABLE_UNTIL_TURNED_IN`."]
    pub enum CourseWorkSubmissionModificationModeEnum {
        #[serde(rename = "SUBMISSION_MODIFICATION_MODE_UNSPECIFIED")]
        #[doc = "No modification mode specified. This is never returned."]
        SubmissionModificationModeUnspecified,
        #[serde(rename = "MODIFIABLE_UNTIL_TURNED_IN")]
        #[doc = "Submissions can be modified before being turned in."]
        ModifiableUntilTurnedIn,
        #[serde(rename = "MODIFIABLE")]
        #[doc = "Submissions can be modified at any time."]
        Modifiable,
    }
    impl ::std::default::Default for CourseWorkSubmissionModificationModeEnum {
        fn default() -> Self {
            Self::SubmissionModificationModeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of this course work. The type is set when the course work is created and cannot be changed."]
    pub enum CourseWorkWorkTypeEnum {
        #[serde(rename = "COURSE_WORK_TYPE_UNSPECIFIED")]
        #[doc = "No work type specified. This is never returned."]
        CourseWorkTypeUnspecified,
        #[serde(rename = "ASSIGNMENT")]
        #[doc = "An assignment."]
        Assignment,
        #[serde(rename = "SHORT_ANSWER_QUESTION")]
        #[doc = "A short answer question."]
        ShortAnswerQuestion,
        #[serde(rename = "MULTIPLE_CHOICE_QUESTION")]
        #[doc = "A multiple-choice question."]
        MultipleChoiceQuestion,
    }
    impl ::std::default::Default for CourseWorkWorkTypeEnum {
        fn default() -> Self {
            Self::CourseWorkTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a `Feed` with a `feed_type` of `COURSE_WORK_CHANGES`."]
    pub struct CourseWorkChangesInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `course_id` of the course to subscribe to work changes for."]
        pub course_id: ::std::option::Option<::std::string::String>,
    }
    impl CourseWorkChangesInfo {
        pub fn builder() -> CourseWorkChangesInfoBuilder {
            CourseWorkChangesInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Course work material created by a teacher for students of the course"]
    pub struct CourseWorkMaterial {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternateLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Absolute link to this course work material in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only."]
        pub alternate_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assigneeMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Assignee mode of the course work material. If unspecified, the default value is `ALL_STUDENTS`."]
        pub assignee_mode: ::std::option::Option<CourseWorkMaterialAssigneeModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of the course. Read-only."]
        pub course_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp when this course work material was created. Read-only."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creatorUserId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier for the user that created the course work material. Read-only."]
        pub creator_user_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional description of this course work material. The text must be a valid UTF-8 string containing no more than 30,000 characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Classroom-assigned identifier of this course work material, unique per course. Read-only."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "individualStudentsOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifiers of students with access to the course work material. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. If the `assigneeMode` is `INDIVIDUAL_STUDENTS`, then only students specified in this field can see the course work material."]
        pub individual_students_options:
            ::std::option::Option<::std::boxed::Box<IndividualStudentsOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "materials")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional materials. A course work material must have no more than 20 material items."]
        pub materials: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Material>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduledTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional timestamp when this course work material is scheduled to be published."]
        pub scheduled_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of this course work material. If unspecified, the default state is `DRAFT`."]
        pub state: ::std::option::Option<CourseWorkMaterialStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of this course work material. The title must be a valid UTF-8 string containing between 1 and 3000 characters."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier for the topic that this course work material is associated with. Must match an existing topic in the course."]
        pub topic_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of the most recent change to this course work material. Read-only."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl CourseWorkMaterial {
        pub fn builder() -> CourseWorkMaterialBuilder {
            CourseWorkMaterialBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Assignee mode of the course work material. If unspecified, the default value is `ALL_STUDENTS`."]
    pub enum CourseWorkMaterialAssigneeModeEnum {
        #[serde(rename = "ASSIGNEE_MODE_UNSPECIFIED")]
        #[doc = "No mode specified. This is never returned."]
        AssigneeModeUnspecified,
        #[serde(rename = "ALL_STUDENTS")]
        #[doc = "All students can see the item. This is the default state."]
        AllStudents,
        #[serde(rename = "INDIVIDUAL_STUDENTS")]
        #[doc = "A subset of the students can see the item."]
        IndividualStudents,
    }
    impl ::std::default::Default for CourseWorkMaterialAssigneeModeEnum {
        fn default() -> Self {
            Self::AssigneeModeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Status of this course work material. If unspecified, the default state is `DRAFT`."]
    pub enum CourseWorkMaterialStateEnum {
        #[serde(rename = "COURSEWORK_MATERIAL_STATE_UNSPECIFIED")]
        #[doc = "No state specified. This is never returned."]
        CourseworkMaterialStateUnspecified,
        #[serde(rename = "PUBLISHED")]
        #[doc = "Status for course work material that has been published. This is the default state."]
        Published,
        #[serde(rename = "DRAFT")]
        #[doc = "Status for an course work material that is not yet published. Course work material in this state is visible only to course teachers and domain administrators."]
        Draft,
        #[serde(rename = "DELETED")]
        #[doc = "Status for course work material that was published but is now deleted. Course work material in this state is visible only to course teachers and domain administrators. Course work material in this state is deleted after some time."]
        Deleted,
    }
    impl ::std::default::Default for CourseWorkMaterialStateEnum {
        fn default() -> Self {
            Self::CourseworkMaterialStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."]
    pub struct Date {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "day")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
        pub day: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "month")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
        pub month: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "year")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
        pub year: ::std::option::Option<::std::primitive::i64>,
    }
    impl Date {
        pub fn builder() -> DateBuilder {
            DateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Representation of a Google Drive file."]
    pub struct DriveFile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternateLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL that can be used to access the Drive item. Read-only."]
        pub alternate_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Drive API resource ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of a thumbnail image of the Drive item. Read-only."]
        pub thumbnail_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of the Drive item. Read-only."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl DriveFile {
        pub fn builder() -> DriveFileBuilder {
            DriveFileBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Representation of a Google Drive folder."]
    pub struct DriveFolder {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternateLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL that can be used to access the Drive folder. Read-only."]
        pub alternate_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Drive API resource ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of the Drive folder. Read-only."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl DriveFolder {
        pub fn builder() -> DriveFolderBuilder {
            DriveFolderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct Empty {}
    impl Empty {
        pub fn builder() -> EmptyBuilder {
            EmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A class of notifications that an application can register to receive. For example: \"all roster changes for a domain\"."]
    pub struct Feed {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseRosterChangesInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about a `Feed` with a `feed_type` of `COURSE_ROSTER_CHANGES`. This field must be specified if `feed_type` is `COURSE_ROSTER_CHANGES`."]
        pub course_roster_changes_info:
            ::std::option::Option<::std::boxed::Box<CourseRosterChangesInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseWorkChangesInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about a `Feed` with a `feed_type` of `COURSE_WORK_CHANGES`. This field must be specified if `feed_type` is `COURSE_WORK_CHANGES`."]
        pub course_work_changes_info:
            ::std::option::Option<::std::boxed::Box<CourseWorkChangesInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feedType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of feed."]
        pub feed_type: ::std::option::Option<FeedFeedTypeEnum>,
    }
    impl Feed {
        pub fn builder() -> FeedBuilder {
            FeedBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of feed."]
    pub enum FeedFeedTypeEnum {
        #[serde(rename = "FEED_TYPE_UNSPECIFIED")]
        #[doc = "Should never be returned or provided."]
        FeedTypeUnspecified,
        #[serde(rename = "DOMAIN_ROSTER_CHANGES")]
        #[doc = "All roster changes for a particular domain. Notifications will be generated whenever a user joins or leaves a course. No notifications will be generated when an invitation is created or deleted, but notifications will be generated when a user joins a course by accepting an invitation."]
        DomainRosterChanges,
        #[serde(rename = "COURSE_ROSTER_CHANGES")]
        #[doc = "All roster changes for a particular course. Notifications will be generated whenever a user joins or leaves a course. No notifications will be generated when an invitation is created or deleted, but notifications will be generated when a user joins a course by accepting an invitation."]
        CourseRosterChanges,
        #[serde(rename = "COURSE_WORK_CHANGES")]
        #[doc = "All course work activity for a particular course. Notifications will be generated when a CourseWork or StudentSubmission object is created or modified. No notification will be generated when a StudentSubmission object is created in connection with the creation or modification of its parent CourseWork object (but a notification will be generated for that CourseWork object's creation or modification)."]
        CourseWorkChanges,
    }
    impl ::std::default::Default for FeedFeedTypeEnum {
        fn default() -> Self {
            Self::FeedTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Google Forms item."]
    pub struct Form {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of the form."]
        pub form_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of the form responses document. Only set if respsonses have been recorded and only when the requesting user is an editor of the form. Read-only."]
        pub response_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of a thumbnail image of the Form. Read-only."]
        pub thumbnail_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of the Form. Read-only."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl Form {
        pub fn builder() -> FormBuilder {
            FormBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Global user permission description."]
    pub struct GlobalPermission {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Permission value."]
        pub permission: ::std::option::Option<GlobalPermissionPermissionEnum>,
    }
    impl GlobalPermission {
        pub fn builder() -> GlobalPermissionBuilder {
            GlobalPermissionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Permission value."]
    pub enum GlobalPermissionPermissionEnum {
        #[serde(rename = "PERMISSION_UNSPECIFIED")]
        #[doc = "No permission is specified. This is not returned and is not a valid value."]
        PermissionUnspecified,
        #[serde(rename = "CREATE_COURSE")]
        #[doc = "User is permitted to create a course."]
        CreateCourse,
    }
    impl ::std::default::Default for GlobalPermissionPermissionEnum {
        fn default() -> Self {
            Self::PermissionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The history of each grade on this submission."]
    pub struct GradeHistory {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actorUserId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The teacher who made the grade change."]
        pub actor_user_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gradeChangeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of grade change at this time in the submission grade history."]
        pub grade_change_type: ::std::option::Option<GradeHistoryGradeChangeTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gradeTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the grade of the submission was changed."]
        pub grade_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxPoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The denominator of the grade at this time in the submission grade history."]
        pub max_points: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pointsEarned")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The numerator of the grade at this time in the submission grade history."]
        pub points_earned: ::std::option::Option<::std::primitive::f64>,
    }
    impl GradeHistory {
        pub fn builder() -> GradeHistoryBuilder {
            GradeHistoryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of grade change at this time in the submission grade history."]
    pub enum GradeHistoryGradeChangeTypeEnum {
        #[serde(rename = "UNKNOWN_GRADE_CHANGE_TYPE")]
        #[doc = "No grade change type specified. This should never be returned."]
        UnknownGradeChangeType,
        #[serde(rename = "DRAFT_GRADE_POINTS_EARNED_CHANGE")]
        #[doc = "A change in the numerator of the draft grade."]
        DraftGradePointsEarnedChange,
        #[serde(rename = "ASSIGNED_GRADE_POINTS_EARNED_CHANGE")]
        #[doc = "A change in the numerator of the assigned grade."]
        AssignedGradePointsEarnedChange,
        #[serde(rename = "MAX_POINTS_CHANGE")]
        #[doc = "A change in the denominator of the grade."]
        MaxPointsChange,
    }
    impl ::std::default::Default for GradeHistoryGradeChangeTypeEnum {
        fn default() -> Self {
            Self::UnknownGradeChangeType
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Association between a student and a guardian of that student. The guardian may receive information about the student's course work."]
    pub struct Guardian {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "guardianId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier for the guardian."]
        pub guardian_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "guardianProfile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User profile for the guardian."]
        pub guardian_profile: ::std::option::Option<::std::boxed::Box<UserProfile>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invitedEmailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address to which the initial guardian invitation was sent. This field is only visible to domain administrators."]
        pub invited_email_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "studentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier for the student to whom the guardian relationship applies."]
        pub student_id: ::std::option::Option<::std::string::String>,
    }
    impl Guardian {
        pub fn builder() -> GuardianBuilder {
            GuardianBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An invitation to become the guardian of a specified user, sent to a specified email address."]
    pub struct GuardianInvitation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that this invitation was created. Read-only."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invitationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier for this invitation. Read-only."]
        pub invitation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invitedEmailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email address that the invitation was sent to. This field is only visible to domain administrators."]
        pub invited_email_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state that this invitation is in."]
        pub state: ::std::option::Option<GuardianInvitationStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "studentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the student (in standard format)"]
        pub student_id: ::std::option::Option<::std::string::String>,
    }
    impl GuardianInvitation {
        pub fn builder() -> GuardianInvitationBuilder {
            GuardianInvitationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state that this invitation is in."]
    pub enum GuardianInvitationStateEnum {
        #[serde(rename = "GUARDIAN_INVITATION_STATE_UNSPECIFIED")]
        #[doc = "Should never be returned."]
        GuardianInvitationStateUnspecified,
        #[serde(rename = "PENDING")]
        #[doc = "The invitation is active and awaiting a response."]
        Pending,
        #[serde(rename = "COMPLETE")]
        #[doc = "The invitation is no longer active. It may have been accepted, declined, withdrawn or it may have expired."]
        Complete,
    }
    impl ::std::default::Default for GuardianInvitationStateEnum {
        fn default() -> Self {
            Self::GuardianInvitationStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Assignee details about a coursework/announcement. This field is set if and only if `assigneeMode` is `INDIVIDUAL_STUDENTS`."]
    pub struct IndividualStudentsOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "studentIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifiers for the students that have access to the coursework/announcement."]
        pub student_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl IndividualStudentsOptions {
        pub fn builder() -> IndividualStudentsOptionsBuilder {
            IndividualStudentsOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An invitation to join a course."]
    pub struct Invitation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of the course to invite the user to."]
        pub course_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier assigned by Classroom. Read-only."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Role to invite the user to have. Must not be `COURSE_ROLE_UNSPECIFIED`."]
        pub role: ::std::option::Option<InvitationRoleEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of the invited user. When specified as a parameter of a request, this identifier can be set to one of the following: * the numeric identifier for the user * the email address of the user * the string literal `\"me\"`, indicating the requesting user"]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl Invitation {
        pub fn builder() -> InvitationBuilder {
            InvitationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Role to invite the user to have. Must not be `COURSE_ROLE_UNSPECIFIED`."]
    pub enum InvitationRoleEnum {
        #[serde(rename = "COURSE_ROLE_UNSPECIFIED")]
        #[doc = "No course role."]
        CourseRoleUnspecified,
        #[serde(rename = "STUDENT")]
        #[doc = "Student in the course."]
        Student,
        #[serde(rename = "TEACHER")]
        #[doc = "Teacher of the course."]
        Teacher,
        #[serde(rename = "OWNER")]
        #[doc = "Owner of the course."]
        Owner,
    }
    impl ::std::default::Default for InvitationRoleEnum {
        fn default() -> Self {
            Self::CourseRoleUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "URL item."]
    pub struct Link {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of a thumbnail image of the target URL. Read-only."]
        pub thumbnail_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of the target of the URL. Read-only."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to link to. This must be a valid UTF-8 string containing between 1 and 2024 characters."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl Link {
        pub fn builder() -> LinkBuilder {
            LinkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response when listing course work."]
    pub struct ListAnnouncementsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "announcements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Announcement items that match the request."]
        pub announcements: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Announcement>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListAnnouncementsResponse {
        pub fn builder() -> ListAnnouncementsResponseBuilder {
            ListAnnouncementsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response when listing course aliases."]
    pub struct ListCourseAliasesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aliases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The course aliases."]
        pub aliases: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CourseAlias>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListCourseAliasesResponse {
        pub fn builder() -> ListCourseAliasesResponseBuilder {
            ListCourseAliasesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response when listing course work material."]
    pub struct ListCourseWorkMaterialResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseWorkMaterial")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Course work material items that match the request."]
        pub course_work_material:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CourseWorkMaterial>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListCourseWorkMaterialResponse {
        pub fn builder() -> ListCourseWorkMaterialResponseBuilder {
            ListCourseWorkMaterialResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response when listing course work."]
    pub struct ListCourseWorkResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseWork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Course work items that match the request."]
        pub course_work: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CourseWork>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListCourseWorkResponse {
        pub fn builder() -> ListCourseWorkResponseBuilder {
            ListCourseWorkResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response when listing courses."]
    pub struct ListCoursesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Courses that match the list request."]
        pub courses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Course>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListCoursesResponse {
        pub fn builder() -> ListCoursesResponseBuilder {
            ListCoursesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response when listing guardian invitations."]
    pub struct ListGuardianInvitationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "guardianInvitations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Guardian invitations that matched the list request."]
        pub guardian_invitations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GuardianInvitation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListGuardianInvitationsResponse {
        pub fn builder() -> ListGuardianInvitationsResponseBuilder {
            ListGuardianInvitationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response when listing guardians."]
    pub struct ListGuardiansResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "guardians")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Guardians on this page of results that met the criteria specified in the request."]
        pub guardians: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Guardian>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListGuardiansResponse {
        pub fn builder() -> ListGuardiansResponseBuilder {
            ListGuardiansResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response when listing invitations."]
    pub struct ListInvitationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invitations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Invitations that match the list request."]
        pub invitations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Invitation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListInvitationsResponse {
        pub fn builder() -> ListInvitationsResponseBuilder {
            ListInvitationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response when listing student submissions."]
    pub struct ListStudentSubmissionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "studentSubmissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Student work that matches the request."]
        pub student_submissions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StudentSubmission>>>,
    }
    impl ListStudentSubmissionsResponse {
        pub fn builder() -> ListStudentSubmissionsResponseBuilder {
            ListStudentSubmissionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response when listing students."]
    pub struct ListStudentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "students")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Students who match the list request."]
        pub students: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Student>>>,
    }
    impl ListStudentsResponse {
        pub fn builder() -> ListStudentsResponseBuilder {
            ListStudentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response when listing teachers."]
    pub struct ListTeachersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teachers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Teachers who match the list request."]
        pub teachers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Teacher>>>,
    }
    impl ListTeachersResponse {
        pub fn builder() -> ListTeachersResponseBuilder {
            ListTeachersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response when listing topics."]
    pub struct ListTopicResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topic items that match the request."]
        pub topic: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Topic>>>,
    }
    impl ListTopicResponse {
        pub fn builder() -> ListTopicResponseBuilder {
            ListTopicResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Material attached to course work. When creating attachments, setting the `form` field is not supported."]
    pub struct Material {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveFile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Drive file material."]
        pub drive_file: ::std::option::Option<::std::boxed::Box<SharedDriveFile>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "form")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Forms material."]
        pub form: ::std::option::Option<::std::boxed::Box<Form>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link material. On creation, this is upgraded to a more appropriate type if possible, and this is reflected in the response."]
        pub link: ::std::option::Option<::std::boxed::Box<Link>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "youtubeVideo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "YouTube video material."]
        pub youtube_video: ::std::option::Option<::std::boxed::Box<YouTubeVideo>>,
    }
    impl Material {
        pub fn builder() -> MaterialBuilder {
            MaterialBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to modify assignee mode and options of an announcement."]
    pub struct ModifyAnnouncementAssigneesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assigneeMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mode of the announcement describing whether it is accessible by all students or specified individual students."]
        pub assignee_mode:
            ::std::option::Option<ModifyAnnouncementAssigneesRequestAssigneeModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modifyIndividualStudentsOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set which students can view or cannot view the announcement. Must be specified only when `assigneeMode` is `INDIVIDUAL_STUDENTS`."]
        pub modify_individual_students_options:
            ::std::option::Option<::std::boxed::Box<ModifyIndividualStudentsOptions>>,
    }
    impl ModifyAnnouncementAssigneesRequest {
        pub fn builder() -> ModifyAnnouncementAssigneesRequestBuilder {
            ModifyAnnouncementAssigneesRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Mode of the announcement describing whether it is accessible by all students or specified individual students."]
    pub enum ModifyAnnouncementAssigneesRequestAssigneeModeEnum {
        #[serde(rename = "ASSIGNEE_MODE_UNSPECIFIED")]
        #[doc = "No mode specified. This is never returned."]
        AssigneeModeUnspecified,
        #[serde(rename = "ALL_STUDENTS")]
        #[doc = "All students can see the item. This is the default state."]
        AllStudents,
        #[serde(rename = "INDIVIDUAL_STUDENTS")]
        #[doc = "A subset of the students can see the item."]
        IndividualStudents,
    }
    impl ::std::default::Default for ModifyAnnouncementAssigneesRequestAssigneeModeEnum {
        fn default() -> Self {
            Self::AssigneeModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to modify the attachments of a student submission."]
    pub struct ModifyAttachmentsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addAttachments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Attachments to add. A student submission may not have more than 20 attachments. Form attachments are not supported."]
        pub add_attachments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Attachment>>>,
    }
    impl ModifyAttachmentsRequest {
        pub fn builder() -> ModifyAttachmentsRequestBuilder {
            ModifyAttachmentsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to modify assignee mode and options of a coursework."]
    pub struct ModifyCourseWorkAssigneesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assigneeMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mode of the coursework describing whether it will be assigned to all students or specified individual students."]
        pub assignee_mode: ::std::option::Option<ModifyCourseWorkAssigneesRequestAssigneeModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modifyIndividualStudentsOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set which students are assigned or not assigned to the coursework. Must be specified only when `assigneeMode` is `INDIVIDUAL_STUDENTS`."]
        pub modify_individual_students_options:
            ::std::option::Option<::std::boxed::Box<ModifyIndividualStudentsOptions>>,
    }
    impl ModifyCourseWorkAssigneesRequest {
        pub fn builder() -> ModifyCourseWorkAssigneesRequestBuilder {
            ModifyCourseWorkAssigneesRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Mode of the coursework describing whether it will be assigned to all students or specified individual students."]
    pub enum ModifyCourseWorkAssigneesRequestAssigneeModeEnum {
        #[serde(rename = "ASSIGNEE_MODE_UNSPECIFIED")]
        #[doc = "No mode specified. This is never returned."]
        AssigneeModeUnspecified,
        #[serde(rename = "ALL_STUDENTS")]
        #[doc = "All students can see the item. This is the default state."]
        AllStudents,
        #[serde(rename = "INDIVIDUAL_STUDENTS")]
        #[doc = "A subset of the students can see the item."]
        IndividualStudents,
    }
    impl ::std::default::Default for ModifyCourseWorkAssigneesRequestAssigneeModeEnum {
        fn default() -> Self {
            Self::AssigneeModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains fields to add or remove students from a course work or announcement where the `assigneeMode` is set to `INDIVIDUAL_STUDENTS`."]
    pub struct ModifyIndividualStudentsOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addStudentIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IDs of students to be added as having access to this coursework/announcement."]
        pub add_student_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "removeStudentIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IDs of students to be removed from having access to this coursework/announcement."]
        pub remove_student_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ModifyIndividualStudentsOptions {
        pub fn builder() -> ModifyIndividualStudentsOptionsBuilder {
            ModifyIndividualStudentsOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional details for multiple-choice questions."]
    pub struct MultipleChoiceQuestion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "choices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Possible choices."]
        pub choices: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl MultipleChoiceQuestion {
        pub fn builder() -> MultipleChoiceQuestionBuilder {
            MultipleChoiceQuestionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Student work for a multiple-choice question."]
    pub struct MultipleChoiceSubmission {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "answer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Student's select choice."]
        pub answer: ::std::option::Option<::std::string::String>,
    }
    impl MultipleChoiceSubmission {
        pub fn builder() -> MultipleChoiceSubmissionBuilder {
            MultipleChoiceSubmissionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of the user's name."]
    pub struct Name {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "familyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's last name. Read-only."]
        pub family_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's full name formed by concatenating the first and last name values. Read-only."]
        pub full_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "givenName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's first name. Read-only."]
        pub given_name: ::std::option::Option<::std::string::String>,
    }
    impl Name {
        pub fn builder() -> NameBuilder {
            NameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to reclaim a student submission."]
    pub struct ReclaimStudentSubmissionRequest {}
    impl ReclaimStudentSubmissionRequest {
        pub fn builder() -> ReclaimStudentSubmissionRequestBuilder {
            ReclaimStudentSubmissionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An instruction to Classroom to send notifications from the `feed` to the provided destination."]
    pub struct Registration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudPubsubTopic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud Pub/Sub topic that notifications are to be sent to."]
        pub cloud_pubsub_topic: ::std::option::Option<::std::boxed::Box<CloudPubsubTopic>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiryTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time until which the `Registration` is effective. This is a read-only field assigned by the server."]
        pub expiry_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specification for the class of notifications that Classroom should deliver to the destination."]
        pub feed: ::std::option::Option<::std::boxed::Box<Feed>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "registrationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A server-generated unique identifier for this `Registration`. Read-only."]
        pub registration_id: ::std::option::Option<::std::string::String>,
    }
    impl Registration {
        pub fn builder() -> RegistrationBuilder {
            RegistrationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to return a student submission."]
    pub struct ReturnStudentSubmissionRequest {}
    impl ReturnStudentSubmissionRequest {
        pub fn builder() -> ReturnStudentSubmissionRequestBuilder {
            ReturnStudentSubmissionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Drive file that is used as material for course work."]
    pub struct SharedDriveFile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveFile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Drive file details."]
        pub drive_file: ::std::option::Option<::std::boxed::Box<DriveFile>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shareMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mechanism by which students access the Drive item."]
        pub share_mode: ::std::option::Option<SharedDriveFileShareModeEnum>,
    }
    impl SharedDriveFile {
        pub fn builder() -> SharedDriveFileBuilder {
            SharedDriveFileBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Mechanism by which students access the Drive item."]
    pub enum SharedDriveFileShareModeEnum {
        #[serde(rename = "UNKNOWN_SHARE_MODE")]
        #[doc = "No sharing mode specified. This should never be returned."]
        UnknownShareMode,
        #[serde(rename = "VIEW")]
        #[doc = "Students can view the shared file."]
        View,
        #[serde(rename = "EDIT")]
        #[doc = "Students can edit the shared file."]
        Edit,
        #[serde(rename = "STUDENT_COPY")]
        #[doc = "Students have a personal copy of the shared file."]
        StudentCopy,
    }
    impl ::std::default::Default for SharedDriveFileShareModeEnum {
        fn default() -> Self {
            Self::UnknownShareMode
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Student work for a short answer question."]
    pub struct ShortAnswerSubmission {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "answer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Student response to a short-answer question."]
        pub answer: ::std::option::Option<::std::string::String>,
    }
    impl ShortAnswerSubmission {
        pub fn builder() -> ShortAnswerSubmissionBuilder {
            ShortAnswerSubmissionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The history of each state this submission has been in."]
    pub struct StateHistory {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actorUserId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The teacher or student who made the change."]
        pub actor_user_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The workflow pipeline stage."]
        pub state: ::std::option::Option<StateHistoryStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the submission entered this state."]
        pub state_timestamp: ::std::option::Option<::std::string::String>,
    }
    impl StateHistory {
        pub fn builder() -> StateHistoryBuilder {
            StateHistoryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The workflow pipeline stage."]
    pub enum StateHistoryStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "No state specified. This should never be returned."]
        StateUnspecified,
        #[serde(rename = "CREATED")]
        #[doc = "The Submission has been created."]
        Created,
        #[serde(rename = "TURNED_IN")]
        #[doc = "The student has turned in an assigned document, which may or may not be a template."]
        TurnedIn,
        #[serde(rename = "RETURNED")]
        #[doc = "The teacher has returned the assigned document to the student."]
        Returned,
        #[serde(rename = "RECLAIMED_BY_STUDENT")]
        #[doc = "The student turned in the assigned document, and then chose to \"unsubmit\" the assignment, giving the student control again as the owner."]
        ReclaimedByStudent,
        #[serde(rename = "STUDENT_EDITED_AFTER_TURN_IN")]
        #[doc = "The student edited their submission after turning it in. Currently, only used by Questions, when the student edits their answer."]
        StudentEditedAfterTurnIn,
    }
    impl ::std::default::Default for StateHistoryStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Student in a course."]
    pub struct Student {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of the course. Read-only."]
        pub course_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Global user information for the student. Read-only."]
        pub profile: ::std::option::Option<::std::boxed::Box<UserProfile>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "studentWorkFolder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about a Drive Folder for this student's work in this course. Only visible to the student and domain administrators. Read-only."]
        pub student_work_folder: ::std::option::Option<::std::boxed::Box<DriveFolder>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of the user. When specified as a parameter of a request, this identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `\"me\"`, indicating the requesting user"]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl Student {
        pub fn builder() -> StudentBuilder {
            StudentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Student submission for course work. StudentSubmission items are generated when a CourseWork item is created. StudentSubmissions that have never been accessed (i.e. with `state` = NEW) may not have a creation time or update time."]
    pub struct StudentSubmission {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternateLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Absolute link to the submission in the Classroom web UI. Read-only."]
        pub alternate_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assignedGrade")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional grade. If unset, no grade was set. This value must be non-negative. Decimal (that is, non-integer) values are allowed, but are rounded to two decimal places. This may be modified only by course teachers."]
        pub assigned_grade: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assignmentSubmission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Submission content when course_work_type is ASSIGNMENT. Students can modify this content using ModifyAttachments."]
        pub assignment_submission: ::std::option::Option<::std::boxed::Box<AssignmentSubmission>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "associatedWithDeveloper")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this student submission is associated with the Developer Console project making the request. See CreateCourseWork for more details. Read-only."]
        pub associated_with_developer: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of the course. Read-only."]
        pub course_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseWorkId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier for the course work this corresponds to. Read-only."]
        pub course_work_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseWorkType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of course work this submission is for. Read-only."]
        pub course_work_type: ::std::option::Option<StudentSubmissionCourseWorkTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creation time of this submission. This may be unset if the student has not accessed this item. Read-only."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "draftGrade")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional pending grade. If unset, no grade was set. This value must be non-negative. Decimal (that is, non-integer) values are allowed, but are rounded to two decimal places. This is only visible to and modifiable by course teachers."]
        pub draft_grade: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Classroom-assigned Identifier for the student submission. This is unique among submissions for the relevant course work. Read-only."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "late")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this submission is late. Read-only."]
        pub late: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "multipleChoiceSubmission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Submission content when course_work_type is MULTIPLE_CHOICE_QUESTION."]
        pub multiple_choice_submission:
            ::std::option::Option<::std::boxed::Box<MultipleChoiceSubmission>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shortAnswerSubmission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Submission content when course_work_type is SHORT_ANSWER_QUESTION."]
        pub short_answer_submission:
            ::std::option::Option<::std::boxed::Box<ShortAnswerSubmission>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State of this submission. Read-only."]
        pub state: ::std::option::Option<StudentSubmissionStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "submissionHistory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The history of the submission (includes state and grade histories). Read-only."]
        pub submission_history:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SubmissionHistory>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last update time of this submission. This may be unset if the student has not accessed this item. Read-only."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier for the student that owns this submission. Read-only."]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl StudentSubmission {
        pub fn builder() -> StudentSubmissionBuilder {
            StudentSubmissionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of course work this submission is for. Read-only."]
    pub enum StudentSubmissionCourseWorkTypeEnum {
        #[serde(rename = "COURSE_WORK_TYPE_UNSPECIFIED")]
        #[doc = "No work type specified. This is never returned."]
        CourseWorkTypeUnspecified,
        #[serde(rename = "ASSIGNMENT")]
        #[doc = "An assignment."]
        Assignment,
        #[serde(rename = "SHORT_ANSWER_QUESTION")]
        #[doc = "A short answer question."]
        ShortAnswerQuestion,
        #[serde(rename = "MULTIPLE_CHOICE_QUESTION")]
        #[doc = "A multiple-choice question."]
        MultipleChoiceQuestion,
    }
    impl ::std::default::Default for StudentSubmissionCourseWorkTypeEnum {
        fn default() -> Self {
            Self::CourseWorkTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "State of this submission. Read-only."]
    pub enum StudentSubmissionStateEnum {
        #[serde(rename = "SUBMISSION_STATE_UNSPECIFIED")]
        #[doc = "No state specified. This should never be returned."]
        SubmissionStateUnspecified,
        #[serde(rename = "NEW")]
        #[doc = "The student has never accessed this submission. Attachments are not returned and timestamps is not set."]
        New,
        #[serde(rename = "CREATED")]
        #[doc = "Has been created."]
        Created,
        #[serde(rename = "TURNED_IN")]
        #[doc = "Has been turned in to the teacher."]
        TurnedIn,
        #[serde(rename = "RETURNED")]
        #[doc = "Has been returned to the student."]
        Returned,
        #[serde(rename = "RECLAIMED_BY_STUDENT")]
        #[doc = "Student chose to \"unsubmit\" the assignment."]
        ReclaimedByStudent,
    }
    impl ::std::default::Default for StudentSubmissionStateEnum {
        fn default() -> Self {
            Self::SubmissionStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The history of the submission. This currently includes state and grade histories."]
    pub struct SubmissionHistory {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gradeHistory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The grade history information of the submission, if present."]
        pub grade_history: ::std::option::Option<::std::boxed::Box<GradeHistory>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateHistory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state history information of the submission, if present."]
        pub state_history: ::std::option::Option<::std::boxed::Box<StateHistory>>,
    }
    impl SubmissionHistory {
        pub fn builder() -> SubmissionHistoryBuilder {
            SubmissionHistoryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Teacher of a course."]
    pub struct Teacher {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of the course. Read-only."]
        pub course_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Global user information for the teacher. Read-only."]
        pub profile: ::std::option::Option<::std::boxed::Box<UserProfile>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of the user. When specified as a parameter of a request, this identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `\"me\"`, indicating the requesting user"]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl Teacher {
        pub fn builder() -> TeacherBuilder {
            TeacherBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`."]
    pub struct TimeOfDay {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hours")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
        pub hours: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minutes of hour of day. Must be from 0 to 59."]
        pub minutes: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nanos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
        pub nanos: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds."]
        pub seconds: ::std::option::Option<::std::primitive::i64>,
    }
    impl TimeOfDay {
        pub fn builder() -> TimeOfDayBuilder {
            TimeOfDayBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Topic created by a teacher for the course"]
    pub struct Topic {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "courseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of the course. Read-only."]
        pub course_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the topic, generated by the user. Leading and trailing whitespaces, if any, are trimmed. Also, multiple consecutive whitespaces are collapsed into one inside the name. The result must be a non-empty string. Topic names are case sensitive, and must be no longer than 100 characters."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier for the topic. Read-only."]
        pub topic_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time the topic was last updated by the system. Read-only."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl Topic {
        pub fn builder() -> TopicBuilder {
            TopicBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to turn in a student submission."]
    pub struct TurnInStudentSubmissionRequest {}
    impl TurnInStudentSubmissionRequest {
        pub fn builder() -> TurnInStudentSubmissionRequestBuilder {
            TurnInStudentSubmissionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Global information for a user."]
    pub struct UserProfile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email address of the user. Read-only."]
        pub email_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of the user. Read-only."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the user. Read-only."]
        pub name: ::std::option::Option<::std::boxed::Box<Name>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Global permissions of the user. Read-only."]
        pub permissions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GlobalPermission>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of user's profile photo. Read-only."]
        pub photo_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verifiedTeacher")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents whether a G Suite for Education user's domain administrator has explicitly verified them as being a teacher. If the user is not a member of a G Suite for Education domain, than this field is always false. Read-only"]
        pub verified_teacher: ::std::option::Option<::std::primitive::bool>,
    }
    impl UserProfile {
        pub fn builder() -> UserProfileBuilder {
            UserProfileBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "YouTube video item."]
    pub struct YouTubeVideo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternateLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL that can be used to view the YouTube video. Read-only."]
        pub alternate_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "YouTube API resource ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of a thumbnail image of the YouTube video. Read-only."]
        pub thumbnail_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of the YouTube video. Read-only."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl YouTubeVideo {
        pub fn builder() -> YouTubeVideoBuilder {
            YouTubeVideoBuilder::default()
        }
    }
}
