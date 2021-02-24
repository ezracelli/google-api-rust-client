#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single book in the library."]
pub struct GoogleExampleLibraryagentV1Book {
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the book author."]
    pub author: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the book. Book names have the form `shelves/{shelf_id}/books/{book_id}`. The name is ignored when creating a book."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "read")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value indicating whether the book has been read."]
    pub read: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the book."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for LibraryAgent.ListBooks."]
pub struct GoogleExampleLibraryagentV1ListBooksResponse {
    #[serde(rename = "books")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of books."]
    pub books:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleExampleLibraryagentV1Book>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve next page of results. Pass this value in the ListBooksRequest.page_token field in the subsequent call to `ListBooks` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for LibraryAgent.ListShelves."]
pub struct GoogleExampleLibraryagentV1ListShelvesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve next page of results. Pass this value in the ListShelvesRequest.page_token field in the subsequent call to `ListShelves` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shelves")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of shelves."]
    pub shelves:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleExampleLibraryagentV1Shelf>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Shelf contains a collection of books with a theme."]
pub struct GoogleExampleLibraryagentV1Shelf {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the shelf. Shelf names have the form `shelves/{shelf_id}`. The name is ignored when creating a shelf."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "theme")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The theme of the shelf"]
    pub theme: ::std::option::Option<::std::string::String>,
}
