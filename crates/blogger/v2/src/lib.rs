#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Blog {
    #[serde(rename = "customMetaData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The JSON custom meta-data for the Blog."]
    pub custom_meta_data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of this blog. This is displayed underneath the title."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier for this resource."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this entry. Always blogger#blog."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The locale this Blog is set to."]
    pub locale: ::std::option::Option<BlogLocale>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this blog. This is displayed as the title."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The container of pages in this blog."]
    pub pages: ::std::option::Option<BlogPages>,
    #[serde(rename = "posts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The container of posts in this blog."]
    pub posts: ::std::option::Option<BlogPosts>,
    #[serde(rename = "published")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "RFC 3339 date-time when this blog was published."]
    pub published: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API REST URL to fetch this resource from."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the blog."]
    pub status: ::std::option::Option<BlogStatusEnum>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "RFC 3339 date-time when this blog was last updated."]
    pub updated: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL where this blog is published."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The locale this Blog is set to."]
pub struct BlogLocale {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The country this blog's locale is set to."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language this blog is authored in."]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "variant")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language variant this blog is authored in."]
    pub variant: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The container of pages in this blog."]
pub struct BlogPages {
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the container for pages in this blog."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The count of pages in this blog."]
    pub total_items: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The container of posts in this blog."]
pub struct BlogPosts {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The List of Posts for this Blog."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Post>>>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the container for posts in this blog."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The count of posts in this blog."]
    pub total_items: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The status of the blog."]
pub enum BlogStatusEnum {
    #[serde(rename = "LIVE")]
    #[doc = ""]
    Live,
    #[serde(rename = "DELETED")]
    #[doc = ""]
    Deleted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BlogList {
    #[serde(rename = "blogUserInfos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Admin level list of blog per-user information."]
    pub blog_user_infos: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BlogUserInfo>>>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of Blogs this user has Authorship or Admin rights over."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Blog>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this entity. Always blogger#blogList."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BlogPerUserInfo {
    #[serde(rename = "blogId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the Blog resource."]
    pub blog_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hasAdminAccess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if the user has Admin level access to the blog."]
    pub has_admin_access: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this entity. Always blogger#blogPerUserInfo."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "photosAlbumKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Photo Album Key for the user when adding photos to the blog."]
    pub photos_album_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Access permissions that the user has for the blog (ADMIN, AUTHOR, or READER)."]
    pub role: ::std::option::Option<BlogPerUserInfoRoleEnum>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the User."]
    pub user_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Access permissions that the user has for the blog (ADMIN, AUTHOR, or READER)."]
pub enum BlogPerUserInfoRoleEnum {
    #[serde(rename = "VIEW_TYPE_UNSPECIFIED")]
    #[doc = ""]
    ViewTypeUnspecified,
    #[serde(rename = "READER")]
    #[doc = ""]
    Reader,
    #[serde(rename = "AUTHOR")]
    #[doc = ""]
    Author,
    #[serde(rename = "ADMIN")]
    #[doc = ""]
    Admin,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BlogUserInfo {
    #[serde(rename = "blog")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Blog resource."]
    pub blog: ::std::option::Option<::std::boxed::Box<Blog>>,
    #[serde(rename = "blog_user_info")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about a User for the Blog."]
    pub blog_user_info: ::std::option::Option<::std::boxed::Box<BlogPerUserInfo>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this entity. Always blogger#blogUserInfo."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Comment {
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The author of this Comment."]
    pub author: ::std::option::Option<CommentAuthor>,
    #[serde(rename = "blog")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Data about the blog containing this comment."]
    pub blog: ::std::option::Option<CommentBlog>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The actual content of the comment. May include HTML markup."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier for this resource."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inReplyTo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Data about the comment this is in reply to."]
    pub in_reply_to: ::std::option::Option<CommentInReplyTo>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this entry. Always blogger#comment."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "post")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Data about the post containing this comment."]
    pub post: ::std::option::Option<CommentPost>,
    #[serde(rename = "published")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "RFC 3339 date-time when this comment was published."]
    pub published: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API REST URL to fetch this resource from."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the comment (only populated for admin users)."]
    pub status: ::std::option::Option<CommentStatusEnum>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "RFC 3339 date-time when this comment was last updated."]
    pub updated: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The author of this Comment."]
pub struct CommentAuthor {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of the creator."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creator's avatar."]
    pub image: ::std::option::Option<CommentAuthorImage>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the creator's Profile page."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The creator's avatar."]
pub struct CommentAuthorImage {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creator's avatar URL."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Data about the blog containing this comment."]
pub struct CommentBlog {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of the blog containing this comment."]
    pub id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Data about the comment this is in reply to."]
pub struct CommentInReplyTo {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identified of the parent of this comment."]
    pub id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Data about the post containing this comment."]
pub struct CommentPost {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of the post containing this comment."]
    pub id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The status of the comment (only populated for admin users)."]
pub enum CommentStatusEnum {
    #[serde(rename = "LIVE")]
    #[doc = ""]
    Live,
    #[serde(rename = "EMPTIED")]
    #[doc = ""]
    Emptied,
    #[serde(rename = "PENDING")]
    #[doc = ""]
    Pending,
    #[serde(rename = "SPAM")]
    #[doc = ""]
    Spam,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommentList {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Etag of the response."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The List of Comments for a Post."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Comment>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this entry. Always blogger#commentList."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to fetch the next page, if one exists."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "prevPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to fetch the previous page, if one exists."]
    pub prev_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Page {
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The author of this Page."]
    pub author: ::std::option::Option<PageAuthor>,
    #[serde(rename = "blog")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Data about the blog containing this Page."]
    pub blog: ::std::option::Option<PageBlog>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The body content of this Page, in HTML."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Etag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier for this resource."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this entity. Always blogger#page."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "published")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "RFC 3339 date-time when this Page was published."]
    pub published: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API REST URL to fetch this resource from."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the page for admin resources (either LIVE or DRAFT)."]
    pub status: ::std::option::Option<PageStatusEnum>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of this entity. This is the name displayed in the Admin user interface."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "RFC 3339 date-time when this Page was last updated."]
    pub updated: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL that this Page is displayed at."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The author of this Page."]
pub struct PageAuthor {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of the creator."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creator's avatar."]
    pub image: ::std::option::Option<PageAuthorImage>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the creator's Profile page."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The creator's avatar."]
pub struct PageAuthorImage {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creator's avatar URL."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Data about the blog containing this Page."]
pub struct PageBlog {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of the blog containing this page."]
    pub id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The status of the page for admin resources (either LIVE or DRAFT)."]
pub enum PageStatusEnum {
    #[serde(rename = "LIVE")]
    #[doc = ""]
    Live,
    #[serde(rename = "DRAFT")]
    #[doc = ""]
    Draft,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PageList {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Etag of the response."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of Pages for a Blog."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Page>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this entity. Always blogger#pageList."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to fetch the next page, if one exists."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Post {
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The author of this Post."]
    pub author: ::std::option::Option<PostAuthor>,
    #[serde(rename = "blog")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Data about the blog containing this Post."]
    pub blog: ::std::option::Option<PostBlog>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content of the Post. May contain HTML markup."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customMetaData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The JSON meta-data for the Post."]
    pub custom_meta_data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Etag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of this Post."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "images")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display image for the Post."]
    pub images: ::std::option::Option<::std::vec::Vec<PostImages>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this entity. Always blogger#post."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of labels this Post was tagged with."]
    pub labels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location for geotagged posts."]
    pub location: ::std::option::Option<PostLocation>,
    #[serde(rename = "published")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "RFC 3339 date-time when this Post was published."]
    pub published: ::std::option::Option<::std::string::String>,
    #[serde(rename = "readerComments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Comment control and display setting for readers of this post."]
    pub reader_comments: ::std::option::Option<PostReaderCommentsEnum>,
    #[serde(rename = "replies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The container of comments on this Post."]
    pub replies: ::std::option::Option<PostReplies>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API REST URL to fetch this resource from."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the post. Only set for admin-level requests."]
    pub status: ::std::option::Option<PostStatusEnum>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the Post."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "titleLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title link URL, similar to atom's related link."]
    pub title_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "RFC 3339 date-time when this Post was last updated."]
    pub updated: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL where this Post is displayed."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The author of this Post."]
pub struct PostAuthor {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of the creator."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creator's avatar."]
    pub image: ::std::option::Option<PostAuthorImage>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the creator's Profile page."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The creator's avatar."]
pub struct PostAuthorImage {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creator's avatar URL."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Data about the blog containing this Post."]
pub struct PostBlog {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of the Blog that contains this Post."]
    pub id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PostImages {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The location for geotagged posts."]
pub struct PostLocation {
    #[serde(rename = "lat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location's latitude."]
    pub lat: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "lng")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location's longitude."]
    pub lng: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "span")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location's viewport span. Can be used when rendering a map preview."]
    pub span: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Comment control and display setting for readers of this post."]
pub enum PostReaderCommentsEnum {
    #[serde(rename = "ALLOW")]
    #[doc = ""]
    Allow,
    #[serde(rename = "DONT_ALLOW_SHOW_EXISTING")]
    #[doc = ""]
    DontAllowShowExisting,
    #[serde(rename = "DONT_ALLOW_HIDE_EXISTING")]
    #[doc = ""]
    DontAllowHideExisting,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The container of comments on this Post."]
pub struct PostReplies {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The List of Comments for this Post."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Comment>>>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the comments on this post."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The count of comments on this post."]
    pub total_items: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Status of the post. Only set for admin-level requests."]
pub enum PostStatusEnum {
    #[serde(rename = "LIVE")]
    #[doc = ""]
    Live,
    #[serde(rename = "DRAFT")]
    #[doc = ""]
    Draft,
    #[serde(rename = "SCHEDULED")]
    #[doc = ""]
    Scheduled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PostList {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Etag of the response."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of Posts for this Blog."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Post>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this entity. Always blogger#postList."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to fetch the next page, if one exists."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "prevPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to fetch the previous page, if one exists."]
    pub prev_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct User {
    #[serde(rename = "about")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Profile summary information."]
    pub about: ::std::option::Option<::std::string::String>,
    #[serde(rename = "blogs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The container of blogs for this user."]
    pub blogs: ::std::option::Option<UserBlogs>,
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp of when this profile was created, in seconds since epoch."]
    pub created: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier for this User."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this entity. Always blogger#user."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This user's locale"]
    pub locale: ::std::option::Option<UserLocale>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API REST URL to fetch this resource from."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's profile page."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The container of blogs for this user."]
pub struct UserBlogs {
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the Blogs for this user."]
    pub self_link: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This user's locale"]
pub struct UserLocale {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The country this blog's locale is set to."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language this blog is authored in."]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "variant")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language variant this blog is authored in."]
    pub variant: ::std::option::Option<::std::string::String>,
}
