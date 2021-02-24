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
    pub mod spreadsheets {
        pub mod methods {
            pub mod get {
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
                    #[serde(rename = "includeGridData")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "True if grid data should be returned. This parameter is ignored if a field mask was set in the request."]
                    pub include_grid_data: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ranges")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The ranges to retrieve from the spreadsheet."]
                    pub ranges: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
        pub mod resources {
            pub mod values {
                pub mod methods {
                    pub mod append {
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
                            #[serde(rename = "includeValuesInResponse")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Determines if the update response should include the values of the cells that were appended. By default, responses do not include the updated values."]
                            pub include_values_in_response:
                                ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "insertDataOption")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "How the input data should be inserted."]
                            pub insert_data_option:
                                ::std::option::Option<QueryParametersInsertDataOptionEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "responseDateTimeRenderOption")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is [DateTimeRenderOption.SERIAL_NUMBER]."]
                            pub response_date_time_render_option: ::std::option::Option<
                                QueryParametersResponseDateTimeRenderOptionEnum,
                            >,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "responseValueRenderOption")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Determines how values in the response should be rendered. The default render option is ValueRenderOption.FORMATTED_VALUE."]
                            pub response_value_render_option:
                                ::std::option::Option<QueryParametersResponseValueRenderOptionEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "valueInputOption")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "How the input data should be interpreted."]
                            pub value_input_option:
                                ::std::option::Option<QueryParametersValueInputOptionEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "How the input data should be inserted."]
                        pub enum QueryParametersInsertDataOptionEnum {
                            #[serde(rename = "OVERWRITE")]
                            #[doc = "The new data overwrites existing data in the areas it is written. (Note: adding data to the end of the sheet will still insert new rows or columns so the data can be written.)"]
                            Overwrite,
                            #[serde(rename = "INSERT_ROWS")]
                            #[doc = "Rows are inserted for the new data."]
                            InsertRows,
                        }
                        impl ::std::default::Default for QueryParametersInsertDataOptionEnum {
                            fn default() -> Self {
                                Self::Overwrite
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is [DateTimeRenderOption.SERIAL_NUMBER]."]
                        pub enum QueryParametersResponseDateTimeRenderOptionEnum {
                            #[serde(rename = "SERIAL_NUMBER")]
                            #[doc = "Instructs date, time, datetime, and duration fields to be output as doubles in \"serial number\" format, as popularized by Lotus 1-2-3. The whole number portion of the value (left of the decimal) counts the days since December 30th 1899. The fractional portion (right of the decimal) counts the time as a fraction of the day. For example, January 1st 1900 at noon would be 2.5, 2 because it's 2 days after December 30st 1899, and .5 because noon is half a day. February 1st 1900 at 3pm would be 33.625. This correctly treats the year 1900 as not a leap year."]
                            SerialNumber,
                            #[serde(rename = "FORMATTED_STRING")]
                            #[doc = "Instructs date, time, datetime, and duration fields to be output as strings in their given number format (which is dependent on the spreadsheet locale)."]
                            FormattedString,
                        }
                        impl ::std::default::Default for QueryParametersResponseDateTimeRenderOptionEnum {
                            fn default() -> Self {
                                Self::SerialNumber
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Determines how values in the response should be rendered. The default render option is ValueRenderOption.FORMATTED_VALUE."]
                        pub enum QueryParametersResponseValueRenderOptionEnum {
                            #[serde(rename = "FORMATTED_VALUE")]
                            #[doc = "Values will be calculated & formatted in the reply according to the cell's formatting. Formatting is based on the spreadsheet's locale, not the requesting user's locale. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return `\"$1.23\"`."]
                            FormattedValue,
                            #[serde(rename = "UNFORMATTED_VALUE")]
                            #[doc = "Values will be calculated, but not formatted in the reply. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return the number `1.23`."]
                            UnformattedValue,
                            #[serde(rename = "FORMULA")]
                            #[doc = "Values will not be calculated. The reply will include the formulas. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then A2 would return `\"=A1\"`."]
                            Formula,
                        }
                        impl ::std::default::Default for QueryParametersResponseValueRenderOptionEnum {
                            fn default() -> Self {
                                Self::FormattedValue
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "How the input data should be interpreted."]
                        pub enum QueryParametersValueInputOptionEnum {
                            #[serde(rename = "INPUT_VALUE_OPTION_UNSPECIFIED")]
                            #[doc = "Default input value. This value must not be used."]
                            InputValueOptionUnspecified,
                            #[serde(rename = "RAW")]
                            #[doc = "The values the user has entered will not be parsed and will be stored as-is."]
                            Raw,
                            #[serde(rename = "USER_ENTERED")]
                            #[doc = "The values will be parsed as if the user typed them into the UI. Numbers will stay as numbers, but strings may be converted to numbers, dates, etc. following the same rules that are applied when entering text into a cell via the Google Sheets UI."]
                            UserEntered,
                        }
                        impl ::std::default::Default for QueryParametersValueInputOptionEnum {
                            fn default() -> Self {
                                Self::InputValueOptionUnspecified
                            }
                        }
                    }
                    pub mod batch_get {
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
                            #[serde(rename = "dateTimeRenderOption")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is [DateTimeRenderOption.SERIAL_NUMBER]."]
                            pub date_time_render_option:
                                ::std::option::Option<QueryParametersDateTimeRenderOptionEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "majorDimension")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The major dimension that results should use. For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then requesting `range=A1:B2,majorDimension=ROWS` returns `[[1,2],[3,4]]`, whereas requesting `range=A1:B2,majorDimension=COLUMNS` returns `[[1,3],[2,4]]`."]
                            pub major_dimension:
                                ::std::option::Option<QueryParametersMajorDimensionEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "ranges")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The A1 notation of the values to retrieve."]
                            pub ranges: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "valueRenderOption")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "How values should be represented in the output. The default render option is ValueRenderOption.FORMATTED_VALUE."]
                            pub value_render_option:
                                ::std::option::Option<QueryParametersValueRenderOptionEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is [DateTimeRenderOption.SERIAL_NUMBER]."]
                        pub enum QueryParametersDateTimeRenderOptionEnum {
                            #[serde(rename = "SERIAL_NUMBER")]
                            #[doc = "Instructs date, time, datetime, and duration fields to be output as doubles in \"serial number\" format, as popularized by Lotus 1-2-3. The whole number portion of the value (left of the decimal) counts the days since December 30th 1899. The fractional portion (right of the decimal) counts the time as a fraction of the day. For example, January 1st 1900 at noon would be 2.5, 2 because it's 2 days after December 30st 1899, and .5 because noon is half a day. February 1st 1900 at 3pm would be 33.625. This correctly treats the year 1900 as not a leap year."]
                            SerialNumber,
                            #[serde(rename = "FORMATTED_STRING")]
                            #[doc = "Instructs date, time, datetime, and duration fields to be output as strings in their given number format (which is dependent on the spreadsheet locale)."]
                            FormattedString,
                        }
                        impl ::std::default::Default for QueryParametersDateTimeRenderOptionEnum {
                            fn default() -> Self {
                                Self::SerialNumber
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The major dimension that results should use. For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then requesting `range=A1:B2,majorDimension=ROWS` returns `[[1,2],[3,4]]`, whereas requesting `range=A1:B2,majorDimension=COLUMNS` returns `[[1,3],[2,4]]`."]
                        pub enum QueryParametersMajorDimensionEnum {
                            #[serde(rename = "DIMENSION_UNSPECIFIED")]
                            #[doc = "The default value, do not use."]
                            DimensionUnspecified,
                            #[serde(rename = "ROWS")]
                            #[doc = "Operates on the rows of a sheet."]
                            Rows,
                            #[serde(rename = "COLUMNS")]
                            #[doc = "Operates on the columns of a sheet."]
                            Columns,
                        }
                        impl ::std::default::Default for QueryParametersMajorDimensionEnum {
                            fn default() -> Self {
                                Self::DimensionUnspecified
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "How values should be represented in the output. The default render option is ValueRenderOption.FORMATTED_VALUE."]
                        pub enum QueryParametersValueRenderOptionEnum {
                            #[serde(rename = "FORMATTED_VALUE")]
                            #[doc = "Values will be calculated & formatted in the reply according to the cell's formatting. Formatting is based on the spreadsheet's locale, not the requesting user's locale. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return `\"$1.23\"`."]
                            FormattedValue,
                            #[serde(rename = "UNFORMATTED_VALUE")]
                            #[doc = "Values will be calculated, but not formatted in the reply. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return the number `1.23`."]
                            UnformattedValue,
                            #[serde(rename = "FORMULA")]
                            #[doc = "Values will not be calculated. The reply will include the formulas. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then A2 would return `\"=A1\"`."]
                            Formula,
                        }
                        impl ::std::default::Default for QueryParametersValueRenderOptionEnum {
                            fn default() -> Self {
                                Self::FormattedValue
                            }
                        }
                    }
                    pub mod get {
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
                            #[serde(rename = "dateTimeRenderOption")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is [DateTimeRenderOption.SERIAL_NUMBER]."]
                            pub date_time_render_option:
                                ::std::option::Option<QueryParametersDateTimeRenderOptionEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "majorDimension")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The major dimension that results should use. For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then requesting `range=A1:B2,majorDimension=ROWS` returns `[[1,2],[3,4]]`, whereas requesting `range=A1:B2,majorDimension=COLUMNS` returns `[[1,3],[2,4]]`."]
                            pub major_dimension:
                                ::std::option::Option<QueryParametersMajorDimensionEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "valueRenderOption")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "How values should be represented in the output. The default render option is ValueRenderOption.FORMATTED_VALUE."]
                            pub value_render_option:
                                ::std::option::Option<QueryParametersValueRenderOptionEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is [DateTimeRenderOption.SERIAL_NUMBER]."]
                        pub enum QueryParametersDateTimeRenderOptionEnum {
                            #[serde(rename = "SERIAL_NUMBER")]
                            #[doc = "Instructs date, time, datetime, and duration fields to be output as doubles in \"serial number\" format, as popularized by Lotus 1-2-3. The whole number portion of the value (left of the decimal) counts the days since December 30th 1899. The fractional portion (right of the decimal) counts the time as a fraction of the day. For example, January 1st 1900 at noon would be 2.5, 2 because it's 2 days after December 30st 1899, and .5 because noon is half a day. February 1st 1900 at 3pm would be 33.625. This correctly treats the year 1900 as not a leap year."]
                            SerialNumber,
                            #[serde(rename = "FORMATTED_STRING")]
                            #[doc = "Instructs date, time, datetime, and duration fields to be output as strings in their given number format (which is dependent on the spreadsheet locale)."]
                            FormattedString,
                        }
                        impl ::std::default::Default for QueryParametersDateTimeRenderOptionEnum {
                            fn default() -> Self {
                                Self::SerialNumber
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The major dimension that results should use. For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then requesting `range=A1:B2,majorDimension=ROWS` returns `[[1,2],[3,4]]`, whereas requesting `range=A1:B2,majorDimension=COLUMNS` returns `[[1,3],[2,4]]`."]
                        pub enum QueryParametersMajorDimensionEnum {
                            #[serde(rename = "DIMENSION_UNSPECIFIED")]
                            #[doc = "The default value, do not use."]
                            DimensionUnspecified,
                            #[serde(rename = "ROWS")]
                            #[doc = "Operates on the rows of a sheet."]
                            Rows,
                            #[serde(rename = "COLUMNS")]
                            #[doc = "Operates on the columns of a sheet."]
                            Columns,
                        }
                        impl ::std::default::Default for QueryParametersMajorDimensionEnum {
                            fn default() -> Self {
                                Self::DimensionUnspecified
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "How values should be represented in the output. The default render option is ValueRenderOption.FORMATTED_VALUE."]
                        pub enum QueryParametersValueRenderOptionEnum {
                            #[serde(rename = "FORMATTED_VALUE")]
                            #[doc = "Values will be calculated & formatted in the reply according to the cell's formatting. Formatting is based on the spreadsheet's locale, not the requesting user's locale. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return `\"$1.23\"`."]
                            FormattedValue,
                            #[serde(rename = "UNFORMATTED_VALUE")]
                            #[doc = "Values will be calculated, but not formatted in the reply. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return the number `1.23`."]
                            UnformattedValue,
                            #[serde(rename = "FORMULA")]
                            #[doc = "Values will not be calculated. The reply will include the formulas. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then A2 would return `\"=A1\"`."]
                            Formula,
                        }
                        impl ::std::default::Default for QueryParametersValueRenderOptionEnum {
                            fn default() -> Self {
                                Self::FormattedValue
                            }
                        }
                    }
                    pub mod update {
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
                            #[serde(rename = "includeValuesInResponse")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Determines if the update response should include the values of the cells that were updated. By default, responses do not include the updated values. If the range to write was larger than the range actually written, the response includes all values in the requested range (excluding trailing empty rows and columns)."]
                            pub include_values_in_response:
                                ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "responseDateTimeRenderOption")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is DateTimeRenderOption.SERIAL_NUMBER."]
                            pub response_date_time_render_option: ::std::option::Option<
                                QueryParametersResponseDateTimeRenderOptionEnum,
                            >,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "responseValueRenderOption")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Determines how values in the response should be rendered. The default render option is ValueRenderOption.FORMATTED_VALUE."]
                            pub response_value_render_option:
                                ::std::option::Option<QueryParametersResponseValueRenderOptionEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "valueInputOption")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "How the input data should be interpreted."]
                            pub value_input_option:
                                ::std::option::Option<QueryParametersValueInputOptionEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is DateTimeRenderOption.SERIAL_NUMBER."]
                        pub enum QueryParametersResponseDateTimeRenderOptionEnum {
                            #[serde(rename = "SERIAL_NUMBER")]
                            #[doc = "Instructs date, time, datetime, and duration fields to be output as doubles in \"serial number\" format, as popularized by Lotus 1-2-3. The whole number portion of the value (left of the decimal) counts the days since December 30th 1899. The fractional portion (right of the decimal) counts the time as a fraction of the day. For example, January 1st 1900 at noon would be 2.5, 2 because it's 2 days after December 30st 1899, and .5 because noon is half a day. February 1st 1900 at 3pm would be 33.625. This correctly treats the year 1900 as not a leap year."]
                            SerialNumber,
                            #[serde(rename = "FORMATTED_STRING")]
                            #[doc = "Instructs date, time, datetime, and duration fields to be output as strings in their given number format (which is dependent on the spreadsheet locale)."]
                            FormattedString,
                        }
                        impl ::std::default::Default for QueryParametersResponseDateTimeRenderOptionEnum {
                            fn default() -> Self {
                                Self::SerialNumber
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Determines how values in the response should be rendered. The default render option is ValueRenderOption.FORMATTED_VALUE."]
                        pub enum QueryParametersResponseValueRenderOptionEnum {
                            #[serde(rename = "FORMATTED_VALUE")]
                            #[doc = "Values will be calculated & formatted in the reply according to the cell's formatting. Formatting is based on the spreadsheet's locale, not the requesting user's locale. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return `\"$1.23\"`."]
                            FormattedValue,
                            #[serde(rename = "UNFORMATTED_VALUE")]
                            #[doc = "Values will be calculated, but not formatted in the reply. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return the number `1.23`."]
                            UnformattedValue,
                            #[serde(rename = "FORMULA")]
                            #[doc = "Values will not be calculated. The reply will include the formulas. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then A2 would return `\"=A1\"`."]
                            Formula,
                        }
                        impl ::std::default::Default for QueryParametersResponseValueRenderOptionEnum {
                            fn default() -> Self {
                                Self::FormattedValue
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "How the input data should be interpreted."]
                        pub enum QueryParametersValueInputOptionEnum {
                            #[serde(rename = "INPUT_VALUE_OPTION_UNSPECIFIED")]
                            #[doc = "Default input value. This value must not be used."]
                            InputValueOptionUnspecified,
                            #[serde(rename = "RAW")]
                            #[doc = "The values the user has entered will not be parsed and will be stored as-is."]
                            Raw,
                            #[serde(rename = "USER_ENTERED")]
                            #[doc = "The values will be parsed as if the user typed them into the UI. Numbers will stay as numbers, but strings may be converted to numbers, dates, etc. following the same rules that are applied when entering text into a cell via the Google Sheets UI."]
                            UserEntered,
                        }
                        impl ::std::default::Default for QueryParametersValueInputOptionEnum {
                            fn default() -> Self {
                                Self::InputValueOptionUnspecified
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
    #[doc = "Adds a new banded range to the spreadsheet."]
    pub struct AddBandingRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bandedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The banded range to add. The bandedRangeId field is optional; if one is not set, an id will be randomly generated. (It is an error to specify the ID of a range that already exists.)"]
        pub banded_range: ::std::option::Option<::std::boxed::Box<BandedRange>>,
    }
    impl AddBandingRequest {
        pub fn builder() -> AddBandingRequestBuilder {
            AddBandingRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of adding a banded range."]
    pub struct AddBandingResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bandedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The banded range that was added."]
        pub banded_range: ::std::option::Option<::std::boxed::Box<BandedRange>>,
    }
    impl AddBandingResponse {
        pub fn builder() -> AddBandingResponseBuilder {
            AddBandingResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Adds a chart to a sheet in the spreadsheet."]
    pub struct AddChartRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The chart that should be added to the spreadsheet, including the position where it should be placed. The chartId field is optional; if one is not set, an id will be randomly generated. (It is an error to specify the ID of an embedded object that already exists.)"]
        pub chart: ::std::option::Option<::std::boxed::Box<EmbeddedChart>>,
    }
    impl AddChartRequest {
        pub fn builder() -> AddChartRequestBuilder {
            AddChartRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of adding a chart to a spreadsheet."]
    pub struct AddChartResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The newly added chart."]
        pub chart: ::std::option::Option<::std::boxed::Box<EmbeddedChart>>,
    }
    impl AddChartResponse {
        pub fn builder() -> AddChartResponseBuilder {
            AddChartResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Adds a new conditional format rule at the given index. All subsequent rules' indexes are incremented."]
    pub struct AddConditionalFormatRuleRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zero-based index where the rule should be inserted."]
        pub index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rule to add."]
        pub rule: ::std::option::Option<::std::boxed::Box<ConditionalFormatRule>>,
    }
    impl AddConditionalFormatRuleRequest {
        pub fn builder() -> AddConditionalFormatRuleRequestBuilder {
            AddConditionalFormatRuleRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Adds a data source. After the data source is added successfully, an associated DATA_SOURCE sheet is created and an execution is triggered to refresh the sheet to read data from the data source. The request requires an additional `bigquery.readonly` OAuth scope."]
    pub struct AddDataSourceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data source to add."]
        pub data_source: ::std::option::Option<::std::boxed::Box<DataSource>>,
    }
    impl AddDataSourceRequest {
        pub fn builder() -> AddDataSourceRequestBuilder {
            AddDataSourceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of adding a data source."]
    pub struct AddDataSourceResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataExecutionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data execution status."]
        pub data_execution_status: ::std::option::Option<::std::boxed::Box<DataExecutionStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data source that was created."]
        pub data_source: ::std::option::Option<::std::boxed::Box<DataSource>>,
    }
    impl AddDataSourceResponse {
        pub fn builder() -> AddDataSourceResponseBuilder {
            AddDataSourceResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Creates a group over the specified range. If the requested range is a superset of the range of an existing group G, then the depth of G is incremented and this new group G' has the depth of that group. For example, a group [C:D, depth 1] + [B:E] results in groups [B:E, depth 1] and [C:D, depth 2]. If the requested range is a subset of the range of an existing group G, then the depth of the new group G' becomes one greater than the depth of G. For example, a group [B:E, depth 1] + [C:D] results in groups [B:E, depth 1] and [C:D, depth 2]. If the requested range starts before and ends within, or starts within and ends after, the range of an existing group G, then the range of the existing group G becomes the union of the ranges, and the new group G' has depth one greater than the depth of G and range as the intersection of the ranges. For example, a group [B:D, depth 1] + [C:E] results in groups [B:E, depth 1] and [C:D, depth 2]."]
    pub struct AddDimensionGroupRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range over which to create a group."]
        pub range: ::std::option::Option<::std::boxed::Box<DimensionRange>>,
    }
    impl AddDimensionGroupRequest {
        pub fn builder() -> AddDimensionGroupRequestBuilder {
            AddDimensionGroupRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of adding a group."]
    pub struct AddDimensionGroupResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionGroups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All groups of a dimension after adding a group to that dimension."]
        pub dimension_groups:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionGroup>>>,
    }
    impl AddDimensionGroupResponse {
        pub fn builder() -> AddDimensionGroupResponseBuilder {
            AddDimensionGroupResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Adds a filter view."]
    pub struct AddFilterViewRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filter to add. The filterViewId field is optional; if one is not set, an id will be randomly generated. (It is an error to specify the ID of a filter that already exists.)"]
        pub filter: ::std::option::Option<::std::boxed::Box<FilterView>>,
    }
    impl AddFilterViewRequest {
        pub fn builder() -> AddFilterViewRequestBuilder {
            AddFilterViewRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of adding a filter view."]
    pub struct AddFilterViewResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The newly added filter view."]
        pub filter: ::std::option::Option<::std::boxed::Box<FilterView>>,
    }
    impl AddFilterViewResponse {
        pub fn builder() -> AddFilterViewResponseBuilder {
            AddFilterViewResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Adds a named range to the spreadsheet."]
    pub struct AddNamedRangeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The named range to add. The namedRangeId field is optional; if one is not set, an id will be randomly generated. (It is an error to specify the ID of a range that already exists.)"]
        pub named_range: ::std::option::Option<::std::boxed::Box<NamedRange>>,
    }
    impl AddNamedRangeRequest {
        pub fn builder() -> AddNamedRangeRequestBuilder {
            AddNamedRangeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of adding a named range."]
    pub struct AddNamedRangeResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The named range to add."]
        pub named_range: ::std::option::Option<::std::boxed::Box<NamedRange>>,
    }
    impl AddNamedRangeResponse {
        pub fn builder() -> AddNamedRangeResponseBuilder {
            AddNamedRangeResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Adds a new protected range."]
    pub struct AddProtectedRangeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protectedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The protected range to be added. The protectedRangeId field is optional; if one is not set, an id will be randomly generated. (It is an error to specify the ID of a range that already exists.)"]
        pub protected_range: ::std::option::Option<::std::boxed::Box<ProtectedRange>>,
    }
    impl AddProtectedRangeRequest {
        pub fn builder() -> AddProtectedRangeRequestBuilder {
            AddProtectedRangeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of adding a new protected range."]
    pub struct AddProtectedRangeResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protectedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The newly added protected range."]
        pub protected_range: ::std::option::Option<::std::boxed::Box<ProtectedRange>>,
    }
    impl AddProtectedRangeResponse {
        pub fn builder() -> AddProtectedRangeResponseBuilder {
            AddProtectedRangeResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Adds a new sheet. When a sheet is added at a given index, all subsequent sheets' indexes are incremented. To add an object sheet, use AddChartRequest instead and specify EmbeddedObjectPosition.sheetId or EmbeddedObjectPosition.newSheet."]
    pub struct AddSheetRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties the new sheet should have. All properties are optional. The sheetId field is optional; if one is not set, an id will be randomly generated. (It is an error to specify the ID of a sheet that already exists.)"]
        pub properties: ::std::option::Option<::std::boxed::Box<SheetProperties>>,
    }
    impl AddSheetRequest {
        pub fn builder() -> AddSheetRequestBuilder {
            AddSheetRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of adding a sheet."]
    pub struct AddSheetResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties of the newly added sheet."]
        pub properties: ::std::option::Option<::std::boxed::Box<SheetProperties>>,
    }
    impl AddSheetResponse {
        pub fn builder() -> AddSheetResponseBuilder {
            AddSheetResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Adds a slicer to a sheet in the spreadsheet."]
    pub struct AddSlicerRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slicer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The slicer that should be added to the spreadsheet, including the position where it should be placed. The slicerId field is optional; if one is not set, an id will be randomly generated. (It is an error to specify the ID of a slicer that already exists.)"]
        pub slicer: ::std::option::Option<::std::boxed::Box<Slicer>>,
    }
    impl AddSlicerRequest {
        pub fn builder() -> AddSlicerRequestBuilder {
            AddSlicerRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of adding a slicer to a spreadsheet."]
    pub struct AddSlicerResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slicer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The newly added slicer."]
        pub slicer: ::std::option::Option<::std::boxed::Box<Slicer>>,
    }
    impl AddSlicerResponse {
        pub fn builder() -> AddSlicerResponseBuilder {
            AddSlicerResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Adds new cells after the last row with data in a sheet, inserting new rows into the sheet if necessary."]
    pub struct AppendCellsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `\"*\"` can be used as short-hand for listing every field."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data to append."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RowData>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sheet ID to append the data to."]
        pub sheet_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl AppendCellsRequest {
        pub fn builder() -> AppendCellsRequestBuilder {
            AppendCellsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Appends rows or columns to the end of a sheet."]
    pub struct AppendDimensionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether rows or columns should be appended."]
        pub dimension: ::std::option::Option<AppendDimensionRequestDimensionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "length")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of rows or columns to append."]
        pub length: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sheet to append rows or columns to."]
        pub sheet_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl AppendDimensionRequest {
        pub fn builder() -> AppendDimensionRequestBuilder {
            AppendDimensionRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Whether rows or columns should be appended."]
    pub enum AppendDimensionRequestDimensionEnum {
        #[serde(rename = "DIMENSION_UNSPECIFIED")]
        #[doc = "The default value, do not use."]
        DimensionUnspecified,
        #[serde(rename = "ROWS")]
        #[doc = "Operates on the rows of a sheet."]
        Rows,
        #[serde(rename = "COLUMNS")]
        #[doc = "Operates on the columns of a sheet."]
        Columns,
    }
    impl ::std::default::Default for AppendDimensionRequestDimensionEnum {
        fn default() -> Self {
            Self::DimensionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response when updating a range of values in a spreadsheet."]
    pub struct AppendValuesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spreadsheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The spreadsheet the updates were applied to."]
        pub spreadsheet_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range (in A1 notation) of the table that values are being appended to (before the values were appended). Empty if no table was found."]
        pub table_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the updates that were applied."]
        pub updates: ::std::option::Option<::std::boxed::Box<UpdateValuesResponse>>,
    }
    impl AppendValuesResponse {
        pub fn builder() -> AppendValuesResponseBuilder {
            AppendValuesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Fills in more data based on existing data."]
    pub struct AutoFillRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range to autofill. This will examine the range and detect the location that has data and automatically fill that data in to the rest of the range."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceAndDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source and destination areas to autofill. This explicitly lists the source of the autofill and where to extend that data."]
        pub source_and_destination: ::std::option::Option<::std::boxed::Box<SourceAndDestination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useAlternateSeries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if we should generate data with the \"alternate\" series. This differs based on the type and amount of source data."]
        pub use_alternate_series: ::std::option::Option<::std::primitive::bool>,
    }
    impl AutoFillRequest {
        pub fn builder() -> AutoFillRequestBuilder {
            AutoFillRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Automatically resizes one or more dimensions based on the contents of the cells in that dimension."]
    pub struct AutoResizeDimensionsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceSheetDimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimensions on a data source sheet to automatically resize."]
        pub data_source_sheet_dimensions:
            ::std::option::Option<::std::boxed::Box<DataSourceSheetDimensionRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimensions to automatically resize."]
        pub dimensions: ::std::option::Option<::std::boxed::Box<DimensionRange>>,
    }
    impl AutoResizeDimensionsRequest {
        pub fn builder() -> AutoResizeDimensionsRequestBuilder {
            AutoResizeDimensionsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A banded (alternating colors) range in a sheet."]
    pub struct BandedRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bandedRangeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of the banded range."]
        pub banded_range_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Properties for column bands. These properties are applied on a column- by-column basis throughout all the columns in the range. At least one of row_properties or column_properties must be specified."]
        pub column_properties: ::std::option::Option<::std::boxed::Box<BandingProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range over which these properties are applied."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Properties for row bands. These properties are applied on a row-by-row basis throughout all the rows in the range. At least one of row_properties or column_properties must be specified."]
        pub row_properties: ::std::option::Option<::std::boxed::Box<BandingProperties>>,
    }
    impl BandedRange {
        pub fn builder() -> BandedRangeBuilder {
            BandedRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Properties referring a single dimension (either row or column). If both BandedRange.row_properties and BandedRange.column_properties are set, the fill colors are applied to cells according to the following rules: * header_color and footer_color take priority over band colors. * first_band_color takes priority over second_band_color. * row_properties takes priority over column_properties. For example, the first row color takes priority over the first column color, but the first column color takes priority over the second row color. Similarly, the row header takes priority over the column header in the top left cell, but the column header takes priority over the first row color if the row header is not set."]
    pub struct BandingProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstBandColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The first color that is alternating. (Required)"]
        pub first_band_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstBandColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The first color that is alternating. (Required) If first_band_color is also set, this field takes precedence."]
        pub first_band_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "footerColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the last row or column. If this field is not set, the last row or column is filled with either first_band_color or second_band_color, depending on the color of the previous row or column."]
        pub footer_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "footerColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the last row or column. If this field is not set, the last row or column is filled with either first_band_color or second_band_color, depending on the color of the previous row or column. If footer_color is also set, this field takes precedence."]
        pub footer_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headerColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the first row or column. If this field is set, the first row or column is filled with this color and the colors alternate between first_band_color and second_band_color starting from the second row or column. Otherwise, the first row or column is filled with first_band_color and the colors proceed to alternate as they normally would."]
        pub header_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headerColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the first row or column. If this field is set, the first row or column is filled with this color and the colors alternate between first_band_color and second_band_color starting from the second row or column. Otherwise, the first row or column is filled with first_band_color and the colors proceed to alternate as they normally would. If header_color is also set, this field takes precedence."]
        pub header_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secondBandColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The second color that is alternating. (Required)"]
        pub second_band_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secondBandColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The second color that is alternating. (Required) If second_band_color is also set, this field takes precedence."]
        pub second_band_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
    }
    impl BandingProperties {
        pub fn builder() -> BandingPropertiesBuilder {
            BandingPropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Formatting options for baseline value."]
    pub struct BaselineValueFormat {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comparisonType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The comparison type of key value with baseline value."]
        pub comparison_type: ::std::option::Option<BaselineValueFormatComparisonTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description which is appended after the baseline value. This field is optional."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "negativeColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Color to be used, in case baseline value represents a negative change for key value. This field is optional."]
        pub negative_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "negativeColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Color to be used, in case baseline value represents a negative change for key value. This field is optional. If negative_color is also set, this field takes precedence."]
        pub negative_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the horizontal text positioning of baseline value. This field is optional. If not specified, default positioning is used."]
        pub position: ::std::option::Option<::std::boxed::Box<TextPosition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "positiveColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Color to be used, in case baseline value represents a positive change for key value. This field is optional."]
        pub positive_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "positiveColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Color to be used, in case baseline value represents a positive change for key value. This field is optional. If positive_color is also set, this field takes precedence."]
        pub positive_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text formatting options for baseline value."]
        pub text_format: ::std::option::Option<::std::boxed::Box<TextFormat>>,
    }
    impl BaselineValueFormat {
        pub fn builder() -> BaselineValueFormatBuilder {
            BaselineValueFormatBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The comparison type of key value with baseline value."]
    pub enum BaselineValueFormatComparisonTypeEnum {
        #[serde(rename = "COMPARISON_TYPE_UNDEFINED")]
        #[doc = "Default value, do not use."]
        ComparisonTypeUndefined,
        #[serde(rename = "ABSOLUTE_DIFFERENCE")]
        #[doc = "Use absolute difference between key and baseline value."]
        AbsoluteDifference,
        #[serde(rename = "PERCENTAGE_DIFFERENCE")]
        #[doc = "Use percentage difference between key and baseline value."]
        PercentageDifference,
    }
    impl ::std::default::Default for BaselineValueFormatComparisonTypeEnum {
        fn default() -> Self {
            Self::ComparisonTypeUndefined
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An axis of the chart. A chart may not have more than one axis per axis position."]
    pub struct BasicChartAxis {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "format")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The format of the title. Only valid if the axis is not associated with the domain."]
        pub format: ::std::option::Option<::std::boxed::Box<TextFormat>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The position of this axis."]
        pub position: ::std::option::Option<BasicChartAxisPositionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of this axis. If set, this overrides any title inferred from headers of the data."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "titleTextPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The axis title text position."]
        pub title_text_position: ::std::option::Option<::std::boxed::Box<TextPosition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewWindowOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The view window options for this axis."]
        pub view_window_options:
            ::std::option::Option<::std::boxed::Box<ChartAxisViewWindowOptions>>,
    }
    impl BasicChartAxis {
        pub fn builder() -> BasicChartAxisBuilder {
            BasicChartAxisBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The position of this axis."]
    pub enum BasicChartAxisPositionEnum {
        #[serde(rename = "BASIC_CHART_AXIS_POSITION_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        BasicChartAxisPositionUnspecified,
        #[serde(rename = "BOTTOM_AXIS")]
        #[doc = "The axis rendered at the bottom of a chart. For most charts, this is the standard major axis. For bar charts, this is a minor axis."]
        BottomAxis,
        #[serde(rename = "LEFT_AXIS")]
        #[doc = "The axis rendered at the left of a chart. For most charts, this is a minor axis. For bar charts, this is the standard major axis."]
        LeftAxis,
        #[serde(rename = "RIGHT_AXIS")]
        #[doc = "The axis rendered at the right of a chart. For most charts, this is a minor axis. For bar charts, this is an unusual major axis."]
        RightAxis,
    }
    impl ::std::default::Default for BasicChartAxisPositionEnum {
        fn default() -> Self {
            Self::BasicChartAxisPositionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The domain of a chart. For example, if charting stock prices over time, this would be the date."]
    pub struct BasicChartDomain {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data of the domain. For example, if charting stock prices over time, this is the data representing the dates."]
        pub domain: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reversed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True to reverse the order of the domain values (horizontal axis)."]
        pub reversed: ::std::option::Option<::std::primitive::bool>,
    }
    impl BasicChartDomain {
        pub fn builder() -> BasicChartDomainBuilder {
            BasicChartDomainBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single series of data in a chart. For example, if charting stock prices over time, multiple series may exist, one for the \"Open Price\", \"High Price\", \"Low Price\" and \"Close Price\"."]
    pub struct BasicChartSeries {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color for elements (such as bars, lines, and points) associated with this series. If empty, a default color is used."]
        pub color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color for elements (such as bars, lines, and points) associated with this series. If empty, a default color is used. If color is also set, this field takes precedence."]
        pub color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the data labels for this series."]
        pub data_label: ::std::option::Option<::std::boxed::Box<DataLabel>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The line style of this series. Valid only if the chartType is AREA, LINE, or SCATTER. COMBO charts are also supported if the series chart type is AREA or LINE."]
        pub line_style: ::std::option::Option<::std::boxed::Box<LineStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pointStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The style for points associated with this series. Valid only if the chartType is AREA, LINE, or SCATTER. COMBO charts are also supported if the series chart type is AREA, LINE, or SCATTER. If empty, a default point style is used."]
        pub point_style: ::std::option::Option<::std::boxed::Box<PointStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "series")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data being visualized in this chart series."]
        pub series: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "styleOverrides")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Style override settings for series data points."]
        pub style_overrides: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<BasicSeriesDataPointStyleOverride>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetAxis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minor axis that will specify the range of values for this series. For example, if charting stocks over time, the \"Volume\" series may want to be pinned to the right with the prices pinned to the left, because the scale of trading volume is different than the scale of prices. It is an error to specify an axis that isn't a valid minor axis for the chart's type."]
        pub target_axis: ::std::option::Option<BasicChartSeriesTargetAxisEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of this series. Valid only if the chartType is COMBO. Different types will change the way the series is visualized. Only LINE, AREA, and COLUMN are supported."]
        pub _type: ::std::option::Option<BasicChartSeriesTypeEnum>,
    }
    impl BasicChartSeries {
        pub fn builder() -> BasicChartSeriesBuilder {
            BasicChartSeriesBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The minor axis that will specify the range of values for this series. For example, if charting stocks over time, the \"Volume\" series may want to be pinned to the right with the prices pinned to the left, because the scale of trading volume is different than the scale of prices. It is an error to specify an axis that isn't a valid minor axis for the chart's type."]
    pub enum BasicChartSeriesTargetAxisEnum {
        #[serde(rename = "BASIC_CHART_AXIS_POSITION_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        BasicChartAxisPositionUnspecified,
        #[serde(rename = "BOTTOM_AXIS")]
        #[doc = "The axis rendered at the bottom of a chart. For most charts, this is the standard major axis. For bar charts, this is a minor axis."]
        BottomAxis,
        #[serde(rename = "LEFT_AXIS")]
        #[doc = "The axis rendered at the left of a chart. For most charts, this is a minor axis. For bar charts, this is the standard major axis."]
        LeftAxis,
        #[serde(rename = "RIGHT_AXIS")]
        #[doc = "The axis rendered at the right of a chart. For most charts, this is a minor axis. For bar charts, this is an unusual major axis."]
        RightAxis,
    }
    impl ::std::default::Default for BasicChartSeriesTargetAxisEnum {
        fn default() -> Self {
            Self::BasicChartAxisPositionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of this series. Valid only if the chartType is COMBO. Different types will change the way the series is visualized. Only LINE, AREA, and COLUMN are supported."]
    pub enum BasicChartSeriesTypeEnum {
        #[serde(rename = "BASIC_CHART_TYPE_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        BasicChartTypeUnspecified,
        #[serde(rename = "BAR")]
        #[doc = "A bar chart."]
        Bar,
        #[serde(rename = "LINE")]
        #[doc = "A line chart."]
        Line,
        #[serde(rename = "AREA")]
        #[doc = "An area chart."]
        Area,
        #[serde(rename = "COLUMN")]
        #[doc = "A column chart."]
        Column,
        #[serde(rename = "SCATTER")]
        #[doc = "A scatter chart."]
        Scatter,
        #[serde(rename = "COMBO")]
        #[doc = "A combo chart."]
        Combo,
        #[serde(rename = "STEPPED_AREA")]
        #[doc = "A stepped area chart."]
        SteppedArea,
    }
    impl ::std::default::Default for BasicChartSeriesTypeEnum {
        fn default() -> Self {
            Self::BasicChartTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The specification for a basic chart. See BasicChartType for the list of charts this supports."]
    pub struct BasicChartSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "axis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The axis on the chart."]
        pub axis: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BasicChartAxis>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chartType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the chart."]
        pub chart_type: ::std::option::Option<BasicChartSpecChartTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compareMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The behavior of tooltips and data highlighting when hovering on data and chart area."]
        pub compare_mode: ::std::option::Option<BasicChartSpecCompareModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domains")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain of data this is charting. Only a single domain is supported."]
        pub domains: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BasicChartDomain>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headerCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of rows or columns in the data that are \"headers\". If not set, Google Sheets will guess how many rows are headers based on the data. (Note that BasicChartAxis.title may override the axis title inferred from the header values.)"]
        pub header_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "interpolateNulls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If some values in a series are missing, gaps may appear in the chart (e.g, segments of lines in a line chart will be missing). To eliminate these gaps set this to true. Applies to Line, Area, and Combo charts."]
        pub interpolate_nulls: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "legendPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The position of the chart legend."]
        pub legend_position: ::std::option::Option<BasicChartSpecLegendPositionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineSmoothing")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Gets whether all lines should be rendered smooth or straight by default. Applies to Line charts."]
        pub line_smoothing: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "series")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data this chart is visualizing."]
        pub series: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BasicChartSeries>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stackedType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stacked type for charts that support vertical stacking. Applies to Area, Bar, Column, Combo, and Stepped Area charts."]
        pub stacked_type: ::std::option::Option<BasicChartSpecStackedTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threeDimensional")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True to make the chart 3D. Applies to Bar and Column charts."]
        pub three_dimensional: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalDataLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Controls whether to display additional data labels on stacked charts which sum the total value of all stacked values at each value along the domain axis. These data labels can only be set when chart_type is one of AREA, BAR, COLUMN, COMBO or STEPPED_AREA and stacked_type is either STACKED or PERCENT_STACKED. In addition, for COMBO, this will only be supported if there is only one type of stackable series type or one type has more series than the others and each of the other types have no more than one series. For example, if a chart has two stacked bar series and one area series, the total data labels will be supported. If it has three bar series and two area series, total data labels are not allowed. Neither CUSTOM nor placement can be set on the total_data_label."]
        pub total_data_label: ::std::option::Option<::std::boxed::Box<DataLabel>>,
    }
    impl BasicChartSpec {
        pub fn builder() -> BasicChartSpecBuilder {
            BasicChartSpecBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the chart."]
    pub enum BasicChartSpecChartTypeEnum {
        #[serde(rename = "BASIC_CHART_TYPE_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        BasicChartTypeUnspecified,
        #[serde(rename = "BAR")]
        #[doc = "A bar chart."]
        Bar,
        #[serde(rename = "LINE")]
        #[doc = "A line chart."]
        Line,
        #[serde(rename = "AREA")]
        #[doc = "An area chart."]
        Area,
        #[serde(rename = "COLUMN")]
        #[doc = "A column chart."]
        Column,
        #[serde(rename = "SCATTER")]
        #[doc = "A scatter chart."]
        Scatter,
        #[serde(rename = "COMBO")]
        #[doc = "A combo chart."]
        Combo,
        #[serde(rename = "STEPPED_AREA")]
        #[doc = "A stepped area chart."]
        SteppedArea,
    }
    impl ::std::default::Default for BasicChartSpecChartTypeEnum {
        fn default() -> Self {
            Self::BasicChartTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The behavior of tooltips and data highlighting when hovering on data and chart area."]
    pub enum BasicChartSpecCompareModeEnum {
        #[serde(rename = "BASIC_CHART_COMPARE_MODE_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        BasicChartCompareModeUnspecified,
        #[serde(rename = "DATUM")]
        #[doc = "Only the focused data element is highlighted and shown in the tooltip."]
        Datum,
        #[serde(rename = "CATEGORY")]
        #[doc = "All data elements with the same category (e.g., domain value) are highlighted and shown in the tooltip."]
        Category,
    }
    impl ::std::default::Default for BasicChartSpecCompareModeEnum {
        fn default() -> Self {
            Self::BasicChartCompareModeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The position of the chart legend."]
    pub enum BasicChartSpecLegendPositionEnum {
        #[serde(rename = "BASIC_CHART_LEGEND_POSITION_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        BasicChartLegendPositionUnspecified,
        #[serde(rename = "BOTTOM_LEGEND")]
        #[doc = "The legend is rendered on the bottom of the chart."]
        BottomLegend,
        #[serde(rename = "LEFT_LEGEND")]
        #[doc = "The legend is rendered on the left of the chart."]
        LeftLegend,
        #[serde(rename = "RIGHT_LEGEND")]
        #[doc = "The legend is rendered on the right of the chart."]
        RightLegend,
        #[serde(rename = "TOP_LEGEND")]
        #[doc = "The legend is rendered on the top of the chart."]
        TopLegend,
        #[serde(rename = "NO_LEGEND")]
        #[doc = "No legend is rendered."]
        NoLegend,
    }
    impl ::std::default::Default for BasicChartSpecLegendPositionEnum {
        fn default() -> Self {
            Self::BasicChartLegendPositionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The stacked type for charts that support vertical stacking. Applies to Area, Bar, Column, Combo, and Stepped Area charts."]
    pub enum BasicChartSpecStackedTypeEnum {
        #[serde(rename = "BASIC_CHART_STACKED_TYPE_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        BasicChartStackedTypeUnspecified,
        #[serde(rename = "NOT_STACKED")]
        #[doc = "Series are not stacked."]
        NotStacked,
        #[serde(rename = "STACKED")]
        #[doc = "Series values are stacked, each value is rendered vertically beginning from the top of the value below it."]
        Stacked,
        #[serde(rename = "PERCENT_STACKED")]
        #[doc = "Vertical stacks are stretched to reach the top of the chart, with values laid out as percentages of each other."]
        PercentStacked,
    }
    impl ::std::default::Default for BasicChartSpecStackedTypeEnum {
        fn default() -> Self {
            Self::BasicChartStackedTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The default filter associated with a sheet."]
    pub struct BasicFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "criteria")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The criteria for showing/hiding values per column. The map's key is the column index, and the value is the criteria for that column. This field is deprecated in favor of filter_specs."]
        pub criteria: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<FilterCriteria>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterSpecs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filter criteria per column. Both criteria and filter_specs are populated in responses. If both fields are specified in an update request, this field takes precedence."]
        pub filter_specs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FilterSpec>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range the filter covers."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortSpecs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sort order per column. Later specifications are used when values are equal in the earlier specifications."]
        pub sort_specs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SortSpec>>>,
    }
    impl BasicFilter {
        pub fn builder() -> BasicFilterBuilder {
            BasicFilterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Style override settings for a single series data point."]
    pub struct BasicSeriesDataPointStyleOverride {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Color of the series data point. If empty, the series default is used."]
        pub color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Color of the series data point. If empty, the series default is used. If color is also set, this field takes precedence."]
        pub color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Zero based index of the series data point."]
        pub index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pointStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Point style of the series data point. Valid only if the chartType is AREA, LINE, or SCATTER. COMBO charts are also supported if the series chart type is AREA, LINE, or SCATTER. If empty, the series default is used."]
        pub point_style: ::std::option::Option<::std::boxed::Box<PointStyle>>,
    }
    impl BasicSeriesDataPointStyleOverride {
        pub fn builder() -> BasicSeriesDataPointStyleOverrideBuilder {
            BasicSeriesDataPointStyleOverrideBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for clearing more than one range selected by a DataFilter in a spreadsheet."]
    pub struct BatchClearValuesByDataFilterRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataFilters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The DataFilters used to determine which ranges to clear."]
        pub data_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataFilter>>>,
    }
    impl BatchClearValuesByDataFilterRequest {
        pub fn builder() -> BatchClearValuesByDataFilterRequestBuilder {
            BatchClearValuesByDataFilterRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response when clearing a range of values selected with DataFilters in a spreadsheet."]
    pub struct BatchClearValuesByDataFilterResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clearedRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ranges that were cleared, in A1 notation. If the requests are for an unbounded range or a ranger larger than the bounds of the sheet, this is the actual ranges that were cleared, bounded to the sheet's limits."]
        pub cleared_ranges: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spreadsheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The spreadsheet the updates were applied to."]
        pub spreadsheet_id: ::std::option::Option<::std::string::String>,
    }
    impl BatchClearValuesByDataFilterResponse {
        pub fn builder() -> BatchClearValuesByDataFilterResponseBuilder {
            BatchClearValuesByDataFilterResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for clearing more than one range of values in a spreadsheet."]
    pub struct BatchClearValuesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ranges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ranges to clear, in A1 notation."]
        pub ranges: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl BatchClearValuesRequest {
        pub fn builder() -> BatchClearValuesRequestBuilder {
            BatchClearValuesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response when clearing a range of values in a spreadsheet."]
    pub struct BatchClearValuesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clearedRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ranges that were cleared, in A1 notation. If the requests are for an unbounded range or a ranger larger than the bounds of the sheet, this is the actual ranges that were cleared, bounded to the sheet's limits."]
        pub cleared_ranges: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spreadsheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The spreadsheet the updates were applied to."]
        pub spreadsheet_id: ::std::option::Option<::std::string::String>,
    }
    impl BatchClearValuesResponse {
        pub fn builder() -> BatchClearValuesResponseBuilder {
            BatchClearValuesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for retrieving a range of values in a spreadsheet selected by a set of DataFilters."]
    pub struct BatchGetValuesByDataFilterRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataFilters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data filters used to match the ranges of values to retrieve. Ranges that match any of the specified data filters are included in the response."]
        pub data_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataFilter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateTimeRenderOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is [DateTimeRenderOption.SERIAL_NUMBER]."]
        pub date_time_render_option:
            ::std::option::Option<BatchGetValuesByDataFilterRequestDateTimeRenderOptionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "majorDimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The major dimension that results should use. For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then a request that selects that range and sets `majorDimension=ROWS` returns `[[1,2],[3,4]]`, whereas a request that sets `majorDimension=COLUMNS` returns `[[1,3],[2,4]]`."]
        pub major_dimension:
            ::std::option::Option<BatchGetValuesByDataFilterRequestMajorDimensionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueRenderOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How values should be represented in the output. The default render option is ValueRenderOption.FORMATTED_VALUE."]
        pub value_render_option:
            ::std::option::Option<BatchGetValuesByDataFilterRequestValueRenderOptionEnum>,
    }
    impl BatchGetValuesByDataFilterRequest {
        pub fn builder() -> BatchGetValuesByDataFilterRequestBuilder {
            BatchGetValuesByDataFilterRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is [DateTimeRenderOption.SERIAL_NUMBER]."]
    pub enum BatchGetValuesByDataFilterRequestDateTimeRenderOptionEnum {
        #[serde(rename = "SERIAL_NUMBER")]
        #[doc = "Instructs date, time, datetime, and duration fields to be output as doubles in \"serial number\" format, as popularized by Lotus 1-2-3. The whole number portion of the value (left of the decimal) counts the days since December 30th 1899. The fractional portion (right of the decimal) counts the time as a fraction of the day. For example, January 1st 1900 at noon would be 2.5, 2 because it's 2 days after December 30st 1899, and .5 because noon is half a day. February 1st 1900 at 3pm would be 33.625. This correctly treats the year 1900 as not a leap year."]
        SerialNumber,
        #[serde(rename = "FORMATTED_STRING")]
        #[doc = "Instructs date, time, datetime, and duration fields to be output as strings in their given number format (which is dependent on the spreadsheet locale)."]
        FormattedString,
    }
    impl ::std::default::Default for BatchGetValuesByDataFilterRequestDateTimeRenderOptionEnum {
        fn default() -> Self {
            Self::SerialNumber
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The major dimension that results should use. For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then a request that selects that range and sets `majorDimension=ROWS` returns `[[1,2],[3,4]]`, whereas a request that sets `majorDimension=COLUMNS` returns `[[1,3],[2,4]]`."]
    pub enum BatchGetValuesByDataFilterRequestMajorDimensionEnum {
        #[serde(rename = "DIMENSION_UNSPECIFIED")]
        #[doc = "The default value, do not use."]
        DimensionUnspecified,
        #[serde(rename = "ROWS")]
        #[doc = "Operates on the rows of a sheet."]
        Rows,
        #[serde(rename = "COLUMNS")]
        #[doc = "Operates on the columns of a sheet."]
        Columns,
    }
    impl ::std::default::Default for BatchGetValuesByDataFilterRequestMajorDimensionEnum {
        fn default() -> Self {
            Self::DimensionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How values should be represented in the output. The default render option is ValueRenderOption.FORMATTED_VALUE."]
    pub enum BatchGetValuesByDataFilterRequestValueRenderOptionEnum {
        #[serde(rename = "FORMATTED_VALUE")]
        #[doc = "Values will be calculated & formatted in the reply according to the cell's formatting. Formatting is based on the spreadsheet's locale, not the requesting user's locale. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return `\"$1.23\"`."]
        FormattedValue,
        #[serde(rename = "UNFORMATTED_VALUE")]
        #[doc = "Values will be calculated, but not formatted in the reply. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return the number `1.23`."]
        UnformattedValue,
        #[serde(rename = "FORMULA")]
        #[doc = "Values will not be calculated. The reply will include the formulas. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then A2 would return `\"=A1\"`."]
        Formula,
    }
    impl ::std::default::Default for BatchGetValuesByDataFilterRequestValueRenderOptionEnum {
        fn default() -> Self {
            Self::FormattedValue
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response when retrieving more than one range of values in a spreadsheet selected by DataFilters."]
    pub struct BatchGetValuesByDataFilterResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spreadsheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the spreadsheet the data was retrieved from."]
        pub spreadsheet_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requested values with the list of data filters that matched them."]
        pub value_ranges:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MatchedValueRange>>>,
    }
    impl BatchGetValuesByDataFilterResponse {
        pub fn builder() -> BatchGetValuesByDataFilterResponseBuilder {
            BatchGetValuesByDataFilterResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response when retrieving more than one range of values in a spreadsheet."]
    pub struct BatchGetValuesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spreadsheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the spreadsheet the data was retrieved from."]
        pub spreadsheet_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requested values. The order of the ValueRanges is the same as the order of the requested ranges."]
        pub value_ranges: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ValueRange>>>,
    }
    impl BatchGetValuesResponse {
        pub fn builder() -> BatchGetValuesResponseBuilder {
            BatchGetValuesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for updating any aspect of a spreadsheet."]
    pub struct BatchUpdateSpreadsheetRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeSpreadsheetInResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines if the update response should include the spreadsheet resource."]
        pub include_spreadsheet_in_response: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of updates to apply to the spreadsheet. Requests will be applied in the order they are specified. If any request is not valid, no requests will be applied."]
        pub requests: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Request>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseIncludeGridData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if grid data should be returned. Meaningful only if include_spreadsheet_in_response is 'true'. This parameter is ignored if a field mask was set in the request."]
        pub response_include_grid_data: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Limits the ranges included in the response spreadsheet. Meaningful only if include_spreadsheet_in_response is 'true'."]
        pub response_ranges: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl BatchUpdateSpreadsheetRequest {
        pub fn builder() -> BatchUpdateSpreadsheetRequestBuilder {
            BatchUpdateSpreadsheetRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The reply for batch updating a spreadsheet."]
    pub struct BatchUpdateSpreadsheetResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reply of the updates. This maps 1:1 with the updates, although replies to some requests may be empty."]
        pub replies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Response>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spreadsheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The spreadsheet the updates were applied to."]
        pub spreadsheet_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatedSpreadsheet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The spreadsheet after updates were applied. This is only set if [BatchUpdateSpreadsheetRequest.include_spreadsheet_in_response] is `true`."]
        pub updated_spreadsheet: ::std::option::Option<::std::boxed::Box<Spreadsheet>>,
    }
    impl BatchUpdateSpreadsheetResponse {
        pub fn builder() -> BatchUpdateSpreadsheetResponseBuilder {
            BatchUpdateSpreadsheetResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for updating more than one range of values in a spreadsheet."]
    pub struct BatchUpdateValuesByDataFilterRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new values to apply to the spreadsheet. If more than one range is matched by the specified DataFilter the specified values are applied to all of those ranges."]
        pub data: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataFilterValueRange>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeValuesInResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines if the update response should include the values of the cells that were updated. By default, responses do not include the updated values. The `updatedData` field within each of the BatchUpdateValuesResponse.responses contains the updated values. If the range to write was larger than the range actually written, the response includes all values in the requested range (excluding trailing empty rows and columns)."]
        pub include_values_in_response: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseDateTimeRenderOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is DateTimeRenderOption.SERIAL_NUMBER."]
        pub response_date_time_render_option: ::std::option::Option<
            BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOptionEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseValueRenderOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines how values in the response should be rendered. The default render option is ValueRenderOption.FORMATTED_VALUE."]
        pub response_value_render_option: ::std::option::Option<
            BatchUpdateValuesByDataFilterRequestResponseValueRenderOptionEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueInputOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How the input data should be interpreted."]
        pub value_input_option:
            ::std::option::Option<BatchUpdateValuesByDataFilterRequestValueInputOptionEnum>,
    }
    impl BatchUpdateValuesByDataFilterRequest {
        pub fn builder() -> BatchUpdateValuesByDataFilterRequestBuilder {
            BatchUpdateValuesByDataFilterRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is DateTimeRenderOption.SERIAL_NUMBER."]
    pub enum BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOptionEnum {
        #[serde(rename = "SERIAL_NUMBER")]
        #[doc = "Instructs date, time, datetime, and duration fields to be output as doubles in \"serial number\" format, as popularized by Lotus 1-2-3. The whole number portion of the value (left of the decimal) counts the days since December 30th 1899. The fractional portion (right of the decimal) counts the time as a fraction of the day. For example, January 1st 1900 at noon would be 2.5, 2 because it's 2 days after December 30st 1899, and .5 because noon is half a day. February 1st 1900 at 3pm would be 33.625. This correctly treats the year 1900 as not a leap year."]
        SerialNumber,
        #[serde(rename = "FORMATTED_STRING")]
        #[doc = "Instructs date, time, datetime, and duration fields to be output as strings in their given number format (which is dependent on the spreadsheet locale)."]
        FormattedString,
    }
    impl ::std::default::Default
        for BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOptionEnum
    {
        fn default() -> Self {
            Self::SerialNumber
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Determines how values in the response should be rendered. The default render option is ValueRenderOption.FORMATTED_VALUE."]
    pub enum BatchUpdateValuesByDataFilterRequestResponseValueRenderOptionEnum {
        #[serde(rename = "FORMATTED_VALUE")]
        #[doc = "Values will be calculated & formatted in the reply according to the cell's formatting. Formatting is based on the spreadsheet's locale, not the requesting user's locale. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return `\"$1.23\"`."]
        FormattedValue,
        #[serde(rename = "UNFORMATTED_VALUE")]
        #[doc = "Values will be calculated, but not formatted in the reply. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return the number `1.23`."]
        UnformattedValue,
        #[serde(rename = "FORMULA")]
        #[doc = "Values will not be calculated. The reply will include the formulas. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then A2 would return `\"=A1\"`."]
        Formula,
    }
    impl ::std::default::Default for BatchUpdateValuesByDataFilterRequestResponseValueRenderOptionEnum {
        fn default() -> Self {
            Self::FormattedValue
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How the input data should be interpreted."]
    pub enum BatchUpdateValuesByDataFilterRequestValueInputOptionEnum {
        #[serde(rename = "INPUT_VALUE_OPTION_UNSPECIFIED")]
        #[doc = "Default input value. This value must not be used."]
        InputValueOptionUnspecified,
        #[serde(rename = "RAW")]
        #[doc = "The values the user has entered will not be parsed and will be stored as-is."]
        Raw,
        #[serde(rename = "USER_ENTERED")]
        #[doc = "The values will be parsed as if the user typed them into the UI. Numbers will stay as numbers, but strings may be converted to numbers, dates, etc. following the same rules that are applied when entering text into a cell via the Google Sheets UI."]
        UserEntered,
    }
    impl ::std::default::Default for BatchUpdateValuesByDataFilterRequestValueInputOptionEnum {
        fn default() -> Self {
            Self::InputValueOptionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response when updating a range of values in a spreadsheet."]
    pub struct BatchUpdateValuesByDataFilterResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The response for each range updated."]
        pub responses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<UpdateValuesByDataFilterResponse>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spreadsheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The spreadsheet the updates were applied to."]
        pub spreadsheet_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalUpdatedCells")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of cells updated."]
        pub total_updated_cells: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalUpdatedColumns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of columns where at least one cell in the column was updated."]
        pub total_updated_columns: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalUpdatedRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of rows where at least one cell in the row was updated."]
        pub total_updated_rows: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalUpdatedSheets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of sheets where at least one cell in the sheet was updated."]
        pub total_updated_sheets: ::std::option::Option<::std::primitive::i64>,
    }
    impl BatchUpdateValuesByDataFilterResponse {
        pub fn builder() -> BatchUpdateValuesByDataFilterResponseBuilder {
            BatchUpdateValuesByDataFilterResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for updating more than one range of values in a spreadsheet."]
    pub struct BatchUpdateValuesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new values to apply to the spreadsheet."]
        pub data: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ValueRange>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeValuesInResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines if the update response should include the values of the cells that were updated. By default, responses do not include the updated values. The `updatedData` field within each of the BatchUpdateValuesResponse.responses contains the updated values. If the range to write was larger than the range actually written, the response includes all values in the requested range (excluding trailing empty rows and columns)."]
        pub include_values_in_response: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseDateTimeRenderOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is DateTimeRenderOption.SERIAL_NUMBER."]
        pub response_date_time_render_option:
            ::std::option::Option<BatchUpdateValuesRequestResponseDateTimeRenderOptionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseValueRenderOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines how values in the response should be rendered. The default render option is ValueRenderOption.FORMATTED_VALUE."]
        pub response_value_render_option:
            ::std::option::Option<BatchUpdateValuesRequestResponseValueRenderOptionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueInputOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How the input data should be interpreted."]
        pub value_input_option: ::std::option::Option<BatchUpdateValuesRequestValueInputOptionEnum>,
    }
    impl BatchUpdateValuesRequest {
        pub fn builder() -> BatchUpdateValuesRequestBuilder {
            BatchUpdateValuesRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is DateTimeRenderOption.SERIAL_NUMBER."]
    pub enum BatchUpdateValuesRequestResponseDateTimeRenderOptionEnum {
        #[serde(rename = "SERIAL_NUMBER")]
        #[doc = "Instructs date, time, datetime, and duration fields to be output as doubles in \"serial number\" format, as popularized by Lotus 1-2-3. The whole number portion of the value (left of the decimal) counts the days since December 30th 1899. The fractional portion (right of the decimal) counts the time as a fraction of the day. For example, January 1st 1900 at noon would be 2.5, 2 because it's 2 days after December 30st 1899, and .5 because noon is half a day. February 1st 1900 at 3pm would be 33.625. This correctly treats the year 1900 as not a leap year."]
        SerialNumber,
        #[serde(rename = "FORMATTED_STRING")]
        #[doc = "Instructs date, time, datetime, and duration fields to be output as strings in their given number format (which is dependent on the spreadsheet locale)."]
        FormattedString,
    }
    impl ::std::default::Default for BatchUpdateValuesRequestResponseDateTimeRenderOptionEnum {
        fn default() -> Self {
            Self::SerialNumber
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Determines how values in the response should be rendered. The default render option is ValueRenderOption.FORMATTED_VALUE."]
    pub enum BatchUpdateValuesRequestResponseValueRenderOptionEnum {
        #[serde(rename = "FORMATTED_VALUE")]
        #[doc = "Values will be calculated & formatted in the reply according to the cell's formatting. Formatting is based on the spreadsheet's locale, not the requesting user's locale. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return `\"$1.23\"`."]
        FormattedValue,
        #[serde(rename = "UNFORMATTED_VALUE")]
        #[doc = "Values will be calculated, but not formatted in the reply. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return the number `1.23`."]
        UnformattedValue,
        #[serde(rename = "FORMULA")]
        #[doc = "Values will not be calculated. The reply will include the formulas. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then A2 would return `\"=A1\"`."]
        Formula,
    }
    impl ::std::default::Default for BatchUpdateValuesRequestResponseValueRenderOptionEnum {
        fn default() -> Self {
            Self::FormattedValue
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How the input data should be interpreted."]
    pub enum BatchUpdateValuesRequestValueInputOptionEnum {
        #[serde(rename = "INPUT_VALUE_OPTION_UNSPECIFIED")]
        #[doc = "Default input value. This value must not be used."]
        InputValueOptionUnspecified,
        #[serde(rename = "RAW")]
        #[doc = "The values the user has entered will not be parsed and will be stored as-is."]
        Raw,
        #[serde(rename = "USER_ENTERED")]
        #[doc = "The values will be parsed as if the user typed them into the UI. Numbers will stay as numbers, but strings may be converted to numbers, dates, etc. following the same rules that are applied when entering text into a cell via the Google Sheets UI."]
        UserEntered,
    }
    impl ::std::default::Default for BatchUpdateValuesRequestValueInputOptionEnum {
        fn default() -> Self {
            Self::InputValueOptionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response when updating a range of values in a spreadsheet."]
    pub struct BatchUpdateValuesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One UpdateValuesResponse per requested range, in the same order as the requests appeared."]
        pub responses:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UpdateValuesResponse>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spreadsheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The spreadsheet the updates were applied to."]
        pub spreadsheet_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalUpdatedCells")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of cells updated."]
        pub total_updated_cells: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalUpdatedColumns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of columns where at least one cell in the column was updated."]
        pub total_updated_columns: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalUpdatedRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of rows where at least one cell in the row was updated."]
        pub total_updated_rows: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalUpdatedSheets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of sheets where at least one cell in the sheet was updated."]
        pub total_updated_sheets: ::std::option::Option<::std::primitive::i64>,
    }
    impl BatchUpdateValuesResponse {
        pub fn builder() -> BatchUpdateValuesResponseBuilder {
            BatchUpdateValuesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The specification of a BigQuery data source that's connected to a sheet."]
    pub struct BigQueryDataSourceSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of a BigQuery enabled GCP project with a billing account attached. For any queries executed against the data source, the project is charged."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "querySpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A BigQueryQuerySpec."]
        pub query_spec: ::std::option::Option<::std::boxed::Box<BigQueryQuerySpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A BigQueryTableSpec."]
        pub table_spec: ::std::option::Option<::std::boxed::Box<BigQueryTableSpec>>,
    }
    impl BigQueryDataSourceSpec {
        pub fn builder() -> BigQueryDataSourceSpecBuilder {
            BigQueryDataSourceSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies a custom BigQuery query."]
    pub struct BigQueryQuerySpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rawQuery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The raw query string."]
        pub raw_query: ::std::option::Option<::std::string::String>,
    }
    impl BigQueryQuerySpec {
        pub fn builder() -> BigQueryQuerySpecBuilder {
            BigQueryQuerySpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies a BigQuery table definition. Only [native tables](https://cloud.google.com/bigquery/docs/tables-intro) is allowed."]
    pub struct BigQueryTableSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datasetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BigQuery dataset id."]
        pub dataset_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BigQuery table id."]
        pub table_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableProjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of a BigQuery project the table belongs to. If not specified, the project_id is assumed."]
        pub table_project_id: ::std::option::Option<::std::string::String>,
    }
    impl BigQueryTableSpec {
        pub fn builder() -> BigQueryTableSpecBuilder {
            BigQueryTableSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A condition that can evaluate to true or false. BooleanConditions are used by conditional formatting, data validation, and the criteria in filters."]
    pub struct BooleanCondition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of condition."]
        pub _type: ::std::option::Option<BooleanConditionTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values of the condition. The number of supported values depends on the condition type. Some support zero values, others one or two values, and ConditionType.ONE_OF_LIST supports an arbitrary number of values."]
        pub values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ConditionValue>>>,
    }
    impl BooleanCondition {
        pub fn builder() -> BooleanConditionBuilder {
            BooleanConditionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of condition."]
    pub enum BooleanConditionTypeEnum {
        #[serde(rename = "CONDITION_TYPE_UNSPECIFIED")]
        #[doc = "The default value, do not use."]
        ConditionTypeUnspecified,
        #[serde(rename = "NUMBER_GREATER")]
        #[doc = "The cell's value must be greater than the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue."]
        NumberGreater,
        #[serde(rename = "NUMBER_GREATER_THAN_EQ")]
        #[doc = "The cell's value must be greater than or equal to the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue."]
        NumberGreaterThanEq,
        #[serde(rename = "NUMBER_LESS")]
        #[doc = "The cell's value must be less than the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue."]
        NumberLess,
        #[serde(rename = "NUMBER_LESS_THAN_EQ")]
        #[doc = "The cell's value must be less than or equal to the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue."]
        NumberLessThanEq,
        #[serde(rename = "NUMBER_EQ")]
        #[doc = "The cell's value must be equal to the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue for data validation, conditional formatting, and filters on non-data source objects and at least one ConditionValue for filters on data source objects."]
        NumberEq,
        #[serde(rename = "NUMBER_NOT_EQ")]
        #[doc = "The cell's value must be not equal to the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue for data validation, conditional formatting, and filters on non-data source objects and at least one ConditionValue for filters on data source objects."]
        NumberNotEq,
        #[serde(rename = "NUMBER_BETWEEN")]
        #[doc = "The cell's value must be between the two condition values. Supported by data validation, conditional formatting and filters. Requires exactly two ConditionValues."]
        NumberBetween,
        #[serde(rename = "NUMBER_NOT_BETWEEN")]
        #[doc = "The cell's value must not be between the two condition values. Supported by data validation, conditional formatting and filters. Requires exactly two ConditionValues."]
        NumberNotBetween,
        #[serde(rename = "TEXT_CONTAINS")]
        #[doc = "The cell's value must contain the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue."]
        TextContains,
        #[serde(rename = "TEXT_NOT_CONTAINS")]
        #[doc = "The cell's value must not contain the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue."]
        TextNotContains,
        #[serde(rename = "TEXT_STARTS_WITH")]
        #[doc = "The cell's value must start with the condition's value. Supported by conditional formatting and filters. Requires a single ConditionValue."]
        TextStartsWith,
        #[serde(rename = "TEXT_ENDS_WITH")]
        #[doc = "The cell's value must end with the condition's value. Supported by conditional formatting and filters. Requires a single ConditionValue."]
        TextEndsWith,
        #[serde(rename = "TEXT_EQ")]
        #[doc = "The cell's value must be exactly the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue for data validation, conditional formatting, and filters on non-data source objects and at least one ConditionValue for filters on data source objects."]
        TextEq,
        #[serde(rename = "TEXT_IS_EMAIL")]
        #[doc = "The cell's value must be a valid email address. Supported by data validation. Requires no ConditionValues."]
        TextIsEmail,
        #[serde(rename = "TEXT_IS_URL")]
        #[doc = "The cell's value must be a valid URL. Supported by data validation. Requires no ConditionValues."]
        TextIsUrl,
        #[serde(rename = "DATE_EQ")]
        #[doc = "The cell's value must be the same date as the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue for data validation, conditional formatting, and filters on non-data source objects and at least one ConditionValue for filters on data source objects."]
        DateEq,
        #[serde(rename = "DATE_BEFORE")]
        #[doc = "The cell's value must be before the date of the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue that may be a relative date."]
        DateBefore,
        #[serde(rename = "DATE_AFTER")]
        #[doc = "The cell's value must be after the date of the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue that may be a relative date."]
        DateAfter,
        #[serde(rename = "DATE_ON_OR_BEFORE")]
        #[doc = "The cell's value must be on or before the date of the condition's value. Supported by data validation. Requires a single ConditionValue that may be a relative date."]
        DateOnOrBefore,
        #[serde(rename = "DATE_ON_OR_AFTER")]
        #[doc = "The cell's value must be on or after the date of the condition's value. Supported by data validation. Requires a single ConditionValue that may be a relative date."]
        DateOnOrAfter,
        #[serde(rename = "DATE_BETWEEN")]
        #[doc = "The cell's value must be between the dates of the two condition values. Supported by data validation. Requires exactly two ConditionValues."]
        DateBetween,
        #[serde(rename = "DATE_NOT_BETWEEN")]
        #[doc = "The cell's value must be outside the dates of the two condition values. Supported by data validation. Requires exactly two ConditionValues."]
        DateNotBetween,
        #[serde(rename = "DATE_IS_VALID")]
        #[doc = "The cell's value must be a date. Supported by data validation. Requires no ConditionValues."]
        DateIsValid,
        #[serde(rename = "ONE_OF_RANGE")]
        #[doc = "The cell's value must be listed in the grid in condition value's range. Supported by data validation. Requires a single ConditionValue, and the value must be a valid range in A1 notation."]
        OneOfRange,
        #[serde(rename = "ONE_OF_LIST")]
        #[doc = "The cell's value must be in the list of condition values. Supported by data validation. Supports any number of condition values, one per item in the list. Formulas are not supported in the values."]
        OneOfList,
        #[serde(rename = "BLANK")]
        #[doc = "The cell's value must be empty. Supported by conditional formatting and filters. Requires no ConditionValues."]
        Blank,
        #[serde(rename = "NOT_BLANK")]
        #[doc = "The cell's value must not be empty. Supported by conditional formatting and filters. Requires no ConditionValues."]
        NotBlank,
        #[serde(rename = "CUSTOM_FORMULA")]
        #[doc = "The condition's formula must evaluate to true. Supported by data validation, conditional formatting and filters. Not supported by data source sheet filters. Requires a single ConditionValue."]
        CustomFormula,
        #[serde(rename = "BOOLEAN")]
        #[doc = "The cell's value must be TRUE/FALSE or in the list of condition values. Supported by data validation. Renders as a cell checkbox. Supports zero, one or two ConditionValues. No values indicates the cell must be TRUE or FALSE, where TRUE renders as checked and FALSE renders as unchecked. One value indicates the cell will render as checked when it contains that value and unchecked when it is blank. Two values indicate that the cell will render as checked when it contains the first value and unchecked when it contains the second value. For example, [\"Yes\",\"No\"] indicates that the cell will render a checked box when it has the value \"Yes\" and an unchecked box when it has the value \"No\"."]
        Boolean,
        #[serde(rename = "TEXT_NOT_EQ")]
        #[doc = "The cell's value must be exactly not the condition's value. Supported by filters on data source objects. Requires at least one ConditionValue."]
        TextNotEq,
        #[serde(rename = "DATE_NOT_EQ")]
        #[doc = "The cell's value must be exactly not the condition's value. Supported by filters on data source objects. Requires at least one ConditionValue."]
        DateNotEq,
    }
    impl ::std::default::Default for BooleanConditionTypeEnum {
        fn default() -> Self {
            Self::ConditionTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A rule that may or may not match, depending on the condition."]
    pub struct BooleanRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The condition of the rule. If the condition evaluates to true, the format is applied."]
        pub condition: ::std::option::Option<::std::boxed::Box<BooleanCondition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "format")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The format to apply. Conditional formatting can only apply a subset of formatting: bold, italic, strikethrough, foreground color & background color."]
        pub format: ::std::option::Option<::std::boxed::Box<CellFormat>>,
    }
    impl BooleanRule {
        pub fn builder() -> BooleanRuleBuilder {
            BooleanRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A border along a cell."]
    pub struct Border {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the border."]
        pub color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the border. If color is also set, this field takes precedence."]
        pub color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "style")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The style of the border."]
        pub style: ::std::option::Option<BorderStyleEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the border, in pixels. Deprecated; the width is determined by the \"style\" field."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl Border {
        pub fn builder() -> BorderBuilder {
            BorderBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The style of the border."]
    pub enum BorderStyleEnum {
        #[serde(rename = "STYLE_UNSPECIFIED")]
        #[doc = "The style is not specified. Do not use this."]
        StyleUnspecified,
        #[serde(rename = "DOTTED")]
        #[doc = "The border is dotted."]
        Dotted,
        #[serde(rename = "DASHED")]
        #[doc = "The border is dashed."]
        Dashed,
        #[serde(rename = "SOLID")]
        #[doc = "The border is a thin solid line."]
        Solid,
        #[serde(rename = "SOLID_MEDIUM")]
        #[doc = "The border is a medium solid line."]
        SolidMedium,
        #[serde(rename = "SOLID_THICK")]
        #[doc = "The border is a thick solid line."]
        SolidThick,
        #[serde(rename = "NONE")]
        #[doc = "No border. Used only when updating a border in order to erase it."]
        None,
        #[serde(rename = "DOUBLE")]
        #[doc = "The border is two solid lines."]
        Double,
    }
    impl ::std::default::Default for BorderStyleEnum {
        fn default() -> Self {
            Self::StyleUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The borders of the cell."]
    pub struct Borders {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bottom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bottom border of the cell."]
        pub bottom: ::std::option::Option<::std::boxed::Box<Border>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "left")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The left border of the cell."]
        pub left: ::std::option::Option<::std::boxed::Box<Border>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "right")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The right border of the cell."]
        pub right: ::std::option::Option<::std::boxed::Box<Border>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "top")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The top border of the cell."]
        pub top: ::std::option::Option<::std::boxed::Box<Border>>,
    }
    impl Borders {
        pub fn builder() -> BordersBuilder {
            BordersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A bubble chart."]
    pub struct BubbleChartSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bubbleBorderColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bubble border color."]
        pub bubble_border_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bubbleBorderColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bubble border color. If bubble_border_color is also set, this field takes precedence."]
        pub bubble_border_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bubbleLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data containing the bubble labels. These do not need to be unique."]
        pub bubble_labels: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bubbleMaxRadiusSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The max radius size of the bubbles, in pixels. If specified, the field must be a positive value."]
        pub bubble_max_radius_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bubbleMinRadiusSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum radius size of the bubbles, in pixels. If specific, the field must be a positive value."]
        pub bubble_min_radius_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bubbleOpacity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The opacity of the bubbles between 0 and 1.0. 0 is fully transparent and 1 is fully opaque."]
        pub bubble_opacity: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bubbleSizes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data contianing the bubble sizes. Bubble sizes are used to draw the bubbles at different sizes relative to each other. If specified, group_ids must also be specified. This field is optional."]
        pub bubble_sizes: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bubbleTextStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The format of the text inside the bubbles. Strikethrough and underline are not supported."]
        pub bubble_text_style: ::std::option::Option<::std::boxed::Box<TextFormat>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data containing the bubble x-values. These values locate the bubbles in the chart horizontally."]
        pub domain: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data containing the bubble group IDs. All bubbles with the same group ID are drawn in the same color. If bubble_sizes is specified then this field must also be specified but may contain blank values. This field is optional."]
        pub group_ids: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "legendPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Where the legend of the chart should be drawn."]
        pub legend_position: ::std::option::Option<BubbleChartSpecLegendPositionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "series")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data contianing the bubble y-values. These values locate the bubbles in the chart vertically."]
        pub series: ::std::option::Option<::std::boxed::Box<ChartData>>,
    }
    impl BubbleChartSpec {
        pub fn builder() -> BubbleChartSpecBuilder {
            BubbleChartSpecBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Where the legend of the chart should be drawn."]
    pub enum BubbleChartSpecLegendPositionEnum {
        #[serde(rename = "BUBBLE_CHART_LEGEND_POSITION_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        BubbleChartLegendPositionUnspecified,
        #[serde(rename = "BOTTOM_LEGEND")]
        #[doc = "The legend is rendered on the bottom of the chart."]
        BottomLegend,
        #[serde(rename = "LEFT_LEGEND")]
        #[doc = "The legend is rendered on the left of the chart."]
        LeftLegend,
        #[serde(rename = "RIGHT_LEGEND")]
        #[doc = "The legend is rendered on the right of the chart."]
        RightLegend,
        #[serde(rename = "TOP_LEGEND")]
        #[doc = "The legend is rendered on the top of the chart."]
        TopLegend,
        #[serde(rename = "NO_LEGEND")]
        #[doc = "No legend is rendered."]
        NoLegend,
        #[serde(rename = "INSIDE_LEGEND")]
        #[doc = "The legend is rendered inside the chart area."]
        InsideLegend,
    }
    impl ::std::default::Default for BubbleChartSpecLegendPositionEnum {
        fn default() -> Self {
            Self::BubbleChartLegendPositionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A candlestick chart."]
    pub struct CandlestickChartSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Candlestick chart data. Only one CandlestickData is supported."]
        pub data: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CandlestickData>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain data (horizontal axis) for the candlestick chart. String data will be treated as discrete labels, other data will be treated as continuous values."]
        pub domain: ::std::option::Option<::std::boxed::Box<CandlestickDomain>>,
    }
    impl CandlestickChartSpec {
        pub fn builder() -> CandlestickChartSpecBuilder {
            CandlestickChartSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Candlestick chart data, each containing the low, open, close, and high values for a series."]
    pub struct CandlestickData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "closeSeries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range data (vertical axis) for the close/final value for each candle. This is the top of the candle body. If greater than the open value the candle will be filled. Otherwise the candle will be hollow."]
        pub close_series: ::std::option::Option<::std::boxed::Box<CandlestickSeries>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "highSeries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range data (vertical axis) for the high/maximum value for each candle. This is the top of the candle's center line."]
        pub high_series: ::std::option::Option<::std::boxed::Box<CandlestickSeries>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lowSeries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range data (vertical axis) for the low/minimum value for each candle. This is the bottom of the candle's center line."]
        pub low_series: ::std::option::Option<::std::boxed::Box<CandlestickSeries>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "openSeries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range data (vertical axis) for the open/initial value for each candle. This is the bottom of the candle body. If less than the close value the candle will be filled. Otherwise the candle will be hollow."]
        pub open_series: ::std::option::Option<::std::boxed::Box<CandlestickSeries>>,
    }
    impl CandlestickData {
        pub fn builder() -> CandlestickDataBuilder {
            CandlestickDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The domain of a CandlestickChart."]
    pub struct CandlestickDomain {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data of the CandlestickDomain."]
        pub data: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reversed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True to reverse the order of the domain values (horizontal axis)."]
        pub reversed: ::std::option::Option<::std::primitive::bool>,
    }
    impl CandlestickDomain {
        pub fn builder() -> CandlestickDomainBuilder {
            CandlestickDomainBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The series of a CandlestickData."]
    pub struct CandlestickSeries {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data of the CandlestickSeries."]
        pub data: ::std::option::Option<::std::boxed::Box<ChartData>>,
    }
    impl CandlestickSeries {
        pub fn builder() -> CandlestickSeriesBuilder {
            CandlestickSeriesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data about a specific cell."]
    pub struct CellData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceFormula")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Information about a data source formula on the cell. The field is set if user_entered_value is a formula referencing some DATA_SOURCE sheet, e.g `=SUM(DataSheet!Column)`."]
        pub data_source_formula: ::std::option::Option<::std::boxed::Box<DataSourceFormula>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A data source table anchored at this cell. The size of data source table itself is computed dynamically based on its configuration. Only the first cell of the data source table contains the data source table definition. The other cells will contain the display values of the data source table result in their effective_value fields."]
        pub data_source_table: ::std::option::Option<::std::boxed::Box<DataSourceTable>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataValidation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A data validation rule on the cell, if any. When writing, the new data validation rule will overwrite any prior rule."]
        pub data_validation: ::std::option::Option<::std::boxed::Box<DataValidationRule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "effectiveFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The effective format being used by the cell. This includes the results of applying any conditional formatting and, if the cell contains a formula, the computed number format. If the effective format is the default format, effective format will not be written. This field is read-only."]
        pub effective_format: ::std::option::Option<::std::boxed::Box<CellFormat>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "effectiveValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The effective value of the cell. For cells with formulas, this is the calculated value. For cells with literals, this is the same as the user_entered_value. This field is read-only."]
        pub effective_value: ::std::option::Option<::std::boxed::Box<ExtendedValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formattedValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The formatted value of the cell. This is the value as it's shown to the user. This field is read-only."]
        pub formatted_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hyperlink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A hyperlink this cell points to, if any. If the cell contains multiple hyperlinks, this field will be empty. This field is read-only. To set it, use a `=HYPERLINK` formula in the userEnteredValue.formulaValue field."]
        pub hyperlink: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "note")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Any note on the cell."]
        pub note: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pivotTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A pivot table anchored at this cell. The size of pivot table itself is computed dynamically based on its data, grouping, filters, values, etc. Only the top-left cell of the pivot table contains the pivot table definition. The other cells will contain the calculated values of the results of the pivot in their effective_value fields."]
        pub pivot_table: ::std::option::Option<::std::boxed::Box<PivotTable>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textFormatRuns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Runs of rich text applied to subsections of the cell. Runs are only valid on user entered strings, not formulas, bools, or numbers. Properties of a run start at a specific index in the text and continue until the next run. Runs will inherit the properties of the cell unless explicitly changed. When writing, the new runs will overwrite any prior runs. When writing a new user_entered_value, previous runs are erased."]
        pub text_format_runs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TextFormatRun>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEnteredFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The format the user entered for the cell. When writing, the new format will be merged with the existing format."]
        pub user_entered_format: ::std::option::Option<::std::boxed::Box<CellFormat>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEnteredValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value the user entered in the cell. e.g, `1234`, `'Hello'`, or `=NOW()` Note: Dates, Times and DateTimes are represented as doubles in serial number format."]
        pub user_entered_value: ::std::option::Option<::std::boxed::Box<ExtendedValue>>,
    }
    impl CellData {
        pub fn builder() -> CellDataBuilder {
            CellDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The format of a cell."]
    pub struct CellFormat {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color of the cell."]
        pub background_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color of the cell. If background_color is also set, this field takes precedence."]
        pub background_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "borders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The borders of the cell."]
        pub borders: ::std::option::Option<::std::boxed::Box<Borders>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "horizontalAlignment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The horizontal alignment of the value in the cell."]
        pub horizontal_alignment: ::std::option::Option<CellFormatHorizontalAlignmentEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hyperlinkDisplayType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How a hyperlink, if it exists, should be displayed in the cell."]
        pub hyperlink_display_type: ::std::option::Option<CellFormatHyperlinkDisplayTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numberFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A format describing how number values should be represented to the user."]
        pub number_format: ::std::option::Option<::std::boxed::Box<NumberFormat>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "padding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The padding of the cell."]
        pub padding: ::std::option::Option<::std::boxed::Box<Padding>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textDirection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The direction of the text in the cell."]
        pub text_direction: ::std::option::Option<CellFormatTextDirectionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The format of the text in the cell (unless overridden by a format run)."]
        pub text_format: ::std::option::Option<::std::boxed::Box<TextFormat>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textRotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rotation applied to text in a cell"]
        pub text_rotation: ::std::option::Option<::std::boxed::Box<TextRotation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verticalAlignment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The vertical alignment of the value in the cell."]
        pub vertical_alignment: ::std::option::Option<CellFormatVerticalAlignmentEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "wrapStrategy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The wrap strategy for the value in the cell."]
        pub wrap_strategy: ::std::option::Option<CellFormatWrapStrategyEnum>,
    }
    impl CellFormat {
        pub fn builder() -> CellFormatBuilder {
            CellFormatBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The horizontal alignment of the value in the cell."]
    pub enum CellFormatHorizontalAlignmentEnum {
        #[serde(rename = "HORIZONTAL_ALIGN_UNSPECIFIED")]
        #[doc = "The horizontal alignment is not specified. Do not use this."]
        HorizontalAlignUnspecified,
        #[serde(rename = "LEFT")]
        #[doc = "The text is explicitly aligned to the left of the cell."]
        Left,
        #[serde(rename = "CENTER")]
        #[doc = "The text is explicitly aligned to the center of the cell."]
        Center,
        #[serde(rename = "RIGHT")]
        #[doc = "The text is explicitly aligned to the right of the cell."]
        Right,
    }
    impl ::std::default::Default for CellFormatHorizontalAlignmentEnum {
        fn default() -> Self {
            Self::HorizontalAlignUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How a hyperlink, if it exists, should be displayed in the cell."]
    pub enum CellFormatHyperlinkDisplayTypeEnum {
        #[serde(rename = "HYPERLINK_DISPLAY_TYPE_UNSPECIFIED")]
        #[doc = "The default value: the hyperlink is rendered. Do not use this."]
        HyperlinkDisplayTypeUnspecified,
        #[serde(rename = "LINKED")]
        #[doc = "A hyperlink should be explicitly rendered."]
        Linked,
        #[serde(rename = "PLAIN_TEXT")]
        #[doc = "A hyperlink should not be rendered."]
        PlainText,
    }
    impl ::std::default::Default for CellFormatHyperlinkDisplayTypeEnum {
        fn default() -> Self {
            Self::HyperlinkDisplayTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The direction of the text in the cell."]
    pub enum CellFormatTextDirectionEnum {
        #[serde(rename = "TEXT_DIRECTION_UNSPECIFIED")]
        #[doc = "The text direction is not specified. Do not use this."]
        TextDirectionUnspecified,
        #[serde(rename = "LEFT_TO_RIGHT")]
        #[doc = "The text direction of left-to-right was set by the user."]
        LeftToRight,
        #[serde(rename = "RIGHT_TO_LEFT")]
        #[doc = "The text direction of right-to-left was set by the user."]
        RightToLeft,
    }
    impl ::std::default::Default for CellFormatTextDirectionEnum {
        fn default() -> Self {
            Self::TextDirectionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The vertical alignment of the value in the cell."]
    pub enum CellFormatVerticalAlignmentEnum {
        #[serde(rename = "VERTICAL_ALIGN_UNSPECIFIED")]
        #[doc = "The vertical alignment is not specified. Do not use this."]
        VerticalAlignUnspecified,
        #[serde(rename = "TOP")]
        #[doc = "The text is explicitly aligned to the top of the cell."]
        Top,
        #[serde(rename = "MIDDLE")]
        #[doc = "The text is explicitly aligned to the middle of the cell."]
        Middle,
        #[serde(rename = "BOTTOM")]
        #[doc = "The text is explicitly aligned to the bottom of the cell."]
        Bottom,
    }
    impl ::std::default::Default for CellFormatVerticalAlignmentEnum {
        fn default() -> Self {
            Self::VerticalAlignUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The wrap strategy for the value in the cell."]
    pub enum CellFormatWrapStrategyEnum {
        #[serde(rename = "WRAP_STRATEGY_UNSPECIFIED")]
        #[doc = "The default value, do not use."]
        WrapStrategyUnspecified,
        #[serde(rename = "OVERFLOW_CELL")]
        #[doc = "Lines that are longer than the cell width will be written in the next cell over, so long as that cell is empty. If the next cell over is non-empty, this behaves the same as CLIP. The text will never wrap to the next line unless the user manually inserts a new line. Example: | First sentence. | | Manual newline that is very long. <- Text continues into next cell | Next newline. |"]
        OverflowCell,
        #[serde(rename = "LEGACY_WRAP")]
        #[doc = "This wrap strategy represents the old Google Sheets wrap strategy where words that are longer than a line are clipped rather than broken. This strategy is not supported on all platforms and is being phased out. Example: | Cell has a | | loooooooooo| <- Word is clipped. | word. |"]
        LegacyWrap,
        #[serde(rename = "CLIP")]
        #[doc = "Lines that are longer than the cell width will be clipped. The text will never wrap to the next line unless the user manually inserts a new line. Example: | First sentence. | | Manual newline t| <- Text is clipped | Next newline. |"]
        Clip,
        #[serde(rename = "WRAP")]
        #[doc = "Words that are longer than a line are wrapped at the character level rather than clipped. Example: | Cell has a | | loooooooooo| <- Word is broken. | ong word. |"]
        Wrap,
    }
    impl ::std::default::Default for CellFormatWrapStrategyEnum {
        fn default() -> Self {
            Self::WrapStrategyUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The options that define a \"view window\" for a chart (such as the visible values in an axis)."]
    pub struct ChartAxisViewWindowOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewWindowMax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum numeric value to be shown in this view window. If unset, will automatically determine a maximum value that looks good for the data."]
        pub view_window_max: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewWindowMin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum numeric value to be shown in this view window. If unset, will automatically determine a minimum value that looks good for the data."]
        pub view_window_min: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewWindowMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The view window's mode."]
        pub view_window_mode: ::std::option::Option<ChartAxisViewWindowOptionsViewWindowModeEnum>,
    }
    impl ChartAxisViewWindowOptions {
        pub fn builder() -> ChartAxisViewWindowOptionsBuilder {
            ChartAxisViewWindowOptionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The view window's mode."]
    pub enum ChartAxisViewWindowOptionsViewWindowModeEnum {
        #[serde(rename = "DEFAULT_VIEW_WINDOW_MODE")]
        #[doc = "The default view window mode used in the Sheets editor for this chart type. In most cases, if set, the default mode is equivalent to `PRETTY`."]
        DefaultViewWindowMode,
        #[serde(rename = "VIEW_WINDOW_MODE_UNSUPPORTED")]
        #[doc = "Do not use. Represents that the currently set mode is not supported by the API."]
        ViewWindowModeUnsupported,
        #[serde(rename = "EXPLICIT")]
        #[doc = "Follows the min and max exactly if specified. If a value is unspecified, it will fall back to the `PRETTY` value."]
        Explicit,
        #[serde(rename = "PRETTY")]
        #[doc = "Chooses a min and max that make the chart look good. Both min and max are ignored in this mode."]
        Pretty,
    }
    impl ::std::default::Default for ChartAxisViewWindowOptionsViewWindowModeEnum {
        fn default() -> Self {
            Self::DefaultViewWindowMode
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Custom number formatting options for chart attributes."]
    pub struct ChartCustomNumberFormatOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom prefix to be prepended to the chart attribute. This field is optional."]
        pub prefix: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suffix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom suffix to be appended to the chart attribute. This field is optional."]
        pub suffix: ::std::option::Option<::std::string::String>,
    }
    impl ChartCustomNumberFormatOptions {
        pub fn builder() -> ChartCustomNumberFormatOptionsBuilder {
            ChartCustomNumberFormatOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The data included in a domain or series."]
    pub struct ChartData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aggregateType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The aggregation type for the series of a data source chart. Not supported for regular charts."]
        pub aggregate_type: ::std::option::Option<ChartDataAggregateTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reference to the data source column that the data reads from."]
        pub column_reference: ::std::option::Option<::std::boxed::Box<DataSourceColumnReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rule to group the data by if the ChartData backs the domain of a data source chart. Not supported for regular charts."]
        pub group_rule: ::std::option::Option<::std::boxed::Box<ChartGroupRule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source ranges of the data."]
        pub source_range: ::std::option::Option<::std::boxed::Box<ChartSourceRange>>,
    }
    impl ChartData {
        pub fn builder() -> ChartDataBuilder {
            ChartDataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The aggregation type for the series of a data source chart. Not supported for regular charts."]
    pub enum ChartDataAggregateTypeEnum {
        #[serde(rename = "CHART_AGGREGATE_TYPE_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        ChartAggregateTypeUnspecified,
        #[serde(rename = "AVERAGE")]
        #[doc = "Average aggregate function."]
        Average,
        #[serde(rename = "COUNT")]
        #[doc = "Count aggregate function."]
        Count,
        #[serde(rename = "MAX")]
        #[doc = "Maximum aggregate function."]
        Max,
        #[serde(rename = "MEDIAN")]
        #[doc = "Median aggregate function."]
        Median,
        #[serde(rename = "MIN")]
        #[doc = "Minimum aggregate function."]
        Min,
        #[serde(rename = "SUM")]
        #[doc = "Sum aggregate function."]
        Sum,
    }
    impl ::std::default::Default for ChartDataAggregateTypeEnum {
        fn default() -> Self {
            Self::ChartAggregateTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Allows you to organize the date-time values in a source data column into buckets based on selected parts of their date or time values."]
    pub struct ChartDateTimeRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of date-time grouping to apply."]
        pub _type: ::std::option::Option<ChartDateTimeRuleTypeEnum>,
    }
    impl ChartDateTimeRule {
        pub fn builder() -> ChartDateTimeRuleBuilder {
            ChartDateTimeRuleBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of date-time grouping to apply."]
    pub enum ChartDateTimeRuleTypeEnum {
        #[serde(rename = "CHART_DATE_TIME_RULE_TYPE_UNSPECIFIED")]
        #[doc = "The default type, do not use."]
        ChartDateTimeRuleTypeUnspecified,
        #[serde(rename = "SECOND")]
        #[doc = "Group dates by second, from 0 to 59."]
        Second,
        #[serde(rename = "MINUTE")]
        #[doc = "Group dates by minute, from 0 to 59."]
        Minute,
        #[serde(rename = "HOUR")]
        #[doc = "Group dates by hour using a 24-hour system, from 0 to 23."]
        Hour,
        #[serde(rename = "HOUR_MINUTE")]
        #[doc = "Group dates by hour and minute using a 24-hour system, for example 19:45."]
        HourMinute,
        #[serde(rename = "HOUR_MINUTE_AMPM")]
        #[doc = "Group dates by hour and minute using a 12-hour system, for example 7:45 PM. The AM/PM designation is translated based on the spreadsheet locale."]
        HourMinuteAmpm,
        #[serde(rename = "DAY_OF_WEEK")]
        #[doc = "Group dates by day of week, for example Sunday. The days of the week will be translated based on the spreadsheet locale."]
        DayOfWeek,
        #[serde(rename = "DAY_OF_YEAR")]
        #[doc = "Group dates by day of year, from 1 to 366. Note that dates after Feb. 29 fall in different buckets in leap years than in non-leap years."]
        DayOfYear,
        #[serde(rename = "DAY_OF_MONTH")]
        #[doc = "Group dates by day of month, from 1 to 31."]
        DayOfMonth,
        #[serde(rename = "DAY_MONTH")]
        #[doc = "Group dates by day and month, for example 22-Nov. The month is translated based on the spreadsheet locale."]
        DayMonth,
        #[serde(rename = "MONTH")]
        #[doc = "Group dates by month, for example Nov. The month is translated based on the spreadsheet locale."]
        Month,
        #[serde(rename = "QUARTER")]
        #[doc = "Group dates by quarter, for example Q1 (which represents Jan-Mar)."]
        Quarter,
        #[serde(rename = "YEAR")]
        #[doc = "Group dates by year, for example 2008."]
        Year,
        #[serde(rename = "YEAR_MONTH")]
        #[doc = "Group dates by year and month, for example 2008-Nov. The month is translated based on the spreadsheet locale."]
        YearMonth,
        #[serde(rename = "YEAR_QUARTER")]
        #[doc = "Group dates by year and quarter, for example 2008 Q4."]
        YearQuarter,
        #[serde(rename = "YEAR_MONTH_DAY")]
        #[doc = "Group dates by year, month, and day, for example 2008-11-22."]
        YearMonthDay,
    }
    impl ::std::default::Default for ChartDateTimeRuleTypeEnum {
        fn default() -> Self {
            Self::ChartDateTimeRuleTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An optional setting on the ChartData of the domain of a data source chart that defines buckets for the values in the domain rather than breaking out each individual value. For example, when plotting a data source chart, you can specify a histogram rule on the domain (it should only contain numeric values), grouping its values into buckets. Any values of a chart series that fall into the same bucket are aggregated based on the aggregate_type."]
    pub struct ChartGroupRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateTimeRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A ChartDateTimeRule."]
        pub date_time_rule: ::std::option::Option<::std::boxed::Box<ChartDateTimeRule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "histogramRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A ChartHistogramRule"]
        pub histogram_rule: ::std::option::Option<::std::boxed::Box<ChartHistogramRule>>,
    }
    impl ChartGroupRule {
        pub fn builder() -> ChartGroupRuleBuilder {
            ChartGroupRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Allows you to organize numeric values in a source data column into buckets of constant size."]
    pub struct ChartHistogramRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intervalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the buckets that are created. Must be positive."]
        pub interval_size: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum value at which items are placed into buckets. Values greater than the maximum are grouped into a single bucket. If omitted, it is determined by the maximum item value."]
        pub max_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum value at which items are placed into buckets. Values that are less than the minimum are grouped into a single bucket. If omitted, it is determined by the minimum item value."]
        pub min_value: ::std::option::Option<::std::primitive::f64>,
    }
    impl ChartHistogramRule {
        pub fn builder() -> ChartHistogramRuleBuilder {
            ChartHistogramRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Source ranges for a chart."]
    pub struct ChartSourceRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ranges of data for a series or domain. Exactly one dimension must have a length of 1, and all sources in the list must have the same dimension with length 1. The domain (if it exists) & all series must have the same number of source ranges. If using more than one source range, then the source range at a given offset must be in order and contiguous across the domain and series. For example, these are valid configurations: domain sources: A1:A5 series1 sources: B1:B5 series2 sources: D6:D10 domain sources: A1:A5, C10:C12 series1 sources: B1:B5, D10:D12 series2 sources: C1:C5, E10:E12"]
        pub sources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GridRange>>>,
    }
    impl ChartSourceRange {
        pub fn builder() -> ChartSourceRangeBuilder {
            ChartSourceRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The specifications of a chart."]
    pub struct ChartSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "altText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The alternative text that describes the chart. This is often used for accessibility."]
        pub alt_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color of the entire chart. Not applicable to Org charts."]
        pub background_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color of the entire chart. Not applicable to Org charts. If background_color is also set, this field takes precedence."]
        pub background_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A basic chart specification, can be one of many kinds of charts. See BasicChartType for the list of all charts this supports."]
        pub basic_chart: ::std::option::Option<::std::boxed::Box<BasicChartSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bubbleChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A bubble chart specification."]
        pub bubble_chart: ::std::option::Option<::std::boxed::Box<BubbleChartSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "candlestickChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A candlestick chart specification."]
        pub candlestick_chart: ::std::option::Option<::std::boxed::Box<CandlestickChartSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceChartProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, the field contains data source chart specific properties."]
        pub data_source_chart_properties:
            ::std::option::Option<::std::boxed::Box<DataSourceChartProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterSpecs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filters applied to the source data of the chart. Only supported for data source charts."]
        pub filter_specs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FilterSpec>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fontName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the font to use by default for all chart text (e.g. title, axis labels, legend). If a font is specified for a specific part of the chart it will override this font name."]
        pub font_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hiddenDimensionStrategy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines how the charts will use hidden rows or columns."]
        pub hidden_dimension_strategy: ::std::option::Option<ChartSpecHiddenDimensionStrategyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "histogramChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A histogram chart specification."]
        pub histogram_chart: ::std::option::Option<::std::boxed::Box<HistogramChartSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maximized")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True to make a chart fill the entire space in which it's rendered with minimum padding. False to use the default padding. (Not applicable to Geo and Org charts.)"]
        pub maximized: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orgChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An org chart specification."]
        pub org_chart: ::std::option::Option<::std::boxed::Box<OrgChartSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pieChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A pie chart specification."]
        pub pie_chart: ::std::option::Option<::std::boxed::Box<PieChartSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scorecardChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A scorecard chart specification."]
        pub scorecard_chart: ::std::option::Option<::std::boxed::Box<ScorecardChartSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortSpecs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The order to sort the chart data by. Only a single sort spec is supported. Only supported for data source charts."]
        pub sort_specs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SortSpec>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The subtitle of the chart."]
        pub subtitle: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtitleTextFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The subtitle text format. Strikethrough and underline are not supported."]
        pub subtitle_text_format: ::std::option::Option<::std::boxed::Box<TextFormat>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtitleTextPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The subtitle text position. This field is optional."]
        pub subtitle_text_position: ::std::option::Option<::std::boxed::Box<TextPosition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the chart."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "titleTextFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title text format. Strikethrough and underline are not supported."]
        pub title_text_format: ::std::option::Option<::std::boxed::Box<TextFormat>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "titleTextPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title text position. This field is optional."]
        pub title_text_position: ::std::option::Option<::std::boxed::Box<TextPosition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "treemapChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A treemap chart specification."]
        pub treemap_chart: ::std::option::Option<::std::boxed::Box<TreemapChartSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "waterfallChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A waterfall chart specification."]
        pub waterfall_chart: ::std::option::Option<::std::boxed::Box<WaterfallChartSpec>>,
    }
    impl ChartSpec {
        pub fn builder() -> ChartSpecBuilder {
            ChartSpecBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Determines how the charts will use hidden rows or columns."]
    pub enum ChartSpecHiddenDimensionStrategyEnum {
        #[serde(rename = "CHART_HIDDEN_DIMENSION_STRATEGY_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        ChartHiddenDimensionStrategyUnspecified,
        #[serde(rename = "SKIP_HIDDEN_ROWS_AND_COLUMNS")]
        #[doc = "Charts will skip hidden rows and columns."]
        SkipHiddenRowsAndColumns,
        #[serde(rename = "SKIP_HIDDEN_ROWS")]
        #[doc = "Charts will skip hidden rows only."]
        SkipHiddenRows,
        #[serde(rename = "SKIP_HIDDEN_COLUMNS")]
        #[doc = "Charts will skip hidden columns only."]
        SkipHiddenColumns,
        #[serde(rename = "SHOW_ALL")]
        #[doc = "Charts will not skip any hidden rows or columns."]
        ShowAll,
    }
    impl ::std::default::Default for ChartSpecHiddenDimensionStrategyEnum {
        fn default() -> Self {
            Self::ChartHiddenDimensionStrategyUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Clears the basic filter, if any exists on the sheet."]
    pub struct ClearBasicFilterRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sheet ID on which the basic filter should be cleared."]
        pub sheet_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl ClearBasicFilterRequest {
        pub fn builder() -> ClearBasicFilterRequestBuilder {
            ClearBasicFilterRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for clearing a range of values in a spreadsheet."]
    pub struct ClearValuesRequest {}
    impl ClearValuesRequest {
        pub fn builder() -> ClearValuesRequestBuilder {
            ClearValuesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response when clearing a range of values in a spreadsheet."]
    pub struct ClearValuesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clearedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range (in A1 notation) that was cleared. (If the request was for an unbounded range or a ranger larger than the bounds of the sheet, this will be the actual range that was cleared, bounded to the sheet's limits.)"]
        pub cleared_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spreadsheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The spreadsheet the updates were applied to."]
        pub spreadsheet_id: ::std::option::Option<::std::string::String>,
    }
    impl ClearValuesResponse {
        pub fn builder() -> ClearValuesResponseBuilder {
            ClearValuesResponseBuilder::default()
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
    #[doc = "A color value."]
    pub struct ColorStyle {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rgbColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RGB color."]
        pub rgb_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "themeColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Theme color."]
        pub theme_color: ::std::option::Option<ColorStyleThemeColorEnum>,
    }
    impl ColorStyle {
        pub fn builder() -> ColorStyleBuilder {
            ColorStyleBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Theme color."]
    pub enum ColorStyleThemeColorEnum {
        #[serde(rename = "THEME_COLOR_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified theme color"]
        ThemeColorTypeUnspecified,
        #[serde(rename = "TEXT")]
        #[doc = "Represents the primary text color"]
        Text,
        #[serde(rename = "BACKGROUND")]
        #[doc = "Represents the primary background color"]
        Background,
        #[serde(rename = "ACCENT1")]
        #[doc = "Represents the first accent color"]
        Accent1,
        #[serde(rename = "ACCENT2")]
        #[doc = "Represents the second accent color"]
        Accent2,
        #[serde(rename = "ACCENT3")]
        #[doc = "Represents the third accent color"]
        Accent3,
        #[serde(rename = "ACCENT4")]
        #[doc = "Represents the fourth accent color"]
        Accent4,
        #[serde(rename = "ACCENT5")]
        #[doc = "Represents the fifth accent color"]
        Accent5,
        #[serde(rename = "ACCENT6")]
        #[doc = "Represents the sixth accent color"]
        Accent6,
        #[serde(rename = "LINK")]
        #[doc = "Represents the color to use for hyperlinks"]
        Link,
    }
    impl ::std::default::Default for ColorStyleThemeColorEnum {
        fn default() -> Self {
            Self::ThemeColorTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The value of the condition."]
    pub struct ConditionValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relativeDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A relative date (based on the current date). Valid only if the type is DATE_BEFORE, DATE_AFTER, DATE_ON_OR_BEFORE or DATE_ON_OR_AFTER. Relative dates are not supported in data validation. They are supported only in conditional formatting and conditional filters."]
        pub relative_date: ::std::option::Option<ConditionValueRelativeDateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEnteredValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A value the condition is based on. The value is parsed as if the user typed into a cell. Formulas are supported (and must begin with an `=` or a '+')."]
        pub user_entered_value: ::std::option::Option<::std::string::String>,
    }
    impl ConditionValue {
        pub fn builder() -> ConditionValueBuilder {
            ConditionValueBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "A relative date (based on the current date). Valid only if the type is DATE_BEFORE, DATE_AFTER, DATE_ON_OR_BEFORE or DATE_ON_OR_AFTER. Relative dates are not supported in data validation. They are supported only in conditional formatting and conditional filters."]
    pub enum ConditionValueRelativeDateEnum {
        #[serde(rename = "RELATIVE_DATE_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        RelativeDateUnspecified,
        #[serde(rename = "PAST_YEAR")]
        #[doc = "The value is one year before today."]
        PastYear,
        #[serde(rename = "PAST_MONTH")]
        #[doc = "The value is one month before today."]
        PastMonth,
        #[serde(rename = "PAST_WEEK")]
        #[doc = "The value is one week before today."]
        PastWeek,
        #[serde(rename = "YESTERDAY")]
        #[doc = "The value is yesterday."]
        Yesterday,
        #[serde(rename = "TODAY")]
        #[doc = "The value is today."]
        Today,
        #[serde(rename = "TOMORROW")]
        #[doc = "The value is tomorrow."]
        Tomorrow,
    }
    impl ::std::default::Default for ConditionValueRelativeDateEnum {
        fn default() -> Self {
            Self::RelativeDateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A rule describing a conditional format."]
    pub struct ConditionalFormatRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "booleanRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The formatting is either \"on\" or \"off\" according to the rule."]
        pub boolean_rule: ::std::option::Option<::std::boxed::Box<BooleanRule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gradientRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The formatting will vary based on the gradients in the rule."]
        pub gradient_rule: ::std::option::Option<::std::boxed::Box<GradientRule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ranges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ranges that are formatted if the condition is true. All the ranges must be on the same grid."]
        pub ranges: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GridRange>>>,
    }
    impl ConditionalFormatRule {
        pub fn builder() -> ConditionalFormatRuleBuilder {
            ConditionalFormatRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Copies data from the source to the destination."]
    pub struct CopyPasteRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location to paste to. If the range covers a span that's a multiple of the source's height or width, then the data will be repeated to fill in the destination range. If the range is smaller than the source range, the entire source data will still be copied (beyond the end of the destination range)."]
        pub destination: ::std::option::Option<::std::boxed::Box<GridRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pasteOrientation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How that data should be oriented when pasting."]
        pub paste_orientation: ::std::option::Option<CopyPasteRequestPasteOrientationEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pasteType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "What kind of data to paste."]
        pub paste_type: ::std::option::Option<CopyPasteRequestPasteTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source range to copy."]
        pub source: ::std::option::Option<::std::boxed::Box<GridRange>>,
    }
    impl CopyPasteRequest {
        pub fn builder() -> CopyPasteRequestBuilder {
            CopyPasteRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How that data should be oriented when pasting."]
    pub enum CopyPasteRequestPasteOrientationEnum {
        #[serde(rename = "NORMAL")]
        #[doc = "Paste normally."]
        Normal,
        #[serde(rename = "TRANSPOSE")]
        #[doc = "Paste transposed, where all rows become columns and vice versa."]
        Transpose,
    }
    impl ::std::default::Default for CopyPasteRequestPasteOrientationEnum {
        fn default() -> Self {
            Self::Normal
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "What kind of data to paste."]
    pub enum CopyPasteRequestPasteTypeEnum {
        #[serde(rename = "PASTE_NORMAL")]
        #[doc = "Paste values, formulas, formats, and merges."]
        PasteNormal,
        #[serde(rename = "PASTE_VALUES")]
        #[doc = "Paste the values ONLY without formats, formulas, or merges."]
        PasteValues,
        #[serde(rename = "PASTE_FORMAT")]
        #[doc = "Paste the format and data validation only."]
        PasteFormat,
        #[serde(rename = "PASTE_NO_BORDERS")]
        #[doc = "Like PASTE_NORMAL but without borders."]
        PasteNoBorders,
        #[serde(rename = "PASTE_FORMULA")]
        #[doc = "Paste the formulas only."]
        PasteFormula,
        #[serde(rename = "PASTE_DATA_VALIDATION")]
        #[doc = "Paste the data validation only."]
        PasteDataValidation,
        #[serde(rename = "PASTE_CONDITIONAL_FORMATTING")]
        #[doc = "Paste the conditional formatting rules only."]
        PasteConditionalFormatting,
    }
    impl ::std::default::Default for CopyPasteRequestPasteTypeEnum {
        fn default() -> Self {
            Self::PasteNormal
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request to copy a sheet across spreadsheets."]
    pub struct CopySheetToAnotherSpreadsheetRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationSpreadsheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the spreadsheet to copy the sheet to."]
        pub destination_spreadsheet_id: ::std::option::Option<::std::string::String>,
    }
    impl CopySheetToAnotherSpreadsheetRequest {
        pub fn builder() -> CopySheetToAnotherSpreadsheetRequestBuilder {
            CopySheetToAnotherSpreadsheetRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to create developer metadata."]
    pub struct CreateDeveloperMetadataRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "developerMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The developer metadata to create."]
        pub developer_metadata: ::std::option::Option<::std::boxed::Box<DeveloperMetadata>>,
    }
    impl CreateDeveloperMetadataRequest {
        pub fn builder() -> CreateDeveloperMetadataRequestBuilder {
            CreateDeveloperMetadataRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response from creating developer metadata."]
    pub struct CreateDeveloperMetadataResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "developerMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The developer metadata that was created."]
        pub developer_metadata: ::std::option::Option<::std::boxed::Box<DeveloperMetadata>>,
    }
    impl CreateDeveloperMetadataResponse {
        pub fn builder() -> CreateDeveloperMetadataResponseBuilder {
            CreateDeveloperMetadataResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Moves data from the source to the destination."]
    pub struct CutPasteRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The top-left coordinate where the data should be pasted."]
        pub destination: ::std::option::Option<::std::boxed::Box<GridCoordinate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pasteType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "What kind of data to paste. All the source data will be cut, regardless of what is pasted."]
        pub paste_type: ::std::option::Option<CutPasteRequestPasteTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source data to cut."]
        pub source: ::std::option::Option<::std::boxed::Box<GridRange>>,
    }
    impl CutPasteRequest {
        pub fn builder() -> CutPasteRequestBuilder {
            CutPasteRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "What kind of data to paste. All the source data will be cut, regardless of what is pasted."]
    pub enum CutPasteRequestPasteTypeEnum {
        #[serde(rename = "PASTE_NORMAL")]
        #[doc = "Paste values, formulas, formats, and merges."]
        PasteNormal,
        #[serde(rename = "PASTE_VALUES")]
        #[doc = "Paste the values ONLY without formats, formulas, or merges."]
        PasteValues,
        #[serde(rename = "PASTE_FORMAT")]
        #[doc = "Paste the format and data validation only."]
        PasteFormat,
        #[serde(rename = "PASTE_NO_BORDERS")]
        #[doc = "Like PASTE_NORMAL but without borders."]
        PasteNoBorders,
        #[serde(rename = "PASTE_FORMULA")]
        #[doc = "Paste the formulas only."]
        PasteFormula,
        #[serde(rename = "PASTE_DATA_VALIDATION")]
        #[doc = "Paste the data validation only."]
        PasteDataValidation,
        #[serde(rename = "PASTE_CONDITIONAL_FORMATTING")]
        #[doc = "Paste the conditional formatting rules only."]
        PasteConditionalFormatting,
    }
    impl ::std::default::Default for CutPasteRequestPasteTypeEnum {
        fn default() -> Self {
            Self::PasteNormal
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The data execution status. A data execution is created to sync a data source object with the latest data from a DataSource. It is usually scheduled to run at background, you can check its state to tell if an execution completes There are several scenarios where a data execution is triggered to run: * Adding a data source creates an associated data source sheet as well as a data execution to sync the data from the data source to the sheet. * Updating a data source creates a data execution to refresh the associated data source sheet similarly. * You can send refresh request to explicitly refresh one or multiple data source objects."]
    pub struct DataExecutionStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error code."]
        pub error_code: ::std::option::Option<DataExecutionStatusErrorCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error message, which may be empty."]
        pub error_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastRefreshTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Gets the time the data last successfully refreshed."]
        pub last_refresh_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the data execution."]
        pub state: ::std::option::Option<DataExecutionStatusStateEnum>,
    }
    impl DataExecutionStatus {
        pub fn builder() -> DataExecutionStatusBuilder {
            DataExecutionStatusBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The error code."]
    pub enum DataExecutionStatusErrorCodeEnum {
        #[serde(rename = "DATA_EXECUTION_ERROR_CODE_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        DataExecutionErrorCodeUnspecified,
        #[serde(rename = "TIMED_OUT")]
        #[doc = "The data execution timed out."]
        TimedOut,
        #[serde(rename = "TOO_MANY_ROWS")]
        #[doc = "The data execution returns more rows than the limit."]
        TooManyRows,
        #[serde(rename = "TOO_MANY_CELLS")]
        #[doc = "The data execution returns more cells than the limit."]
        TooManyCells,
        #[serde(rename = "ENGINE")]
        #[doc = "Error is received from the backend data execution engine (e.g. BigQuery). Check error_message for details."]
        Engine,
        #[serde(rename = "PARAMETER_INVALID")]
        #[doc = "One or some of the provided data source parameters are invalid."]
        ParameterInvalid,
        #[serde(rename = "UNSUPPORTED_DATA_TYPE")]
        #[doc = "The data execution returns an unsupported data type."]
        UnsupportedDataType,
        #[serde(rename = "DUPLICATE_COLUMN_NAMES")]
        #[doc = "The data execution returns duplicate column names or aliases."]
        DuplicateColumnNames,
        #[serde(rename = "INTERRUPTED")]
        #[doc = "The data execution is interrupted. Please refresh later."]
        Interrupted,
        #[serde(rename = "CONCURRENT_QUERY")]
        #[doc = "The data execution is currently in progress, can not be refreshed until it completes."]
        ConcurrentQuery,
        #[serde(rename = "OTHER")]
        #[doc = "Other errors."]
        Other,
        #[serde(rename = "TOO_MANY_CHARS_PER_CELL")]
        #[doc = "The data execution returns values that exceed the maximum characters allowed in a single cell."]
        TooManyCharsPerCell,
        #[serde(rename = "DATA_NOT_FOUND")]
        #[doc = "The database referenced by the data source is not found. */"]
        DataNotFound,
        #[serde(rename = "PERMISSION_DENIED")]
        #[doc = "The user does not have access to the database referenced by the data source."]
        PermissionDenied,
        #[serde(rename = "MISSING_COLUMN_ALIAS")]
        #[doc = "The data execution returns columns with missing aliases."]
        MissingColumnAlias,
        #[serde(rename = "OBJECT_NOT_FOUND")]
        #[doc = "The data source object does not exist."]
        ObjectNotFound,
        #[serde(rename = "OBJECT_IN_ERROR_STATE")]
        #[doc = "The data source object is currently in error state. To force refresh, set force in RefreshDataSourceRequest."]
        ObjectInErrorState,
        #[serde(rename = "OBJECT_SPEC_INVALID")]
        #[doc = "The data source object specification is invalid."]
        ObjectSpecInvalid,
    }
    impl ::std::default::Default for DataExecutionStatusErrorCodeEnum {
        fn default() -> Self {
            Self::DataExecutionErrorCodeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state of the data execution."]
    pub enum DataExecutionStatusStateEnum {
        #[serde(rename = "DATA_EXECUTION_STATE_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        DataExecutionStateUnspecified,
        #[serde(rename = "NOT_STARTED")]
        #[doc = "The data execution has not started."]
        NotStarted,
        #[serde(rename = "RUNNING")]
        #[doc = "The data execution has started and is running."]
        Running,
        #[serde(rename = "SUCCEEDED")]
        #[doc = "The data execution has completed successfully."]
        Succeeded,
        #[serde(rename = "FAILED")]
        #[doc = "The data execution has completed with errors."]
        Failed,
    }
    impl ::std::default::Default for DataExecutionStatusStateEnum {
        fn default() -> Self {
            Self::DataExecutionStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Filter that describes what data should be selected or returned from a request."]
    pub struct DataFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "a1Range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Selects data that matches the specified A1 range."]
        pub a1_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "developerMetadataLookup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Selects data associated with the developer metadata matching the criteria described by this DeveloperMetadataLookup."]
        pub developer_metadata_lookup:
            ::std::option::Option<::std::boxed::Box<DeveloperMetadataLookup>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gridRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Selects data that matches the range described by the GridRange."]
        pub grid_range: ::std::option::Option<::std::boxed::Box<GridRange>>,
    }
    impl DataFilter {
        pub fn builder() -> DataFilterBuilder {
            DataFilterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A range of values whose location is specified by a DataFilter."]
    pub struct DataFilterValueRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data filter describing the location of the values in the spreadsheet."]
        pub data_filter: ::std::option::Option<::std::boxed::Box<DataFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "majorDimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The major dimension of the values."]
        pub major_dimension: ::std::option::Option<DataFilterValueRangeMajorDimensionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data to be written. If the provided values exceed any of the ranges matched by the data filter then the request fails. If the provided values are less than the matched ranges only the specified values are written, existing values in the matched ranges remain unaffected."]
        pub values: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::serde_json::Value>>>,
    }
    impl DataFilterValueRange {
        pub fn builder() -> DataFilterValueRangeBuilder {
            DataFilterValueRangeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The major dimension of the values."]
    pub enum DataFilterValueRangeMajorDimensionEnum {
        #[serde(rename = "DIMENSION_UNSPECIFIED")]
        #[doc = "The default value, do not use."]
        DimensionUnspecified,
        #[serde(rename = "ROWS")]
        #[doc = "Operates on the rows of a sheet."]
        Rows,
        #[serde(rename = "COLUMNS")]
        #[doc = "Operates on the columns of a sheet."]
        Columns,
    }
    impl ::std::default::Default for DataFilterValueRangeMajorDimensionEnum {
        fn default() -> Self {
            Self::DimensionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings for one set of data labels. Data labels are annotations that appear next to a set of data, such as the points on a line chart, and provide additional information about what the data represents, such as a text representation of the value behind that point on the graph."]
    pub struct DataLabel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customLabelData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data to use for custom labels. Only used if type is set to CUSTOM. This data must be the same length as the series or other element this data label is applied to. In addition, if the series is split into multiple source ranges, this source data must come from the next column in the source data. For example, if the series is B2:B4,E6:E8 then this data must come from C2:C4,F6:F8."]
        pub custom_label_data: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "placement")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The placement of the data label relative to the labeled data."]
        pub placement: ::std::option::Option<DataLabelPlacementEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text format used for the data label."]
        pub text_format: ::std::option::Option<::std::boxed::Box<TextFormat>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the data label."]
        pub _type: ::std::option::Option<DataLabelTypeEnum>,
    }
    impl DataLabel {
        pub fn builder() -> DataLabelBuilder {
            DataLabelBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The placement of the data label relative to the labeled data."]
    pub enum DataLabelPlacementEnum {
        #[serde(rename = "DATA_LABEL_PLACEMENT_UNSPECIFIED")]
        #[doc = "The positioning is determined automatically by the renderer."]
        DataLabelPlacementUnspecified,
        #[serde(rename = "CENTER")]
        #[doc = "Center within a bar or column, both horizontally and vertically."]
        Center,
        #[serde(rename = "LEFT")]
        #[doc = "To the left of a data point."]
        Left,
        #[serde(rename = "RIGHT")]
        #[doc = "To the right of a data point."]
        Right,
        #[serde(rename = "ABOVE")]
        #[doc = "Above a data point."]
        Above,
        #[serde(rename = "BELOW")]
        #[doc = "Below a data point."]
        Below,
        #[serde(rename = "INSIDE_END")]
        #[doc = "Inside a bar or column at the end (top if positive, bottom if negative)."]
        InsideEnd,
        #[serde(rename = "INSIDE_BASE")]
        #[doc = "Inside a bar or column at the base."]
        InsideBase,
        #[serde(rename = "OUTSIDE_END")]
        #[doc = "Outside a bar or column at the end."]
        OutsideEnd,
    }
    impl ::std::default::Default for DataLabelPlacementEnum {
        fn default() -> Self {
            Self::DataLabelPlacementUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the data label."]
    pub enum DataLabelTypeEnum {
        #[serde(rename = "DATA_LABEL_TYPE_UNSPECIFIED")]
        #[doc = "The data label type is not specified and will be interpreted depending on the context of the data label within the chart."]
        DataLabelTypeUnspecified,
        #[serde(rename = "NONE")]
        #[doc = "The data label is not displayed."]
        None,
        #[serde(rename = "DATA")]
        #[doc = "The data label is displayed using values from the series data."]
        Data,
        #[serde(rename = "CUSTOM")]
        #[doc = "The data label is displayed using values from a custom data source indicated by customLabelData."]
        Custom,
    }
    impl ::std::default::Default for DataLabelTypeEnum {
        fn default() -> Self {
            Self::DataLabelTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about an external data source in the spreadsheet."]
    pub struct DataSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "calculatedColumns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All calculated columns in the data source."]
        pub calculated_columns:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSourceColumn>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The spreadsheet-scoped unique ID that identifies the data source. Example: 1080547365."]
        pub data_source_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the Sheet connected with the data source. The field cannot be changed once set. When creating a data source, an associated DATA_SOURCE sheet is also created, if the field is not specified, the ID of the created sheet will be randomly generated."]
        pub sheet_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The DataSourceSpec for the data source connected with this spreadsheet."]
        pub spec: ::std::option::Option<::std::boxed::Box<DataSourceSpec>>,
    }
    impl DataSource {
        pub fn builder() -> DataSourceBuilder {
            DataSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Properties of a data source chart."]
    pub struct DataSourceChartProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataExecutionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The data execution status."]
        pub data_execution_status: ::std::option::Option<::std::boxed::Box<DataExecutionStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the data source that the chart is associated with."]
        pub data_source_id: ::std::option::Option<::std::string::String>,
    }
    impl DataSourceChartProperties {
        pub fn builder() -> DataSourceChartPropertiesBuilder {
            DataSourceChartPropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A column in a data source."]
    pub struct DataSourceColumn {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formula")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The formula of the calculated column."]
        pub formula: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The column reference."]
        pub reference: ::std::option::Option<::std::boxed::Box<DataSourceColumnReference>>,
    }
    impl DataSourceColumn {
        pub fn builder() -> DataSourceColumnBuilder {
            DataSourceColumnBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An unique identifier that references a data source column."]
    pub struct DataSourceColumnReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display name of the column. It should be unique within a data source."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl DataSourceColumnReference {
        pub fn builder() -> DataSourceColumnReferenceBuilder {
            DataSourceColumnReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A data source formula."]
    pub struct DataSourceFormula {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataExecutionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The data execution status."]
        pub data_execution_status: ::std::option::Option<::std::boxed::Box<DataExecutionStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the data source the formula is associated with."]
        pub data_source_id: ::std::option::Option<::std::string::String>,
    }
    impl DataSourceFormula {
        pub fn builder() -> DataSourceFormulaBuilder {
            DataSourceFormulaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Reference to a data source object."]
    pub struct DataSourceObjectReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chartId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "References to a data source chart."]
        pub chart_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceFormulaCell")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "References to a cell containing DataSourceFormula."]
        pub data_source_formula_cell: ::std::option::Option<::std::boxed::Box<GridCoordinate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourcePivotTableAnchorCell")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "References to a data source PivotTable anchored at the cell."]
        pub data_source_pivot_table_anchor_cell:
            ::std::option::Option<::std::boxed::Box<GridCoordinate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceTableAnchorCell")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "References to a DataSourceTable anchored at the cell."]
        pub data_source_table_anchor_cell: ::std::option::Option<::std::boxed::Box<GridCoordinate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "References to a DATA_SOURCE sheet."]
        pub sheet_id: ::std::option::Option<::std::string::String>,
    }
    impl DataSourceObjectReference {
        pub fn builder() -> DataSourceObjectReferenceBuilder {
            DataSourceObjectReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of references to data source objects."]
    pub struct DataSourceObjectReferences {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "references")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The references."]
        pub references:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSourceObjectReference>>>,
    }
    impl DataSourceObjectReferences {
        pub fn builder() -> DataSourceObjectReferencesBuilder {
            DataSourceObjectReferencesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A parameter in a data source's query. The parameter allows the user to pass in values from the spreadsheet into a query."]
    pub struct DataSourceParameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Named parameter. Must be a legitimate identifier for the DataSource that supports it. For example, [BigQuery identifier](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#identifiers)."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namedRangeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of a NamedRange. Its size must be 1x1."]
        pub named_range_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A range that contains the value of the parameter. Its size must be 1x1."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
    }
    impl DataSourceParameter {
        pub fn builder() -> DataSourceParameterBuilder {
            DataSourceParameterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A schedule for data to refresh every day in a given time interval."]
    pub struct DataSourceRefreshDailySchedule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start time of a time interval in which a data source refresh is scheduled. Only `hours` part is used. The time interval size defaults to that in the Sheets editor."]
        pub start_time: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
    }
    impl DataSourceRefreshDailySchedule {
        pub fn builder() -> DataSourceRefreshDailyScheduleBuilder {
            DataSourceRefreshDailyScheduleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A monthly schedule for data to refresh on specific days in the month in a given time interval."]
    pub struct DataSourceRefreshMonthlySchedule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "daysOfMonth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Days of the month to refresh. Only 1-28 are supported, mapping to the 1st to the 28th day. At lesat one day must be specified."]
        pub days_of_month: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start time of a time interval in which a data source refresh is scheduled. Only `hours` part is used. The time interval size defaults to that in the Sheets editor."]
        pub start_time: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
    }
    impl DataSourceRefreshMonthlySchedule {
        pub fn builder() -> DataSourceRefreshMonthlyScheduleBuilder {
            DataSourceRefreshMonthlyScheduleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Schedule for refreshing the data source. Data sources in the spreadsheet are refreshed within a time interval. You can specify the start time by clicking the Scheduled Refresh button in the Sheets editor, but the interval is fixed at 4 hours. For example, if you specify a start time of 8am , the refresh will take place between 8am and 12pm every day."]
    pub struct DataSourceRefreshSchedule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dailySchedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Daily refresh schedule."]
        pub daily_schedule:
            ::std::option::Option<::std::boxed::Box<DataSourceRefreshDailySchedule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the refresh schedule is enabled, or false otherwise."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monthlySchedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Monthly refresh schedule."]
        pub monthly_schedule:
            ::std::option::Option<::std::boxed::Box<DataSourceRefreshMonthlySchedule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextRun")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time interval of the next run."]
        pub next_run: ::std::option::Option<::std::boxed::Box<Interval>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "refreshScope")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The scope of the refresh. Must be ALL_DATA_SOURCES."]
        pub refresh_scope: ::std::option::Option<DataSourceRefreshScheduleRefreshScopeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "weeklySchedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Weekly refresh schedule."]
        pub weekly_schedule:
            ::std::option::Option<::std::boxed::Box<DataSourceRefreshWeeklySchedule>>,
    }
    impl DataSourceRefreshSchedule {
        pub fn builder() -> DataSourceRefreshScheduleBuilder {
            DataSourceRefreshScheduleBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The scope of the refresh. Must be ALL_DATA_SOURCES."]
    pub enum DataSourceRefreshScheduleRefreshScopeEnum {
        #[serde(rename = "DATA_SOURCE_REFRESH_SCOPE_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        DataSourceRefreshScopeUnspecified,
        #[serde(rename = "ALL_DATA_SOURCES")]
        #[doc = "Refreshes all data sources and their associated data source objects in the spreadsheet."]
        AllDataSources,
    }
    impl ::std::default::Default for DataSourceRefreshScheduleRefreshScopeEnum {
        fn default() -> Self {
            Self::DataSourceRefreshScopeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A weekly schedule for data to refresh on specific days in a given time interval."]
    pub struct DataSourceRefreshWeeklySchedule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "daysOfWeek")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Days of the week to refresh. At least one day must be specified."]
        pub days_of_week:
            ::std::option::Option<::std::vec::Vec<DataSourceRefreshWeeklyScheduleDaysOfWeekEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start time of a time interval in which a data source refresh is scheduled. Only `hours` part is used. The time interval size defaults to that in the Sheets editor."]
        pub start_time: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
    }
    impl DataSourceRefreshWeeklySchedule {
        pub fn builder() -> DataSourceRefreshWeeklyScheduleBuilder {
            DataSourceRefreshWeeklyScheduleBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum DataSourceRefreshWeeklyScheduleDaysOfWeekEnum {
        #[serde(rename = "DAY_OF_WEEK_UNSPECIFIED")]
        #[doc = "The day of the week is unspecified."]
        DayOfWeekUnspecified,
        #[serde(rename = "MONDAY")]
        #[doc = "Monday"]
        Monday,
        #[serde(rename = "TUESDAY")]
        #[doc = "Tuesday"]
        Tuesday,
        #[serde(rename = "WEDNESDAY")]
        #[doc = "Wednesday"]
        Wednesday,
        #[serde(rename = "THURSDAY")]
        #[doc = "Thursday"]
        Thursday,
        #[serde(rename = "FRIDAY")]
        #[doc = "Friday"]
        Friday,
        #[serde(rename = "SATURDAY")]
        #[doc = "Saturday"]
        Saturday,
        #[serde(rename = "SUNDAY")]
        #[doc = "Sunday"]
        Sunday,
    }
    impl ::std::default::Default for DataSourceRefreshWeeklyScheduleDaysOfWeekEnum {
        fn default() -> Self {
            Self::DayOfWeekUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A range along a single dimension on a DATA_SOURCE sheet."]
    pub struct DataSourceSheetDimensionRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnReferences")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The columns on the data source sheet."]
        pub column_references:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSourceColumnReference>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the data source sheet the range is on."]
        pub sheet_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl DataSourceSheetDimensionRange {
        pub fn builder() -> DataSourceSheetDimensionRangeBuilder {
            DataSourceSheetDimensionRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional properties of a DATA_SOURCE sheet."]
    pub struct DataSourceSheetProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The columns displayed on the sheet, corresponding to the values in RowData."]
        pub columns: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSourceColumn>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataExecutionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data execution status."]
        pub data_execution_status: ::std::option::Option<::std::boxed::Box<DataExecutionStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the DataSource the sheet is connected to."]
        pub data_source_id: ::std::option::Option<::std::string::String>,
    }
    impl DataSourceSheetProperties {
        pub fn builder() -> DataSourceSheetPropertiesBuilder {
            DataSourceSheetPropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This specifies the details of the data source. For example, for BigQuery, this specifies information about the BigQuery source."]
    pub struct DataSourceSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bigQuery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A BigQueryDataSourceSpec."]
        pub big_query: ::std::option::Option<::std::boxed::Box<BigQueryDataSourceSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The parameters of the data source, used when querying the data source."]
        pub parameters:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSourceParameter>>>,
    }
    impl DataSourceSpec {
        pub fn builder() -> DataSourceSpecBuilder {
            DataSourceSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A data source table, which allows the user to import a static table of data from the DataSource into Sheets. This is also known as \"Extract\" in the Sheets editor."]
    pub struct DataSourceTable {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnSelectionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type to select columns for the data source table. Defaults to SELECTED."]
        pub column_selection_type: ::std::option::Option<DataSourceTableColumnSelectionTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Columns selected for the data source table. The column_selection_type must be SELECTED."]
        pub columns:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSourceColumnReference>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataExecutionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The data execution status."]
        pub data_execution_status: ::std::option::Option<::std::boxed::Box<DataExecutionStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the data source the data source table is associated with."]
        pub data_source_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterSpecs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Filter specifications in the data source table."]
        pub filter_specs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FilterSpec>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowLimit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The limit of rows to return. If not set, a default limit is applied. Please refer to the Sheets editor for the default and max limit."]
        pub row_limit: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortSpecs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sort specifications in the data source table. The result of the data source table is sorted based on the sort specifications in order."]
        pub sort_specs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SortSpec>>>,
    }
    impl DataSourceTable {
        pub fn builder() -> DataSourceTableBuilder {
            DataSourceTableBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type to select columns for the data source table. Defaults to SELECTED."]
    pub enum DataSourceTableColumnSelectionTypeEnum {
        #[serde(rename = "DATA_SOURCE_TABLE_COLUMN_SELECTION_TYPE_UNSPECIFIED")]
        #[doc = "The default column selection type, do not use."]
        DataSourceTableColumnSelectionTypeUnspecified,
        #[serde(rename = "SELECTED")]
        #[doc = "Select columns specified by columns field."]
        Selected,
        #[serde(rename = "SYNC_ALL")]
        #[doc = "Sync all current and future columns in the data source. If set, the data source table fetches all the columns in the data source at the time of refresh."]
        SyncAll,
    }
    impl ::std::default::Default for DataSourceTableColumnSelectionTypeEnum {
        fn default() -> Self {
            Self::DataSourceTableColumnSelectionTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A data validation rule."]
    pub struct DataValidationRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The condition that data in the cell must match."]
        pub condition: ::std::option::Option<::std::boxed::Box<BooleanCondition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message to show the user when adding data to the cell."]
        pub input_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "showCustomUi")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the UI should be customized based on the kind of condition. If true, \"List\" conditions will show a dropdown."]
        pub show_custom_ui: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "strict")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if invalid data should be rejected."]
        pub strict: ::std::option::Option<::std::primitive::bool>,
    }
    impl DataValidationRule {
        pub fn builder() -> DataValidationRuleBuilder {
            DataValidationRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Allows you to organize the date-time values in a source data column into buckets based on selected parts of their date or time values. For example, consider a pivot table showing sales transactions by date: +----------+--------------+ | Date | SUM of Sales | +----------+--------------+ | 1/1/2017 | $621.14 | | 2/3/2017 | $708.84 | | 5/8/2017 | $326.84 | ... +----------+--------------+ Applying a date-time group rule with a DateTimeRuleType of YEAR_MONTH results in the following pivot table. +--------------+--------------+ | Grouped Date | SUM of Sales | +--------------+--------------+ | 2017-Jan | $53,731.78 | | 2017-Feb | $83,475.32 | | 2017-Mar | $94,385.05 | ... +--------------+--------------+"]
    pub struct DateTimeRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of date-time grouping to apply."]
        pub _type: ::std::option::Option<DateTimeRuleTypeEnum>,
    }
    impl DateTimeRule {
        pub fn builder() -> DateTimeRuleBuilder {
            DateTimeRuleBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of date-time grouping to apply."]
    pub enum DateTimeRuleTypeEnum {
        #[serde(rename = "DATE_TIME_RULE_TYPE_UNSPECIFIED")]
        #[doc = "The default type, do not use."]
        DateTimeRuleTypeUnspecified,
        #[serde(rename = "SECOND")]
        #[doc = "Group dates by second, from 0 to 59."]
        Second,
        #[serde(rename = "MINUTE")]
        #[doc = "Group dates by minute, from 0 to 59."]
        Minute,
        #[serde(rename = "HOUR")]
        #[doc = "Group dates by hour using a 24-hour system, from 0 to 23."]
        Hour,
        #[serde(rename = "HOUR_MINUTE")]
        #[doc = "Group dates by hour and minute using a 24-hour system, for example 19:45."]
        HourMinute,
        #[serde(rename = "HOUR_MINUTE_AMPM")]
        #[doc = "Group dates by hour and minute using a 12-hour system, for example 7:45 PM. The AM/PM designation is translated based on the spreadsheet locale."]
        HourMinuteAmpm,
        #[serde(rename = "DAY_OF_WEEK")]
        #[doc = "Group dates by day of week, for example Sunday. The days of the week will be translated based on the spreadsheet locale."]
        DayOfWeek,
        #[serde(rename = "DAY_OF_YEAR")]
        #[doc = "Group dates by day of year, from 1 to 366. Note that dates after Feb. 29 fall in different buckets in leap years than in non-leap years."]
        DayOfYear,
        #[serde(rename = "DAY_OF_MONTH")]
        #[doc = "Group dates by day of month, from 1 to 31."]
        DayOfMonth,
        #[serde(rename = "DAY_MONTH")]
        #[doc = "Group dates by day and month, for example 22-Nov. The month is translated based on the spreadsheet locale."]
        DayMonth,
        #[serde(rename = "MONTH")]
        #[doc = "Group dates by month, for example Nov. The month is translated based on the spreadsheet locale."]
        Month,
        #[serde(rename = "QUARTER")]
        #[doc = "Group dates by quarter, for example Q1 (which represents Jan-Mar)."]
        Quarter,
        #[serde(rename = "YEAR")]
        #[doc = "Group dates by year, for example 2008."]
        Year,
        #[serde(rename = "YEAR_MONTH")]
        #[doc = "Group dates by year and month, for example 2008-Nov. The month is translated based on the spreadsheet locale."]
        YearMonth,
        #[serde(rename = "YEAR_QUARTER")]
        #[doc = "Group dates by year and quarter, for example 2008 Q4."]
        YearQuarter,
        #[serde(rename = "YEAR_MONTH_DAY")]
        #[doc = "Group dates by year, month, and day, for example 2008-11-22."]
        YearMonthDay,
    }
    impl ::std::default::Default for DateTimeRuleTypeEnum {
        fn default() -> Self {
            Self::DateTimeRuleTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Removes the banded range with the given ID from the spreadsheet."]
    pub struct DeleteBandingRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bandedRangeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the banded range to delete."]
        pub banded_range_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl DeleteBandingRequest {
        pub fn builder() -> DeleteBandingRequestBuilder {
            DeleteBandingRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deletes a conditional format rule at the given index. All subsequent rules' indexes are decremented."]
    pub struct DeleteConditionalFormatRuleRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zero-based index of the rule to be deleted."]
        pub index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sheet the rule is being deleted from."]
        pub sheet_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl DeleteConditionalFormatRuleRequest {
        pub fn builder() -> DeleteConditionalFormatRuleRequestBuilder {
            DeleteConditionalFormatRuleRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of deleting a conditional format rule."]
    pub struct DeleteConditionalFormatRuleResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rule that was deleted."]
        pub rule: ::std::option::Option<::std::boxed::Box<ConditionalFormatRule>>,
    }
    impl DeleteConditionalFormatRuleResponse {
        pub fn builder() -> DeleteConditionalFormatRuleResponseBuilder {
            DeleteConditionalFormatRuleResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deletes a data source. The request also deletes the associated data source sheet, and unlinks all associated data source objects."]
    pub struct DeleteDataSourceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the data source to delete."]
        pub data_source_id: ::std::option::Option<::std::string::String>,
    }
    impl DeleteDataSourceRequest {
        pub fn builder() -> DeleteDataSourceRequestBuilder {
            DeleteDataSourceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to delete developer metadata."]
    pub struct DeleteDeveloperMetadataRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data filter describing the criteria used to select which developer metadata entry to delete."]
        pub data_filter: ::std::option::Option<::std::boxed::Box<DataFilter>>,
    }
    impl DeleteDeveloperMetadataRequest {
        pub fn builder() -> DeleteDeveloperMetadataRequestBuilder {
            DeleteDeveloperMetadataRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response from deleting developer metadata."]
    pub struct DeleteDeveloperMetadataResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deletedDeveloperMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metadata that was deleted."]
        pub deleted_developer_metadata:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeveloperMetadata>>>,
    }
    impl DeleteDeveloperMetadataResponse {
        pub fn builder() -> DeleteDeveloperMetadataResponseBuilder {
            DeleteDeveloperMetadataResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deletes a group over the specified range by decrementing the depth of the dimensions in the range. For example, assume the sheet has a depth-1 group over B:E and a depth-2 group over C:D. Deleting a group over D:E leaves the sheet with a depth-1 group over B:D and a depth-2 group over C:C."]
    pub struct DeleteDimensionGroupRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range of the group to be deleted."]
        pub range: ::std::option::Option<::std::boxed::Box<DimensionRange>>,
    }
    impl DeleteDimensionGroupRequest {
        pub fn builder() -> DeleteDimensionGroupRequestBuilder {
            DeleteDimensionGroupRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of deleting a group."]
    pub struct DeleteDimensionGroupResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionGroups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All groups of a dimension after deleting a group from that dimension."]
        pub dimension_groups:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionGroup>>>,
    }
    impl DeleteDimensionGroupResponse {
        pub fn builder() -> DeleteDimensionGroupResponseBuilder {
            DeleteDimensionGroupResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deletes the dimensions from the sheet."]
    pub struct DeleteDimensionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimensions to delete from the sheet."]
        pub range: ::std::option::Option<::std::boxed::Box<DimensionRange>>,
    }
    impl DeleteDimensionRequest {
        pub fn builder() -> DeleteDimensionRequestBuilder {
            DeleteDimensionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Removes rows within this range that contain values in the specified columns that are duplicates of values in any previous row. Rows with identical values but different letter cases, formatting, or formulas are considered to be duplicates. This request also removes duplicate rows hidden from view (for example, due to a filter). When removing duplicates, the first instance of each duplicate row scanning from the top downwards is kept in the resulting range. Content outside of the specified range isn't removed, and rows considered duplicates do not have to be adjacent to each other in the range."]
    pub struct DeleteDuplicatesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comparisonColumns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The columns in the range to analyze for duplicate values. If no columns are selected then all columns are analyzed for duplicates."]
        pub comparison_columns:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionRange>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range to remove duplicates rows from."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
    }
    impl DeleteDuplicatesRequest {
        pub fn builder() -> DeleteDuplicatesRequestBuilder {
            DeleteDuplicatesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of removing duplicates in a range."]
    pub struct DeleteDuplicatesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duplicatesRemovedCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of duplicate rows removed."]
        pub duplicates_removed_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl DeleteDuplicatesResponse {
        pub fn builder() -> DeleteDuplicatesResponseBuilder {
            DeleteDuplicatesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deletes the embedded object with the given ID."]
    pub struct DeleteEmbeddedObjectRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the embedded object to delete."]
        pub object_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl DeleteEmbeddedObjectRequest {
        pub fn builder() -> DeleteEmbeddedObjectRequestBuilder {
            DeleteEmbeddedObjectRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deletes a particular filter view."]
    pub struct DeleteFilterViewRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the filter to delete."]
        pub filter_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl DeleteFilterViewRequest {
        pub fn builder() -> DeleteFilterViewRequestBuilder {
            DeleteFilterViewRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Removes the named range with the given ID from the spreadsheet."]
    pub struct DeleteNamedRangeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namedRangeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the named range to delete."]
        pub named_range_id: ::std::option::Option<::std::string::String>,
    }
    impl DeleteNamedRangeRequest {
        pub fn builder() -> DeleteNamedRangeRequestBuilder {
            DeleteNamedRangeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deletes the protected range with the given ID."]
    pub struct DeleteProtectedRangeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protectedRangeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the protected range to delete."]
        pub protected_range_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl DeleteProtectedRangeRequest {
        pub fn builder() -> DeleteProtectedRangeRequestBuilder {
            DeleteProtectedRangeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deletes a range of cells, shifting other cells into the deleted area."]
    pub struct DeleteRangeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range of cells to delete."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shiftDimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimension from which deleted cells will be replaced with. If ROWS, existing cells will be shifted upward to replace the deleted cells. If COLUMNS, existing cells will be shifted left to replace the deleted cells."]
        pub shift_dimension: ::std::option::Option<DeleteRangeRequestShiftDimensionEnum>,
    }
    impl DeleteRangeRequest {
        pub fn builder() -> DeleteRangeRequestBuilder {
            DeleteRangeRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The dimension from which deleted cells will be replaced with. If ROWS, existing cells will be shifted upward to replace the deleted cells. If COLUMNS, existing cells will be shifted left to replace the deleted cells."]
    pub enum DeleteRangeRequestShiftDimensionEnum {
        #[serde(rename = "DIMENSION_UNSPECIFIED")]
        #[doc = "The default value, do not use."]
        DimensionUnspecified,
        #[serde(rename = "ROWS")]
        #[doc = "Operates on the rows of a sheet."]
        Rows,
        #[serde(rename = "COLUMNS")]
        #[doc = "Operates on the columns of a sheet."]
        Columns,
    }
    impl ::std::default::Default for DeleteRangeRequestShiftDimensionEnum {
        fn default() -> Self {
            Self::DimensionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deletes the requested sheet."]
    pub struct DeleteSheetRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the sheet to delete. If the sheet is of SheetType.DATA_SOURCE type, the associated DataSource is also deleted."]
        pub sheet_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl DeleteSheetRequest {
        pub fn builder() -> DeleteSheetRequestBuilder {
            DeleteSheetRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Developer metadata associated with a location or object in a spreadsheet. Developer metadata may be used to associate arbitrary data with various parts of a spreadsheet and will remain associated at those locations as they move around and the spreadsheet is edited. For example, if developer metadata is associated with row 5 and another row is then subsequently inserted above row 5, that original metadata will still be associated with the row it was first associated with (what is now row 6). If the associated object is deleted its metadata is deleted too."]
    pub struct DeveloperMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location where the metadata is associated."]
        pub location: ::std::option::Option<::std::boxed::Box<DeveloperMetadataLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadataId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The spreadsheet-scoped unique ID that identifies the metadata. IDs may be specified when metadata is created, otherwise one will be randomly generated and assigned. Must be positive."]
        pub metadata_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadataKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metadata key. There may be multiple metadata in a spreadsheet with the same key. Developer metadata must always have a key specified."]
        pub metadata_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadataValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data associated with the metadata's key."]
        pub metadata_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibility")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metadata visibility. Developer metadata must always have a visibility specified."]
        pub visibility: ::std::option::Option<DeveloperMetadataVisibilityEnum>,
    }
    impl DeveloperMetadata {
        pub fn builder() -> DeveloperMetadataBuilder {
            DeveloperMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The metadata visibility. Developer metadata must always have a visibility specified."]
    pub enum DeveloperMetadataVisibilityEnum {
        #[serde(rename = "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED")]
        #[doc = "Default value."]
        DeveloperMetadataVisibilityUnspecified,
        #[serde(rename = "DOCUMENT")]
        #[doc = "Document-visible metadata is accessible from any developer project with access to the document."]
        Document,
        #[serde(rename = "PROJECT")]
        #[doc = "Project-visible metadata is only visible to and accessible by the developer project that created the metadata."]
        Project,
    }
    impl ::std::default::Default for DeveloperMetadataVisibilityEnum {
        fn default() -> Self {
            Self::DeveloperMetadataVisibilityUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A location where metadata may be associated in a spreadsheet."]
    pub struct DeveloperMetadataLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents the row or column when metadata is associated with a dimension. The specified DimensionRange must represent a single row or column; it cannot be unbounded or span multiple rows or columns."]
        pub dimension_range: ::std::option::Option<::std::boxed::Box<DimensionRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of location this object represents. This field is read-only."]
        pub location_type: ::std::option::Option<DeveloperMetadataLocationLocationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the sheet when metadata is associated with an entire sheet."]
        pub sheet_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spreadsheet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True when metadata is associated with an entire spreadsheet."]
        pub spreadsheet: ::std::option::Option<::std::primitive::bool>,
    }
    impl DeveloperMetadataLocation {
        pub fn builder() -> DeveloperMetadataLocationBuilder {
            DeveloperMetadataLocationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of location this object represents. This field is read-only."]
    pub enum DeveloperMetadataLocationLocationTypeEnum {
        #[serde(rename = "DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED")]
        #[doc = "Default value."]
        DeveloperMetadataLocationTypeUnspecified,
        #[serde(rename = "ROW")]
        #[doc = "Developer metadata associated on an entire row dimension."]
        Row,
        #[serde(rename = "COLUMN")]
        #[doc = "Developer metadata associated on an entire column dimension."]
        Column,
        #[serde(rename = "SHEET")]
        #[doc = "Developer metadata associated on an entire sheet."]
        Sheet,
        #[serde(rename = "SPREADSHEET")]
        #[doc = "Developer metadata associated on the entire spreadsheet."]
        Spreadsheet,
    }
    impl ::std::default::Default for DeveloperMetadataLocationLocationTypeEnum {
        fn default() -> Self {
            Self::DeveloperMetadataLocationTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Selects DeveloperMetadata that matches all of the specified fields. For example, if only a metadata ID is specified this considers the DeveloperMetadata with that particular unique ID. If a metadata key is specified, this considers all developer metadata with that key. If a key, visibility, and location type are all specified, this considers all developer metadata with that key and visibility that are associated with a location of that type. In general, this selects all DeveloperMetadata that matches the intersection of all the specified fields; any field or combination of fields may be specified."]
    pub struct DeveloperMetadataLookup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationMatchingStrategy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines how this lookup matches the location. If this field is specified as EXACT, only developer metadata associated on the exact location specified is matched. If this field is specified to INTERSECTING, developer metadata associated on intersecting locations is also matched. If left unspecified, this field assumes a default value of INTERSECTING. If this field is specified, a metadataLocation must also be specified."]
        pub location_matching_strategy:
            ::std::option::Option<DeveloperMetadataLookupLocationMatchingStrategyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Limits the selected developer metadata to those entries which are associated with locations of the specified type. For example, when this field is specified as ROW this lookup only considers developer metadata associated on rows. If the field is left unspecified, all location types are considered. This field cannot be specified as SPREADSHEET when the locationMatchingStrategy is specified as INTERSECTING or when the metadataLocation is specified as a non-spreadsheet location: spreadsheet metadata cannot intersect any other developer metadata location. This field also must be left unspecified when the locationMatchingStrategy is specified as EXACT."]
        pub location_type: ::std::option::Option<DeveloperMetadataLookupLocationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadataId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Limits the selected developer metadata to that which has a matching DeveloperMetadata.metadata_id."]
        pub metadata_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadataKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Limits the selected developer metadata to that which has a matching DeveloperMetadata.metadata_key."]
        pub metadata_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadataLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Limits the selected developer metadata to those entries associated with the specified location. This field either matches exact locations or all intersecting locations according the specified locationMatchingStrategy."]
        pub metadata_location: ::std::option::Option<::std::boxed::Box<DeveloperMetadataLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadataValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Limits the selected developer metadata to that which has a matching DeveloperMetadata.metadata_value."]
        pub metadata_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibility")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Limits the selected developer metadata to that which has a matching DeveloperMetadata.visibility. If left unspecified, all developer metadata visibile to the requesting project is considered."]
        pub visibility: ::std::option::Option<DeveloperMetadataLookupVisibilityEnum>,
    }
    impl DeveloperMetadataLookup {
        pub fn builder() -> DeveloperMetadataLookupBuilder {
            DeveloperMetadataLookupBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Determines how this lookup matches the location. If this field is specified as EXACT, only developer metadata associated on the exact location specified is matched. If this field is specified to INTERSECTING, developer metadata associated on intersecting locations is also matched. If left unspecified, this field assumes a default value of INTERSECTING. If this field is specified, a metadataLocation must also be specified."]
    pub enum DeveloperMetadataLookupLocationMatchingStrategyEnum {
        #[serde(rename = "DEVELOPER_METADATA_LOCATION_MATCHING_STRATEGY_UNSPECIFIED")]
        #[doc = "Default value. This value must not be used."]
        DeveloperMetadataLocationMatchingStrategyUnspecified,
        #[serde(rename = "EXACT_LOCATION")]
        #[doc = "Indicates that a specified location should be matched exactly. For example, if row three were specified as a location this matching strategy would only match developer metadata also associated on row three. Metadata associated on other locations would not be considered."]
        ExactLocation,
        #[serde(rename = "INTERSECTING_LOCATION")]
        #[doc = "Indicates that a specified location should match that exact location as well as any intersecting locations. For example, if row three were specified as a location this matching strategy would match developer metadata associated on row three as well as metadata associated on locations that intersect row three. If, for instance, there was developer metadata associated on column B, this matching strategy would also match that location because column B intersects row three."]
        IntersectingLocation,
    }
    impl ::std::default::Default for DeveloperMetadataLookupLocationMatchingStrategyEnum {
        fn default() -> Self {
            Self::DeveloperMetadataLocationMatchingStrategyUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Limits the selected developer metadata to those entries which are associated with locations of the specified type. For example, when this field is specified as ROW this lookup only considers developer metadata associated on rows. If the field is left unspecified, all location types are considered. This field cannot be specified as SPREADSHEET when the locationMatchingStrategy is specified as INTERSECTING or when the metadataLocation is specified as a non-spreadsheet location: spreadsheet metadata cannot intersect any other developer metadata location. This field also must be left unspecified when the locationMatchingStrategy is specified as EXACT."]
    pub enum DeveloperMetadataLookupLocationTypeEnum {
        #[serde(rename = "DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED")]
        #[doc = "Default value."]
        DeveloperMetadataLocationTypeUnspecified,
        #[serde(rename = "ROW")]
        #[doc = "Developer metadata associated on an entire row dimension."]
        Row,
        #[serde(rename = "COLUMN")]
        #[doc = "Developer metadata associated on an entire column dimension."]
        Column,
        #[serde(rename = "SHEET")]
        #[doc = "Developer metadata associated on an entire sheet."]
        Sheet,
        #[serde(rename = "SPREADSHEET")]
        #[doc = "Developer metadata associated on the entire spreadsheet."]
        Spreadsheet,
    }
    impl ::std::default::Default for DeveloperMetadataLookupLocationTypeEnum {
        fn default() -> Self {
            Self::DeveloperMetadataLocationTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Limits the selected developer metadata to that which has a matching DeveloperMetadata.visibility. If left unspecified, all developer metadata visibile to the requesting project is considered."]
    pub enum DeveloperMetadataLookupVisibilityEnum {
        #[serde(rename = "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED")]
        #[doc = "Default value."]
        DeveloperMetadataVisibilityUnspecified,
        #[serde(rename = "DOCUMENT")]
        #[doc = "Document-visible metadata is accessible from any developer project with access to the document."]
        Document,
        #[serde(rename = "PROJECT")]
        #[doc = "Project-visible metadata is only visible to and accessible by the developer project that created the metadata."]
        Project,
    }
    impl ::std::default::Default for DeveloperMetadataLookupVisibilityEnum {
        fn default() -> Self {
            Self::DeveloperMetadataVisibilityUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A group over an interval of rows or columns on a sheet, which can contain or be contained within other groups. A group can be collapsed or expanded as a unit on the sheet."]
    pub struct DimensionGroup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "collapsed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is true if this group is collapsed. A collapsed group remains collapsed if an overlapping group at a shallower depth is expanded. A true value does not imply that all dimensions within the group are hidden, since a dimension's visibility can change independently from this group property. However, when this property is updated, all dimensions within it are set to hidden if this field is true, or set to visible if this field is false."]
        pub collapsed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "depth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The depth of the group, representing how many groups have a range that wholly contains the range of this group."]
        pub depth: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range over which this group exists."]
        pub range: ::std::option::Option<::std::boxed::Box<DimensionRange>>,
    }
    impl DimensionGroup {
        pub fn builder() -> DimensionGroupBuilder {
            DimensionGroupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Properties about a dimension."]
    pub struct DimensionProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceColumnReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If set, this is a column in a data source sheet."]
        pub data_source_column_reference:
            ::std::option::Option<::std::boxed::Box<DataSourceColumnReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "developerMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The developer metadata associated with a single row or column."]
        pub developer_metadata:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeveloperMetadata>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hiddenByFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if this dimension is being filtered. This field is read-only."]
        pub hidden_by_filter: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hiddenByUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if this dimension is explicitly hidden."]
        pub hidden_by_user: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pixelSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height (if a row) or width (if a column) of the dimension in pixels."]
        pub pixel_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl DimensionProperties {
        pub fn builder() -> DimensionPropertiesBuilder {
            DimensionPropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A range along a single dimension on a sheet. All indexes are zero-based. Indexes are half open: the start index is inclusive and the end index is exclusive. Missing indexes indicate the range is unbounded on that side."]
    pub struct DimensionRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimension of the span."]
        pub dimension: ::std::option::Option<DimensionRangeDimensionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end (exclusive) of the span, or not set if unbounded."]
        pub end_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sheet this span is on."]
        pub sheet_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start (inclusive) of the span, or not set if unbounded."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
    }
    impl DimensionRange {
        pub fn builder() -> DimensionRangeBuilder {
            DimensionRangeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The dimension of the span."]
    pub enum DimensionRangeDimensionEnum {
        #[serde(rename = "DIMENSION_UNSPECIFIED")]
        #[doc = "The default value, do not use."]
        DimensionUnspecified,
        #[serde(rename = "ROWS")]
        #[doc = "Operates on the rows of a sheet."]
        Rows,
        #[serde(rename = "COLUMNS")]
        #[doc = "Operates on the columns of a sheet."]
        Columns,
    }
    impl ::std::default::Default for DimensionRangeDimensionEnum {
        fn default() -> Self {
            Self::DimensionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Duplicates a particular filter view."]
    pub struct DuplicateFilterViewRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the filter being duplicated."]
        pub filter_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl DuplicateFilterViewRequest {
        pub fn builder() -> DuplicateFilterViewRequestBuilder {
            DuplicateFilterViewRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of a filter view being duplicated."]
    pub struct DuplicateFilterViewResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The newly created filter."]
        pub filter: ::std::option::Option<::std::boxed::Box<FilterView>>,
    }
    impl DuplicateFilterViewResponse {
        pub fn builder() -> DuplicateFilterViewResponseBuilder {
            DuplicateFilterViewResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Duplicates the contents of a sheet."]
    pub struct DuplicateSheetRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertSheetIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zero-based index where the new sheet should be inserted. The index of all sheets after this are incremented."]
        pub insert_sheet_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newSheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, the ID of the new sheet. If not set, an ID is chosen. If set, the ID must not conflict with any existing sheet ID. If set, it must be non-negative."]
        pub new_sheet_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newSheetName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the new sheet. If empty, a new name is chosen for you."]
        pub new_sheet_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceSheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sheet to duplicate. If the source sheet is of DATA_SOURCE type, its backing DataSource is also duplicated and associated with the new copy of the sheet. No data execution is triggered, the grid data of this sheet is also copied over but only available after the batch request completes."]
        pub source_sheet_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl DuplicateSheetRequest {
        pub fn builder() -> DuplicateSheetRequestBuilder {
            DuplicateSheetRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of duplicating a sheet."]
    pub struct DuplicateSheetResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties of the duplicate sheet."]
        pub properties: ::std::option::Option<::std::boxed::Box<SheetProperties>>,
    }
    impl DuplicateSheetResponse {
        pub fn builder() -> DuplicateSheetResponseBuilder {
            DuplicateSheetResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The editors of a protected range."]
    pub struct Editors {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainUsersCanEdit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if anyone in the document's domain has edit access to the protected range. Domain protection is only supported on documents within a domain."]
        pub domain_users_can_edit: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email addresses of groups with edit access to the protected range."]
        pub groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "users")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email addresses of users with edit access to the protected range."]
        pub users: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Editors {
        pub fn builder() -> EditorsBuilder {
            EditorsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A chart embedded in a sheet."]
    pub struct EmbeddedChart {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "border")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The border of the chart."]
        pub border: ::std::option::Option<::std::boxed::Box<EmbeddedObjectBorder>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chartId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the chart."]
        pub chart_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The position of the chart."]
        pub position: ::std::option::Option<::std::boxed::Box<EmbeddedObjectPosition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The specification of the chart."]
        pub spec: ::std::option::Option<::std::boxed::Box<ChartSpec>>,
    }
    impl EmbeddedChart {
        pub fn builder() -> EmbeddedChartBuilder {
            EmbeddedChartBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A border along an embedded object."]
    pub struct EmbeddedObjectBorder {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the border."]
        pub color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the border. If color is also set, this field takes precedence."]
        pub color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
    }
    impl EmbeddedObjectBorder {
        pub fn builder() -> EmbeddedObjectBorderBuilder {
            EmbeddedObjectBorderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The position of an embedded object such as a chart."]
    pub struct EmbeddedObjectPosition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newSheet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, the embedded object is put on a new sheet whose ID is chosen for you. Used only when writing."]
        pub new_sheet: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "overlayPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The position at which the object is overlaid on top of a grid."]
        pub overlay_position: ::std::option::Option<::std::boxed::Box<OverlayPosition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sheet this is on. Set only if the embedded object is on its own sheet. Must be non-negative."]
        pub sheet_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl EmbeddedObjectPosition {
        pub fn builder() -> EmbeddedObjectPositionBuilder {
            EmbeddedObjectPositionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An error in a cell."]
    pub struct ErrorValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message with more information about the error (in the spreadsheet's locale)."]
        pub message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of error."]
        pub _type: ::std::option::Option<ErrorValueTypeEnum>,
    }
    impl ErrorValue {
        pub fn builder() -> ErrorValueBuilder {
            ErrorValueBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of error."]
    pub enum ErrorValueTypeEnum {
        #[serde(rename = "ERROR_TYPE_UNSPECIFIED")]
        #[doc = "The default error type, do not use this."]
        ErrorTypeUnspecified,
        #[serde(rename = "ERROR")]
        #[doc = "Corresponds to the `#ERROR!` error."]
        Error,
        #[serde(rename = "NULL_VALUE")]
        #[doc = "Corresponds to the `#NULL!` error."]
        NullValue,
        #[serde(rename = "DIVIDE_BY_ZERO")]
        #[doc = "Corresponds to the `#DIV/0` error."]
        DivideByZero,
        #[serde(rename = "VALUE")]
        #[doc = "Corresponds to the `#VALUE!` error."]
        Value,
        #[serde(rename = "REF")]
        #[doc = "Corresponds to the `#REF!` error."]
        Ref,
        #[serde(rename = "NAME")]
        #[doc = "Corresponds to the `#NAME?` error."]
        Name,
        #[serde(rename = "NUM")]
        #[doc = "Corresponds to the `#NUM!` error."]
        Num,
        #[serde(rename = "N_A")]
        #[doc = "Corresponds to the `#N/A` error."]
        NA,
        #[serde(rename = "LOADING")]
        #[doc = "Corresponds to the `Loading...` state."]
        Loading,
    }
    impl ::std::default::Default for ErrorValueTypeEnum {
        fn default() -> Self {
            Self::ErrorTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The kinds of value that a cell in a spreadsheet can have."]
    pub struct ExtendedValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boolValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a boolean value."]
        pub bool_value: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents an error. This field is read-only."]
        pub error_value: ::std::option::Option<::std::boxed::Box<ErrorValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formulaValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a formula."]
        pub formula_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numberValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a double value. Note: Dates, Times and DateTimes are represented as doubles in \"serial number\" format."]
        pub number_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a string value. Leading single quotes are not included. For example, if the user typed `'123` into the UI, this would be represented as a `stringValue` of `\"123\"`."]
        pub string_value: ::std::option::Option<::std::string::String>,
    }
    impl ExtendedValue {
        pub fn builder() -> ExtendedValueBuilder {
            ExtendedValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Criteria for showing/hiding rows in a filter or filter view."]
    pub struct FilterCriteria {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A condition that must be true for values to be shown. (This does not override hidden_values -- if a value is listed there, it will still be hidden.)"]
        pub condition: ::std::option::Option<::std::boxed::Box<BooleanCondition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hiddenValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Values that should be hidden."]
        pub hidden_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibleBackgroundColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background fill color to filter by; only cells with this fill color are shown. Mutually exclusive with visible_foreground_color."]
        pub visible_background_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibleBackgroundColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background fill color to filter by; only cells with this fill color are shown. This field is mutually exclusive with visible_foreground_color, and must be set to an RGB-type color. If visible_background_color is also set, this field takes precedence."]
        pub visible_background_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibleForegroundColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The foreground color to filter by; only cells with this foreground color are shown. Mutually exclusive with visible_background_color."]
        pub visible_foreground_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibleForegroundColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The foreground color to filter by; only cells with this foreground color are shown. This field is mutually exclusive with visible_background_color, and must be set to an RGB-type color. If visible_foreground_color is also set, this field takes precedence."]
        pub visible_foreground_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
    }
    impl FilterCriteria {
        pub fn builder() -> FilterCriteriaBuilder {
            FilterCriteriaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The filter criteria associated with a specific column."]
    pub struct FilterSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The column index."]
        pub column_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceColumnReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reference to a data source column."]
        pub data_source_column_reference:
            ::std::option::Option<::std::boxed::Box<DataSourceColumnReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterCriteria")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The criteria for the column."]
        pub filter_criteria: ::std::option::Option<::std::boxed::Box<FilterCriteria>>,
    }
    impl FilterSpec {
        pub fn builder() -> FilterSpecBuilder {
            FilterSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A filter view."]
    pub struct FilterView {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "criteria")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The criteria for showing/hiding values per column. The map's key is the column index, and the value is the criteria for that column. This field is deprecated in favor of filter_specs."]
        pub criteria: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<FilterCriteria>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterSpecs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filter criteria for showing/hiding values per column. Both criteria and filter_specs are populated in responses. If both fields are specified in an update request, this field takes precedence."]
        pub filter_specs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FilterSpec>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterViewId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the filter view."]
        pub filter_view_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namedRangeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The named range this filter view is backed by, if any. When writing, only one of range or named_range_id may be set."]
        pub named_range_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range this filter view covers. When writing, only one of range or named_range_id may be set."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortSpecs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sort order per column. Later specifications are used when values are equal in the earlier specifications."]
        pub sort_specs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SortSpec>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the filter view."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl FilterView {
        pub fn builder() -> FilterViewBuilder {
            FilterViewBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Finds and replaces data in cells over a range, sheet, or all sheets."]
    pub struct FindReplaceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allSheets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True to find/replace over all sheets."]
        pub all_sheets: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "find")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value to search."]
        pub find: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeFormulas")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the search should include cells with formulas. False to skip cells with formulas."]
        pub include_formulas: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchCase")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the search is case sensitive."]
        pub match_case: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchEntireCell")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the find value should match the entire cell."]
        pub match_entire_cell: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range to find/replace over."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replacement")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value to use as the replacement."]
        pub replacement: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchByRegex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the find value is a regex. The regular expression and replacement should follow Java regex rules at https://docs.oracle.com/javase/8/docs/api/java/util/regex/Pattern.html. The replacement string is allowed to refer to capturing groups. For example, if one cell has the contents `\"Google Sheets\"` and another has `\"Google Docs\"`, then searching for `\"o.* (.*)\"` with a replacement of `\"$1 Rocks\"` would change the contents of the cells to `\"GSheets Rocks\"` and `\"GDocs Rocks\"` respectively."]
        pub search_by_regex: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sheet to find/replace over."]
        pub sheet_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl FindReplaceRequest {
        pub fn builder() -> FindReplaceRequestBuilder {
            FindReplaceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of the find/replace."]
    pub struct FindReplaceResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formulasChanged")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of formula cells changed."]
        pub formulas_changed: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "occurrencesChanged")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of occurrences (possibly multiple within a cell) changed. For example, if replacing `\"e\"` with `\"o\"` in `\"Google Sheets\"`, this would be `\"3\"` because `\"Google Sheets\"` -> `\"Googlo Shoots\"`."]
        pub occurrences_changed: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowsChanged")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of rows changed."]
        pub rows_changed: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetsChanged")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of sheets changed."]
        pub sheets_changed: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valuesChanged")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of non-formula cells changed."]
        pub values_changed: ::std::option::Option<::std::primitive::i64>,
    }
    impl FindReplaceResponse {
        pub fn builder() -> FindReplaceResponseBuilder {
            FindReplaceResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for retrieving a Spreadsheet."]
    pub struct GetSpreadsheetByDataFilterRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataFilters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The DataFilters used to select which ranges to retrieve from the spreadsheet."]
        pub data_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataFilter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeGridData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if grid data should be returned. This parameter is ignored if a field mask was set in the request."]
        pub include_grid_data: ::std::option::Option<::std::primitive::bool>,
    }
    impl GetSpreadsheetByDataFilterRequest {
        pub fn builder() -> GetSpreadsheetByDataFilterRequestBuilder {
            GetSpreadsheetByDataFilterRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A rule that applies a gradient color scale format, based on the interpolation points listed. The format of a cell will vary based on its contents as compared to the values of the interpolation points."]
    pub struct GradientRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxpoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The final interpolation point."]
        pub maxpoint: ::std::option::Option<::std::boxed::Box<InterpolationPoint>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "midpoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional midway interpolation point."]
        pub midpoint: ::std::option::Option<::std::boxed::Box<InterpolationPoint>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minpoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting interpolation point."]
        pub minpoint: ::std::option::Option<::std::boxed::Box<InterpolationPoint>>,
    }
    impl GradientRule {
        pub fn builder() -> GradientRuleBuilder {
            GradientRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A coordinate in a sheet. All indexes are zero-based."]
    pub struct GridCoordinate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The column index of the coordinate."]
        pub column_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The row index of the coordinate."]
        pub row_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sheet this coordinate is on."]
        pub sheet_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl GridCoordinate {
        pub fn builder() -> GridCoordinateBuilder {
            GridCoordinateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data in the grid, as well as metadata about the dimensions."]
    pub struct GridData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata about the requested columns in the grid, starting with the column in start_column."]
        pub column_metadata:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionProperties>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data in the grid, one entry per row, starting with the row in startRow. The values in RowData will correspond to columns starting at start_column."]
        pub row_data: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RowData>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata about the requested rows in the grid, starting with the row in start_row."]
        pub row_metadata:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionProperties>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startColumn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The first column this GridData refers to, zero-based."]
        pub start_column: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startRow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The first row this GridData refers to, zero-based."]
        pub start_row: ::std::option::Option<::std::primitive::i64>,
    }
    impl GridData {
        pub fn builder() -> GridDataBuilder {
            GridDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Properties of a grid."]
    pub struct GridProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of columns in the grid."]
        pub column_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnGroupControlAfter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the column grouping control toggle is shown after the group."]
        pub column_group_control_after: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frozenColumnCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of columns that are frozen in the grid."]
        pub frozen_column_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frozenRowCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of rows that are frozen in the grid."]
        pub frozen_row_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hideGridlines")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the grid isn't showing gridlines in the UI."]
        pub hide_gridlines: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of rows in the grid."]
        pub row_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowGroupControlAfter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the row grouping control toggle is shown after the group."]
        pub row_group_control_after: ::std::option::Option<::std::primitive::bool>,
    }
    impl GridProperties {
        pub fn builder() -> GridPropertiesBuilder {
            GridPropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A range on a sheet. All indexes are zero-based. Indexes are half open, i.e. the start index is inclusive and the end index is exclusive -- [start_index, end_index). Missing indexes indicate the range is unbounded on that side. For example, if `\"Sheet1\"` is sheet ID 0, then: `Sheet1!A1:A1 == sheet_id: 0, start_row_index: 0, end_row_index: 1, start_column_index: 0, end_column_index: 1` `Sheet1!A3:B4 == sheet_id: 0, start_row_index: 2, end_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1!A:B == sheet_id: 0, start_column_index: 0, end_column_index: 2` `Sheet1!A5:B == sheet_id: 0, start_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1 == sheet_id:0` The start index must always be less than or equal to the end index. If the start index equals the end index, then the range is empty. Empty ranges are typically not meaningful and are usually rendered in the UI as `#REF!`."]
    pub struct GridRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endColumnIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end column (exclusive) of the range, or not set if unbounded."]
        pub end_column_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endRowIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end row (exclusive) of the range, or not set if unbounded."]
        pub end_row_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sheet this range is on."]
        pub sheet_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startColumnIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start column (inclusive) of the range, or not set if unbounded."]
        pub start_column_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startRowIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start row (inclusive) of the range, or not set if unbounded."]
        pub start_row_index: ::std::option::Option<::std::primitive::i64>,
    }
    impl GridRange {
        pub fn builder() -> GridRangeBuilder {
            GridRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A histogram chart. A histogram chart groups data items into bins, displaying each bin as a column of stacked items. Histograms are used to display the distribution of a dataset. Each column of items represents a range into which those items fall. The number of bins can be chosen automatically or specified explicitly."]
    pub struct HistogramChartSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "By default the bucket size (the range of values stacked in a single column) is chosen automatically, but it may be overridden here. E.g., A bucket size of 1.5 results in buckets from 0 - 1.5, 1.5 - 3.0, etc. Cannot be negative. This field is optional."]
        pub bucket_size: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "legendPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The position of the chart legend."]
        pub legend_position: ::std::option::Option<HistogramChartSpecLegendPositionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outlierPercentile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The outlier percentile is used to ensure that outliers do not adversely affect the calculation of bucket sizes. For example, setting an outlier percentile of 0.05 indicates that the top and bottom 5% of values when calculating buckets. The values are still included in the chart, they will be added to the first or last buckets instead of their own buckets. Must be between 0.0 and 0.5."]
        pub outlier_percentile: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "series")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The series for a histogram may be either a single series of values to be bucketed or multiple series, each of the same length, containing the name of the series followed by the values to be bucketed for that series."]
        pub series: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HistogramSeries>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "showItemDividers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether horizontal divider lines should be displayed between items in each column."]
        pub show_item_dividers: ::std::option::Option<::std::primitive::bool>,
    }
    impl HistogramChartSpec {
        pub fn builder() -> HistogramChartSpecBuilder {
            HistogramChartSpecBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The position of the chart legend."]
    pub enum HistogramChartSpecLegendPositionEnum {
        #[serde(rename = "HISTOGRAM_CHART_LEGEND_POSITION_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        HistogramChartLegendPositionUnspecified,
        #[serde(rename = "BOTTOM_LEGEND")]
        #[doc = "The legend is rendered on the bottom of the chart."]
        BottomLegend,
        #[serde(rename = "LEFT_LEGEND")]
        #[doc = "The legend is rendered on the left of the chart."]
        LeftLegend,
        #[serde(rename = "RIGHT_LEGEND")]
        #[doc = "The legend is rendered on the right of the chart."]
        RightLegend,
        #[serde(rename = "TOP_LEGEND")]
        #[doc = "The legend is rendered on the top of the chart."]
        TopLegend,
        #[serde(rename = "NO_LEGEND")]
        #[doc = "No legend is rendered."]
        NoLegend,
        #[serde(rename = "INSIDE_LEGEND")]
        #[doc = "The legend is rendered inside the chart area."]
        InsideLegend,
    }
    impl ::std::default::Default for HistogramChartSpecLegendPositionEnum {
        fn default() -> Self {
            Self::HistogramChartLegendPositionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Allows you to organize the numeric values in a source data column into buckets of a constant size. All values from HistogramRule.start to HistogramRule.end are placed into groups of size HistogramRule.interval. In addition, all values below HistogramRule.start are placed in one group, and all values above HistogramRule.end are placed in another. Only HistogramRule.interval is required, though if HistogramRule.start and HistogramRule.end are both provided, HistogramRule.start must be less than HistogramRule.end. For example, a pivot table showing average purchase amount by age that has 50+ rows: +-----+-------------------+ | Age | AVERAGE of Amount | +-----+-------------------+ | 16 | $27.13 | | 17 | $5.24 | | 18 | $20.15 | ... +-----+-------------------+ could be turned into a pivot table that looks like the one below by applying a histogram group rule with a HistogramRule.start of 25, an HistogramRule.interval of 20, and an HistogramRule.end of 65. +-------------+-------------------+ | Grouped Age | AVERAGE of Amount | +-------------+-------------------+ | < 25 | $19.34 | | 25-45 | $31.43 | | 45-65 | $35.87 | | > 65 | $27.55 | +-------------+-------------------+ | Grand Total | $29.12 | +-------------+-------------------+"]
    pub struct HistogramRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "end")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum value at which items are placed into buckets of constant size. Values above end are lumped into a single bucket. This field is optional."]
        pub end: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "interval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the buckets that are created. Must be positive."]
        pub interval: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum value at which items are placed into buckets of constant size. Values below start are lumped into a single bucket. This field is optional."]
        pub start: ::std::option::Option<::std::primitive::f64>,
    }
    impl HistogramRule {
        pub fn builder() -> HistogramRuleBuilder {
            HistogramRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A histogram series containing the series color and data."]
    pub struct HistogramSeries {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "barColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the column representing this series in each bucket. This field is optional."]
        pub bar_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "barColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the column representing this series in each bucket. This field is optional. If bar_color is also set, this field takes precedence."]
        pub bar_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data for this histogram series."]
        pub data: ::std::option::Option<::std::boxed::Box<ChartData>>,
    }
    impl HistogramSeries {
        pub fn builder() -> HistogramSeriesBuilder {
            HistogramSeriesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Inserts rows or columns in a sheet at a particular index."]
    pub struct InsertDimensionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inheritFromBefore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether dimension properties should be extended from the dimensions before or after the newly inserted dimensions. True to inherit from the dimensions before (in which case the start index must be greater than 0), and false to inherit from the dimensions after. For example, if row index 0 has red background and row index 1 has a green background, then inserting 2 rows at index 1 can inherit either the green or red background. If `inheritFromBefore` is true, the two new rows will be red (because the row before the insertion point was red), whereas if `inheritFromBefore` is false, the two new rows will be green (because the row after the insertion point was green)."]
        pub inherit_from_before: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimensions to insert. Both the start and end indexes must be bounded."]
        pub range: ::std::option::Option<::std::boxed::Box<DimensionRange>>,
    }
    impl InsertDimensionRequest {
        pub fn builder() -> InsertDimensionRequestBuilder {
            InsertDimensionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Inserts cells into a range, shifting the existing cells over or down."]
    pub struct InsertRangeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range to insert new cells into."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shiftDimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimension which will be shifted when inserting cells. If ROWS, existing cells will be shifted down. If COLUMNS, existing cells will be shifted right."]
        pub shift_dimension: ::std::option::Option<InsertRangeRequestShiftDimensionEnum>,
    }
    impl InsertRangeRequest {
        pub fn builder() -> InsertRangeRequestBuilder {
            InsertRangeRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The dimension which will be shifted when inserting cells. If ROWS, existing cells will be shifted down. If COLUMNS, existing cells will be shifted right."]
    pub enum InsertRangeRequestShiftDimensionEnum {
        #[serde(rename = "DIMENSION_UNSPECIFIED")]
        #[doc = "The default value, do not use."]
        DimensionUnspecified,
        #[serde(rename = "ROWS")]
        #[doc = "Operates on the rows of a sheet."]
        Rows,
        #[serde(rename = "COLUMNS")]
        #[doc = "Operates on the columns of a sheet."]
        Columns,
    }
    impl ::std::default::Default for InsertRangeRequestShiftDimensionEnum {
        fn default() -> Self {
            Self::DimensionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single interpolation point on a gradient conditional format. These pin the gradient color scale according to the color, type and value chosen."]
    pub struct InterpolationPoint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color this interpolation point should use."]
        pub color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color this interpolation point should use. If color is also set, this field takes precedence."]
        pub color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How the value should be interpreted."]
        pub _type: ::std::option::Option<InterpolationPointTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value this interpolation point uses. May be a formula. Unused if type is MIN or MAX."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl InterpolationPoint {
        pub fn builder() -> InterpolationPointBuilder {
            InterpolationPointBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How the value should be interpreted."]
    pub enum InterpolationPointTypeEnum {
        #[serde(rename = "INTERPOLATION_POINT_TYPE_UNSPECIFIED")]
        #[doc = "The default value, do not use."]
        InterpolationPointTypeUnspecified,
        #[serde(rename = "MIN")]
        #[doc = "The interpolation point uses the minimum value in the cells over the range of the conditional format."]
        Min,
        #[serde(rename = "MAX")]
        #[doc = "The interpolation point uses the maximum value in the cells over the range of the conditional format."]
        Max,
        #[serde(rename = "NUMBER")]
        #[doc = "The interpolation point uses exactly the value in InterpolationPoint.value."]
        Number,
        #[serde(rename = "PERCENT")]
        #[doc = "The interpolation point is the given percentage over all the cells in the range of the conditional format. This is equivalent to NUMBER if the value was: `=(MAX(FLATTEN(range)) * (value / 100)) + (MIN(FLATTEN(range)) * (1 - (value / 100)))` (where errors in the range are ignored when flattening)."]
        Percent,
        #[serde(rename = "PERCENTILE")]
        #[doc = "The interpolation point is the given percentile over all the cells in the range of the conditional format. This is equivalent to NUMBER if the value was: `=PERCENTILE(FLATTEN(range), value / 100)` (where errors in the range are ignored when flattening)."]
        Percentile,
    }
    impl ::std::default::Default for InterpolationPointTypeEnum {
        fn default() -> Self {
            Self::InterpolationPointTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a time interval, encoded as a Timestamp start (inclusive) and a Timestamp end (exclusive). The start must be less than or equal to the end. When the start equals the end, the interval is empty (matches no time). When both start and end are unspecified, the interval matches any time."]
    pub struct Interval {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Exclusive end of the interval. If specified, a Timestamp matching this interval will have to be before the end."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Inclusive start of the interval. If specified, a Timestamp matching this interval will have to be the same or after the start."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl Interval {
        pub fn builder() -> IntervalBuilder {
            IntervalBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings to control how circular dependencies are resolved with iterative calculation."]
    pub struct IterativeCalculationSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "convergenceThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When iterative calculation is enabled and successive results differ by less than this threshold value, the calculation rounds stop."]
        pub convergence_threshold: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxIterations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When iterative calculation is enabled, the maximum number of calculation rounds to perform."]
        pub max_iterations: ::std::option::Option<::std::primitive::i64>,
    }
    impl IterativeCalculationSettings {
        pub fn builder() -> IterativeCalculationSettingsBuilder {
            IterativeCalculationSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Formatting options for key value."]
    pub struct KeyValueFormat {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the horizontal text positioning of key value. This field is optional. If not specified, default positioning is used."]
        pub position: ::std::option::Option<::std::boxed::Box<TextPosition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text formatting options for key value."]
        pub text_format: ::std::option::Option<::std::boxed::Box<TextFormat>>,
    }
    impl KeyValueFormat {
        pub fn builder() -> KeyValueFormatBuilder {
            KeyValueFormatBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Properties that describe the style of a line."]
    pub struct LineStyle {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dash type of the line."]
        pub _type: ::std::option::Option<LineStyleTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thickness of the line, in px."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl LineStyle {
        pub fn builder() -> LineStyleBuilder {
            LineStyleBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The dash type of the line."]
    pub enum LineStyleTypeEnum {
        #[serde(rename = "LINE_DASH_TYPE_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        LineDashTypeUnspecified,
        #[serde(rename = "INVISIBLE")]
        #[doc = "No dash type, which is equivalent to a non-visible line."]
        Invisible,
        #[serde(rename = "CUSTOM")]
        #[doc = "A custom dash for a line. Modifying the exact custom dash style is currently unsupported."]
        Custom,
        #[serde(rename = "SOLID")]
        #[doc = "A solid line."]
        Solid,
        #[serde(rename = "DOTTED")]
        #[doc = "A dotted line."]
        Dotted,
        #[serde(rename = "MEDIUM_DASHED")]
        #[doc = "A dashed line where the dashes have \"medium\" length."]
        MediumDashed,
        #[serde(rename = "MEDIUM_DASHED_DOTTED")]
        #[doc = "A line that alternates between a \"medium\" dash and a dot."]
        MediumDashedDotted,
        #[serde(rename = "LONG_DASHED")]
        #[doc = "A dashed line where the dashes have \"long\" length."]
        LongDashed,
        #[serde(rename = "LONG_DASHED_DOTTED")]
        #[doc = "A line that alternates between a \"long\" dash and a dot."]
        LongDashedDotted,
    }
    impl ::std::default::Default for LineStyleTypeEnum {
        fn default() -> Self {
            Self::LineDashTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Allows you to manually organize the values in a source data column into buckets with names of your choosing. For example, a pivot table that aggregates population by state: +-------+-------------------+ | State | SUM of Population | +-------+-------------------+ | AK | 0.7 | | AL | 4.8 | | AR | 2.9 | ... +-------+-------------------+ could be turned into a pivot table that aggregates population by time zone by providing a list of groups (for example, groupName = 'Central', items = ['AL', 'AR', 'IA', ...]) to a manual group rule. Note that a similar effect could be achieved by adding a time zone column to the source data and adjusting the pivot table. +-----------+-------------------+ | Time Zone | SUM of Population | +-----------+-------------------+ | Central | 106.3 | | Eastern | 151.9 | | Mountain | 17.4 | ... +-----------+-------------------+"]
    pub struct ManualRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of group names and the corresponding items from the source data that map to each group name."]
        pub groups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ManualRuleGroup>>>,
    }
    impl ManualRule {
        pub fn builder() -> ManualRuleBuilder {
            ManualRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A group name and a list of items from the source data that should be placed in the group with this name."]
    pub struct ManualRuleGroup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The group name, which must be a string. Each group in a given ManualRule must have a unique group name."]
        pub group_name: ::std::option::Option<::std::boxed::Box<ExtendedValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The items in the source data that should be placed into this group. Each item may be a string, number, or boolean. Items may appear in at most one group within a given ManualRule. Items that do not appear in any group will appear on their own."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ExtendedValue>>>,
    }
    impl ManualRuleGroup {
        pub fn builder() -> ManualRuleGroupBuilder {
            ManualRuleGroupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A developer metadata entry and the data filters specified in the original request that matched it."]
    pub struct MatchedDeveloperMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataFilters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All filters matching the returned developer metadata."]
        pub data_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataFilter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "developerMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The developer metadata matching the specified filters."]
        pub developer_metadata: ::std::option::Option<::std::boxed::Box<DeveloperMetadata>>,
    }
    impl MatchedDeveloperMetadata {
        pub fn builder() -> MatchedDeveloperMetadataBuilder {
            MatchedDeveloperMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A value range that was matched by one or more data filers."]
    pub struct MatchedValueRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataFilters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The DataFilters from the request that matched the range of values."]
        pub data_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataFilter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values matched by the DataFilter."]
        pub value_range: ::std::option::Option<::std::boxed::Box<ValueRange>>,
    }
    impl MatchedValueRange {
        pub fn builder() -> MatchedValueRangeBuilder {
            MatchedValueRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Merges all cells in the range."]
    pub struct MergeCellsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mergeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How the cells should be merged."]
        pub merge_type: ::std::option::Option<MergeCellsRequestMergeTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range of cells to merge."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
    }
    impl MergeCellsRequest {
        pub fn builder() -> MergeCellsRequestBuilder {
            MergeCellsRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How the cells should be merged."]
    pub enum MergeCellsRequestMergeTypeEnum {
        #[serde(rename = "MERGE_ALL")]
        #[doc = "Create a single merge from the range"]
        MergeAll,
        #[serde(rename = "MERGE_COLUMNS")]
        #[doc = "Create a merge for each column in the range"]
        MergeColumns,
        #[serde(rename = "MERGE_ROWS")]
        #[doc = "Create a merge for each row in the range"]
        MergeRows,
    }
    impl ::std::default::Default for MergeCellsRequestMergeTypeEnum {
        fn default() -> Self {
            Self::MergeAll
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Moves one or more rows or columns."]
    pub struct MoveDimensionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zero-based start index of where to move the source data to, based on the coordinates *before* the source data is removed from the grid. Existing data will be shifted down or right (depending on the dimension) to make room for the moved dimensions. The source dimensions are removed from the grid, so the the data may end up in a different index than specified. For example, given `A1..A5` of `0, 1, 2, 3, 4` and wanting to move `\"1\"` and `\"2\"` to between `\"3\"` and `\"4\"`, the source would be `ROWS [1..3)`,and the destination index would be `\"4\"` (the zero-based index of row 5). The end result would be `A1..A5` of `0, 3, 1, 2, 4`."]
        pub destination_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source dimensions to move."]
        pub source: ::std::option::Option<::std::boxed::Box<DimensionRange>>,
    }
    impl MoveDimensionRequest {
        pub fn builder() -> MoveDimensionRequestBuilder {
            MoveDimensionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A named range."]
    pub struct NamedRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the named range."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namedRangeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the named range."]
        pub named_range_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range this represents."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
    }
    impl NamedRange {
        pub fn builder() -> NamedRangeBuilder {
            NamedRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The number format of a cell."]
    pub struct NumberFormat {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pattern")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pattern string used for formatting. If not set, a default pattern based on the user's locale will be used if necessary for the given type. See the [Date and Number Formats guide](/sheets/api/guides/formats) for more information about the supported patterns."]
        pub pattern: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the number format. When writing, this field must be set."]
        pub _type: ::std::option::Option<NumberFormatTypeEnum>,
    }
    impl NumberFormat {
        pub fn builder() -> NumberFormatBuilder {
            NumberFormatBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the number format. When writing, this field must be set."]
    pub enum NumberFormatTypeEnum {
        #[serde(rename = "NUMBER_FORMAT_TYPE_UNSPECIFIED")]
        #[doc = "The number format is not specified and is based on the contents of the cell. Do not explicitly use this."]
        NumberFormatTypeUnspecified,
        #[serde(rename = "TEXT")]
        #[doc = "Text formatting, e.g `1000.12`"]
        Text,
        #[serde(rename = "NUMBER")]
        #[doc = "Number formatting, e.g, `1,000.12`"]
        Number,
        #[serde(rename = "PERCENT")]
        #[doc = "Percent formatting, e.g `10.12%`"]
        Percent,
        #[serde(rename = "CURRENCY")]
        #[doc = "Currency formatting, e.g `$1,000.12`"]
        Currency,
        #[serde(rename = "DATE")]
        #[doc = "Date formatting, e.g `9/26/2008`"]
        Date,
        #[serde(rename = "TIME")]
        #[doc = "Time formatting, e.g `3:59:00 PM`"]
        Time,
        #[serde(rename = "DATE_TIME")]
        #[doc = "Date+Time formatting, e.g `9/26/08 15:59:00`"]
        DateTime,
        #[serde(rename = "SCIENTIFIC")]
        #[doc = "Scientific number formatting, e.g `1.01E+03`"]
        Scientific,
    }
    impl ::std::default::Default for NumberFormatTypeEnum {
        fn default() -> Self {
            Self::NumberFormatTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An org chart. Org charts require a unique set of labels in labels and may optionally include parent_labels and tooltips. parent_labels contain, for each node, the label identifying the parent node. tooltips contain, for each node, an optional tooltip. For example, to describe an OrgChart with Alice as the CEO, Bob as the President (reporting to Alice) and Cathy as VP of Sales (also reporting to Alice), have labels contain \"Alice\", \"Bob\", \"Cathy\", parent_labels contain \"\", \"Alice\", \"Alice\" and tooltips contain \"CEO\", \"President\", \"VP Sales\"."]
    pub struct OrgChartSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data containing the labels for all the nodes in the chart. Labels must be unique."]
        pub labels: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the org chart nodes."]
        pub node_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the org chart nodes. If node_color is also set, this field takes precedence."]
        pub node_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the org chart nodes."]
        pub node_size: ::std::option::Option<OrgChartSpecNodeSizeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data containing the label of the parent for the corresponding node. A blank value indicates that the node has no parent and is a top-level node. This field is optional."]
        pub parent_labels: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selectedNodeColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the selected org chart nodes."]
        pub selected_node_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selectedNodeColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the selected org chart nodes. If selected_node_color is also set, this field takes precedence."]
        pub selected_node_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tooltips")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data containing the tooltip for the corresponding node. A blank value results in no tooltip being displayed for the node. This field is optional."]
        pub tooltips: ::std::option::Option<::std::boxed::Box<ChartData>>,
    }
    impl OrgChartSpec {
        pub fn builder() -> OrgChartSpecBuilder {
            OrgChartSpecBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The size of the org chart nodes."]
    pub enum OrgChartSpecNodeSizeEnum {
        #[serde(rename = "ORG_CHART_LABEL_SIZE_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        OrgChartLabelSizeUnspecified,
        #[serde(rename = "SMALL")]
        #[doc = "The small org chart node size."]
        Small,
        #[serde(rename = "MEDIUM")]
        #[doc = "The medium org chart node size."]
        Medium,
        #[serde(rename = "LARGE")]
        #[doc = "The large org chart node size."]
        Large,
    }
    impl ::std::default::Default for OrgChartSpecNodeSizeEnum {
        fn default() -> Self {
            Self::OrgChartLabelSizeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The location an object is overlaid on top of a grid."]
    pub struct OverlayPosition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "anchorCell")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cell the object is anchored to."]
        pub anchor_cell: ::std::option::Option<::std::boxed::Box<GridCoordinate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "heightPixels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height of the object, in pixels. Defaults to 371."]
        pub height_pixels: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offsetXPixels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The horizontal offset, in pixels, that the object is offset from the anchor cell."]
        pub offset_x_pixels: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offsetYPixels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The vertical offset, in pixels, that the object is offset from the anchor cell."]
        pub offset_y_pixels: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "widthPixels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the object, in pixels. Defaults to 600."]
        pub width_pixels: ::std::option::Option<::std::primitive::i64>,
    }
    impl OverlayPosition {
        pub fn builder() -> OverlayPositionBuilder {
            OverlayPositionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The amount of padding around the cell, in pixels. When updating padding, every field must be specified."]
    pub struct Padding {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bottom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bottom padding of the cell."]
        pub bottom: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "left")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The left padding of the cell."]
        pub left: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "right")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The right padding of the cell."]
        pub right: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "top")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The top padding of the cell."]
        pub top: ::std::option::Option<::std::primitive::i64>,
    }
    impl Padding {
        pub fn builder() -> PaddingBuilder {
            PaddingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Inserts data into the spreadsheet starting at the specified coordinate."]
    pub struct PasteDataRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "coordinate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The coordinate at which the data should start being inserted."]
        pub coordinate: ::std::option::Option<::std::boxed::Box<GridCoordinate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data to insert."]
        pub data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delimiter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The delimiter in the data."]
        pub delimiter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "html")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the data is HTML."]
        pub html: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How the data should be pasted."]
        pub _type: ::std::option::Option<PasteDataRequestTypeEnum>,
    }
    impl PasteDataRequest {
        pub fn builder() -> PasteDataRequestBuilder {
            PasteDataRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How the data should be pasted."]
    pub enum PasteDataRequestTypeEnum {
        #[serde(rename = "PASTE_NORMAL")]
        #[doc = "Paste values, formulas, formats, and merges."]
        PasteNormal,
        #[serde(rename = "PASTE_VALUES")]
        #[doc = "Paste the values ONLY without formats, formulas, or merges."]
        PasteValues,
        #[serde(rename = "PASTE_FORMAT")]
        #[doc = "Paste the format and data validation only."]
        PasteFormat,
        #[serde(rename = "PASTE_NO_BORDERS")]
        #[doc = "Like PASTE_NORMAL but without borders."]
        PasteNoBorders,
        #[serde(rename = "PASTE_FORMULA")]
        #[doc = "Paste the formulas only."]
        PasteFormula,
        #[serde(rename = "PASTE_DATA_VALIDATION")]
        #[doc = "Paste the data validation only."]
        PasteDataValidation,
        #[serde(rename = "PASTE_CONDITIONAL_FORMATTING")]
        #[doc = "Paste the conditional formatting rules only."]
        PasteConditionalFormatting,
    }
    impl ::std::default::Default for PasteDataRequestTypeEnum {
        fn default() -> Self {
            Self::PasteNormal
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A pie chart."]
    pub struct PieChartSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data that covers the domain of the pie chart."]
        pub domain: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "legendPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Where the legend of the pie chart should be drawn."]
        pub legend_position: ::std::option::Option<PieChartSpecLegendPositionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pieHole")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the hole in the pie chart."]
        pub pie_hole: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "series")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data that covers the one and only series of the pie chart."]
        pub series: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threeDimensional")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the pie is three dimensional."]
        pub three_dimensional: ::std::option::Option<::std::primitive::bool>,
    }
    impl PieChartSpec {
        pub fn builder() -> PieChartSpecBuilder {
            PieChartSpecBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Where the legend of the pie chart should be drawn."]
    pub enum PieChartSpecLegendPositionEnum {
        #[serde(rename = "PIE_CHART_LEGEND_POSITION_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        PieChartLegendPositionUnspecified,
        #[serde(rename = "BOTTOM_LEGEND")]
        #[doc = "The legend is rendered on the bottom of the chart."]
        BottomLegend,
        #[serde(rename = "LEFT_LEGEND")]
        #[doc = "The legend is rendered on the left of the chart."]
        LeftLegend,
        #[serde(rename = "RIGHT_LEGEND")]
        #[doc = "The legend is rendered on the right of the chart."]
        RightLegend,
        #[serde(rename = "TOP_LEGEND")]
        #[doc = "The legend is rendered on the top of the chart."]
        TopLegend,
        #[serde(rename = "NO_LEGEND")]
        #[doc = "No legend is rendered."]
        NoLegend,
        #[serde(rename = "LABELED_LEGEND")]
        #[doc = "Each pie slice has a label attached to it."]
        LabeledLegend,
    }
    impl ::std::default::Default for PieChartSpecLegendPositionEnum {
        fn default() -> Self {
            Self::PieChartLegendPositionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Criteria for showing/hiding rows in a pivot table."]
    pub struct PivotFilterCriteria {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A condition that must be true for values to be shown. (`visibleValues` does not override this -- even if a value is listed there, it is still hidden if it does not meet the condition.) Condition values that refer to ranges in A1-notation are evaluated relative to the pivot table sheet. References are treated absolutely, so are not filled down the pivot table. For example, a condition value of `=A1` on \"Pivot Table 1\" is treated as `'Pivot Table 1'!$A$1`. The source data of the pivot table can be referenced by column header name. For example, if the source data has columns named \"Revenue\" and \"Cost\" and a condition is applied to the \"Revenue\" column with type `NUMBER_GREATER` and value `=Cost`, then only columns where \"Revenue\" > \"Cost\" are included."]
        pub condition: ::std::option::Option<::std::boxed::Box<BooleanCondition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibleByDefault")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether values are visible by default. If true, the visible_values are ignored, all values that meet condition (if specified) are shown. If false, values that are both in visible_values and meet condition are shown."]
        pub visible_by_default: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibleValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Values that should be included. Values not listed here are excluded."]
        pub visible_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl PivotFilterCriteria {
        pub fn builder() -> PivotFilterCriteriaBuilder {
            PivotFilterCriteriaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The pivot table filter criteria associated with a specific source column offset."]
    pub struct PivotFilterSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnOffsetIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The column offset of the source range."]
        pub column_offset_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceColumnReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reference to the data source column."]
        pub data_source_column_reference:
            ::std::option::Option<::std::boxed::Box<DataSourceColumnReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterCriteria")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The criteria for the column."]
        pub filter_criteria: ::std::option::Option<::std::boxed::Box<PivotFilterCriteria>>,
    }
    impl PivotFilterSpec {
        pub fn builder() -> PivotFilterSpecBuilder {
            PivotFilterSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single grouping (either row or column) in a pivot table."]
    pub struct PivotGroup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceColumnReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reference to the data source column this grouping is based on."]
        pub data_source_column_reference:
            ::std::option::Option<::std::boxed::Box<DataSourceColumnReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupLimit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The count limit on rows or columns to apply to this pivot group."]
        pub group_limit: ::std::option::Option<::std::boxed::Box<PivotGroupLimit>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The group rule to apply to this row/column group."]
        pub group_rule: ::std::option::Option<::std::boxed::Box<PivotGroupRule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The labels to use for the row/column groups which can be customized. For example, in the following pivot table, the row label is `Region` (which could be renamed to `State`) and the column label is `Product` (which could be renamed `Item`). Pivot tables created before December 2017 do not have header labels. If you'd like to add header labels to an existing pivot table, please delete the existing pivot table and then create a new pivot table with same parameters. +--------------+---------+-------+ | SUM of Units | Product | | | Region | Pen | Paper | +--------------+---------+-------+ | New York | 345 | 98 | | Oregon | 234 | 123 | | Tennessee | 531 | 415 | +--------------+---------+-------+ | Grand Total | 1110 | 636 | +--------------+---------+-------+"]
        pub label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repeatHeadings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the headings in this pivot group should be repeated. This is only valid for row groupings and is ignored by columns. By default, we minimize repitition of headings by not showing higher level headings where they are the same. For example, even though the third row below corresponds to \"Q1 Mar\", \"Q1\" is not shown because it is redundant with previous rows. Setting repeat_headings to true would cause \"Q1\" to be repeated for \"Feb\" and \"Mar\". +--------------+ | Q1 | Jan | | | Feb | | | Mar | +--------+-----+ | Q1 Total | +--------------+"]
        pub repeat_headings: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "showTotals")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the pivot table should include the totals for this grouping."]
        pub show_totals: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The order the values in this group should be sorted."]
        pub sort_order: ::std::option::Option<PivotGroupSortOrderEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceColumnOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The column offset of the source range that this grouping is based on. For example, if the source was `C10:E15`, a `sourceColumnOffset` of `0` means this group refers to column `C`, whereas the offset `1` would refer to column `D`."]
        pub source_column_offset: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueBucket")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bucket of the opposite pivot group to sort by. If not specified, sorting is alphabetical by this group's values."]
        pub value_bucket: ::std::option::Option<::std::boxed::Box<PivotGroupSortValueBucket>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata about values in the grouping."]
        pub value_metadata:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PivotGroupValueMetadata>>>,
    }
    impl PivotGroup {
        pub fn builder() -> PivotGroupBuilder {
            PivotGroupBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The order the values in this group should be sorted."]
    pub enum PivotGroupSortOrderEnum {
        #[serde(rename = "SORT_ORDER_UNSPECIFIED")]
        #[doc = "Default value, do not use this."]
        SortOrderUnspecified,
        #[serde(rename = "ASCENDING")]
        #[doc = "Sort ascending."]
        Ascending,
        #[serde(rename = "DESCENDING")]
        #[doc = "Sort descending."]
        Descending,
    }
    impl ::std::default::Default for PivotGroupSortOrderEnum {
        fn default() -> Self {
            Self::SortOrderUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The count limit on rows or columns in the pivot group."]
    pub struct PivotGroupLimit {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applyOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The order in which the group limit is applied to the pivot table. Pivot group limits are applied from lower to higher order number. Order numbers are normalized to consecutive integers from 0. For write request, to fully customize the applying orders, all pivot group limits should have this field set with an unique number. Otherwise, the order is determined by the index in the PivotTable.rows list and then the PivotTable.columns list."]
        pub apply_order: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "countLimit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The count limit."]
        pub count_limit: ::std::option::Option<::std::primitive::i64>,
    }
    impl PivotGroupLimit {
        pub fn builder() -> PivotGroupLimitBuilder {
            PivotGroupLimitBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An optional setting on a PivotGroup that defines buckets for the values in the source data column rather than breaking out each individual value. Only one PivotGroup with a group rule may be added for each column in the source data, though on any given column you may add both a PivotGroup that has a rule and a PivotGroup that does not."]
    pub struct PivotGroupRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateTimeRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A DateTimeRule."]
        pub date_time_rule: ::std::option::Option<::std::boxed::Box<DateTimeRule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "histogramRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A HistogramRule."]
        pub histogram_rule: ::std::option::Option<::std::boxed::Box<HistogramRule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manualRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A ManualRule."]
        pub manual_rule: ::std::option::Option<::std::boxed::Box<ManualRule>>,
    }
    impl PivotGroupRule {
        pub fn builder() -> PivotGroupRuleBuilder {
            PivotGroupRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about which values in a pivot group should be used for sorting."]
    pub struct PivotGroupSortValueBucket {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buckets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines the bucket from which values are chosen to sort. For example, in a pivot table with one row group & two column groups, the row group can list up to two values. The first value corresponds to a value within the first column group, and the second value corresponds to a value in the second column group. If no values are listed, this would indicate that the row should be sorted according to the \"Grand Total\" over the column groups. If a single value is listed, this would correspond to using the \"Total\" of that bucket."]
        pub buckets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ExtendedValue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valuesIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The offset in the PivotTable.values list which the values in this grouping should be sorted by."]
        pub values_index: ::std::option::Option<::std::primitive::i64>,
    }
    impl PivotGroupSortValueBucket {
        pub fn builder() -> PivotGroupSortValueBucketBuilder {
            PivotGroupSortValueBucketBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata about a value in a pivot grouping."]
    pub struct PivotGroupValueMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "collapsed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the data corresponding to the value is collapsed."]
        pub collapsed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The calculated value the metadata corresponds to. (Note that formulaValue is not valid, because the values will be calculated.)"]
        pub value: ::std::option::Option<::std::boxed::Box<ExtendedValue>>,
    }
    impl PivotGroupValueMetadata {
        pub fn builder() -> PivotGroupValueMetadataBuilder {
            PivotGroupValueMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A pivot table."]
    pub struct PivotTable {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Each column grouping in the pivot table."]
        pub columns: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PivotGroup>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "criteria")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional mapping of filters per source column offset. The filters are applied before aggregating data into the pivot table. The map's key is the column offset of the source range that you want to filter, and the value is the criteria for that column. For example, if the source was `C10:E15`, a key of `0` will have the filter for column `C`, whereas the key `1` is for column `D`. This field is deprecated in favor of filter_specs."]
        pub criteria: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<PivotFilterCriteria>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataExecutionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The data execution status for data source pivot tables."]
        pub data_execution_status: ::std::option::Option<::std::boxed::Box<DataExecutionStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the data source the pivot table is reading data from."]
        pub data_source_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterSpecs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filters applied to the source columns before aggregating data for the pivot table. Both criteria and filter_specs are populated in responses. If both fields are specified in an update request, this field takes precedence."]
        pub filter_specs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PivotFilterSpec>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Each row grouping in the pivot table."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PivotGroup>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range the pivot table is reading data from."]
        pub source: ::std::option::Option<::std::boxed::Box<GridRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueLayout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether values should be listed horizontally (as columns) or vertically (as rows)."]
        pub value_layout: ::std::option::Option<PivotTableValueLayoutEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of values to include in the pivot table."]
        pub values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PivotValue>>>,
    }
    impl PivotTable {
        pub fn builder() -> PivotTableBuilder {
            PivotTableBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Whether values should be listed horizontally (as columns) or vertically (as rows)."]
    pub enum PivotTableValueLayoutEnum {
        #[serde(rename = "HORIZONTAL")]
        #[doc = "Values are laid out horizontally (as columns)."]
        Horizontal,
        #[serde(rename = "VERTICAL")]
        #[doc = "Values are laid out vertically (as rows)."]
        Vertical,
    }
    impl ::std::default::Default for PivotTableValueLayoutEnum {
        fn default() -> Self {
            Self::Horizontal
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The definition of how a value in a pivot table should be calculated."]
    pub struct PivotValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "calculatedDisplayType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, indicates that pivot values should be displayed as the result of a calculation with another pivot value. For example, if calculated_display_type is specified as PERCENT_OF_GRAND_TOTAL, all the pivot values are displayed as the percentage of the grand total. In the Sheets editor, this is referred to as \"Show As\" in the value section of a pivot table."]
        pub calculated_display_type: ::std::option::Option<PivotValueCalculatedDisplayTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceColumnReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reference to the data source column that this value reads from."]
        pub data_source_column_reference:
            ::std::option::Option<::std::boxed::Box<DataSourceColumnReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formula")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A custom formula to calculate the value. The formula must start with an `=` character."]
        pub formula: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A name to use for the value."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceColumnOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The column offset of the source range that this value reads from. For example, if the source was `C10:E15`, a `sourceColumnOffset` of `0` means this value refers to column `C`, whereas the offset `1` would refer to column `D`."]
        pub source_column_offset: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "summarizeFunction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A function to summarize the value. If formula is set, the only supported values are SUM and CUSTOM. If sourceColumnOffset is set, then `CUSTOM` is not supported."]
        pub summarize_function: ::std::option::Option<PivotValueSummarizeFunctionEnum>,
    }
    impl PivotValue {
        pub fn builder() -> PivotValueBuilder {
            PivotValueBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "If specified, indicates that pivot values should be displayed as the result of a calculation with another pivot value. For example, if calculated_display_type is specified as PERCENT_OF_GRAND_TOTAL, all the pivot values are displayed as the percentage of the grand total. In the Sheets editor, this is referred to as \"Show As\" in the value section of a pivot table."]
    pub enum PivotValueCalculatedDisplayTypeEnum {
        #[serde(rename = "PIVOT_VALUE_CALCULATED_DISPLAY_TYPE_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        PivotValueCalculatedDisplayTypeUnspecified,
        #[serde(rename = "PERCENT_OF_ROW_TOTAL")]
        #[doc = "Shows the pivot values as percentage of the row total values."]
        PercentOfRowTotal,
        #[serde(rename = "PERCENT_OF_COLUMN_TOTAL")]
        #[doc = "Shows the pivot values as percentage of the column total values."]
        PercentOfColumnTotal,
        #[serde(rename = "PERCENT_OF_GRAND_TOTAL")]
        #[doc = "Shows the pivot values as percentage of the grand total values."]
        PercentOfGrandTotal,
    }
    impl ::std::default::Default for PivotValueCalculatedDisplayTypeEnum {
        fn default() -> Self {
            Self::PivotValueCalculatedDisplayTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "A function to summarize the value. If formula is set, the only supported values are SUM and CUSTOM. If sourceColumnOffset is set, then `CUSTOM` is not supported."]
    pub enum PivotValueSummarizeFunctionEnum {
        #[serde(rename = "PIVOT_STANDARD_VALUE_FUNCTION_UNSPECIFIED")]
        #[doc = "The default, do not use."]
        PivotStandardValueFunctionUnspecified,
        #[serde(rename = "SUM")]
        #[doc = "Corresponds to the `SUM` function."]
        Sum,
        #[serde(rename = "COUNTA")]
        #[doc = "Corresponds to the `COUNTA` function."]
        Counta,
        #[serde(rename = "COUNT")]
        #[doc = "Corresponds to the `COUNT` function."]
        Count,
        #[serde(rename = "COUNTUNIQUE")]
        #[doc = "Corresponds to the `COUNTUNIQUE` function."]
        Countunique,
        #[serde(rename = "AVERAGE")]
        #[doc = "Corresponds to the `AVERAGE` function."]
        Average,
        #[serde(rename = "MAX")]
        #[doc = "Corresponds to the `MAX` function."]
        Max,
        #[serde(rename = "MIN")]
        #[doc = "Corresponds to the `MIN` function."]
        Min,
        #[serde(rename = "MEDIAN")]
        #[doc = "Corresponds to the `MEDIAN` function."]
        Median,
        #[serde(rename = "PRODUCT")]
        #[doc = "Corresponds to the `PRODUCT` function."]
        Product,
        #[serde(rename = "STDEV")]
        #[doc = "Corresponds to the `STDEV` function."]
        Stdev,
        #[serde(rename = "STDEVP")]
        #[doc = "Corresponds to the `STDEVP` function."]
        Stdevp,
        #[serde(rename = "VAR")]
        #[doc = "Corresponds to the `VAR` function."]
        Var,
        #[serde(rename = "VARP")]
        #[doc = "Corresponds to the `VARP` function."]
        Varp,
        #[serde(rename = "CUSTOM")]
        #[doc = "Indicates the formula should be used as-is. Only valid if PivotValue.formula was set."]
        Custom,
    }
    impl ::std::default::Default for PivotValueSummarizeFunctionEnum {
        fn default() -> Self {
            Self::PivotStandardValueFunctionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The style of a point on the chart."]
    pub struct PointStyle {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shape")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The point shape. If empty or unspecified, a default shape is used."]
        pub shape: ::std::option::Option<PointStyleShapeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The point size. If empty, a default size is used."]
        pub size: ::std::option::Option<::std::primitive::f64>,
    }
    impl PointStyle {
        pub fn builder() -> PointStyleBuilder {
            PointStyleBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The point shape. If empty or unspecified, a default shape is used."]
    pub enum PointStyleShapeEnum {
        #[serde(rename = "POINT_SHAPE_UNSPECIFIED")]
        #[doc = "Default value."]
        PointShapeUnspecified,
        #[serde(rename = "CIRCLE")]
        #[doc = "A circle shape."]
        Circle,
        #[serde(rename = "DIAMOND")]
        #[doc = "A diamond shape."]
        Diamond,
        #[serde(rename = "HEXAGON")]
        #[doc = "A hexagon shape."]
        Hexagon,
        #[serde(rename = "PENTAGON")]
        #[doc = "A pentagon shape."]
        Pentagon,
        #[serde(rename = "SQUARE")]
        #[doc = "A square shape."]
        Square,
        #[serde(rename = "STAR")]
        #[doc = "A star shape."]
        Star,
        #[serde(rename = "TRIANGLE")]
        #[doc = "A triangle shape."]
        Triangle,
        #[serde(rename = "X_MARK")]
        #[doc = "An x-mark shape."]
        XMark,
    }
    impl ::std::default::Default for PointStyleShapeEnum {
        fn default() -> Self {
            Self::PointShapeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A protected range."]
    pub struct ProtectedRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of this protected range."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "editors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The users and groups with edit access to the protected range. This field is only visible to users with edit access to the protected range and the document. Editors are not supported with warning_only protection."]
        pub editors: ::std::option::Option<::std::boxed::Box<Editors>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namedRangeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The named range this protected range is backed by, if any. When writing, only one of range or named_range_id may be set."]
        pub named_range_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protectedRangeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the protected range. This field is read-only."]
        pub protected_range_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range that is being protected. The range may be fully unbounded, in which case this is considered a protected sheet. When writing, only one of range or named_range_id may be set."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestingUserCanEdit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the user who requested this protected range can edit the protected area. This field is read-only."]
        pub requesting_user_can_edit: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unprotectedRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of unprotected ranges within a protected sheet. Unprotected ranges are only supported on protected sheets."]
        pub unprotected_ranges:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GridRange>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warningOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if this protected range will show a warning when editing. Warning-based protection means that every user can edit data in the protected range, except editing will prompt a warning asking the user to confirm the edit. When writing: if this field is true, then editors is ignored. Additionally, if this field is changed from true to false and the `editors` field is not set (nor included in the field mask), then the editors will be set to all the editors in the document."]
        pub warning_only: ::std::option::Option<::std::primitive::bool>,
    }
    impl ProtectedRange {
        pub fn builder() -> ProtectedRangeBuilder {
            ProtectedRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Randomizes the order of the rows in a range."]
    pub struct RandomizeRangeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range to randomize."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
    }
    impl RandomizeRangeRequest {
        pub fn builder() -> RandomizeRangeRequestBuilder {
            RandomizeRangeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The execution status of refreshing one data source object."]
    pub struct RefreshDataSourceObjectExecutionStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataExecutionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data execution status."]
        pub data_execution_status: ::std::option::Option<::std::boxed::Box<DataExecutionStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reference to a data source object being refreshed."]
        pub reference: ::std::option::Option<::std::boxed::Box<DataSourceObjectReference>>,
    }
    impl RefreshDataSourceObjectExecutionStatus {
        pub fn builder() -> RefreshDataSourceObjectExecutionStatusBuilder {
            RefreshDataSourceObjectExecutionStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Refreshes one or multiple data source objects in the spreadsheet by the specified references. The request requires an additional `bigquery.readonly` OAuth scope. If there are multiple refresh requests referencing the same data source objects in one batch, only the last refresh request is processed, and all those requests will have the same response accordingly."]
    pub struct RefreshDataSourceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reference to a DataSource. If specified, refreshes all associated data source objects for the data source."]
        pub data_source_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "force")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Refreshes the data source objects regardless of the current state. If not set and a referenced data source object was in error state, the refresh will fail immediately."]
        pub force: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isAll")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Refreshes all existing data source objects in the spreadsheet."]
        pub is_all: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "references")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "References to data source objects to refresh."]
        pub references: ::std::option::Option<::std::boxed::Box<DataSourceObjectReferences>>,
    }
    impl RefreshDataSourceRequest {
        pub fn builder() -> RefreshDataSourceRequestBuilder {
            RefreshDataSourceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response from refreshing one or multiple data source objects."]
    pub struct RefreshDataSourceResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statuses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All the refresh status for the data source object references specified in the request. If is_all is specified, the field contains only those in failure status."]
        pub statuses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<RefreshDataSourceObjectExecutionStatus>>,
        >,
    }
    impl RefreshDataSourceResponse {
        pub fn builder() -> RefreshDataSourceResponseBuilder {
            RefreshDataSourceResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates all cells in the range to the values in the given Cell object. Only the fields listed in the fields field are updated; others are unchanged. If writing a cell with a formula, the formula's ranges will automatically increment for each field in the range. For example, if writing a cell with formula `=A1` into range B2:C4, B2 would be `=A1`, B3 would be `=A2`, B4 would be `=A3`, C2 would be `=B1`, C3 would be `=B2`, C4 would be `=B3`. To keep the formula's ranges static, use the `$` indicator. For example, use the formula `=$A$1` to prevent both the row and the column from incrementing."]
    pub struct RepeatCellRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cell")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data to write."]
        pub cell: ::std::option::Option<::std::boxed::Box<CellData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `cell` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range to repeat the cell in."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
    }
    impl RepeatCellRequest {
        pub fn builder() -> RepeatCellRequestBuilder {
            RepeatCellRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single kind of update to apply to a spreadsheet."]
    pub struct Request {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addBanding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Adds a new banded range"]
        pub add_banding: ::std::option::Option<::std::boxed::Box<AddBandingRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Adds a chart."]
        pub add_chart: ::std::option::Option<::std::boxed::Box<AddChartRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addConditionalFormatRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Adds a new conditional format rule."]
        pub add_conditional_format_rule:
            ::std::option::Option<::std::boxed::Box<AddConditionalFormatRuleRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addDataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Adds a data source."]
        pub add_data_source: ::std::option::Option<::std::boxed::Box<AddDataSourceRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addDimensionGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creates a group over the specified range."]
        pub add_dimension_group: ::std::option::Option<::std::boxed::Box<AddDimensionGroupRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addFilterView")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Adds a filter view."]
        pub add_filter_view: ::std::option::Option<::std::boxed::Box<AddFilterViewRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addNamedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Adds a named range."]
        pub add_named_range: ::std::option::Option<::std::boxed::Box<AddNamedRangeRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addProtectedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Adds a protected range."]
        pub add_protected_range: ::std::option::Option<::std::boxed::Box<AddProtectedRangeRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addSheet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Adds a sheet."]
        pub add_sheet: ::std::option::Option<::std::boxed::Box<AddSheetRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addSlicer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Adds a slicer."]
        pub add_slicer: ::std::option::Option<::std::boxed::Box<AddSlicerRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appendCells")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Appends cells after the last row with data in a sheet."]
        pub append_cells: ::std::option::Option<::std::boxed::Box<AppendCellsRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appendDimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Appends dimensions to the end of a sheet."]
        pub append_dimension: ::std::option::Option<::std::boxed::Box<AppendDimensionRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoFill")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Automatically fills in more data based on existing data."]
        pub auto_fill: ::std::option::Option<::std::boxed::Box<AutoFillRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoResizeDimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Automatically resizes one or more dimensions based on the contents of the cells in that dimension."]
        pub auto_resize_dimensions:
            ::std::option::Option<::std::boxed::Box<AutoResizeDimensionsRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clearBasicFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Clears the basic filter on a sheet."]
        pub clear_basic_filter: ::std::option::Option<::std::boxed::Box<ClearBasicFilterRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "copyPaste")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Copies data from one area and pastes it to another."]
        pub copy_paste: ::std::option::Option<::std::boxed::Box<CopyPasteRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createDeveloperMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creates new developer metadata"]
        pub create_developer_metadata:
            ::std::option::Option<::std::boxed::Box<CreateDeveloperMetadataRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cutPaste")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cuts data from one area and pastes it to another."]
        pub cut_paste: ::std::option::Option<::std::boxed::Box<CutPasteRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteBanding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Removes a banded range"]
        pub delete_banding: ::std::option::Option<::std::boxed::Box<DeleteBandingRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteConditionalFormatRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deletes an existing conditional format rule."]
        pub delete_conditional_format_rule:
            ::std::option::Option<::std::boxed::Box<DeleteConditionalFormatRuleRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteDataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deletes a data source."]
        pub delete_data_source: ::std::option::Option<::std::boxed::Box<DeleteDataSourceRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteDeveloperMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deletes developer metadata"]
        pub delete_developer_metadata:
            ::std::option::Option<::std::boxed::Box<DeleteDeveloperMetadataRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteDimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deletes rows or columns in a sheet."]
        pub delete_dimension: ::std::option::Option<::std::boxed::Box<DeleteDimensionRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteDimensionGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deletes a group over the specified range."]
        pub delete_dimension_group:
            ::std::option::Option<::std::boxed::Box<DeleteDimensionGroupRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteDuplicates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Removes rows containing duplicate values in specified columns of a cell range."]
        pub delete_duplicates: ::std::option::Option<::std::boxed::Box<DeleteDuplicatesRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteEmbeddedObject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deletes an embedded object (e.g, chart, image) in a sheet."]
        pub delete_embedded_object:
            ::std::option::Option<::std::boxed::Box<DeleteEmbeddedObjectRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteFilterView")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deletes a filter view from a sheet."]
        pub delete_filter_view: ::std::option::Option<::std::boxed::Box<DeleteFilterViewRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteNamedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deletes a named range."]
        pub delete_named_range: ::std::option::Option<::std::boxed::Box<DeleteNamedRangeRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteProtectedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deletes a protected range."]
        pub delete_protected_range:
            ::std::option::Option<::std::boxed::Box<DeleteProtectedRangeRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deletes a range of cells from a sheet, shifting the remaining cells."]
        pub delete_range: ::std::option::Option<::std::boxed::Box<DeleteRangeRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteSheet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deletes a sheet."]
        pub delete_sheet: ::std::option::Option<::std::boxed::Box<DeleteSheetRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duplicateFilterView")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Duplicates a filter view."]
        pub duplicate_filter_view:
            ::std::option::Option<::std::boxed::Box<DuplicateFilterViewRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duplicateSheet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Duplicates a sheet."]
        pub duplicate_sheet: ::std::option::Option<::std::boxed::Box<DuplicateSheetRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "findReplace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Finds and replaces occurrences of some text with other text."]
        pub find_replace: ::std::option::Option<::std::boxed::Box<FindReplaceRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertDimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inserts new rows or columns in a sheet."]
        pub insert_dimension: ::std::option::Option<::std::boxed::Box<InsertDimensionRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inserts new cells in a sheet, shifting the existing cells."]
        pub insert_range: ::std::option::Option<::std::boxed::Box<InsertRangeRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mergeCells")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Merges cells together."]
        pub merge_cells: ::std::option::Option<::std::boxed::Box<MergeCellsRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "moveDimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Moves rows or columns to another location in a sheet."]
        pub move_dimension: ::std::option::Option<::std::boxed::Box<MoveDimensionRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pasteData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pastes data (HTML or delimited) into a sheet."]
        pub paste_data: ::std::option::Option<::std::boxed::Box<PasteDataRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "randomizeRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Randomizes the order of the rows in a range."]
        pub randomize_range: ::std::option::Option<::std::boxed::Box<RandomizeRangeRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "refreshDataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Refreshs one or multiple data sources and associated dbobjects."]
        pub refresh_data_source: ::std::option::Option<::std::boxed::Box<RefreshDataSourceRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repeatCell")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Repeats a single cell across a range."]
        pub repeat_cell: ::std::option::Option<::std::boxed::Box<RepeatCellRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "setBasicFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sets the basic filter on a sheet."]
        pub set_basic_filter: ::std::option::Option<::std::boxed::Box<SetBasicFilterRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "setDataValidation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sets data validation for one or more cells."]
        pub set_data_validation: ::std::option::Option<::std::boxed::Box<SetDataValidationRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sorts data in a range."]
        pub sort_range: ::std::option::Option<::std::boxed::Box<SortRangeRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textToColumns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Converts a column of text into many columns of text."]
        pub text_to_columns: ::std::option::Option<::std::boxed::Box<TextToColumnsRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trimWhitespace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Trims cells of whitespace (such as spaces, tabs, or new lines)."]
        pub trim_whitespace: ::std::option::Option<::std::boxed::Box<TrimWhitespaceRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unmergeCells")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unmerges merged cells."]
        pub unmerge_cells: ::std::option::Option<::std::boxed::Box<UnmergeCellsRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateBanding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates a banded range"]
        pub update_banding: ::std::option::Option<::std::boxed::Box<UpdateBandingRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateBorders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the borders in a range of cells."]
        pub update_borders: ::std::option::Option<::std::boxed::Box<UpdateBordersRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateCells")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates many cells at once."]
        pub update_cells: ::std::option::Option<::std::boxed::Box<UpdateCellsRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateChartSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates a chart's specifications."]
        pub update_chart_spec: ::std::option::Option<::std::boxed::Box<UpdateChartSpecRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateConditionalFormatRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates an existing conditional format rule."]
        pub update_conditional_format_rule:
            ::std::option::Option<::std::boxed::Box<UpdateConditionalFormatRuleRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateDataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates a data source."]
        pub update_data_source: ::std::option::Option<::std::boxed::Box<UpdateDataSourceRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateDeveloperMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates an existing developer metadata entry"]
        pub update_developer_metadata:
            ::std::option::Option<::std::boxed::Box<UpdateDeveloperMetadataRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateDimensionGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the state of the specified group."]
        pub update_dimension_group:
            ::std::option::Option<::std::boxed::Box<UpdateDimensionGroupRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateDimensionProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates dimensions' properties."]
        pub update_dimension_properties:
            ::std::option::Option<::std::boxed::Box<UpdateDimensionPropertiesRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateEmbeddedObjectBorder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates an embedded object's border."]
        pub update_embedded_object_border:
            ::std::option::Option<::std::boxed::Box<UpdateEmbeddedObjectBorderRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateEmbeddedObjectPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates an embedded object's (e.g. chart, image) position."]
        pub update_embedded_object_position:
            ::std::option::Option<::std::boxed::Box<UpdateEmbeddedObjectPositionRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateFilterView")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the properties of a filter view."]
        pub update_filter_view: ::std::option::Option<::std::boxed::Box<UpdateFilterViewRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateNamedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates a named range."]
        pub update_named_range: ::std::option::Option<::std::boxed::Box<UpdateNamedRangeRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateProtectedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates a protected range."]
        pub update_protected_range:
            ::std::option::Option<::std::boxed::Box<UpdateProtectedRangeRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateSheetProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates a sheet's properties."]
        pub update_sheet_properties:
            ::std::option::Option<::std::boxed::Box<UpdateSheetPropertiesRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateSlicerSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates a slicer's specifications."]
        pub update_slicer_spec: ::std::option::Option<::std::boxed::Box<UpdateSlicerSpecRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateSpreadsheetProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the spreadsheet's properties."]
        pub update_spreadsheet_properties:
            ::std::option::Option<::std::boxed::Box<UpdateSpreadsheetPropertiesRequest>>,
    }
    impl Request {
        pub fn builder() -> RequestBuilder {
            RequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single response from an update."]
    pub struct Response {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addBanding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from adding a banded range."]
        pub add_banding: ::std::option::Option<::std::boxed::Box<AddBandingResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from adding a chart."]
        pub add_chart: ::std::option::Option<::std::boxed::Box<AddChartResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addDataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from adding a data source."]
        pub add_data_source: ::std::option::Option<::std::boxed::Box<AddDataSourceResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addDimensionGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from adding a dimension group."]
        pub add_dimension_group:
            ::std::option::Option<::std::boxed::Box<AddDimensionGroupResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addFilterView")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from adding a filter view."]
        pub add_filter_view: ::std::option::Option<::std::boxed::Box<AddFilterViewResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addNamedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from adding a named range."]
        pub add_named_range: ::std::option::Option<::std::boxed::Box<AddNamedRangeResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addProtectedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from adding a protected range."]
        pub add_protected_range:
            ::std::option::Option<::std::boxed::Box<AddProtectedRangeResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addSheet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from adding a sheet."]
        pub add_sheet: ::std::option::Option<::std::boxed::Box<AddSheetResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addSlicer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from adding a slicer."]
        pub add_slicer: ::std::option::Option<::std::boxed::Box<AddSlicerResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createDeveloperMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from creating a developer metadata entry."]
        pub create_developer_metadata:
            ::std::option::Option<::std::boxed::Box<CreateDeveloperMetadataResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteConditionalFormatRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from deleting a conditional format rule."]
        pub delete_conditional_format_rule:
            ::std::option::Option<::std::boxed::Box<DeleteConditionalFormatRuleResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteDeveloperMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from deleting a developer metadata entry."]
        pub delete_developer_metadata:
            ::std::option::Option<::std::boxed::Box<DeleteDeveloperMetadataResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteDimensionGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from deleting a dimension group."]
        pub delete_dimension_group:
            ::std::option::Option<::std::boxed::Box<DeleteDimensionGroupResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteDuplicates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from removing rows containing duplicate values."]
        pub delete_duplicates: ::std::option::Option<::std::boxed::Box<DeleteDuplicatesResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duplicateFilterView")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from duplicating a filter view."]
        pub duplicate_filter_view:
            ::std::option::Option<::std::boxed::Box<DuplicateFilterViewResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duplicateSheet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from duplicating a sheet."]
        pub duplicate_sheet: ::std::option::Option<::std::boxed::Box<DuplicateSheetResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "findReplace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from doing a find/replace."]
        pub find_replace: ::std::option::Option<::std::boxed::Box<FindReplaceResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "refreshDataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from refreshing data source objects."]
        pub refresh_data_source:
            ::std::option::Option<::std::boxed::Box<RefreshDataSourceResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trimWhitespace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from trimming whitespace."]
        pub trim_whitespace: ::std::option::Option<::std::boxed::Box<TrimWhitespaceResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateConditionalFormatRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from updating a conditional format rule."]
        pub update_conditional_format_rule:
            ::std::option::Option<::std::boxed::Box<UpdateConditionalFormatRuleResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateDataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from updating a data source."]
        pub update_data_source: ::std::option::Option<::std::boxed::Box<UpdateDataSourceResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateDeveloperMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from updating a developer metadata entry."]
        pub update_developer_metadata:
            ::std::option::Option<::std::boxed::Box<UpdateDeveloperMetadataResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateEmbeddedObjectPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reply from updating an embedded object's position."]
        pub update_embedded_object_position:
            ::std::option::Option<::std::boxed::Box<UpdateEmbeddedObjectPositionResponse>>,
    }
    impl Response {
        pub fn builder() -> ResponseBuilder {
            ResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data about each cell in a row."]
    pub struct RowData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values in the row, one per column."]
        pub values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CellData>>>,
    }
    impl RowData {
        pub fn builder() -> RowDataBuilder {
            RowDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A scorecard chart. Scorecard charts are used to highlight key performance indicators, known as KPIs, on the spreadsheet. A scorecard chart can represent things like total sales, average cost, or a top selling item. You can specify a single data value, or aggregate over a range of data. Percentage or absolute difference from a baseline value can be highlighted, like changes over time."]
    pub struct ScorecardChartSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aggregateType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The aggregation type for key and baseline chart data in scorecard chart. This field is not supported for data source charts. Use the ChartData.aggregateType field of the key_value_data or baseline_value_data instead for data source charts. This field is optional."]
        pub aggregate_type: ::std::option::Option<ScorecardChartSpecAggregateTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "baselineValueData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data for scorecard baseline value. This field is optional."]
        pub baseline_value_data: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "baselineValueFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Formatting options for baseline value. This field is needed only if baseline_value_data is specified."]
        pub baseline_value_format: ::std::option::Option<::std::boxed::Box<BaselineValueFormat>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customFormatOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom formatting options for numeric key/baseline values in scorecard chart. This field is used only when number_format_source is set to CUSTOM. This field is optional."]
        pub custom_format_options:
            ::std::option::Option<::std::boxed::Box<ChartCustomNumberFormatOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyValueData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data for scorecard key value."]
        pub key_value_data: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyValueFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Formatting options for key value."]
        pub key_value_format: ::std::option::Option<::std::boxed::Box<KeyValueFormat>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numberFormatSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number format source used in the scorecard chart. This field is optional."]
        pub number_format_source: ::std::option::Option<ScorecardChartSpecNumberFormatSourceEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scaleFactor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value to scale scorecard key and baseline value. For example, a factor of 10 can be used to divide all values in the chart by 10. This field is optional."]
        pub scale_factor: ::std::option::Option<::std::primitive::f64>,
    }
    impl ScorecardChartSpec {
        pub fn builder() -> ScorecardChartSpecBuilder {
            ScorecardChartSpecBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The aggregation type for key and baseline chart data in scorecard chart. This field is not supported for data source charts. Use the ChartData.aggregateType field of the key_value_data or baseline_value_data instead for data source charts. This field is optional."]
    pub enum ScorecardChartSpecAggregateTypeEnum {
        #[serde(rename = "CHART_AGGREGATE_TYPE_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        ChartAggregateTypeUnspecified,
        #[serde(rename = "AVERAGE")]
        #[doc = "Average aggregate function."]
        Average,
        #[serde(rename = "COUNT")]
        #[doc = "Count aggregate function."]
        Count,
        #[serde(rename = "MAX")]
        #[doc = "Maximum aggregate function."]
        Max,
        #[serde(rename = "MEDIAN")]
        #[doc = "Median aggregate function."]
        Median,
        #[serde(rename = "MIN")]
        #[doc = "Minimum aggregate function."]
        Min,
        #[serde(rename = "SUM")]
        #[doc = "Sum aggregate function."]
        Sum,
    }
    impl ::std::default::Default for ScorecardChartSpecAggregateTypeEnum {
        fn default() -> Self {
            Self::ChartAggregateTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The number format source used in the scorecard chart. This field is optional."]
    pub enum ScorecardChartSpecNumberFormatSourceEnum {
        #[serde(rename = "CHART_NUMBER_FORMAT_SOURCE_UNDEFINED")]
        #[doc = "Default value, do not use."]
        ChartNumberFormatSourceUndefined,
        #[serde(rename = "FROM_DATA")]
        #[doc = "Inherit number formatting from data."]
        FromData,
        #[serde(rename = "CUSTOM")]
        #[doc = "Apply custom formatting as specified by ChartCustomNumberFormatOptions."]
        Custom,
    }
    impl ::std::default::Default for ScorecardChartSpecNumberFormatSourceEnum {
        fn default() -> Self {
            Self::ChartNumberFormatSourceUndefined
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to retrieve all developer metadata matching the set of specified criteria."]
    pub struct SearchDeveloperMetadataRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataFilters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data filters describing the criteria used to determine which DeveloperMetadata entries to return. DeveloperMetadata matching any of the specified filters are included in the response."]
        pub data_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataFilter>>>,
    }
    impl SearchDeveloperMetadataRequest {
        pub fn builder() -> SearchDeveloperMetadataRequestBuilder {
            SearchDeveloperMetadataRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A reply to a developer metadata search request."]
    pub struct SearchDeveloperMetadataResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchedDeveloperMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metadata matching the criteria of the search request."]
        pub matched_developer_metadata:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MatchedDeveloperMetadata>>>,
    }
    impl SearchDeveloperMetadataResponse {
        pub fn builder() -> SearchDeveloperMetadataResponseBuilder {
            SearchDeveloperMetadataResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Sets the basic filter associated with a sheet."]
    pub struct SetBasicFilterRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filter to set."]
        pub filter: ::std::option::Option<::std::boxed::Box<BasicFilter>>,
    }
    impl SetBasicFilterRequest {
        pub fn builder() -> SetBasicFilterRequestBuilder {
            SetBasicFilterRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Sets a data validation rule to every cell in the range. To clear validation in a range, call this with no rule specified."]
    pub struct SetDataValidationRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range the data validation rule should apply to."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data validation rule to set on each cell in the range, or empty to clear the data validation in the range."]
        pub rule: ::std::option::Option<::std::boxed::Box<DataValidationRule>>,
    }
    impl SetDataValidationRequest {
        pub fn builder() -> SetDataValidationRequestBuilder {
            SetDataValidationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A sheet in a spreadsheet."]
    pub struct Sheet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bandedRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The banded (alternating colors) ranges on this sheet."]
        pub banded_ranges: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BandedRange>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filter on this sheet, if any."]
        pub basic_filter: ::std::option::Option<::std::boxed::Box<BasicFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "charts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The specifications of every chart on this sheet."]
        pub charts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EmbeddedChart>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnGroups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All column groups on this sheet, ordered by increasing range start index, then by group depth."]
        pub column_groups:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionGroup>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conditionalFormats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The conditional format rules in this sheet."]
        pub conditional_formats:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ConditionalFormatRule>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data in the grid, if this is a grid sheet. The number of GridData objects returned is dependent on the number of ranges requested on this sheet. For example, if this is representing `Sheet1`, and the spreadsheet was requested with ranges `Sheet1!A1:C10` and `Sheet1!D15:E20`, then the first GridData will have a startRow/startColumn of `0`, while the second one will have `startRow 14` (zero-based row 15), and `startColumn 3` (zero-based column D). For a DATA_SOURCE sheet, you can not request a specific range, the GridData contains all the values."]
        pub data: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GridData>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "developerMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The developer metadata associated with a sheet."]
        pub developer_metadata:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeveloperMetadata>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterViews")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filter views in this sheet."]
        pub filter_views: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FilterView>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ranges that are merged together."]
        pub merges: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GridRange>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties of the sheet."]
        pub properties: ::std::option::Option<::std::boxed::Box<SheetProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protectedRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The protected ranges in this sheet."]
        pub protected_ranges:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProtectedRange>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowGroups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All row groups on this sheet, ordered by increasing range start index, then by group depth."]
        pub row_groups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionGroup>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slicers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The slicers on this sheet."]
        pub slicers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Slicer>>>,
    }
    impl Sheet {
        pub fn builder() -> SheetBuilder {
            SheetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Properties of a sheet."]
    pub struct SheetProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceSheetProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If present, the field contains DATA_SOURCE sheet specific properties."]
        pub data_source_sheet_properties:
            ::std::option::Option<::std::boxed::Box<DataSourceSheetProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gridProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional properties of the sheet if this sheet is a grid. (If the sheet is an object sheet, containing a chart or image, then this field will be absent.) When writing it is an error to set any grid properties on non-grid sheets. If this sheet is a DATA_SOURCE sheet, this field is output only but contains the properties that reflect how a data source sheet is rendered in the UI, e.g. row_count."]
        pub grid_properties: ::std::option::Option<::std::boxed::Box<GridProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hidden")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the sheet is hidden in the UI, false if it's visible."]
        pub hidden: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the sheet within the spreadsheet. When adding or updating sheet properties, if this field is excluded then the sheet is added or moved to the end of the sheet list. When updating sheet indices or inserting sheets, movement is considered in \"before the move\" indexes. For example, if there were 3 sheets (S1, S2, S3) in order to move S1 ahead of S2 the index would have to be set to 2. A sheet index update request is ignored if the requested index is identical to the sheets current index or if the requested new index is equal to the current sheet index + 1."]
        pub index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rightToLeft")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the sheet is an RTL sheet instead of an LTR sheet."]
        pub right_to_left: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the sheet. Must be non-negative. This field cannot be changed once set."]
        pub sheet_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of sheet. Defaults to GRID. This field cannot be changed once set."]
        pub sheet_type: ::std::option::Option<SheetPropertiesSheetTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tabColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the tab in the UI."]
        pub tab_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tabColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the tab in the UI. If tab_color is also set, this field takes precedence."]
        pub tab_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the sheet."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl SheetProperties {
        pub fn builder() -> SheetPropertiesBuilder {
            SheetPropertiesBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of sheet. Defaults to GRID. This field cannot be changed once set."]
    pub enum SheetPropertiesSheetTypeEnum {
        #[serde(rename = "SHEET_TYPE_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        SheetTypeUnspecified,
        #[serde(rename = "GRID")]
        #[doc = "The sheet is a grid."]
        Grid,
        #[serde(rename = "OBJECT")]
        #[doc = "The sheet has no grid and instead has an object like a chart or image."]
        Object,
        #[serde(rename = "DATA_SOURCE")]
        #[doc = "The sheet connects with an external DataSource and shows the preview of data."]
        DataSource,
    }
    impl ::std::default::Default for SheetPropertiesSheetTypeEnum {
        fn default() -> Self {
            Self::SheetTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A slicer in a sheet."]
    pub struct Slicer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The position of the slicer. Note that slicer can be positioned only on existing sheet. Also, width and height of slicer can be automatically adjusted to keep it within permitted limits."]
        pub position: ::std::option::Option<::std::boxed::Box<EmbeddedObjectPosition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slicerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the slicer."]
        pub slicer_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The specification of the slicer."]
        pub spec: ::std::option::Option<::std::boxed::Box<SlicerSpec>>,
    }
    impl Slicer {
        pub fn builder() -> SlicerBuilder {
            SlicerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The specifications of a slicer."]
    pub struct SlicerSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applyToPivotTables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the filter should apply to pivot tables. If not set, default to `True`."]
        pub apply_to_pivot_tables: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color of the slicer."]
        pub background_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color of the slicer. If background_color is also set, this field takes precedence."]
        pub background_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The column index in the data table on which the filter is applied to."]
        pub column_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data range of the slicer."]
        pub data_range: ::std::option::Option<::std::boxed::Box<GridRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterCriteria")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filtering criteria of the slicer."]
        pub filter_criteria: ::std::option::Option<::std::boxed::Box<FilterCriteria>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "horizontalAlignment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The horizontal alignment of title in the slicer. If unspecified, defaults to `LEFT`"]
        pub horizontal_alignment: ::std::option::Option<SlicerSpecHorizontalAlignmentEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text format of title in the slicer."]
        pub text_format: ::std::option::Option<::std::boxed::Box<TextFormat>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the slicer."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl SlicerSpec {
        pub fn builder() -> SlicerSpecBuilder {
            SlicerSpecBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The horizontal alignment of title in the slicer. If unspecified, defaults to `LEFT`"]
    pub enum SlicerSpecHorizontalAlignmentEnum {
        #[serde(rename = "HORIZONTAL_ALIGN_UNSPECIFIED")]
        #[doc = "The horizontal alignment is not specified. Do not use this."]
        HorizontalAlignUnspecified,
        #[serde(rename = "LEFT")]
        #[doc = "The text is explicitly aligned to the left of the cell."]
        Left,
        #[serde(rename = "CENTER")]
        #[doc = "The text is explicitly aligned to the center of the cell."]
        Center,
        #[serde(rename = "RIGHT")]
        #[doc = "The text is explicitly aligned to the right of the cell."]
        Right,
    }
    impl ::std::default::Default for SlicerSpecHorizontalAlignmentEnum {
        fn default() -> Self {
            Self::HorizontalAlignUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Sorts data in rows based on a sort order per column."]
    pub struct SortRangeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range to sort."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortSpecs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sort order per column. Later specifications are used when values are equal in the earlier specifications."]
        pub sort_specs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SortSpec>>>,
    }
    impl SortRangeRequest {
        pub fn builder() -> SortRangeRequestBuilder {
            SortRangeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A sort order associated with a specific column or row."]
    pub struct SortSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background fill color to sort by; cells with this fill color are sorted to the top. Mutually exclusive with foreground_color."]
        pub background_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background fill color to sort by; cells with this fill color are sorted to the top. Mutually exclusive with foreground_color, and must be an RGB-type color. If background_color is also set, this field takes precedence."]
        pub background_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceColumnReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reference to a data source column."]
        pub data_source_column_reference:
            ::std::option::Option<::std::boxed::Box<DataSourceColumnReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimension the sort should be applied to."]
        pub dimension_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "foregroundColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The foreground color to sort by; cells with this foreground color are sorted to the top. Mutually exclusive with background_color."]
        pub foreground_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "foregroundColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The foreground color to sort by; cells with this foreground color are sorted to the top. Mutually exclusive with background_color, and must be an RGB-type color. If foreground_color is also set, this field takes precedence."]
        pub foreground_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The order data should be sorted."]
        pub sort_order: ::std::option::Option<SortSpecSortOrderEnum>,
    }
    impl SortSpec {
        pub fn builder() -> SortSpecBuilder {
            SortSpecBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The order data should be sorted."]
    pub enum SortSpecSortOrderEnum {
        #[serde(rename = "SORT_ORDER_UNSPECIFIED")]
        #[doc = "Default value, do not use this."]
        SortOrderUnspecified,
        #[serde(rename = "ASCENDING")]
        #[doc = "Sort ascending."]
        Ascending,
        #[serde(rename = "DESCENDING")]
        #[doc = "Sort descending."]
        Descending,
    }
    impl ::std::default::Default for SortSpecSortOrderEnum {
        fn default() -> Self {
            Self::SortOrderUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A combination of a source range and how to extend that source."]
    pub struct SourceAndDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimension that data should be filled into."]
        pub dimension: ::std::option::Option<SourceAndDestinationDimensionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fillLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of rows or columns that data should be filled into. Positive numbers expand beyond the last row or last column of the source. Negative numbers expand before the first row or first column of the source."]
        pub fill_length: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location of the data to use as the source of the autofill."]
        pub source: ::std::option::Option<::std::boxed::Box<GridRange>>,
    }
    impl SourceAndDestination {
        pub fn builder() -> SourceAndDestinationBuilder {
            SourceAndDestinationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The dimension that data should be filled into."]
    pub enum SourceAndDestinationDimensionEnum {
        #[serde(rename = "DIMENSION_UNSPECIFIED")]
        #[doc = "The default value, do not use."]
        DimensionUnspecified,
        #[serde(rename = "ROWS")]
        #[doc = "Operates on the rows of a sheet."]
        Rows,
        #[serde(rename = "COLUMNS")]
        #[doc = "Operates on the columns of a sheet."]
        Columns,
    }
    impl ::std::default::Default for SourceAndDestinationDimensionEnum {
        fn default() -> Self {
            Self::DimensionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Resource that represents a spreadsheet."]
    pub struct Spreadsheet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceSchedules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A list of data source refresh schedules."]
        pub data_source_schedules:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSourceRefreshSchedule>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of external data sources connected with the spreadsheet."]
        pub data_sources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSource>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "developerMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The developer metadata associated with a spreadsheet."]
        pub developer_metadata:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeveloperMetadata>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namedRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The named ranges defined in a spreadsheet."]
        pub named_ranges: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NamedRange>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Overall properties of a spreadsheet."]
        pub properties: ::std::option::Option<::std::boxed::Box<SpreadsheetProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sheets that are part of a spreadsheet."]
        pub sheets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Sheet>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spreadsheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the spreadsheet. This field is read-only."]
        pub spreadsheet_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spreadsheetUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The url of the spreadsheet. This field is read-only."]
        pub spreadsheet_url: ::std::option::Option<::std::string::String>,
    }
    impl Spreadsheet {
        pub fn builder() -> SpreadsheetBuilder {
            SpreadsheetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Properties of a spreadsheet."]
    pub struct SpreadsheetProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoRecalc")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of time to wait before volatile functions are recalculated."]
        pub auto_recalc: ::std::option::Option<SpreadsheetPropertiesAutoRecalcEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default format of all cells in the spreadsheet. CellData.effectiveFormat will not be set if the cell's format is equal to this default format. This field is read-only."]
        pub default_format: ::std::option::Option<::std::boxed::Box<CellFormat>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iterativeCalculationSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines whether and how circular references are resolved with iterative calculation. Absence of this field means that circular references result in calculation errors."]
        pub iterative_calculation_settings:
            ::std::option::Option<::std::boxed::Box<IterativeCalculationSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The locale of the spreadsheet in one of the following formats: * an ISO 639-1 language code such as `en` * an ISO 639-2 language code such as `fil`, if no 639-1 code exists * a combination of the ISO language code and country code, such as `en_US` Note: when updating this field, not all locales/languages are supported."]
        pub locale: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spreadsheetTheme")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Theme applied to the spreadsheet."]
        pub spreadsheet_theme: ::std::option::Option<::std::boxed::Box<SpreadsheetTheme>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time zone of the spreadsheet, in CLDR format such as `America/New_York`. If the time zone isn't recognized, this may be a custom time zone such as `GMT-07:00`."]
        pub time_zone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the spreadsheet."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl SpreadsheetProperties {
        pub fn builder() -> SpreadsheetPropertiesBuilder {
            SpreadsheetPropertiesBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The amount of time to wait before volatile functions are recalculated."]
    pub enum SpreadsheetPropertiesAutoRecalcEnum {
        #[serde(rename = "RECALCULATION_INTERVAL_UNSPECIFIED")]
        #[doc = "Default value. This value must not be used."]
        RecalculationIntervalUnspecified,
        #[serde(rename = "ON_CHANGE")]
        #[doc = "Volatile functions are updated on every change."]
        OnChange,
        #[serde(rename = "MINUTE")]
        #[doc = "Volatile functions are updated on every change and every minute."]
        Minute,
        #[serde(rename = "HOUR")]
        #[doc = "Volatile functions are updated on every change and hourly."]
        Hour,
    }
    impl ::std::default::Default for SpreadsheetPropertiesAutoRecalcEnum {
        fn default() -> Self {
            Self::RecalculationIntervalUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents spreadsheet theme"]
    pub struct SpreadsheetTheme {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryFontFamily")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the primary font family."]
        pub primary_font_family: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "themeColors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The spreadsheet theme color pairs. To update you must provide all theme color pairs."]
        pub theme_colors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ThemeColorPair>>>,
    }
    impl SpreadsheetTheme {
        pub fn builder() -> SpreadsheetThemeBuilder {
            SpreadsheetThemeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The format of a run of text in a cell. Absent values indicate that the field isn't specified."]
    pub struct TextFormat {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the text is bold."]
        pub bold: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fontFamily")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The font family."]
        pub font_family: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fontSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the font."]
        pub font_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "foregroundColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The foreground color of the text."]
        pub foreground_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "foregroundColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The foreground color of the text. If foreground_color is also set, this field takes precedence."]
        pub foreground_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "italic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the text is italicized."]
        pub italic: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "strikethrough")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the text has a strikethrough."]
        pub strikethrough: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "underline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the text is underlined."]
        pub underline: ::std::option::Option<::std::primitive::bool>,
    }
    impl TextFormat {
        pub fn builder() -> TextFormatBuilder {
            TextFormatBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A run of a text format. The format of this run continues until the start index of the next run. When updating, all fields must be set."]
    pub struct TextFormatRun {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "format")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The format of this run. Absent values inherit the cell's format."]
        pub format: ::std::option::Option<::std::boxed::Box<TextFormat>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The character index where this run starts."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
    }
    impl TextFormatRun {
        pub fn builder() -> TextFormatRunBuilder {
            TextFormatRunBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Position settings for text."]
    pub struct TextPosition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "horizontalAlignment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Horizontal alignment setting for the piece of text."]
        pub horizontal_alignment: ::std::option::Option<TextPositionHorizontalAlignmentEnum>,
    }
    impl TextPosition {
        pub fn builder() -> TextPositionBuilder {
            TextPositionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Horizontal alignment setting for the piece of text."]
    pub enum TextPositionHorizontalAlignmentEnum {
        #[serde(rename = "HORIZONTAL_ALIGN_UNSPECIFIED")]
        #[doc = "The horizontal alignment is not specified. Do not use this."]
        HorizontalAlignUnspecified,
        #[serde(rename = "LEFT")]
        #[doc = "The text is explicitly aligned to the left of the cell."]
        Left,
        #[serde(rename = "CENTER")]
        #[doc = "The text is explicitly aligned to the center of the cell."]
        Center,
        #[serde(rename = "RIGHT")]
        #[doc = "The text is explicitly aligned to the right of the cell."]
        Right,
    }
    impl ::std::default::Default for TextPositionHorizontalAlignmentEnum {
        fn default() -> Self {
            Self::HorizontalAlignUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The rotation applied to text in a cell."]
    pub struct TextRotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "angle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The angle between the standard orientation and the desired orientation. Measured in degrees. Valid values are between -90 and 90. Positive angles are angled upwards, negative are angled downwards. Note: For LTR text direction positive angles are in the counterclockwise direction, whereas for RTL they are in the clockwise direction"]
        pub angle: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertical")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, text reads top to bottom, but the orientation of individual characters is unchanged. For example: | V | | e | | r | | t | | i | | c | | a | | l |"]
        pub vertical: ::std::option::Option<::std::primitive::bool>,
    }
    impl TextRotation {
        pub fn builder() -> TextRotationBuilder {
            TextRotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Splits a column of text into multiple columns, based on a delimiter in each cell."]
    pub struct TextToColumnsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delimiter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The delimiter to use. Used only if delimiterType is CUSTOM."]
        pub delimiter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delimiterType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The delimiter type to use."]
        pub delimiter_type: ::std::option::Option<TextToColumnsRequestDelimiterTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source data range. This must span exactly one column."]
        pub source: ::std::option::Option<::std::boxed::Box<GridRange>>,
    }
    impl TextToColumnsRequest {
        pub fn builder() -> TextToColumnsRequestBuilder {
            TextToColumnsRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The delimiter type to use."]
    pub enum TextToColumnsRequestDelimiterTypeEnum {
        #[serde(rename = "DELIMITER_TYPE_UNSPECIFIED")]
        #[doc = "Default value. This value must not be used."]
        DelimiterTypeUnspecified,
        #[serde(rename = "COMMA")]
        #[doc = "\",\""]
        Comma,
        #[serde(rename = "SEMICOLON")]
        #[doc = "\";\""]
        Semicolon,
        #[serde(rename = "PERIOD")]
        #[doc = "\".\""]
        Period,
        #[serde(rename = "SPACE")]
        #[doc = "\" \""]
        Space,
        #[serde(rename = "CUSTOM")]
        #[doc = "A custom value as defined in delimiter."]
        Custom,
        #[serde(rename = "AUTODETECT")]
        #[doc = "Automatically detect columns."]
        Autodetect,
    }
    impl ::std::default::Default for TextToColumnsRequestDelimiterTypeEnum {
        fn default() -> Self {
            Self::DelimiterTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A pair mapping a spreadsheet theme color type to the concrete color it represents."]
    pub struct ThemeColorPair {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The concrete color corresponding to the theme color type."]
        pub color: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colorType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the spreadsheet theme color."]
        pub color_type: ::std::option::Option<ThemeColorPairColorTypeEnum>,
    }
    impl ThemeColorPair {
        pub fn builder() -> ThemeColorPairBuilder {
            ThemeColorPairBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the spreadsheet theme color."]
    pub enum ThemeColorPairColorTypeEnum {
        #[serde(rename = "THEME_COLOR_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified theme color"]
        ThemeColorTypeUnspecified,
        #[serde(rename = "TEXT")]
        #[doc = "Represents the primary text color"]
        Text,
        #[serde(rename = "BACKGROUND")]
        #[doc = "Represents the primary background color"]
        Background,
        #[serde(rename = "ACCENT1")]
        #[doc = "Represents the first accent color"]
        Accent1,
        #[serde(rename = "ACCENT2")]
        #[doc = "Represents the second accent color"]
        Accent2,
        #[serde(rename = "ACCENT3")]
        #[doc = "Represents the third accent color"]
        Accent3,
        #[serde(rename = "ACCENT4")]
        #[doc = "Represents the fourth accent color"]
        Accent4,
        #[serde(rename = "ACCENT5")]
        #[doc = "Represents the fifth accent color"]
        Accent5,
        #[serde(rename = "ACCENT6")]
        #[doc = "Represents the sixth accent color"]
        Accent6,
        #[serde(rename = "LINK")]
        #[doc = "Represents the color to use for hyperlinks"]
        Link,
    }
    impl ::std::default::Default for ThemeColorPairColorTypeEnum {
        fn default() -> Self {
            Self::ThemeColorTypeUnspecified
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
    #[doc = "A color scale for a treemap chart."]
    pub struct TreemapChartColorScale {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxValueColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color for cells with a color value greater than or equal to maxValue. Defaults to #109618 if not specified."]
        pub max_value_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxValueColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color for cells with a color value greater than or equal to maxValue. Defaults to #109618 if not specified. If max_value_color is also set, this field takes precedence."]
        pub max_value_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "midValueColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color for cells with a color value at the midpoint between minValue and maxValue. Defaults to #efe6dc if not specified."]
        pub mid_value_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "midValueColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color for cells with a color value at the midpoint between minValue and maxValue. Defaults to #efe6dc if not specified. If mid_value_color is also set, this field takes precedence."]
        pub mid_value_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minValueColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color for cells with a color value less than or equal to minValue. Defaults to #dc3912 if not specified."]
        pub min_value_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minValueColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color for cells with a color value less than or equal to minValue. Defaults to #dc3912 if not specified. If min_value_color is also set, this field takes precedence."]
        pub min_value_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "noDataColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color for cells that have no color data associated with them. Defaults to #000000 if not specified."]
        pub no_data_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "noDataColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color for cells that have no color data associated with them. Defaults to #000000 if not specified. If no_data_color is also set, this field takes precedence."]
        pub no_data_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
    }
    impl TreemapChartColorScale {
        pub fn builder() -> TreemapChartColorScaleBuilder {
            TreemapChartColorScaleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Treemap chart."]
    pub struct TreemapChartSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colorData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data that determines the background color of each treemap data cell. This field is optional. If not specified, size_data is used to determine background colors. If specified, the data is expected to be numeric. color_scale will determine how the values in this data map to data cell background colors."]
        pub color_data: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colorScale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color scale for data cells in the treemap chart. Data cells are assigned colors based on their color values. These color values come from color_data, or from size_data if color_data is not specified. Cells with color values less than or equal to min_value will have minValueColor as their background color. Cells with color values greater than or equal to max_value will have maxValueColor as their background color. Cells with color values between min_value and max_value will have background colors on a gradient between minValueColor and maxValueColor, the midpoint of the gradient being midValueColor. Cells with missing or non-numeric color values will have noDataColor as their background color."]
        pub color_scale: ::std::option::Option<::std::boxed::Box<TreemapChartColorScale>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headerColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color for header cells."]
        pub header_color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headerColorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color for header cells. If header_color is also set, this field takes precedence."]
        pub header_color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hideTooltips")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True to hide tooltips."]
        pub hide_tooltips: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hintedLevels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of additional data levels beyond the labeled levels to be shown on the treemap chart. These levels are not interactive and are shown without their labels. Defaults to 0 if not specified."]
        pub hinted_levels: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data that contains the treemap cell labels."]
        pub labels: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "levels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of data levels to show on the treemap chart. These levels are interactive and are shown with their labels. Defaults to 2 if not specified."]
        pub levels: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum possible data value. Cells with values greater than this will have the same color as cells with this value. If not specified, defaults to the actual maximum value from color_data, or the maximum value from size_data if color_data is not specified."]
        pub max_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum possible data value. Cells with values less than this will have the same color as cells with this value. If not specified, defaults to the actual minimum value from color_data, or the minimum value from size_data if color_data is not specified."]
        pub min_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data the contains the treemap cells' parent labels."]
        pub parent_labels: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sizeData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data that determines the size of each treemap data cell. This data is expected to be numeric. The cells corresponding to non-numeric or missing data will not be rendered. If color_data is not specified, this data is used to determine data cell background colors as well."]
        pub size_data: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text format for all labels on the chart."]
        pub text_format: ::std::option::Option<::std::boxed::Box<TextFormat>>,
    }
    impl TreemapChartSpec {
        pub fn builder() -> TreemapChartSpecBuilder {
            TreemapChartSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Trims the whitespace (such as spaces, tabs, or new lines) in every cell in the specified range. This request removes all whitespace from the start and end of each cell's text, and reduces any subsequence of remaining whitespace characters to a single space. If the resulting trimmed text starts with a '+' or '=' character, the text remains as a string value and isn't interpreted as a formula."]
    pub struct TrimWhitespaceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range whose cells to trim."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
    }
    impl TrimWhitespaceRequest {
        pub fn builder() -> TrimWhitespaceRequestBuilder {
            TrimWhitespaceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of trimming whitespace in cells."]
    pub struct TrimWhitespaceResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cellsChangedCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of cells that were trimmed of whitespace."]
        pub cells_changed_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl TrimWhitespaceResponse {
        pub fn builder() -> TrimWhitespaceResponseBuilder {
            TrimWhitespaceResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Unmerges cells in the given range."]
    pub struct UnmergeCellsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range within which all cells should be unmerged. If the range spans multiple merges, all will be unmerged. The range must not partially span any merge."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
    }
    impl UnmergeCellsRequest {
        pub fn builder() -> UnmergeCellsRequestBuilder {
            UnmergeCellsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates properties of the supplied banded range."]
    pub struct UpdateBandingRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bandedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The banded range to update with the new properties."]
        pub banded_range: ::std::option::Option<::std::boxed::Box<BandedRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `bandedRange` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field."]
        pub fields: ::std::option::Option<::std::string::String>,
    }
    impl UpdateBandingRequest {
        pub fn builder() -> UpdateBandingRequestBuilder {
            UpdateBandingRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates the borders of a range. If a field is not set in the request, that means the border remains as-is. For example, with two subsequent UpdateBordersRequest: 1. range: A1:A5 `{ top: RED, bottom: WHITE }` 2. range: A1:A5 `{ left: BLUE }` That would result in A1:A5 having a borders of `{ top: RED, bottom: WHITE, left: BLUE }`. If you want to clear a border, explicitly set the style to NONE."]
    pub struct UpdateBordersRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bottom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The border to put at the bottom of the range."]
        pub bottom: ::std::option::Option<::std::boxed::Box<Border>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "innerHorizontal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The horizontal border to put within the range."]
        pub inner_horizontal: ::std::option::Option<::std::boxed::Box<Border>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "innerVertical")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The vertical border to put within the range."]
        pub inner_vertical: ::std::option::Option<::std::boxed::Box<Border>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "left")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The border to put at the left of the range."]
        pub left: ::std::option::Option<::std::boxed::Box<Border>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range whose borders should be updated."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "right")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The border to put at the right of the range."]
        pub right: ::std::option::Option<::std::boxed::Box<Border>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "top")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The border to put at the top of the range."]
        pub top: ::std::option::Option<::std::boxed::Box<Border>>,
    }
    impl UpdateBordersRequest {
        pub fn builder() -> UpdateBordersRequestBuilder {
            UpdateBordersRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates all cells in a range with new data."]
    pub struct UpdateCellsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `\"*\"` can be used as short-hand for listing every field."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range to write data to. If the data in rows does not cover the entire requested range, the fields matching those set in fields will be cleared."]
        pub range: ::std::option::Option<::std::boxed::Box<GridRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data to write."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RowData>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The coordinate to start writing data at. Any number of rows and columns (including a different number of columns per row) may be written."]
        pub start: ::std::option::Option<::std::boxed::Box<GridCoordinate>>,
    }
    impl UpdateCellsRequest {
        pub fn builder() -> UpdateCellsRequestBuilder {
            UpdateCellsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates a chart's specifications. (This does not move or resize a chart. To move or resize a chart, use UpdateEmbeddedObjectPositionRequest.)"]
    pub struct UpdateChartSpecRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chartId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the chart to update."]
        pub chart_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The specification to apply to the chart."]
        pub spec: ::std::option::Option<::std::boxed::Box<ChartSpec>>,
    }
    impl UpdateChartSpecRequest {
        pub fn builder() -> UpdateChartSpecRequestBuilder {
            UpdateChartSpecRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates a conditional format rule at the given index, or moves a conditional format rule to another index."]
    pub struct UpdateConditionalFormatRuleRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zero-based index of the rule that should be replaced or moved."]
        pub index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zero-based new index the rule should end up at."]
        pub new_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rule that should replace the rule at the given index."]
        pub rule: ::std::option::Option<::std::boxed::Box<ConditionalFormatRule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sheet of the rule to move. Required if new_index is set, unused otherwise."]
        pub sheet_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl UpdateConditionalFormatRuleRequest {
        pub fn builder() -> UpdateConditionalFormatRuleRequestBuilder {
            UpdateConditionalFormatRuleRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of updating a conditional format rule."]
    pub struct UpdateConditionalFormatRuleResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the new rule."]
        pub new_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new rule that replaced the old rule (if replacing), or the rule that was moved (if moved)"]
        pub new_rule: ::std::option::Option<::std::boxed::Box<ConditionalFormatRule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oldIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The old index of the rule. Not set if a rule was replaced (because it is the same as new_index)."]
        pub old_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oldRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The old (deleted) rule. Not set if a rule was moved (because it is the same as new_rule)."]
        pub old_rule: ::std::option::Option<::std::boxed::Box<ConditionalFormatRule>>,
    }
    impl UpdateConditionalFormatRuleResponse {
        pub fn builder() -> UpdateConditionalFormatRuleResponseBuilder {
            UpdateConditionalFormatRuleResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates a data source. After the data source is updated successfully, an execution is triggered to refresh the associated DATA_SOURCE sheet to read data from the updated data source. The request requires an additional `bigquery.readonly` OAuth scope."]
    pub struct UpdateDataSourceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data source to update."]
        pub data_source: ::std::option::Option<::std::boxed::Box<DataSource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `dataSource` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field."]
        pub fields: ::std::option::Option<::std::string::String>,
    }
    impl UpdateDataSourceRequest {
        pub fn builder() -> UpdateDataSourceRequestBuilder {
            UpdateDataSourceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response from updating data source."]
    pub struct UpdateDataSourceResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataExecutionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data execution status."]
        pub data_execution_status: ::std::option::Option<::std::boxed::Box<DataExecutionStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The updated data source."]
        pub data_source: ::std::option::Option<::std::boxed::Box<DataSource>>,
    }
    impl UpdateDataSourceResponse {
        pub fn builder() -> UpdateDataSourceResponseBuilder {
            UpdateDataSourceResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to update properties of developer metadata. Updates the properties of the developer metadata selected by the filters to the values provided in the DeveloperMetadata resource. Callers must specify the properties they wish to update in the fields parameter, as well as specify at least one DataFilter matching the metadata they wish to update."]
    pub struct UpdateDeveloperMetadataRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataFilters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filters matching the developer metadata entries to update."]
        pub data_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataFilter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "developerMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value that all metadata matched by the data filters will be updated to."]
        pub developer_metadata: ::std::option::Option<::std::boxed::Box<DeveloperMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `developerMetadata` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field."]
        pub fields: ::std::option::Option<::std::string::String>,
    }
    impl UpdateDeveloperMetadataRequest {
        pub fn builder() -> UpdateDeveloperMetadataRequestBuilder {
            UpdateDeveloperMetadataRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response from updating developer metadata."]
    pub struct UpdateDeveloperMetadataResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "developerMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The updated developer metadata."]
        pub developer_metadata:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeveloperMetadata>>>,
    }
    impl UpdateDeveloperMetadataResponse {
        pub fn builder() -> UpdateDeveloperMetadataResponseBuilder {
            UpdateDeveloperMetadataResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates the state of the specified group."]
    pub struct UpdateDimensionGroupRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The group whose state should be updated. The range and depth of the group should specify a valid group on the sheet, and all other fields updated."]
        pub dimension_group: ::std::option::Option<::std::boxed::Box<DimensionGroup>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `dimensionGroup` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field."]
        pub fields: ::std::option::Option<::std::string::String>,
    }
    impl UpdateDimensionGroupRequest {
        pub fn builder() -> UpdateDimensionGroupRequestBuilder {
            UpdateDimensionGroupRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates properties of dimensions within the specified range."]
    pub struct UpdateDimensionPropertiesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceSheetRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The columns on a data source sheet to update."]
        pub data_source_sheet_range:
            ::std::option::Option<::std::boxed::Box<DataSourceSheetDimensionRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `properties` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Properties to update."]
        pub properties: ::std::option::Option<::std::boxed::Box<DimensionProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rows or columns to update."]
        pub range: ::std::option::Option<::std::boxed::Box<DimensionRange>>,
    }
    impl UpdateDimensionPropertiesRequest {
        pub fn builder() -> UpdateDimensionPropertiesRequestBuilder {
            UpdateDimensionPropertiesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates an embedded object's border property."]
    pub struct UpdateEmbeddedObjectBorderRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "border")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The border that applies to the embedded object."]
        pub border: ::std::option::Option<::std::boxed::Box<EmbeddedObjectBorder>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `border` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the embedded object to update."]
        pub object_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl UpdateEmbeddedObjectBorderRequest {
        pub fn builder() -> UpdateEmbeddedObjectBorderRequestBuilder {
            UpdateEmbeddedObjectBorderRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Update an embedded object's position (such as a moving or resizing a chart or image)."]
    pub struct UpdateEmbeddedObjectPositionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields of OverlayPosition that should be updated when setting a new position. Used only if newPosition.overlayPosition is set, in which case at least one field must be specified. The root `newPosition.overlayPosition` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An explicit position to move the embedded object to. If newPosition.sheetId is set, a new sheet with that ID will be created. If newPosition.newSheet is set to true, a new sheet will be created with an ID that will be chosen for you."]
        pub new_position: ::std::option::Option<::std::boxed::Box<EmbeddedObjectPosition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the object to moved."]
        pub object_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl UpdateEmbeddedObjectPositionRequest {
        pub fn builder() -> UpdateEmbeddedObjectPositionRequestBuilder {
            UpdateEmbeddedObjectPositionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of updating an embedded object's position."]
    pub struct UpdateEmbeddedObjectPositionResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new position of the embedded object."]
        pub position: ::std::option::Option<::std::boxed::Box<EmbeddedObjectPosition>>,
    }
    impl UpdateEmbeddedObjectPositionResponse {
        pub fn builder() -> UpdateEmbeddedObjectPositionResponseBuilder {
            UpdateEmbeddedObjectPositionResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates properties of the filter view."]
    pub struct UpdateFilterViewRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `filter` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new properties of the filter view."]
        pub filter: ::std::option::Option<::std::boxed::Box<FilterView>>,
    }
    impl UpdateFilterViewRequest {
        pub fn builder() -> UpdateFilterViewRequestBuilder {
            UpdateFilterViewRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates properties of the named range with the specified namedRangeId."]
    pub struct UpdateNamedRangeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `namedRange` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The named range to update with the new properties."]
        pub named_range: ::std::option::Option<::std::boxed::Box<NamedRange>>,
    }
    impl UpdateNamedRangeRequest {
        pub fn builder() -> UpdateNamedRangeRequestBuilder {
            UpdateNamedRangeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates an existing protected range with the specified protectedRangeId."]
    pub struct UpdateProtectedRangeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `protectedRange` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protectedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The protected range to update with the new properties."]
        pub protected_range: ::std::option::Option<::std::boxed::Box<ProtectedRange>>,
    }
    impl UpdateProtectedRangeRequest {
        pub fn builder() -> UpdateProtectedRangeRequestBuilder {
            UpdateProtectedRangeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates properties of the sheet with the specified sheetId."]
    pub struct UpdateSheetPropertiesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `properties` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties to update."]
        pub properties: ::std::option::Option<::std::boxed::Box<SheetProperties>>,
    }
    impl UpdateSheetPropertiesRequest {
        pub fn builder() -> UpdateSheetPropertiesRequestBuilder {
            UpdateSheetPropertiesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates a slicer's specifications. (This does not move or resize a slicer. To move or resize a slicer use UpdateEmbeddedObjectPositionRequest."]
    pub struct UpdateSlicerSpecRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `SlicerSpec` is implied and should not be specified. A single \"*\"` can be used as short-hand for listing every field."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slicerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of the slicer to update."]
        pub slicer_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The specification to apply to the slicer."]
        pub spec: ::std::option::Option<::std::boxed::Box<SlicerSpec>>,
    }
    impl UpdateSlicerSpecRequest {
        pub fn builder() -> UpdateSlicerSpecRequestBuilder {
            UpdateSlicerSpecRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates properties of a spreadsheet."]
    pub struct UpdateSpreadsheetPropertiesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root 'properties' is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties to update."]
        pub properties: ::std::option::Option<::std::boxed::Box<SpreadsheetProperties>>,
    }
    impl UpdateSpreadsheetPropertiesRequest {
        pub fn builder() -> UpdateSpreadsheetPropertiesRequestBuilder {
            UpdateSpreadsheetPropertiesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response when updating a range of values by a data filter in a spreadsheet."]
    pub struct UpdateValuesByDataFilterResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data filter that selected the range that was updated."]
        pub data_filter: ::std::option::Option<::std::boxed::Box<DataFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatedCells")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of cells updated."]
        pub updated_cells: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatedColumns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of columns where at least one cell in the column was updated."]
        pub updated_columns: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatedData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values of the cells in the range matched by the dataFilter after all updates were applied. This is only included if the request's `includeValuesInResponse` field was `true`."]
        pub updated_data: ::std::option::Option<::std::boxed::Box<ValueRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range (in A1 notation) that updates were applied to."]
        pub updated_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatedRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of rows where at least one cell in the row was updated."]
        pub updated_rows: ::std::option::Option<::std::primitive::i64>,
    }
    impl UpdateValuesByDataFilterResponse {
        pub fn builder() -> UpdateValuesByDataFilterResponseBuilder {
            UpdateValuesByDataFilterResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response when updating a range of values in a spreadsheet."]
    pub struct UpdateValuesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spreadsheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The spreadsheet the updates were applied to."]
        pub spreadsheet_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatedCells")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of cells updated."]
        pub updated_cells: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatedColumns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of columns where at least one cell in the column was updated."]
        pub updated_columns: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatedData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values of the cells after updates were applied. This is only included if the request's `includeValuesInResponse` field was `true`."]
        pub updated_data: ::std::option::Option<::std::boxed::Box<ValueRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatedRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range (in A1 notation) that updates were applied to."]
        pub updated_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatedRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of rows where at least one cell in the row was updated."]
        pub updated_rows: ::std::option::Option<::std::primitive::i64>,
    }
    impl UpdateValuesResponse {
        pub fn builder() -> UpdateValuesResponseBuilder {
            UpdateValuesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data within a range of the spreadsheet."]
    pub struct ValueRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "majorDimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The major dimension of the values. For output, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then requesting `range=A1:B2,majorDimension=ROWS` will return `[[1,2],[3,4]]`, whereas requesting `range=A1:B2,majorDimension=COLUMNS` will return `[[1,3],[2,4]]`. For input, with `range=A1:B2,majorDimension=ROWS` then `[[1,2],[3,4]]` will set `A1=1,B1=2,A2=3,B2=4`. With `range=A1:B2,majorDimension=COLUMNS` then `[[1,2],[3,4]]` will set `A1=1,B1=3,A2=2,B2=4`. When writing, if this field is not set, it defaults to ROWS."]
        pub major_dimension: ::std::option::Option<ValueRangeMajorDimensionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range the values cover, in A1 notation. For output, this range indicates the entire requested range, even though the values will exclude trailing rows and columns. When appending values, this field represents the range to search for a table, after which values will be appended."]
        pub range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data that was read or to be written. This is an array of arrays, the outer array representing all the data and each inner array representing a major dimension. Each item in the inner array corresponds with one cell. For output, empty trailing rows and columns will not be included. For input, supported value types are: bool, string, and double. Null values will be skipped. To set a cell to an empty value, set the string value to an empty string."]
        pub values: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::serde_json::Value>>>,
    }
    impl ValueRange {
        pub fn builder() -> ValueRangeBuilder {
            ValueRangeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The major dimension of the values. For output, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then requesting `range=A1:B2,majorDimension=ROWS` will return `[[1,2],[3,4]]`, whereas requesting `range=A1:B2,majorDimension=COLUMNS` will return `[[1,3],[2,4]]`. For input, with `range=A1:B2,majorDimension=ROWS` then `[[1,2],[3,4]]` will set `A1=1,B1=2,A2=3,B2=4`. With `range=A1:B2,majorDimension=COLUMNS` then `[[1,2],[3,4]]` will set `A1=1,B1=3,A2=2,B2=4`. When writing, if this field is not set, it defaults to ROWS."]
    pub enum ValueRangeMajorDimensionEnum {
        #[serde(rename = "DIMENSION_UNSPECIFIED")]
        #[doc = "The default value, do not use."]
        DimensionUnspecified,
        #[serde(rename = "ROWS")]
        #[doc = "Operates on the rows of a sheet."]
        Rows,
        #[serde(rename = "COLUMNS")]
        #[doc = "Operates on the columns of a sheet."]
        Columns,
    }
    impl ::std::default::Default for ValueRangeMajorDimensionEnum {
        fn default() -> Self {
            Self::DimensionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Styles for a waterfall chart column."]
    pub struct WaterfallChartColumnStyle {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the column."]
        pub color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colorStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the column. If color is also set, this field takes precedence."]
        pub color_style: ::std::option::Option<::std::boxed::Box<ColorStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The label of the column's legend."]
        pub label: ::std::option::Option<::std::string::String>,
    }
    impl WaterfallChartColumnStyle {
        pub fn builder() -> WaterfallChartColumnStyleBuilder {
            WaterfallChartColumnStyleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A custom subtotal column for a waterfall chart series."]
    pub struct WaterfallChartCustomSubtotal {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataIsSubtotal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the data point at subtotal_index is the subtotal. If false, the subtotal will be computed and appear after the data point."]
        pub data_is_subtotal: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A label for the subtotal column."]
        pub label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtotalIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The 0-based index of a data point within the series. If data_is_subtotal is true, the data point at this index is the subtotal. Otherwise, the subtotal appears after the data point with this index. A series can have multiple subtotals at arbitrary indices, but subtotals do not affect the indices of the data points. For example, if a series has three data points, their indices will always be 0, 1, and 2, regardless of how many subtotals exist on the series or what data points they are associated with."]
        pub subtotal_index: ::std::option::Option<::std::primitive::i64>,
    }
    impl WaterfallChartCustomSubtotal {
        pub fn builder() -> WaterfallChartCustomSubtotalBuilder {
            WaterfallChartCustomSubtotalBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The domain of a waterfall chart."]
    pub struct WaterfallChartDomain {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data of the WaterfallChartDomain."]
        pub data: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reversed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True to reverse the order of the domain values (horizontal axis)."]
        pub reversed: ::std::option::Option<::std::primitive::bool>,
    }
    impl WaterfallChartDomain {
        pub fn builder() -> WaterfallChartDomainBuilder {
            WaterfallChartDomainBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single series of data for a waterfall chart."]
    pub struct WaterfallChartSeries {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customSubtotals")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom subtotal columns appearing in this series. The order in which subtotals are defined is not significant. Only one subtotal may be defined for each data point."]
        pub custom_subtotals:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WaterfallChartCustomSubtotal>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data being visualized in this series."]
        pub data: ::std::option::Option<::std::boxed::Box<ChartData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the data labels for this series."]
        pub data_label: ::std::option::Option<::std::boxed::Box<DataLabel>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hideTrailingSubtotal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True to hide the subtotal column from the end of the series. By default, a subtotal column will appear at the end of each series. Setting this field to true will hide that subtotal column for this series."]
        pub hide_trailing_subtotal: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "negativeColumnsStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Styles for all columns in this series with negative values."]
        pub negative_columns_style:
            ::std::option::Option<::std::boxed::Box<WaterfallChartColumnStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "positiveColumnsStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Styles for all columns in this series with positive values."]
        pub positive_columns_style:
            ::std::option::Option<::std::boxed::Box<WaterfallChartColumnStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtotalColumnsStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Styles for all subtotal columns in this series."]
        pub subtotal_columns_style:
            ::std::option::Option<::std::boxed::Box<WaterfallChartColumnStyle>>,
    }
    impl WaterfallChartSeries {
        pub fn builder() -> WaterfallChartSeriesBuilder {
            WaterfallChartSeriesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A waterfall chart."]
    pub struct WaterfallChartSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "connectorLineStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The line style for the connector lines."]
        pub connector_line_style: ::std::option::Option<::std::boxed::Box<LineStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain data (horizontal axis) for the waterfall chart."]
        pub domain: ::std::option::Option<::std::boxed::Box<WaterfallChartDomain>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstValueIsTotal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True to interpret the first value as a total."]
        pub first_value_is_total: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hideConnectorLines")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True to hide connector lines between columns."]
        pub hide_connector_lines: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "series")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data this waterfall chart is visualizing."]
        pub series: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WaterfallChartSeries>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stackedType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stacked type."]
        pub stacked_type: ::std::option::Option<WaterfallChartSpecStackedTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalDataLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Controls whether to display additional data labels on stacked charts which sum the total value of all stacked values at each value along the domain axis. stacked_type must be STACKED and neither CUSTOM nor placement can be set on the total_data_label."]
        pub total_data_label: ::std::option::Option<::std::boxed::Box<DataLabel>>,
    }
    impl WaterfallChartSpec {
        pub fn builder() -> WaterfallChartSpecBuilder {
            WaterfallChartSpecBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The stacked type."]
    pub enum WaterfallChartSpecStackedTypeEnum {
        #[serde(rename = "WATERFALL_STACKED_TYPE_UNSPECIFIED")]
        #[doc = "Default value, do not use."]
        WaterfallStackedTypeUnspecified,
        #[serde(rename = "STACKED")]
        #[doc = "Values corresponding to the same domain (horizontal axis) value will be stacked vertically."]
        Stacked,
        #[serde(rename = "SEQUENTIAL")]
        #[doc = "Series will spread out along the horizontal axis."]
        Sequential,
    }
    impl ::std::default::Default for WaterfallChartSpecStackedTypeEnum {
        fn default() -> Self {
            Self::WaterfallStackedTypeUnspecified
        }
    }
}
