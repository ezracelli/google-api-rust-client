#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for TablesService.BatchCreateRows."]
pub struct BatchCreateRowsRequest {
    #[serde(rename = "requests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The request message specifying the rows to create. A maximum of 500 rows can be created in a single batch."]
    pub requests: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreateRowRequest>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for TablesService.BatchCreateRows."]
pub struct BatchCreateRowsResponse {
    #[serde(rename = "rows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The created rows."]
    pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for TablesService.BatchDeleteRows"]
pub struct BatchDeleteRowsRequest {
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The names of the rows to delete. All rows must belong to the parent table or else the entire batch will fail. A maximum of 500 rows can be deleted in a batch. Format: tables/{table}/rows/{row}"]
    pub names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for TablesService.BatchUpdateRows."]
pub struct BatchUpdateRowsRequest {
    #[serde(rename = "requests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The request messages specifying the rows to update. A maximum of 500 rows can be modified in a single batch."]
    pub requests: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UpdateRowRequest>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for TablesService.BatchUpdateRows."]
pub struct BatchUpdateRowsResponse {
    #[serde(rename = "rows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The updated rows."]
    pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details on a column in the table."]
pub struct ColumnDescription {
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Data type of the column Supported types are auto_id, boolean, boolean_list, creator, create_timestamp, date, dropdown, location, integer, integer_list, number, number_list, person, person_list, tags, check_list, text, text_list, update_timestamp, updater, relationship, file_attachment_list. These types directly map to the column types supported on Tables website."]
    pub data_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Internal id for a column."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Range of labeled values for the column. Some columns like tags and drop-downs limit the values to a set of possible values. We return the range of values in such cases to help clients implement better user data validation."]
    pub labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LabeledItem>>>,
    #[serde(rename = "lookupDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates that this is a lookup column whose value is derived from the relationship column specified in the details. Lookup columns can not be updated directly. To change the value you must update the associated relationship column."]
    pub lookup_details: ::std::option::Option<::std::boxed::Box<LookupDetails>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "column name"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "relationshipDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Additional details about a relationship column. Specified when data_type is relationship."]
    pub relationship_details: ::std::option::Option<::std::boxed::Box<RelationshipDetails>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for TablesService.CreateRow."]
pub struct CreateRowRequest {
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The parent table where this row will be created. Format: tables/{table}"]
    pub parent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "row")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The row to create."]
    pub row: ::std::option::Option<::std::boxed::Box<Row>>,
    #[serde(rename = "view")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Column key to use for values in the row. Defaults to user entered name."]
    pub view: ::std::option::Option<CreateRowRequestViewEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Column key to use for values in the row. Defaults to user entered name."]
pub enum CreateRowRequestViewEnum {
    #[serde(rename = "VIEW_UNSPECIFIED")]
    #[doc = "Defaults to user entered text."]
    ViewUnspecified,
    #[serde(rename = "COLUMN_ID_VIEW")]
    #[doc = "Uses internally generated column id to identify values."]
    ColumnIdView,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single item in a labeled column."]
pub struct LabeledItem {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Internal id associated with the item."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display string as entered by user."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for TablesService.ListRows."]
pub struct ListRowsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is empty, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The rows from the specified table."]
    pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for TablesService.ListTables."]
pub struct ListTablesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is empty, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of tables."]
    pub tables: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Table>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for TablesService.ListWorkspaces."]
pub struct ListWorkspacesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is empty, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "workspaces")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of workspaces."]
    pub workspaces: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Workspace>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details about a lookup column whose value comes from the associated relationship."]
pub struct LookupDetails {
    #[serde(rename = "relationshipColumn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the relationship column associated with the lookup."]
    pub relationship_column: ::std::option::Option<::std::string::String>,
    #[serde(rename = "relationshipColumnId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the relationship column."]
    pub relationship_column_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details about a relationship column."]
pub struct RelationshipDetails {
    #[serde(rename = "linkedTable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the table this relationship is linked to."]
    pub linked_table: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single row in a table."]
pub struct Row {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time when the row was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the row. Row names have the form `tables/{table}/rows/{row}`. The name is ignored when creating a row."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time when the row was last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The values of the row. This is a map of column key to value. Key is user entered name(default) or the internal column id based on the view in the request."]
    pub values: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single table."]
pub struct Table {
    #[serde(rename = "columns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of columns in this table. Order of columns matches the display order."]
    pub columns: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ColumnDescription>>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time when the table was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human readable title of the table."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the table. Table names have the form `tables/{table}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time when the table was last updated excluding updates to individual rows"]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for TablesService.UpdateRow."]
pub struct UpdateRowRequest {
    #[serde(rename = "row")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The row to update."]
    pub row: ::std::option::Option<::std::boxed::Box<Row>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of fields to update."]
    pub update_mask: ::std::option::Option<::std::string::String>,
    #[serde(rename = "view")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Column key to use for values in the row. Defaults to user entered name."]
    pub view: ::std::option::Option<UpdateRowRequestViewEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Column key to use for values in the row. Defaults to user entered name."]
pub enum UpdateRowRequestViewEnum {
    #[serde(rename = "VIEW_UNSPECIFIED")]
    #[doc = "Defaults to user entered text."]
    ViewUnspecified,
    #[serde(rename = "COLUMN_ID_VIEW")]
    #[doc = "Uses internally generated column id to identify values."]
    ColumnIdView,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single workspace."]
pub struct Workspace {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time when the workspace was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human readable title of the workspace."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the workspace. Workspace names have the form `workspaces/{workspace}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of tables in the workspace."]
    pub tables: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Table>>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time when the workspace was last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
