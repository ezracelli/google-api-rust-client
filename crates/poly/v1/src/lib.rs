#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents and describes an asset in the Poly library. An asset is a 3D model or scene created using [Tilt Brush](//www.tiltbrush.com), [Blocks](//vr.google.com/blocks/), or any 3D program that produces a file that can be upload to Poly."]
pub struct Asset {
    #[serde(rename = "authorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The author's publicly visible name. Use this name when giving credit to the author. For more information, see [Licensing](/poly/discover/licensing)."]
    pub author_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For published assets, the time when the asset was published. For unpublished assets, the time when the asset was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human-readable description, set by the asset's author."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human-readable name, set by the asset's author."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "formats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of Formats where each format describes one representation of the asset."]
    pub formats: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Format>>>,
    #[serde(rename = "isCurated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this asset has been curated by the Poly team."]
    pub is_curated: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "license")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The license under which the author has made the asset available for use, if any."]
    pub license: ::std::option::Option<AssetLicenseEnum>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Application-defined opaque metadata for this asset. This field is only returned when querying for the signed-in user's own assets, not for public assets. This string is limited to 1K chars. It is up to the creator of the asset to define the format for this string (for example, JSON)."]
    pub metadata: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier for the asset in the form: `assets/{ASSET_ID}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "presentationParams")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hints for displaying the asset. Note that these parameters are not immutable; the author of an asset may change them post-publication."]
    pub presentation_params: ::std::option::Option<::std::boxed::Box<PresentationParams>>,
    #[serde(rename = "remixInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The remix info for the asset."]
    pub remix_info: ::std::option::Option<::std::boxed::Box<RemixInfo>>,
    #[serde(rename = "thumbnail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The thumbnail image for the asset."]
    pub thumbnail: ::std::option::Option<::std::boxed::Box<File>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when the asset was last modified. For published assets, whose contents are immutable, the update time changes only when metadata properties, such as visibility, are updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The visibility of the asset and who can access it."]
    pub visibility: ::std::option::Option<AssetVisibilityEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The license under which the author has made the asset available for use, if any."]
pub enum AssetLicenseEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "Unknown license value."]
    Unknown,
    #[serde(rename = "CREATIVE_COMMONS_BY")]
    #[doc = "Creative Commons CC-BY 3.0. https://creativecommons.org/licenses/by/3.0/"]
    CreativeCommonsBy,
    #[serde(rename = "ALL_RIGHTS_RESERVED")]
    #[doc = "Unlicensed: All Rights Reserved by the author. Unlicensed assets are **not** returned by List Assets."]
    AllRightsReserved,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The visibility of the asset and who can access it."]
pub enum AssetVisibilityEnum {
    #[serde(rename = "VISIBILITY_UNSPECIFIED")]
    #[doc = "Unknown (and invalid) visibility."]
    VisibilityUnspecified,
    #[serde(rename = "PRIVATE")]
    #[doc = "Access to the asset and its underlying files and resources is restricted to the author. **Authentication:** You must supply an OAuth token that corresponds to the author's account."]
    Private,
    #[serde(rename = "UNLISTED")]
    #[doc = "Access to the asset and its underlying files and resources is available to anyone with the asset's name. Unlisted assets are **not** returned by List Assets."]
    Unlisted,
    #[serde(rename = "PUBLIC")]
    #[doc = "Access to the asset and its underlying files and resources is available to anyone."]
    Public,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message generated by the asset import process."]
pub struct AssetImportMessage {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The code associated with this message."]
    pub code: ::std::option::Option<AssetImportMessageCodeEnum>,
    #[serde(rename = "filePath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional file path. Only present for those error codes that specify it."]
    pub file_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imageError")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional image error. Only present for INVALID_IMAGE_FILE."]
    pub image_error: ::std::option::Option<::std::boxed::Box<ImageError>>,
    #[serde(rename = "objParseError")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional OBJ parse error. Only present for OBJ_PARSE_ERROR."]
    pub obj_parse_error: ::std::option::Option<::std::boxed::Box<ObjParseError>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The code associated with this message."]
pub enum AssetImportMessageCodeEnum {
    #[serde(rename = "CODE_UNSPECIFIED")]
    #[doc = "Unknown error code."]
    CodeUnspecified,
    #[serde(rename = "NO_IMPORTABLE_FILE")]
    #[doc = "The asset import did not include any file that we can import (i.e. an OBJ file)."]
    NoImportableFile,
    #[serde(rename = "EMPTY_MODEL")]
    #[doc = "When generating the preview for the import, no geometry was found."]
    EmptyModel,
    #[serde(rename = "OBJ_PARSE_ERROR")]
    #[doc = "A problem was encountered while parsing the OBJ file. The converter makes a 'best effort' attempt to continue when encountering such issues. In some cases the resulting preview model may still be acceptable. The details can be found in the parse error message."]
    ObjParseError,
    #[serde(rename = "EXPIRED")]
    #[doc = "The importer was not able to import the model before the expiration time."]
    Expired,
    #[serde(rename = "IMAGE_ERROR")]
    #[doc = "The importer encountered a problem reading an image file."]
    ImageError,
    #[serde(rename = "EXTRA_FILES_WITH_ARCHIVE")]
    #[doc = "Multiple files were encountered in addition to a ZIP archive. When uploading an archive only one file is permitted."]
    ExtraFilesWithArchive,
    #[serde(rename = "DEFAULT_MATERIALS")]
    #[doc = "Default materials are used in the model. This means that one or more faces is using default materials either because no usemtl statement was specified or because the requested material was not found due to a missing material file or bad material name. This does not cover the case of missing textures."]
    DefaultMaterials,
    #[serde(rename = "FATAL_ERROR")]
    #[doc = "The importer encountered a fatal error and was unable to import the model."]
    FatalError,
    #[serde(rename = "INVALID_ELEMENT_TYPE")]
    #[doc = "The import includes a file of an unsupported element type. The file path is specified."]
    InvalidElementType,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a file in Poly, which can be a root, resource, or thumbnail file."]
pub struct File {
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MIME content-type, such as `image/png`. For more information, see [MIME types](//developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)."]
    pub content_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "relativePath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The path of the resource file relative to the root file. For root or thumbnail files, this is just the filename."]
    pub relative_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL where the file data can be retrieved."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The same asset can be represented in different formats, for example, a [WaveFront .obj](//en.wikipedia.org/wiki/Wavefront_.obj_file) file with its corresponding .mtl file or a [Khronos glTF](//www.khronos.org/gltf) file with its corresponding .glb binary data. A format refers to a specific representation of an asset and contains all information needed to retrieve and describe this representation."]
pub struct Format {
    #[serde(rename = "formatComplexity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Complexity stats about this representation of the asset."]
    pub format_complexity: ::std::option::Option<::std::boxed::Box<FormatComplexity>>,
    #[serde(rename = "formatType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short string that identifies the format type of this representation. Possible values are: `FBX`, `GLTF`, `GLTF2`, `OBJ`, and `TILT`."]
    pub format_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of dependencies of the root element. May include, but is not limited to, materials, textures, and shader programs."]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<File>>>,
    #[serde(rename = "root")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The root of the file hierarchy. This will always be populated. For some format_types - such as `TILT`, which are self-contained - this is all of the data. Other types - such as `OBJ` - often reference other data elements. These are contained in the resources field."]
    pub root: ::std::option::Option<::std::boxed::Box<File>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information on the complexity of this Format."]
pub struct FormatComplexity {
    #[serde(rename = "lodHint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A non-negative integer that represents the level of detail (LOD) of this format relative to other formats of the same asset with the same format_type. This hint allows you to sort formats from the most-detailed (0) to least-detailed (integers greater than 0)."]
    pub lod_hint: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "triangleCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The estimated number of triangles."]
    pub triangle_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message resulting from reading an image file."]
pub struct ImageError {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of image error encountered. Optional for older image errors."]
    pub code: ::std::option::Option<ImageErrorCodeEnum>,
    #[serde(rename = "filePath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The file path in the import of the image that was rejected."]
    pub file_path: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of image error encountered. Optional for older image errors."]
pub enum ImageErrorCodeEnum {
    #[serde(rename = "CODE_UNSPECIFIED")]
    #[doc = "Unknown error code."]
    CodeUnspecified,
    #[serde(rename = "INVALID_IMAGE")]
    #[doc = "We were unable to read the image file."]
    InvalidImage,
    #[serde(rename = "IMAGE_TOO_BIG")]
    #[doc = "The image size is too large."]
    ImageTooBig,
    #[serde(rename = "WRONG_IMAGE_TYPE")]
    #[doc = "The image data does not match the expected MIME type of the image."]
    WrongImageType,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response message from a request to list."]
pub struct ListAssetsResponse {
    #[serde(rename = "assets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of assets that match the criteria specified in the request."]
    pub assets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Asset>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The continuation token for retrieving the next page. If empty, indicates that there are no more pages. To get the next page, submit the same request specifying this value as the page_token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of assets in the list, without pagination."]
    pub total_size: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response message from a request to list."]
pub struct ListLikedAssetsResponse {
    #[serde(rename = "assets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of assets that match the criteria specified in the request."]
    pub assets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Asset>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The continuation token for retrieving the next page. If empty, indicates that there are no more pages. To get the next page, submit the same request specifying this value as the page_token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of assets in the list, without pagination."]
    pub total_size: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response message from a request to list."]
pub struct ListUserAssetsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The continuation token for retrieving the next page. If empty, indicates that there are no more pages. To get the next page, submit the same request specifying this value as the page_token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of assets in the list, without pagination."]
    pub total_size: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "userAssets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of UserAssets matching the request."]
    pub user_assets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserAsset>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of an error resulting from parsing an OBJ file"]
pub struct ObjParseError {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of problem found (required)."]
    pub code: ::std::option::Option<ObjParseErrorCodeEnum>,
    #[serde(rename = "endIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ending character index at which the problem was found."]
    pub end_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "filePath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The file path in which the problem was found."]
    pub file_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "line")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text of the line. Note that this may be truncated if the line was very long. This may not include the error if it occurs after line truncation."]
    pub line: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lineNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Line number at which the problem was found."]
    pub line_number: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The starting character index at which the problem was found."]
    pub start_index: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of problem found (required)."]
pub enum ObjParseErrorCodeEnum {
    #[serde(rename = "CODE_UNSPECIFIED")]
    #[doc = "Unknown error code."]
    CodeUnspecified,
    #[serde(rename = "INCONSISTENT_VERTEX_REFS")]
    #[doc = "Vertex references are specified in an inconsistent style for a face (e.g. some vertices specify texture vertices but some don't)."]
    InconsistentVertexRefs,
    #[serde(rename = "INVALID_COMMAND")]
    #[doc = "The command is invalid."]
    InvalidCommand,
    #[serde(rename = "INVALID_NUMBER")]
    #[doc = "A invalid number was specified."]
    InvalidNumber,
    #[serde(rename = "INVALID_VERTEX_REF")]
    #[doc = "An invalid vertex reference was specified."]
    InvalidVertexRef,
    #[serde(rename = "MISSING_GEOMETRIC_VERTEX")]
    #[doc = "A vertex reference does not specify a geometric vertex."]
    MissingGeometricVertex,
    #[serde(rename = "MISSING_TOKEN")]
    #[doc = "An expected token was not found."]
    MissingToken,
    #[serde(rename = "TOO_FEW_DIMENSIONS")]
    #[doc = "The vertex specified too few dimensions for its usage."]
    TooFewDimensions,
    #[serde(rename = "TOO_FEW_VERTICES")]
    #[doc = "The face specified too few vertices."]
    TooFewVertices,
    #[serde(rename = "TOO_MANY_DIMENSIONS")]
    #[doc = "The vertex specified too many dimensions for its usage."]
    TooManyDimensions,
    #[serde(rename = "UNSUPPORTED_COMMAND")]
    #[doc = "This command is a valid OBJ command but is not supported. This error is only generated for the first instance of such a command."]
    UnsupportedCommand,
    #[serde(rename = "UNUSED_TOKENS")]
    #[doc = "This line ended with unparsed token characters."]
    UnusedTokens,
    #[serde(rename = "VERTEX_NOT_FOUND")]
    #[doc = "The specified vertex was not found."]
    VertexNotFound,
    #[serde(rename = "NUMBER_OUT_OF_RANGE")]
    #[doc = "The specified number was too large or small for its usage."]
    NumberOutOfRange,
    #[serde(rename = "INVALID_VALUE")]
    #[doc = "The specified parameter value was not recognized."]
    InvalidValue,
    #[serde(rename = "INVALID_TEXTURE_OPTION")]
    #[doc = "The specified texture option is not valid."]
    InvalidTextureOption,
    #[serde(rename = "TOO_MANY_PROBLEMS")]
    #[doc = "The maximum number of problems to report was reached. Parsing continues, but further problems will be ignored."]
    TooManyProblems,
    #[serde(rename = "MISSING_FILE_NAME")]
    #[doc = "An expected file name was not specified."]
    MissingFileName,
    #[serde(rename = "FILE_NOT_FOUND")]
    #[doc = "The specified file was not found in the import."]
    FileNotFound,
    #[serde(rename = "UNKNOWN_MATERIAL")]
    #[doc = "The specified material was not found in any material definition in the import."]
    UnknownMaterial,
    #[serde(rename = "NO_MATERIAL_DEFINED")]
    #[doc = "Material parameters were specified before the first material definition."]
    NoMaterialDefined,
    #[serde(rename = "INVALID_SMOOTHING_GROUP")]
    #[doc = "The smoothing group is not valid."]
    InvalidSmoothingGroup,
    #[serde(rename = "MISSING_VERTEX_COLORS")]
    #[doc = "Vertex colors were specified for only some vertices of a face."]
    MissingVertexColors,
    #[serde(rename = "FILE_SUBSTITUTION")]
    #[doc = "A missing file was found at a different file path."]
    FileSubstitution,
    #[serde(rename = "LINE_TOO_LONG")]
    #[doc = "A line in an OBJ or MTL file exceeded the maximum line length."]
    LineTooLong,
    #[serde(rename = "INVALID_FILE_PATH")]
    #[doc = "The file path was invalid. Only relative paths are supported."]
    InvalidFilePath,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Hints for displaying the asset, based on information available when the asset was uploaded."]
pub struct PresentationParams {
    #[serde(rename = "backgroundColor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A background color which could be used for displaying the 3D asset in a 'thumbnail' or 'palette' style view. Authors have the option to set this background color when publishing or editing their asset. This is represented as a six-digit hexademical triplet specifying the RGB components of the background color, e.g. #FF0000 for Red."]
    pub background_color: ::std::option::Option<::std::string::String>,
    #[serde(rename = "colorSpace")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The materials' diffuse/albedo color. This does not apply to vertex colors or texture maps."]
    pub color_space: ::std::option::Option<PresentationParamsColorSpaceEnum>,
    #[serde(rename = "orientingRotation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A rotation that should be applied to the object root to make it upright. More precisely, this quaternion transforms from \"object space\" (the space in which the object is defined) to \"presentation space\", a coordinate system where +Y is up, +X is right, -Z is forward. For example, if the object is the Eiffel Tower, in its local coordinate system the object might be laid out such that the base of the tower is on the YZ plane and the tip of the tower is towards positive X. In this case this quaternion would specify a rotation (of 90 degrees about the Z axis) such that in the presentation space the base of the tower is aligned with the XZ plane, and the tip of the tower lies towards +Y. This rotation is unrelated to the object's pose in the web preview, which is just a camera position setting and is *not* reflected in this rotation. Please note: this is applicable only to the gLTF."]
    pub orienting_rotation: ::std::option::Option<::std::boxed::Box<Quaternion>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The materials' diffuse/albedo color. This does not apply to vertex colors or texture maps."]
pub enum PresentationParamsColorSpaceEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "Invalid color value."]
    Unknown,
    #[serde(rename = "LINEAR")]
    #[doc = "Linear color values. Default."]
    Linear,
    #[serde(rename = "GAMMA")]
    #[doc = "Colors should be converted to linear by assuming gamma = 2.0."]
    Gamma,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A [Quaternion](//en.wikipedia.org/wiki/Quaternion). Please note: if in the response you see \"w: 1\" and nothing else this is the default value of [0, 0, 0, 1] where x,y, and z are 0."]
pub struct Quaternion {
    #[serde(rename = "w")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The scalar component."]
    pub w: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The x component."]
    pub x: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "y")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The y component."]
    pub y: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "z")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The z component."]
    pub z: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Info about the sources of this asset (i.e. assets that were remixed to create this asset)."]
pub struct RemixInfo {
    #[serde(rename = "sourceAsset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource ids for the sources of this remix, of the form: `assets/{ASSET_ID}`"]
    pub source_asset: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response message from a request to startImport. This is returned in the response field of the Operation."]
pub struct StartAssetImportResponse {
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of newly created asset. If this is empty when the operation is complete it means the import failed. Please refer to the assetImportMessages field to understand what went wrong."]
    pub asset_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "assetImportId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the asset import."]
    pub asset_import_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "assetImportMessages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The message from the asset import. This will contain any warnings (or - in the case of failure - errors) that occurred during import."]
    pub asset_import_messages:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssetImportMessage>>>,
    #[serde(rename = "publishUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The publish URL for the asset."]
    pub publish_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Data about the user's asset."]
pub struct UserAsset {
    #[serde(rename = "asset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An Asset."]
    pub asset: ::std::option::Option<::std::boxed::Box<Asset>>,
}
